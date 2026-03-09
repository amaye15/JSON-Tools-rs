use memchr::{memchr, memchr2, memchr3, memmem, memrchr};
use rustc_hash::FxHashMap;
use serde_json::{Map, Value};
use smallvec::SmallVec;

use crate::config::ProcessingConfig;
use crate::convert::apply_type_conversion_recursive;
use crate::error::JsonToolsError;
use crate::json_parser;
use crate::transform::{
    apply_key_replacements_for_unflatten, apply_key_replacements_unflatten_with_collisions,
    apply_lowercase_keys_for_unflatten, apply_value_replacement_patterns,
    apply_value_replacements_for_unflatten, filter_nested_value,
    handle_key_collisions_for_unflatten,
};

// ================================================================================================
// Root-Level Primitive Handling
// ================================================================================================

/// Handle root-level primitive values and empty containers for unflattening
#[inline]
fn handle_root_level_primitives_unflatten(
    value: &Value,
    value_replacements: &[(String, String)],
) -> Result<Option<String>, JsonToolsError> {
    match value {
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // For root-level primitives, apply value replacements if any, then return
            let mut single_value = value.clone();
            if !value_replacements.is_empty() {
                apply_value_replacement_patterns(&mut single_value, value_replacements)?;
            }

            Ok(Some(json_parser::to_string(&single_value)?))
        }
        Value::Object(obj) if obj.is_empty() => {
            // Empty object should remain empty object
            Ok(Some("{}".to_string()))
        }
        Value::Array(_) => {
            // Arrays at root level are not valid flattened JSON - convert to empty object
            Ok(Some("{}".to_string()))
        }
        _ => {
            // Continue with normal unflattening for objects with content
            Ok(None)
        }
    }
}

/// Extract flattened object from parsed JSON value
#[inline]
fn extract_flattened_object(flattened: Value) -> Result<Map<String, Value>, JsonToolsError> {
    match flattened {
        Value::Object(obj) => Ok(obj),
        _ => Err(JsonToolsError::invalid_json_structure(
            "Expected object for unflattening",
        )),
    }
}

// ================================================================================================
// Transformation Pipeline for Unflatten
// ================================================================================================

/// Apply all transformations (key replacements, value replacements, lowercase) for unflattening
/// Optimized to avoid unnecessary clone by consuming the input
/// MEDIUM-HOT PATH: Inline for better performance
#[inline]
fn apply_transformations_unflatten(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Result<Map<String, Value>, JsonToolsError> {
    // Consume the input instead of cloning
    let mut processed_obj = flattened_obj;

    // Apply key replacements with collision detection if provided
    if config.replacements.has_key_replacements() {
        // Use optimized version when collision handling is disabled for better performance
        if !config.collision.handle_collisions {
            // Pass ownership to avoid cloning all values
            processed_obj = apply_key_replacements_for_unflatten(
                processed_obj,
                &config.replacements.key_replacements,
            )?;
        } else {
            processed_obj =
                apply_key_replacements_unflatten_with_collisions(processed_obj, config)?;
        }
    }

    // Apply value replacements
    if config.replacements.has_value_replacements() {
        apply_value_replacements_for_unflatten(
            &mut processed_obj,
            &config.replacements.value_replacements,
        )?;
    }

    // Apply lowercase conversion if specified
    if config.lowercase_keys {
        processed_obj = apply_lowercase_keys_for_unflatten(processed_obj);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if config.collision.handle_collisions && !config.replacements.has_key_replacements() {
            processed_obj = handle_key_collisions_for_unflatten(processed_obj, config);
        }
    } else if config.collision.handle_collisions && !config.replacements.has_key_replacements() {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        processed_obj = handle_key_collisions_for_unflatten(processed_obj, config);
    }

    Ok(processed_obj)
}

/// Perform unflattening and apply filtering to the result
#[inline]
fn perform_unflattening_and_filtering(
    processed_obj: Map<String, Value>,
    separator: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> Result<Value, JsonToolsError> {
    // Perform the actual unflattening (takes ownership to avoid cloning values)
    let mut unflattened = unflatten_object(processed_obj, separator)?;

    // Apply filtering to the unflattened result
    if remove_empty_string_values || remove_null_values || remove_empty_dict || remove_empty_list {
        filter_nested_value(
            &mut unflattened,
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
        );
    }

    Ok(unflattened)
}

// ================================================================================================
// Core Unflatten Processing Entry Point
// ================================================================================================

/// Core unflattening logic for a single JSON string
#[inline]
pub(crate) fn process_single_json_for_unflatten(
    json: &str,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // Parse JSON using optimized SIMD parsing
    let mut flattened = json_parser::parse_json(json)?;

    // Apply type conversion FIRST (before other transformations)
    if config.auto_convert_types {
        apply_type_conversion_recursive(&mut flattened);
    }

    // Handle root-level primitives and empty containers
    if let Some(result) =
        handle_root_level_primitives_unflatten(&flattened, &config.replacements.value_replacements)?
    {
        return Ok(result);
    }

    // Extract the flattened object
    let flattened_obj = extract_flattened_object(flattened)?;

    // Apply key and value transformations
    let processed_obj = apply_transformations_unflatten(flattened_obj, config)?;

    // Perform the actual unflattening and apply filtering
    // Pass ownership to avoid cloning values during unflatten
    let unflattened = perform_unflattening_and_filtering(
        processed_obj,
        &config.separator,
        config.filtering.remove_empty_strings,
        config.filtering.remove_nulls,
        config.filtering.remove_empty_objects,
        config.filtering.remove_empty_arrays,
    )?;

    // Serialize the result
    Ok(json_parser::to_string(&unflattened)?)
}

// ================================================================================================
// Unflatten Algorithm
// ================================================================================================

/// Core unflattening algorithm that reconstructs nested JSON from flattened keys
fn unflatten_object(obj: Map<String, Value>, separator: &str) -> Result<Value, JsonToolsError> {
    // OPTIMIZATION #21: Removed O(N log N) sorting - process entries directly
    // The set_nested_value_with_types function uses entry().or_insert_with() to create
    // intermediate nodes on demand, so sorting is not required for correctness.
    // This reduces complexity from O(N log N) to O(N) for the iteration phase.

    let mut result = Map::with_capacity(obj.len() / 2); // Estimate final size

    // Pre-analyze path types before consuming the map
    let path_types = analyze_path_types(&obj, separator);

    // Process entries directly without sorting - O(N) instead of O(N log N)
    // The recursive helper creates intermediate structures on demand
    for (key, value) in obj {
        set_nested_value(&mut result, &key, value, separator, &path_types)?;
    }

    Ok(Value::Object(result))
}

// ================================================================================================
// Path Analysis
// ================================================================================================

/// Analyze all flattened keys to determine whether each path should be an array or object
/// Analyze path types to determine if segments should be arrays or objects
fn analyze_path_types(obj: &Map<String, Value>, separator: &str) -> FxHashMap<String, bool> {
    // Use a more efficient approach with pre-allocated capacity and optimized string operations
    let estimated_paths = obj.len() * 2; // Rough estimate of path count
    let mut state: FxHashMap<String, u8> =
        FxHashMap::with_capacity_and_hasher(estimated_paths, Default::default());

    // Pre-compile separator for faster matching
    let sep_bytes = separator.as_bytes();
    let sep_len = separator.len();

    for key in obj.keys() {
        analyze_key_path(key, sep_bytes, sep_len, &mut state);
    }

    // Convert bitmask state to final decision with pre-allocated result
    let mut result: FxHashMap<String, bool> =
        FxHashMap::with_capacity_and_hasher(state.len(), Default::default());
    for (k, mask) in state.into_iter() {
        let is_array = (mask & 0b10 == 0) && (mask & 0b01 != 0);
        result.insert(k, is_array);
    }
    result
}

/// Optimized key path analysis with efficient string operations
#[inline]
fn analyze_key_path(
    key: &str,
    sep_bytes: &[u8],
    sep_len: usize,
    state: &mut FxHashMap<String, u8>,
) {
    let key_bytes = key.as_bytes();
    let mut search_start = 0;

    // Use Boyer-Moore-like approach for separator finding
    while search_start < key_bytes.len() {
        // Find next separator using optimized byte search
        let next_sep = find_separator(key_bytes, sep_bytes, search_start);

        if next_sep == key_bytes.len() {
            break; // No more separators
        }

        // Extract parent path efficiently
        let parent = &key[..next_sep];

        // Look ahead to classify child
        let child_start = next_sep + sep_len;
        if child_start < key_bytes.len() {
            let child_end = find_separator(key_bytes, sep_bytes, child_start).min(key_bytes.len());
            let child = &key[child_start..child_end];

            // Optimized numeric check
            let is_numeric = is_valid_array_index(child);

            // Update state with efficient entry handling
            match state.get_mut(parent) {
                Some(entry) => {
                    if is_numeric {
                        *entry |= 0b01;
                    } else {
                        *entry |= 0b10;
                    }
                }
                None => {
                    let initial_value = if is_numeric { 0b01 } else { 0b10 };
                    state.insert(parent.to_string(), initial_value);
                }
            }
        }

        search_start = next_sep + sep_len;
    }
}

// ================================================================================================
// SIMD-Optimized Separator Finding
// ================================================================================================

/// SIMD-optimized separator finding using memchr crate
///
/// Uses hardware-accelerated SIMD instructions (SSE2/AVX2/NEON) for byte searching
/// Provides 3-10x speedup over naive byte-by-byte search
#[inline]
pub(crate) fn find_separator(haystack: &[u8], needle: &[u8], start: usize) -> usize {
    if needle.len() == 1 {
        // Single byte separator - use memchr's SIMD-optimized byte search
        // This uses SSE2/AVX2 on x86 and NEON on ARM for 3-10x speedup
        memchr(needle[0], &haystack[start..])
            .map(|pos| start + pos)
            .unwrap_or(haystack.len())
    } else {
        // Multi-byte separator - use memmem's SIMD-optimized substring search
        // Uses Two-Way algorithm with SIMD acceleration
        memmem::find(&haystack[start..], needle)
            .map(|pos| start + pos)
            .unwrap_or(haystack.len())
    }
}

/// Find first occurrence of any of two separators (SIMD-optimized with memchr2)
/// Up to 2x faster than checking each separator individually
/// Available for future optimizations (e.g., finding '.', '_' or '-' in keys)
#[allow(dead_code)]
#[inline]
fn find_separator_dual(haystack: &[u8], start: usize, sep1: u8, sep2: u8) -> Option<usize> {
    memchr2(sep1, sep2, &haystack[start..]).map(|pos| start + pos)
}

/// Find first occurrence of any of three separators (SIMD-optimized with memchr3)
/// Up to 3x faster than checking each separator individually
/// Available for future optimizations
#[allow(dead_code)]
#[inline]
fn find_separator_triple(
    haystack: &[u8],
    start: usize,
    sep1: u8,
    sep2: u8,
    sep3: u8,
) -> Option<usize> {
    memchr3(sep1, sep2, sep3, &haystack[start..]).map(|pos| start + pos)
}

/// Find last occurrence of separator (reverse search, SIMD-optimized with memrchr)
/// Useful for path operations and key manipulation
/// Available for future optimizations (e.g., finding parent paths)
#[allow(dead_code)]
#[inline]
fn find_last_separator(haystack: &[u8], sep: u8) -> Option<usize> {
    memrchr(sep, haystack)
}

// ================================================================================================
// Array Index Validation
// ================================================================================================

/// Optimized check for valid array index
/// OPTIMIZATION #13: Force inline for hot path
#[inline(always)]
fn is_valid_array_index(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Fast path for single digit
    if s.len() == 1 {
        return s.as_bytes()[0].is_ascii_digit();
    }

    // Check for leading zero (invalid except for "0")
    if s.starts_with('0') {
        return s == "0";
    }

    // Check if all characters are digits (vectorizable)
    s.bytes().all(|b| b.is_ascii_digit())
}

// ================================================================================================
// Recursive Nested Value Setting
// ================================================================================================

/// Set a nested value using pre-analyzed path types to handle conflicts
fn set_nested_value(
    result: &mut Map<String, Value>,
    key_path: &str,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
) -> Result<(), JsonToolsError> {
    // TIER 6->2 OPTIMIZATION: Use SmallVec for path splits
    // Most JSON paths have <16 segments, so this stays on stack (Tier 2)
    // Saves 100-300 cycles per unflatten call by avoiding heap allocation
    type PathSegments<'a> = SmallVec<[&'a str; 16]>;
    let parts: PathSegments = key_path.split(separator).collect();

    if parts.is_empty() {
        return Err(JsonToolsError::invalid_json_structure("Empty key path"));
    }

    if parts.len() == 1 {
        // Simple key, just insert
        result.insert(parts[0].to_string(), value);
        return Ok(());
    }

    // OPTIMIZATION: Pre-allocate path buffer to avoid repeated allocations
    let mut path_buffer = String::with_capacity(key_path.len());
    set_nested_value_recursive(
        result,
        &parts,
        0,
        value,
        separator,
        path_types,
        &mut path_buffer,
    )
}

/// Optimized recursive helper that reuses a path buffer to avoid allocations
fn set_nested_value_recursive(
    current: &mut Map<String, Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
) -> Result<(), JsonToolsError> {
    let part = parts[index];

    if index == parts.len() - 1 {
        // Last part, insert the value
        current.insert(part.to_string(), value);
        return Ok(());
    }

    // Build the current path in the buffer
    let buffer_start_len = path_buffer.len();
    if buffer_start_len > 0 {
        path_buffer.push_str(separator);
    }
    path_buffer.push_str(part);

    let should_be_array = path_types
        .get(path_buffer.as_str())
        .copied()
        .unwrap_or(false);

    // TIER 6->3 OPTIMIZATION: Avoid String allocation for existing keys
    // Check if key exists first (takes &str), only allocate String if inserting
    // Saves 50-100 cycles per existing key (common in repeated unflatten operations)
    let entry = if let Some(existing) = current.get_mut(part) {
        existing
    } else {
        current.entry(part.to_string()).or_insert_with(|| {
            if should_be_array {
                Value::Array(vec![])
            } else {
                Value::Object(Map::new())
            }
        })
    };

    let result = match entry {
        Value::Object(ref mut obj) => set_nested_value_recursive(
            obj,
            parts,
            index + 1,
            value,
            separator,
            path_types,
            path_buffer,
        ),
        Value::Array(ref mut arr) => {
            // Handle array indexing
            let next_part = parts[index + 1];
            if let Ok(array_index) = next_part.parse::<usize>() {
                // Ensure array is large enough
                while arr.len() <= array_index {
                    arr.push(Value::Null);
                }

                if index + 2 == parts.len() {
                    // Last part, set the value
                    arr[array_index] = value;
                    Ok(())
                } else {
                    // Build next path in buffer for type lookup
                    path_buffer.push_str(separator);
                    path_buffer.push_str(next_part);
                    let next_should_be_array = path_types
                        .get(path_buffer.as_str())
                        .copied()
                        .unwrap_or(false);

                    if arr[array_index].is_null() {
                        arr[array_index] = if next_should_be_array {
                            Value::Array(vec![])
                        } else {
                            Value::Object(Map::new())
                        };
                    }

                    match &mut arr[array_index] {
                        Value::Object(ref mut obj) => set_nested_value_recursive(
                            obj,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                            path_buffer,
                        ),
                        Value::Array(ref mut nested_arr) => set_nested_array_value(
                            nested_arr,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                            path_buffer,
                        ),
                        _ => Err(JsonToolsError::invalid_json_structure(format!(
                            "Array element at index {} has incompatible type",
                            array_index
                        ))),
                    }
                }
            } else {
                // Non-numeric key in array context - treat as object key
                // Convert array to object
                let mut obj = Map::new();
                for (i, item) in arr.iter().enumerate() {
                    if !item.is_null() {
                        obj.insert(i.to_string(), item.clone());
                    }
                }
                obj.insert(next_part.to_string(), Value::Null); // Placeholder
                *entry = Value::Object(obj);

                // Now continue as object
                if let Value::Object(ref mut obj) = entry {
                    set_nested_value_recursive(
                        obj,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                        path_buffer,
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

    // Restore buffer to its state before this call
    path_buffer.truncate(buffer_start_len);
    result
}

/// Optimized recursive helper for setting nested values in arrays with type awareness
fn set_nested_array_value(
    arr: &mut Vec<Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
) -> Result<(), JsonToolsError> {
    if index >= parts.len() {
        return Err(JsonToolsError::invalid_json_structure(
            "Invalid path for array",
        ));
    }

    let part = parts[index];

    if let Ok(array_index) = part.parse::<usize>() {
        while arr.len() <= array_index {
            arr.push(Value::Null);
        }

        if index == parts.len() - 1 {
            arr[array_index] = value;
            Ok(())
        } else {
            // Build path in buffer for type lookup
            let buffer_start_len = path_buffer.len();
            if buffer_start_len > 0 {
                path_buffer.push_str(separator);
            }
            path_buffer.push_str(part);

            let next_should_be_array = path_types
                .get(path_buffer.as_str())
                .copied()
                .unwrap_or(false);

            if arr[array_index].is_null() {
                arr[array_index] = if next_should_be_array {
                    Value::Array(vec![])
                } else {
                    Value::Object(Map::new())
                };
            }

            let result = match &mut arr[array_index] {
                Value::Object(ref mut obj) => set_nested_value_recursive(
                    obj,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                    path_buffer,
                ),
                Value::Array(ref mut nested_arr) => set_nested_array_value(
                    nested_arr,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                    path_buffer,
                ),
                _ => Err(JsonToolsError::invalid_json_structure(format!(
                    "Array element at index {} has incompatible type",
                    array_index
                ))),
            };

            // Restore buffer to its state before this call
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
