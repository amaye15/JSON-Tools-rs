//! JSON unflattening engine using tape-based streaming.
//!
//! Reconstructs nested JSON structures from flat key-value maps using the same
//! tape scanner as flatten. Values remain as zero-copy byte ranges into the
//! original input via `ValueRef`, avoiding `serde_json::Value` allocation.
//!
//! Pipeline: `scan_and_fixup → extract entries → build UnflatNode tree → serialize directly`

use std::borrow::Cow;

use memchr::{memchr, memmem};
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::config::{FilteringConfig, ProcessingConfig};
use crate::convert::try_convert_string_to_json_bytes;
use crate::error::JsonToolsError;
use crate::flatten::{
    apply_value_replacement_cow, escape_json_string, scan_and_fixup, skip_tape_value,
    tape_content_str, tape_entry, tape_quoted_str, tape_scalar_bytes, unescape_json_string,
    write_json_escaped_key, EntryKind, TapeEntry, ValueRef,
};
use crate::json_parser;
use crate::transform::{apply_key_replacement_patterns, apply_value_replacement_patterns};

// ================================================================================================
// UnflatNode — Lightweight Tree with Zero-Copy Leaves
// ================================================================================================

/// Lightweight tree node for unflattened JSON. Leaf values stay as zero-copy
/// byte ranges from the original input via `ValueRef`.
enum UnflatNode<'a> {
    /// Leaf value — raw bytes from input or owned transformed value
    Leaf(ValueRef<'a>),
    /// Null placeholder for array gaps
    Null,
    /// Object node — keys sorted at serialization time for deterministic output
    Object(FxHashMap<String, UnflatNode<'a>>),
    /// Array with indexed elements
    Array(Vec<UnflatNode<'a>>),
}

// ================================================================================================
// Core Entry Point
// ================================================================================================

/// Core unflattening logic for a single JSON string.
/// Entry point called by `builder.rs`.
#[inline]
pub(crate) fn process_single_json_for_unflatten(
    json: &str,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    let input = json.as_bytes();

    // Reject inputs exceeding u32 addressable range (4 GiB)
    if input.len() > u32::MAX as usize {
        return Err(JsonToolsError::input_validation_error(
            "Input exceeds 4 GiB limit",
        ));
    }

    // Skip leading whitespace
    let start = skip_whitespace(input, 0);
    if start >= input.len() {
        return Ok("{}".to_string());
    }

    let first = unsafe { *input.get_unchecked(start) };

    // Handle root-level primitives (not objects or arrays)
    if first != b'{' && first != b'[' {
        let mut value = json_parser::parse_json(json)?;
        if !config.replacements.value_replacements.is_empty() {
            apply_value_replacement_patterns(&mut value, &config.replacements.value_replacements)?;
        }
        return json_parser::to_string(&value).map_err(JsonToolsError::serialization_error);
    }

    // Handle root arrays — not valid flattened JSON
    if first == b'[' {
        return Ok("{}".to_string());
    }

    // Handle empty object: {}
    let after_open = skip_whitespace(input, start + 1);
    if after_open < input.len() {
        let close = unsafe { *input.get_unchecked(after_open) };
        if close == b'}' {
            return Ok("{}".to_string());
        }
    }

    // Phase 1: Single-pass tape scan
    let tape = scan_and_fixup(input)?;

    // Phase 2: Extract flat entries with inline transforms
    let mut entries = extract_flat_entries(input, &tape, config)?;

    // Phase 3: Collision handling
    if config.collision.has_collision_handling() || has_duplicate_keys(&entries) {
        entries = handle_entry_collisions(entries, config.collision.has_collision_handling());
    }

    // Phase 4: Build UnflatNode tree
    let tree = build_unflatten_tree(entries, &config.separator, config.max_array_index)?;

    // Phase 5: Serialize with integrated filtering
    Ok(serialize_unflatten_tree(&tree, &config.filtering))
}

// ================================================================================================
// Entry Extraction from Tape
// ================================================================================================

/// Walk the top-level tape object and extract key-value pairs with inline transforms.
fn extract_flat_entries<'a>(
    input: &'a [u8],
    tape: &[TapeEntry],
    config: &ProcessingConfig,
) -> Result<Vec<(String, ValueRef<'a>)>, JsonToolsError> {
    if tape.is_empty() {
        return Ok(Vec::new());
    }

    // Root must be ObjectStart
    if tape[0].kind() != EntryKind::ObjectStart {
        return Err(JsonToolsError::invalid_json_structure(
            "Expected object for unflattening",
        ));
    }

    let end_idx = tape[0].aux() as usize;
    let mut entries = Vec::with_capacity(end_idx / 4); // heuristic
    let mut cursor = 1; // skip root ObjectStart

    while cursor < end_idx {
        let entry = tape_entry(tape, cursor);

        if entry.kind() != EntryKind::StringStart {
            cursor += 1;
            continue;
        }

        // Extract key
        let key_str = tape_content_str(input, entry);

        let mut key = if entry.string_has_escapes() {
            unescape_json_string(key_str).into_owned()
        } else {
            key_str.to_string()
        };

        // Apply lowercase
        if config.lowercase_keys {
            key.make_ascii_lowercase();
        }

        // Apply key replacements
        if config.replacements.has_key_replacements() {
            if let Ok(Some(new_key)) =
                apply_key_replacement_patterns(&key, &config.replacements.key_replacements)
            {
                key = new_key;
            }
        }

        // Skip key + colon
        cursor += 1;
        if cursor < end_idx && tape[cursor].kind() == EntryKind::Colon {
            cursor += 1;
        }

        // Extract value
        if cursor >= end_idx {
            break;
        }

        let val_entry = tape_entry(tape, cursor);
        let value = extract_value(input, tape, val_entry, config);
        cursor = advance_past_value(tape, cursor);

        entries.push((key, value));
    }

    Ok(entries)
}

/// Extract a ValueRef from a tape entry, applying value transforms inline.
#[inline]
fn extract_value<'a>(
    input: &'a [u8],
    tape: &[TapeEntry],
    entry: TapeEntry,
    config: &ProcessingConfig,
) -> ValueRef<'a> {
    match entry.kind() {
        EntryKind::StringStart => {
            let content_str = tape_content_str(input, entry);

            // Type conversion: "123" → 123, "true" → true
            if config.auto_convert_types {
                let unescaped = if entry.string_has_escapes() {
                    unescape_json_string(content_str)
                } else {
                    Cow::Borrowed(content_str)
                };

                if let Some(converted) = try_convert_string_to_json_bytes(unescaped.as_ref()) {
                    return ValueRef::Owned(converted.into_owned());
                }
            }

            // Value replacements
            if config.replacements.has_value_replacements() {
                let unescaped = if entry.string_has_escapes() {
                    unescape_json_string(content_str)
                } else {
                    Cow::Borrowed(content_str)
                };

                if let Some(replaced) = apply_value_replacement_cow(
                    unescaped.as_ref(),
                    &config.replacements.value_replacements,
                ) {
                    let escaped = escape_json_string(&replaced);
                    return ValueRef::Owned(format!("\"{}\"", escaped));
                }
            }

            // Zero-copy: raw bytes including quotes
            ValueRef::Raw(tape_quoted_str(input, entry).as_bytes())
        }
        EntryKind::ScalarStart => {
            let raw = tape_scalar_bytes(input, entry);
            let trimmed = crate::flatten::trim_ascii(raw);
            ValueRef::Raw(trimmed)
        }
        EntryKind::ObjectStart | EntryKind::ArrayStart => {
            // Nested container value — copy full byte range from input
            let start_offset = entry.offset();
            let end_tape_idx = entry.aux() as usize;
            let end_entry = tape[end_tape_idx];
            let end_offset = end_entry.offset() + 1; // include closing bracket
            debug_assert!(end_offset <= input.len());
            let raw = unsafe { input.get_unchecked(start_offset..end_offset) };
            ValueRef::Raw(raw)
        }
        _ => ValueRef::Raw(b"null"),
    }
}

/// Advance cursor past a value in the tape.
#[inline(always)]
fn advance_past_value(tape: &[TapeEntry], idx: usize) -> usize {
    skip_tape_value(tape, idx)
}

// ================================================================================================
// Collision Handling
// ================================================================================================

/// Check if any duplicate keys exist (fast path to skip collision handling).
#[inline]
fn has_duplicate_keys(entries: &[(String, ValueRef<'_>)]) -> bool {
    if entries.len() <= 1 {
        return false;
    }
    let mut seen: FxHashMap<&str, ()> =
        FxHashMap::with_capacity_and_hasher(entries.len(), Default::default());
    for (key, _) in entries {
        if seen.insert(key.as_str(), ()).is_some() {
            return true;
        }
    }
    false
}

/// Handle duplicate keys: merge into arrays (if enabled) or last-wins.
fn handle_entry_collisions<'a>(
    entries: Vec<(String, ValueRef<'a>)>,
    merge_collisions: bool,
) -> Vec<(String, ValueRef<'a>)> {
    let n = entries.len();

    // First pass: build index map using borrowed keys (avoids cloning every key)
    let mut key_indices: FxHashMap<&str, SmallVec<[usize; 1]>> =
        FxHashMap::with_capacity_and_hasher(n, Default::default());
    let mut ordered_keys: Vec<usize> = Vec::with_capacity(n);

    for (i, (key, _)) in entries.iter().enumerate() {
        key_indices
            .entry(key.as_str())
            .and_modify(|v| v.push(i))
            .or_insert_with(|| {
                ordered_keys.push(i);
                SmallVec::from_elem(i, 1)
            });
    }

    // Fast path: no collisions
    if ordered_keys.len() == entries.len() {
        return entries;
    }

    // Single pass: iterate ordered_keys, consume from key_indices, build result directly.
    // Uses a consumed bitset instead of Vec<Option<T>> to avoid wrapping overhead.
    let mut consumed = vec![false; n];
    let mut result = Vec::with_capacity(ordered_keys.len());

    for &first_idx in &ordered_keys {
        let key = entries[first_idx].0.as_str();
        let indices = key_indices.remove(key).unwrap_or_default();

        if indices.len() == 1 {
            consumed[first_idx] = true;
            // Deferred: entry will be moved after loop
        } else if merge_collisions {
            // Merge values into a JSON array
            let estimated_len: usize = indices
                .iter()
                .map(|&idx| {
                    let (_, ref v) = entries[idx];
                    (match v {
                        ValueRef::Raw(b) => b.len(),
                        ValueRef::Owned(s) => s.len(),
                    }) + 1 // comma
                })
                .sum::<usize>()
                + 2; // brackets
            let mut array_json = String::with_capacity(estimated_len);
            array_json.push('[');
            for (j, &idx) in indices.iter().enumerate() {
                if j > 0 {
                    array_json.push(',');
                }
                let (_, ref value) = entries[idx];
                match value {
                    ValueRef::Raw(bytes) => {
                        array_json.push_str(unsafe { std::str::from_utf8_unchecked(bytes) });
                    }
                    ValueRef::Owned(s) => {
                        array_json.push_str(s);
                    }
                }
                consumed[idx] = true;
            }
            array_json.push(']');
            // Temporarily push with empty key; will fix below
            result.push((first_idx, Some(ValueRef::Owned(array_json))));
            continue;
        } else {
            // Last wins
            let last_idx = *indices
                .last()
                .expect("collision indices non-empty: at least one index per key");
            for &idx in &indices {
                consumed[idx] = true;
            }
            result.push((last_idx, None)); // None means use value from entries[last_idx]
            continue;
        }
        result.push((first_idx, None));
    }

    // Now move entries out (single drain, avoids per-element Option wrapping)
    let mut entries = entries;
    result
        .into_iter()
        .map(|(idx, override_value)| {
            let (key, original_value) = std::mem::replace(
                &mut entries[idx],
                (String::new(), ValueRef::Raw(b"null")),
            );
            let value = override_value.unwrap_or(original_value);
            (key, value)
        })
        .collect()
}

// ================================================================================================
// Path Analysis
// ================================================================================================

/// Pre-analyze all flattened keys to build a path-type map for array vs object disambiguation.
fn analyze_path_types(
    entries: &[(String, ValueRef<'_>)],
    separator: &str,
) -> FxHashMap<String, bool> {
    let estimated_paths = entries.len() * 2;
    let mut state: FxHashMap<String, u8> =
        FxHashMap::with_capacity_and_hasher(estimated_paths, Default::default());

    let sep_bytes = separator.as_bytes();
    let sep_len = separator.len();

    for (key, _) in entries {
        analyze_key_path(key, sep_bytes, sep_len, &mut state);
    }

    state
        .into_iter()
        .map(|(k, mask)| {
            let is_array = (mask & 0b10 == 0) && (mask & 0b01 != 0);
            (k, is_array)
        })
        .collect()
}

/// Analyze a single key's path segments and record child-type info per parent prefix.
///
/// Bitmask per parent: bit 0 (0b01) = has numeric child, bit 1 (0b10) = has non-numeric child.
/// A parent with only numeric children (mask == 0b01) is treated as an array;
/// mixed or non-numeric only (mask & 0b10 != 0) is treated as an object.
#[inline]
fn analyze_key_path(
    key: &str,
    sep_bytes: &[u8],
    sep_len: usize,
    state: &mut FxHashMap<String, u8>,
) {
    let key_bytes = key.as_bytes();
    let mut search_start = 0;

    while search_start < key_bytes.len() {
        let next_sep = match find_separator(key_bytes, sep_bytes, search_start) {
            Some(pos) => pos,
            None => break,
        };

        let parent = &key[..next_sep];

        let child_start = next_sep + sep_len;
        if child_start < key_bytes.len() {
            let child_end =
                find_separator(key_bytes, sep_bytes, child_start).unwrap_or(key_bytes.len());
            let child = &key[child_start..child_end];

            let bit: u8 = if is_valid_array_index(child) {
                0b01
            } else {
                0b10
            };
            // Avoid to_string() allocation when key already exists (common for shared parent paths)
            if let Some(existing) = state.get_mut(parent) {
                *existing |= bit;
            } else {
                state.insert(parent.to_string(), bit);
            }
        }

        search_start = next_sep + sep_len;
    }
}

// ================================================================================================
// SIMD-Optimized Separator Finding
// ================================================================================================

/// SIMD-optimized separator finding using memchr crate
#[inline]
pub(crate) fn find_separator(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.len() == 1 {
        memchr(needle[0], &haystack[start..]).map(|pos| start + pos)
    } else {
        memmem::find(&haystack[start..], needle).map(|pos| start + pos)
    }
}

// ================================================================================================
// Array Index Validation
// ================================================================================================

/// Optimized check for valid array index
#[inline(always)]
fn is_valid_array_index(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if s.len() == 1 {
        return s.as_bytes()[0].is_ascii_digit();
    }

    if s.starts_with('0') {
        return s == "0";
    }

    s.bytes().all(|b| b.is_ascii_digit())
}

// ================================================================================================
// Build UnflatNode Tree
// ================================================================================================

/// Build an UnflatNode tree from extracted flat entries.
fn build_unflatten_tree<'a>(
    entries: Vec<(String, ValueRef<'a>)>,
    separator: &str,
    max_array_index: usize,
) -> Result<UnflatNode<'a>, JsonToolsError> {
    if entries.is_empty() {
        return Ok(UnflatNode::Object(FxHashMap::default()));
    }

    let path_types = analyze_path_types(&entries, separator);
    let mut root = FxHashMap::default();

    for (key, value) in entries {
        set_nested_value(
            &mut root,
            &key,
            value,
            separator,
            &path_types,
            max_array_index,
        )?;
    }

    Ok(UnflatNode::Object(root))
}

/// Entry point for recursively setting a value at a nested path.
fn set_nested_value<'a>(
    result: &mut FxHashMap<String, UnflatNode<'a>>,
    key_path: &str,
    value: ValueRef<'a>,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    max_array_index: usize,
) -> Result<(), JsonToolsError> {
    type PathSegments<'b> = SmallVec<[&'b str; 16]>;
    let parts: PathSegments = key_path.split(separator).collect();

    if parts.is_empty() {
        return Err(JsonToolsError::invalid_json_structure("Empty key path"));
    }

    if parts.len() == 1 {
        result.insert(parts[0].to_string(), UnflatNode::Leaf(value));
        return Ok(());
    }

    let mut path_buffer = String::with_capacity(key_path.len());
    set_nested_value_recursive(
        result,
        &parts,
        0,
        value,
        separator,
        path_types,
        &mut path_buffer,
        max_array_index,
    )
}

/// Recursive helper that reuses a path buffer to avoid allocations.
#[allow(clippy::too_many_arguments)]
fn set_nested_value_recursive<'a>(
    current: &mut FxHashMap<String, UnflatNode<'a>>,
    parts: &[&str],
    index: usize,
    value: ValueRef<'a>,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
    max_array_index: usize,
) -> Result<(), JsonToolsError> {
    let part = parts[index];

    if index == parts.len() - 1 {
        current.insert(part.to_string(), UnflatNode::Leaf(value));
        return Ok(());
    }

    let buffer_start_len = path_buffer.len();
    if buffer_start_len > 0 {
        path_buffer.push_str(separator);
    }
    path_buffer.push_str(part);

    let should_be_array = path_types
        .get(path_buffer.as_str())
        .copied()
        .unwrap_or(false);

    // Avoid to_string() allocation when key already exists (common for shared path prefixes)
    if !current.contains_key(part) {
        let node = if should_be_array {
            UnflatNode::Array(vec![])
        } else {
            UnflatNode::Object(FxHashMap::default())
        };
        current.insert(part.to_string(), node);
    }
    let entry = current.get_mut(part).unwrap();

    let result = match entry {
        UnflatNode::Object(ref mut obj) => set_nested_value_recursive(
            obj,
            parts,
            index + 1,
            value,
            separator,
            path_types,
            path_buffer,
            max_array_index,
        ),
        UnflatNode::Array(ref mut arr) => {
            let next_part = parts[index + 1];
            if let Ok(array_index) = next_part.parse::<usize>() {
                if array_index > max_array_index {
                    return Err(JsonToolsError::input_validation_error(format!(
                        "Array index {} exceeds maximum allowed index ({}). \
                         Use max_array_index() to increase the limit.",
                        array_index, max_array_index
                    )));
                }

                while arr.len() <= array_index {
                    arr.push(UnflatNode::Null);
                }

                if index + 2 == parts.len() {
                    arr[array_index] = UnflatNode::Leaf(value);
                    Ok(())
                } else {
                    path_buffer.push_str(separator);
                    path_buffer.push_str(next_part);
                    let next_should_be_array = path_types
                        .get(path_buffer.as_str())
                        .copied()
                        .unwrap_or(false);

                    if matches!(arr[array_index], UnflatNode::Null) {
                        arr[array_index] = if next_should_be_array {
                            UnflatNode::Array(vec![])
                        } else {
                            UnflatNode::Object(FxHashMap::default())
                        };
                    }

                    match &mut arr[array_index] {
                        UnflatNode::Object(ref mut obj) => set_nested_value_recursive(
                            obj,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                            path_buffer,
                            max_array_index,
                        ),
                        UnflatNode::Array(ref mut nested_arr) => set_nested_array_value(
                            nested_arr,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                            path_buffer,
                            max_array_index,
                        ),
                        _ => Err(JsonToolsError::invalid_json_structure(format!(
                            "Array element at index {} has incompatible type",
                            array_index
                        ))),
                    }
                }
            } else {
                // Non-numeric key in array context — convert array to object
                let mut obj = FxHashMap::default();
                let mut itoa_buf = itoa::Buffer::new();
                for (i, item) in arr.iter_mut().enumerate() {
                    if !matches!(item, UnflatNode::Null) {
                        let taken = std::mem::replace(item, UnflatNode::Null);
                        obj.insert(itoa_buf.format(i).to_owned(), taken);
                    }
                }
                obj.insert(next_part.to_string(), UnflatNode::Null);
                *entry = UnflatNode::Object(obj);

                if let UnflatNode::Object(ref mut obj) = entry {
                    set_nested_value_recursive(
                        obj,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                        path_buffer,
                        max_array_index,
                    )
                } else {
                    unreachable!()
                }
            }
        }
        _ => Err(JsonToolsError::invalid_json_structure(format!(
            "Cannot navigate into non-object/non-array value at key: {}",
            part
        ))),
    };

    path_buffer.truncate(buffer_start_len);
    result
}

/// Recursive helper for setting nested values in arrays.
#[allow(clippy::too_many_arguments)]
fn set_nested_array_value<'a>(
    arr: &mut Vec<UnflatNode<'a>>,
    parts: &[&str],
    index: usize,
    value: ValueRef<'a>,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
    max_array_index: usize,
) -> Result<(), JsonToolsError> {
    if index >= parts.len() {
        return Err(JsonToolsError::invalid_json_structure(
            "Invalid path for array",
        ));
    }

    let part = parts[index];

    if let Ok(array_index) = part.parse::<usize>() {
        if array_index > max_array_index {
            return Err(JsonToolsError::input_validation_error(format!(
                "Array index {} exceeds maximum allowed index ({}). \
                 Use max_array_index() to increase the limit.",
                array_index, max_array_index
            )));
        }

        while arr.len() <= array_index {
            arr.push(UnflatNode::Null);
        }

        if index == parts.len() - 1 {
            arr[array_index] = UnflatNode::Leaf(value);
            Ok(())
        } else {
            let buffer_start_len = path_buffer.len();
            if buffer_start_len > 0 {
                path_buffer.push_str(separator);
            }
            path_buffer.push_str(part);

            let next_should_be_array = path_types
                .get(path_buffer.as_str())
                .copied()
                .unwrap_or(false);

            if matches!(arr[array_index], UnflatNode::Null) {
                arr[array_index] = if next_should_be_array {
                    UnflatNode::Array(vec![])
                } else {
                    UnflatNode::Object(FxHashMap::default())
                };
            }

            let result = match &mut arr[array_index] {
                UnflatNode::Object(ref mut obj) => set_nested_value_recursive(
                    obj,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                    path_buffer,
                    max_array_index,
                ),
                UnflatNode::Array(ref mut nested_arr) => set_nested_array_value(
                    nested_arr,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                    path_buffer,
                    max_array_index,
                ),
                _ => Err(JsonToolsError::invalid_json_structure(format!(
                    "Array element at index {} has incompatible type",
                    array_index
                ))),
            };

            path_buffer.truncate(buffer_start_len);
            result
        }
    } else {
        Err(JsonToolsError::invalid_json_structure(format!(
            "Expected array index but got: {}",
            part
        )))
    }
}

// ================================================================================================
// Direct Serialization with Integrated Filtering
// ================================================================================================

/// Serialize an UnflatNode tree directly to a JSON string with integrated filtering.
fn serialize_unflatten_tree(root: &UnflatNode<'_>, filtering: &FilteringConfig) -> String {
    // Estimate output size
    let mut output = String::with_capacity(256);
    serialize_node(root, &mut output, filtering);
    output
}

/// Check if a leaf value should be filtered out based on filtering config.
#[inline]
fn should_filter_leaf(s: &str, filtering: &FilteringConfig) -> bool {
    (filtering.remove_nulls && s == "null")
        || (filtering.remove_empty_strings && s == "\"\"")
        || (filtering.remove_empty_objects && s == "{}")
        || (filtering.remove_empty_arrays && s == "[]")
}

/// Recursive serialization. Returns true if the node produced output (not filtered).
fn serialize_node(node: &UnflatNode<'_>, output: &mut String, filtering: &FilteringConfig) -> bool {
    match node {
        UnflatNode::Leaf(vr) => {
            let s = vr.as_str();
            if should_filter_leaf(s, filtering) {
                return false;
            }
            output.push_str(s);
            true
        }
        UnflatNode::Null => {
            if filtering.remove_nulls {
                return false;
            }
            output.push_str("null");
            true
        }
        UnflatNode::Object(map) => {
            if map.is_empty() {
                if filtering.remove_empty_objects {
                    return false;
                }
                output.push_str("{}");
                return true;
            }

            let saved = output.len();
            output.push('{');
            let mut first = true;

            // Sort keys for deterministic output (replaces BTreeMap's implicit ordering)
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_unstable();

            for key in keys {
                let child = &map[key];
                let child_saved = output.len();
                if !first {
                    output.push(',');
                }
                output.push('"');
                write_json_escaped_key(output, key);
                output.push_str("\":");

                if !serialize_node(child, output, filtering) {
                    output.truncate(child_saved);
                } else {
                    first = false;
                }
            }

            if first {
                // All children were filtered out
                if filtering.remove_empty_objects {
                    output.truncate(saved);
                    return false;
                }
                // Write empty object
                output.truncate(saved);
                output.push_str("{}");
                return true;
            }

            output.push('}');
            true
        }
        UnflatNode::Array(vec) => {
            if vec.is_empty() {
                if filtering.remove_empty_arrays {
                    return false;
                }
                output.push_str("[]");
                return true;
            }

            let saved = output.len();
            output.push('[');
            let mut first = true;

            for child in vec {
                let child_saved = output.len();
                if !first {
                    output.push(',');
                }

                if !serialize_node(child, output, filtering) {
                    output.truncate(child_saved);
                } else {
                    first = false;
                }
            }

            if first {
                // All children were filtered out
                if filtering.remove_empty_arrays {
                    output.truncate(saved);
                    return false;
                }
                output.truncate(saved);
                output.push_str("[]");
                return true;
            }

            output.push(']');
            true
        }
    }
}

// ================================================================================================
// Utility
// ================================================================================================

/// Skip whitespace bytes starting from `pos`.
#[inline(always)]
fn skip_whitespace(input: &[u8], mut pos: usize) -> usize {
    let len = input.len();
    while pos < len {
        let b = unsafe { *input.get_unchecked(pos) };
        if b > 0x20 || (b != b' ' && b != b'\t' && b != b'\n' && b != b'\r') {
            break;
        }
        pos += 1;
    }
    pos
}
