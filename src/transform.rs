//! Key and value replacement transformations.
//!
//! Applies regex-based (with literal fallback) replacements to JSON keys and
//! values. Uses cached regex compilation and SIMD-accelerated substring search.
//!
//! Normal-mode processing uses tape-based streaming: scan the input once with
//! `scan_and_fixup`, then walk the tape writing directly to output with inline
//! value transforms, key transforms, and rollback-based filtering.

use crate::cache::get_cached_regex;
use crate::config::ProcessingConfig;
use crate::convert::try_convert_string_to_json_bytes;
use crate::error::JsonToolsError;
use crate::flatten::{
    apply_value_replacement_cow, escape_json_string, scan_and_fixup, skip_tape_value,
    tape_content_str, tape_is_empty_array, tape_is_empty_object, tape_quoted_str,
    tape_scalar_bytes, trim_ascii, unescape_json_string, write_json_escaped_key, EntryKind,
    TapeEntry,
};
use memchr::memmem;
use rustc_hash::FxHashMap;
use serde_json::Value;
use smallvec::SmallVec;
use std::borrow::Cow;

// ================================================================================================
// Key/Value Replacement Core Logic
// ================================================================================================

/// Core key replacement logic that works with both string keys and Cow<str>
/// This eliminates duplication between flatten and unflatten key replacement functions
/// Optimized to minimize string allocations by using efficient Cow operations
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
#[inline(always)]
pub(crate) fn apply_key_replacement_patterns(
    key: &str,
    patterns: &[(String, String)],
) -> Result<Option<String>, JsonToolsError> {
    let mut new_key = Cow::Borrowed(key);
    let mut changed = false;

    // Apply each replacement pattern
    for (pattern, replacement) in patterns {
        // Try to compile as regex first
        match get_cached_regex(pattern) {
            Ok(regex) => {
                // Use replace_all's Cow return to detect matches without a separate is_match()
                // scan -- Cow::Owned means replacement happened, Cow::Borrowed means no match.
                if let Cow::Owned(s) = regex.replace_all(&new_key, replacement.as_str()) {
                    new_key = Cow::Owned(s);
                    changed = true;
                }
            }
            Err(_) => {
                // Failed to compile as regex - fall back to literal replacement
                // Use SIMD-accelerated memmem::find instead of str::contains
                if memmem::find(new_key.as_bytes(), pattern.as_bytes()).is_some() {
                    new_key = Cow::Owned(new_key.replace(pattern, replacement));
                    changed = true;
                }
            }
        }
    }

    if changed {
        Ok(Some(new_key.into_owned()))
    } else {
        Ok(None)
    }
}

/// Core value replacement logic that works with any Value type.
/// Delegates to `apply_key_replacement_patterns` for the shared regex/literal
/// replacement logic, then writes the result back into the Value.
#[inline]
pub(crate) fn apply_value_replacement_patterns(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    if let Value::String(s) = value {
        if let Some(replaced) = apply_key_replacement_patterns(s, patterns)? {
            *s = replaced;
        }
    }
    Ok(())
}

// ================================================================================================
// Normal Mode Processing — Tape-Based Streaming
// ================================================================================================

/// Core normal-mode logic for a single JSON string.
/// Uses tape-based streaming: scan once, walk once, write directly to output.
/// Two paths: fast (no key transforms) and slow (with key transforms + collision handling).
#[inline]
pub(crate) fn process_single_json_normal(
    json: &str,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    let input = json.as_bytes();
    if input.len() > u32::MAX as usize {
        return Err(JsonToolsError::input_validation_error(
            "Input exceeds 4 GiB limit",
        ));
    }

    let trimmed = trim_ascii(input);
    if trimmed.is_empty() {
        return Ok(json.to_string());
    }

    // Root primitives: handle without tape
    match trimmed[0] {
        b'{' | b'[' => {}
        _ => return handle_root_primitive(trimmed, config),
    }

    let tape = scan_and_fixup(input)?;
    if tape.is_empty() {
        return Ok(json.to_string());
    }

    let needs_slow = config.lowercase_keys
        || config.replacements.has_key_replacements()
        || config.collision.has_collision_handling();

    if needs_slow {
        normal_slow_walk(input, &tape, config)
    } else {
        Ok(normal_fast_walk(input, &tape, config))
    }
}

/// Handle root-level primitives (strings, numbers, booleans, null)
fn handle_root_primitive(
    trimmed: &[u8],
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // SAFETY: input is valid UTF-8 (came from &str)
    let s = unsafe { std::str::from_utf8_unchecked(trimmed) };

    // Check if it's a quoted string
    if trimmed.len() >= 2 && trimmed[0] == b'"' && trimmed[trimmed.len() - 1] == b'"' {
        let content = &s[1..s.len() - 1];

        // Value replacement
        if config.replacements.has_value_replacements() {
            let unescaped = if memchr::memchr(b'\\', content.as_bytes()).is_some() {
                unescape_json_string(content)
            } else {
                Cow::Borrowed(content)
            };
            if let Some(replaced) = apply_value_replacement_cow(
                unescaped.as_ref(),
                &config.replacements.value_replacements,
            ) {
                // Re-escape and return
                let escaped = escape_json_string(&replaced);
                return Ok(format!("\"{}\"", escaped));
            }
        }

        // Type conversion
        if config.auto_convert_types {
            let unescaped = if memchr::memchr(b'\\', content.as_bytes()).is_some() {
                unescape_json_string(content)
            } else {
                Cow::Borrowed(content)
            };
            if let Some(converted) = try_convert_string_to_json_bytes(unescaped.as_ref()) {
                return Ok(converted.into_owned());
            }
        }
    }

    // Filtering for null
    if config.filtering.remove_nulls && trimmed == b"null" {
        // Root null with remove_nulls — return "null" since we can't remove the root
        return Ok(s.to_string());
    }

    Ok(s.to_string())
}

// ================================================================================================
// Shared String Emission Logic
// ================================================================================================

/// Shared logic for emitting a string value with replacements, type conversion, and filtering.
/// Used by both NormalFastWalker (fast path) and NormalSlowWalker (key-transform path).
///
/// Returns `(was_filtered, output_was_written)`. When `output_was_written` is false, the caller
/// should fall back to zero-copy output of the raw quoted string.
#[inline(always)]
fn emit_string_value_shared(
    output: &mut String,
    input: &[u8],
    entry: TapeEntry,
    config: &ProcessingConfig,
) -> (bool, bool) {
    let content_str = tape_content_str(input, entry);
    let has_replacements = config.replacements.has_value_replacements();
    let auto_convert = config.auto_convert_types;

    // Early exit: nothing to transform
    if !has_replacements && !auto_convert {
        return (false, false);
    }

    // Unescape once if needed by either replacement or conversion
    let unescaped = if entry.string_has_escapes() {
        unescape_json_string(content_str)
    } else {
        Cow::Borrowed(content_str)
    };

    // Value replacement
    if has_replacements {
        if let Some(replaced) =
            apply_value_replacement_cow(unescaped.as_ref(), &config.replacements.value_replacements)
        {
            if config.filtering.remove_empty_strings && replaced.is_empty() {
                return (true, true);
            }
            // Type convert the replaced value
            if auto_convert {
                if let Some(converted) = try_convert_string_to_json_bytes(&replaced) {
                    if config.filtering.remove_nulls && converted == "null" {
                        return (true, true);
                    }
                    output.push_str(&converted);
                    return (false, true);
                }
            }
            let escaped = escape_json_string(&replaced);
            output.push('"');
            output.push_str(escaped.as_ref());
            output.push('"');
            return (false, true);
        }
    }

    // Type conversion (no replacement occurred)
    if auto_convert {
        if let Some(converted) = try_convert_string_to_json_bytes(unescaped.as_ref()) {
            if config.filtering.remove_nulls && converted == "null" {
                return (true, true);
            }
            output.push_str(&converted);
            return (false, true);
        }
    }

    (false, false)
}

// ================================================================================================
// Fast Path — NormalFastWalker (no key transforms)
// ================================================================================================

/// Tape-based streaming walker for normal mode without key transforms.
/// Single-pass: walks tape and writes directly to output with inline value transforms
/// and rollback-based filtering.
struct NormalFastWalker<'a> {
    input: &'a [u8],
    tape: &'a [TapeEntry],
    config: &'a ProcessingConfig,
    output: String,
}

/// Walk the tape in fast mode (no key transforms) and produce output directly.
fn normal_fast_walk(input: &[u8], tape: &[TapeEntry], config: &ProcessingConfig) -> String {
    let mut walker = NormalFastWalker {
        input,
        tape,
        config,
        output: String::with_capacity(input.len()),
    };
    walker.walk_value(0);
    walker.output
}

impl<'a> NormalFastWalker<'a> {
    /// Walk a single value. Returns (next_tape_idx, was_filtered).
    #[inline(always)]
    fn walk_value(&mut self, idx: usize) -> (usize, bool) {
        if idx >= self.tape.len() {
            return (idx, false);
        }
        let entry = self.tape[idx];
        match entry.kind() {
            EntryKind::ObjectStart => self.walk_object(idx),
            EntryKind::ArrayStart => self.walk_array(idx),
            EntryKind::StringStart => self.emit_string_value(idx),
            EntryKind::ScalarStart => self.emit_scalar_value(idx),
            _ => (idx + 1, false),
        }
    }

    /// Walk an object: emit {key:value,...} with filtering rollback.
    fn walk_object(&mut self, start_idx: usize) -> (usize, bool) {
        let end_idx = self.tape[start_idx].aux() as usize;

        // Empty object check
        if tape_is_empty_object(self.tape, start_idx) {
            if self.config.filtering.remove_empty_objects {
                return (end_idx + 1, true);
            }
            self.output.push_str("{}");
            return (end_idx + 1, false);
        }

        let obj_start = self.output.len();
        self.output.push('{');
        let mut first = true;
        let mut cursor = start_idx + 1;

        while cursor < end_idx {
            let entry = self.tape[cursor];
            if entry.kind() == EntryKind::StringStart {
                // This is a key
                let checkpoint = self.output.len();

                // Emit comma separator
                if !first {
                    self.output.push(',');
                }

                // Emit key (raw copy from input including quotes)
                let key_with_quotes = tape_quoted_str(self.input, entry);
                self.output.push_str(key_with_quotes);

                // Skip key and colon
                cursor += 1;
                if cursor < end_idx && self.tape[cursor].kind() == EntryKind::Colon {
                    self.output.push(':');
                    cursor += 1;
                }

                // Walk value
                let (next, filtered) = self.walk_value(cursor);
                cursor = next;

                if filtered {
                    // Rollback: remove comma + key + colon + value
                    self.output.truncate(checkpoint);
                } else {
                    first = false;
                }
            } else if entry.kind() == EntryKind::Comma {
                // Skip comma tape entries — we manage commas ourselves
                cursor += 1;
            } else {
                cursor += 1;
            }
        }

        // Check if object became empty after filtering
        if self.output.len() == obj_start + 1 && self.config.filtering.remove_empty_objects {
            self.output.truncate(obj_start);
            return (end_idx + 1, true);
        }

        self.output.push('}');
        (end_idx + 1, false)
    }

    /// Walk an array: emit [val,...] with filtering rollback.
    fn walk_array(&mut self, start_idx: usize) -> (usize, bool) {
        let end_idx = self.tape[start_idx].aux() as usize;

        // Empty array check
        if tape_is_empty_array(self.tape, start_idx) {
            if self.config.filtering.remove_empty_arrays {
                return (end_idx + 1, true);
            }
            self.output.push_str("[]");
            return (end_idx + 1, false);
        }

        let arr_start = self.output.len();
        self.output.push('[');
        let mut first = true;
        let mut cursor = start_idx + 1;

        while cursor < end_idx {
            let entry = self.tape[cursor];
            if entry.kind() == EntryKind::Comma {
                cursor += 1;
                continue;
            }

            let checkpoint = self.output.len();
            if !first {
                self.output.push(',');
            }

            let (next, filtered) = self.walk_value(cursor);
            cursor = next;

            if filtered {
                self.output.truncate(checkpoint);
            } else {
                first = false;
            }
        }

        // Check if array became empty after filtering
        if self.output.len() == arr_start + 1 && self.config.filtering.remove_empty_arrays {
            self.output.truncate(arr_start);
            return (end_idx + 1, true);
        }

        self.output.push(']');
        (end_idx + 1, false)
    }

    /// Emit a string value with inline transforms. Returns (next_idx, was_filtered).
    #[inline(always)]
    fn emit_string_value(&mut self, idx: usize) -> (usize, bool) {
        let entry = self.tape[idx];

        // Filtering: empty string
        if self.config.filtering.remove_empty_strings && entry.string_content_len() == 0 {
            return (idx + 1, true);
        }

        let (was_filtered, was_written) =
            emit_string_value_shared(&mut self.output, self.input, entry, self.config);
        if was_written {
            return (idx + 1, was_filtered);
        }

        // Zero-copy: raw string including quotes
        self.output.push_str(tape_quoted_str(self.input, entry));
        (idx + 1, false)
    }

    /// Emit a scalar value (number, bool, null). Returns (next_idx, was_filtered).
    #[inline(always)]
    fn emit_scalar_value(&mut self, idx: usize) -> (usize, bool) {
        let entry = self.tape[idx];
        let trimmed = trim_ascii(tape_scalar_bytes(self.input, entry));

        if self.config.filtering.remove_nulls && trimmed == b"null" {
            return (idx + 1, true);
        }

        let s = unsafe { std::str::from_utf8_unchecked(trimmed) };
        self.output.push_str(s);
        (idx + 1, false)
    }
}

// ================================================================================================
// Slow Path — NormalSlowWalker (key transforms + collision handling)
// ================================================================================================

/// Per-object entry with transformed key and tape index for the value.
struct SlowObjectEntry {
    key: String,
    val_start: usize,
}

/// Walk the tape with key transforms and collision handling.
fn normal_slow_walk(
    input: &[u8],
    tape: &[TapeEntry],
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    let mut walker = NormalSlowWalker {
        input,
        tape,
        config,
        output: String::with_capacity(input.len()),
    };
    walker.walk_value(0)?;
    Ok(walker.output)
}

struct NormalSlowWalker<'a> {
    input: &'a [u8],
    tape: &'a [TapeEntry],
    config: &'a ProcessingConfig,
    output: String,
}

impl<'a> NormalSlowWalker<'a> {
    /// Walk a single value. Returns (next_tape_idx, was_filtered).
    fn walk_value(&mut self, idx: usize) -> Result<(usize, bool), JsonToolsError> {
        if idx >= self.tape.len() {
            return Ok((idx, false));
        }
        let entry = self.tape[idx];
        match entry.kind() {
            EntryKind::ObjectStart => self.walk_object(idx),
            EntryKind::ArrayStart => self.walk_array(idx),
            EntryKind::StringStart => Ok(self.emit_string_value(idx)),
            EntryKind::ScalarStart => Ok(self.emit_scalar_value(idx)),
            _ => Ok((idx + 1, false)),
        }
    }

    /// Walk an object with key transforms and collision handling.
    fn walk_object(&mut self, start_idx: usize) -> Result<(usize, bool), JsonToolsError> {
        let end_idx = self.tape[start_idx].aux() as usize;

        // Empty object check
        if tape_is_empty_object(self.tape, start_idx) {
            if self.config.filtering.remove_empty_objects {
                return Ok((end_idx + 1, true));
            }
            self.output.push_str("{}");
            return Ok((end_idx + 1, false));
        }

        // Phase 1: Collect entries with transformed keys
        let mut entries: SmallVec<[SlowObjectEntry; 16]> = SmallVec::new();
        let mut cursor = start_idx + 1;

        while cursor < end_idx {
            let entry = self.tape[cursor];
            if entry.kind() == EntryKind::StringStart {
                // Extract and transform key
                let key_str = tape_content_str(self.input, entry);

                let key_unescaped = if entry.string_has_escapes() {
                    unescape_json_string(key_str)
                } else {
                    Cow::Borrowed(key_str)
                };

                // Apply key replacement patterns
                let mut transformed_key = if self.config.replacements.has_key_replacements() {
                    apply_key_replacement_patterns(
                        key_unescaped.as_ref(),
                        &self.config.replacements.key_replacements,
                    )?
                    .unwrap_or_else(|| key_unescaped.into_owned())
                } else {
                    key_unescaped.into_owned()
                };

                // Apply lowercase
                if self.config.lowercase_keys {
                    transformed_key = transformed_key.to_lowercase();
                }

                // Skip key and colon
                cursor += 1;
                if cursor < end_idx && self.tape[cursor].kind() == EntryKind::Colon {
                    cursor += 1;
                }

                // Determine value tape position and advance cursor past it
                let val_start = cursor;
                cursor = skip_tape_value(self.tape, cursor);

                entries.push(SlowObjectEntry {
                    key: transformed_key,
                    val_start,
                });
            } else {
                // Skip commas and any other structural entries
                cursor += 1;
            }
        }

        // Phase 2: Detect collisions
        let has_collisions = if self.config.collision.has_collision_handling() {
            let mut seen = FxHashMap::with_capacity_and_hasher(entries.len(), Default::default());
            entries.iter().any(|e| seen.insert(&e.key, ()).is_some())
        } else {
            false
        };

        // Phase 3: Serialize
        let obj_start = self.output.len();
        self.output.push('{');

        if has_collisions {
            self.serialize_with_collisions(&entries)?;
        } else {
            self.serialize_no_collisions(&entries)?;
        }

        // Check if object became empty after filtering
        if self.output.len() == obj_start + 1 && self.config.filtering.remove_empty_objects {
            self.output.truncate(obj_start);
            return Ok((end_idx + 1, true));
        }

        self.output.push('}');
        Ok((end_idx + 1, false))
    }

    /// Serialize object entries without collisions.
    fn serialize_no_collisions(
        &mut self,
        entries: &[SlowObjectEntry],
    ) -> Result<(), JsonToolsError> {
        let mut first = true;
        for entry in entries {
            let checkpoint = self.output.len();
            if !first {
                self.output.push(',');
            }

            // Emit escaped key
            self.output.push('"');
            write_json_escaped_key(&mut self.output, &entry.key);
            self.output.push_str("\":");

            // Recurse into value
            let (_, filtered) = self.walk_value(entry.val_start)?;

            if filtered {
                self.output.truncate(checkpoint);
            } else {
                first = false;
            }
        }
        Ok(())
    }

    /// Serialize object entries with collision merging.
    fn serialize_with_collisions(
        &mut self,
        entries: &[SlowObjectEntry],
    ) -> Result<(), JsonToolsError> {
        // Group by key, preserving order of first occurrence
        let mut key_order: Vec<&str> = Vec::new();
        let mut key_indices: FxHashMap<&str, SmallVec<[usize; 2]>> =
            FxHashMap::with_capacity_and_hasher(entries.len(), Default::default());

        for (i, entry) in entries.iter().enumerate() {
            key_indices
                .entry(entry.key.as_str())
                .and_modify(|v| v.push(i))
                .or_insert_with(|| {
                    key_order.push(entry.key.as_str());
                    SmallVec::from_elem(i, 1)
                });
        }

        let mut first = true;
        for key in &key_order {
            let indices = &key_indices[key];

            if indices.len() == 1 {
                // Single entry — no collision
                let entry = &entries[indices[0]];
                let checkpoint = self.output.len();
                if !first {
                    self.output.push(',');
                }

                self.output.push('"');
                write_json_escaped_key(&mut self.output, &entry.key);
                self.output.push_str("\":");

                let (_, filtered) = self.walk_value(entry.val_start)?;
                if filtered {
                    self.output.truncate(checkpoint);
                } else {
                    first = false;
                }
            } else {
                // Collision — merge values into array
                let checkpoint = self.output.len();
                if !first {
                    self.output.push(',');
                }

                self.output.push('"');
                write_json_escaped_key(&mut self.output, key);
                self.output.push_str("\":[");

                let arr_start = self.output.len();
                let mut arr_first = true;

                for &idx in indices {
                    let entry = &entries[idx];
                    let val_checkpoint = self.output.len();
                    if !arr_first {
                        self.output.push(',');
                    }

                    let (_, filtered) = self.walk_value(entry.val_start)?;
                    if filtered {
                        self.output.truncate(val_checkpoint);
                    } else {
                        arr_first = false;
                    }
                }

                // If all values in the collision array were filtered, remove the entire entry
                if self.output.len() == arr_start {
                    self.output.truncate(checkpoint);
                } else {
                    self.output.push(']');
                    first = false;
                }
            }
        }

        Ok(())
    }

    /// Walk an array with filtering rollback.
    fn walk_array(&mut self, start_idx: usize) -> Result<(usize, bool), JsonToolsError> {
        let end_idx = self.tape[start_idx].aux() as usize;

        // Empty array check
        if tape_is_empty_array(self.tape, start_idx) {
            if self.config.filtering.remove_empty_arrays {
                return Ok((end_idx + 1, true));
            }
            self.output.push_str("[]");
            return Ok((end_idx + 1, false));
        }

        let arr_start = self.output.len();
        self.output.push('[');
        let mut first = true;
        let mut cursor = start_idx + 1;

        while cursor < end_idx {
            let entry = self.tape[cursor];
            if entry.kind() == EntryKind::Comma {
                cursor += 1;
                continue;
            }

            let checkpoint = self.output.len();
            if !first {
                self.output.push(',');
            }

            let (next, filtered) = self.walk_value(cursor)?;
            cursor = next;

            if filtered {
                self.output.truncate(checkpoint);
            } else {
                first = false;
            }
        }

        // Check if array became empty after filtering
        if self.output.len() == arr_start + 1 && self.config.filtering.remove_empty_arrays {
            self.output.truncate(arr_start);
            return Ok((end_idx + 1, true));
        }

        self.output.push(']');
        Ok((end_idx + 1, false))
    }

    /// Emit a string value with inline transforms. Returns (next_idx, was_filtered).
    #[inline(always)]
    fn emit_string_value(&mut self, idx: usize) -> (usize, bool) {
        let entry = self.tape[idx];

        // Filtering: empty string
        if self.config.filtering.remove_empty_strings && entry.string_content_len() == 0 {
            return (idx + 1, true);
        }

        let (was_filtered, was_written) =
            emit_string_value_shared(&mut self.output, self.input, entry, self.config);
        if was_written {
            return (idx + 1, was_filtered);
        }

        // Zero-copy: raw string including quotes
        self.output.push_str(tape_quoted_str(self.input, entry));
        (idx + 1, false)
    }

    /// Emit a scalar value. Returns (next_idx, was_filtered).
    #[inline(always)]
    fn emit_scalar_value(&mut self, idx: usize) -> (usize, bool) {
        let entry = self.tape[idx];
        let trimmed = trim_ascii(tape_scalar_bytes(self.input, entry));

        if self.config.filtering.remove_nulls && trimmed == b"null" {
            return (idx + 1, true);
        }

        let s = unsafe { std::str::from_utf8_unchecked(trimmed) };
        self.output.push_str(s);
        (idx + 1, false)
    }
}
