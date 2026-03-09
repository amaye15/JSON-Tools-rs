use memchr::memmem;
use rustc_hash::FxHashMap;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::sync::Arc;

use crate::cache::get_cached_regex;
use crate::config::{FilteringConfig, ProcessingConfig};
use crate::convert::apply_type_conversion_recursive;
use crate::error::JsonToolsError;
use crate::json_parser;
use crate::types::FlatMap;

// ================================================================================================
// Key/Value Replacement Core Logic
// ================================================================================================

/// Core key replacement logic that works with both string keys and Cow<str>
/// This eliminates duplication between flatten and unflatten key replacement functions
/// Optimized to minimize string allocations by using efficient Cow operations
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
#[inline]
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

/// Core value replacement logic that works with any Value type
/// This eliminates duplication between flatten and unflatten value replacement functions
/// Optimized to minimize string allocations by using efficient Cow operations
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
#[inline]
pub(crate) fn apply_value_replacement_patterns(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    if let Value::String(s) = value {
        let mut current_value = Cow::Borrowed(s.as_str());
        let mut changed = false;

        // Apply each replacement pattern
        for (pattern, replacement) in patterns {
            // Try to compile as regex first
            match get_cached_regex(pattern) {
                Ok(regex) => {
                    // Use replace_all's Cow return to detect matches without a separate is_match()
                    // scan -- Cow::Owned means replacement happened, Cow::Borrowed means no match.
                    if let Cow::Owned(s) = regex.replace_all(&current_value, replacement.as_str()) {
                        current_value = Cow::Owned(s);
                        changed = true;
                    }
                }
                Err(_) => {
                    // Failed to compile as regex - fall back to literal replacement
                    // Use SIMD-accelerated memmem::find instead of str::contains
                    if memmem::find(current_value.as_bytes(), pattern.as_bytes()).is_some() {
                        current_value = Cow::Owned(current_value.replace(pattern, replacement));
                        changed = true;
                    }
                }
            }
        }

        if changed {
            *s = current_value.into_owned();
        }
    }
    Ok(())
}

// ================================================================================================
// Collision Detection and Grouping
// ================================================================================================

/// Core collision detection and grouping logic
/// This eliminates duplication between flatten and unflatten collision handling
#[inline]
pub(crate) fn group_items_by_key<K, V>(items: impl Iterator<Item = (K, V)>) -> FxHashMap<K, Vec<V>>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    // Pre-allocate with estimated capacity for better performance
    let mut key_groups: FxHashMap<K, Vec<V>> =
        FxHashMap::with_capacity_and_hasher(64, Default::default());
    for (key, value) in items {
        key_groups.entry(key).or_default().push(value);
    }
    key_groups
}

/// Apply collision handling strategy by collecting values into arrays
#[inline]
pub(crate) fn apply_collision_handling<K>(
    key: K,
    values: Vec<Value>,
    filter_config: Option<&FilteringConfig>,
) -> Option<(K, Value)> {
    let filtered_values: Vec<Value> = if let Some(config) = filter_config {
        values
            .into_iter()
            .filter(|value| {
                should_include_value(
                    value,
                    config.remove_empty_strings,
                    config.remove_nulls,
                    config.remove_empty_objects,
                    config.remove_empty_arrays,
                )
            })
            .collect()
    } else {
        values
    };

    if !filtered_values.is_empty() {
        Some((key, Value::Array(filtered_values)))
    } else {
        None
    }
}

// ================================================================================================
// Flatten-Specific Transformations
// ================================================================================================

/// Apply key transformations including replacements and lowercase conversion for flattening
#[inline]
pub(crate) fn apply_key_transformations_flatten(
    mut flattened: FlatMap,
    config: &ProcessingConfig,
) -> Result<FlatMap, JsonToolsError> {
    // Apply key replacements with collision detection if provided
    if config.replacements.has_key_replacements() {
        // Convert tuple format to the internal vector format
        let key_patterns = convert_tuples_to_patterns(&config.replacements.key_replacements);

        // Use the consolidated function that handles both optimized and collision scenarios
        flattened =
            apply_key_replacements_with_collision_handling(flattened, &key_patterns, config)?;
    }

    // Apply lowercase conversion to keys if requested
    if config.lowercase_keys {
        flattened = apply_lowercase_keys(flattened);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if config.collision.has_collision_handling() && !config.replacements.has_key_replacements()
        {
            flattened = handle_key_collisions(flattened, config.collision.handle_collisions);
        }
    } else if config.collision.has_collision_handling()
        && !config.replacements.has_key_replacements()
    {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        flattened = handle_key_collisions(flattened, config.collision.handle_collisions);
    }

    Ok(flattened)
}

/// Apply value replacements to flattened data
#[inline]
pub(crate) fn apply_value_replacements_flatten(
    flattened: &mut FlatMap,
    config: &ProcessingConfig,
) -> Result<(), JsonToolsError> {
    if config.replacements.has_value_replacements() {
        // Convert tuple format to the internal vector format
        let value_patterns = convert_tuples_to_patterns(&config.replacements.value_replacements);
        apply_value_replacements(flattened, &value_patterns)?;
    }
    Ok(())
}

/// Apply filtering to flattened data after replacements
#[inline]
pub(crate) fn apply_filtering_flatten(flattened: &mut FlatMap, config: &ProcessingConfig) {
    if !config.filtering.has_any_filter() {
        return;
    }

    // TIER 4->3 OPTIMIZATION: Single-pass filtering instead of two passes
    // Merge array element filtering and top-level filtering into one iteration
    // Saves 50K-100K cycles by avoiding second HashMap iteration
    let handle_collisions = config.collision.handle_collisions;
    let remove_empty_strings = config.filtering.remove_empty_strings;
    let remove_nulls = config.filtering.remove_nulls;
    let remove_empty_objects = config.filtering.remove_empty_objects;
    let remove_empty_arrays = config.filtering.remove_empty_arrays;

    flattened.retain(|_, v| {
        // Filter elements inside collision-created arrays (if collision handling enabled)
        if handle_collisions {
            if let Some(arr) = v.as_array_mut() {
                arr.retain(|element| {
                    should_include_value(
                        element,
                        remove_empty_strings,
                        remove_nulls,
                        remove_empty_objects,
                        remove_empty_arrays,
                    )
                });
            }
        }

        // Filter top-level entry
        should_include_value(
            v,
            remove_empty_strings,
            remove_nulls,
            remove_empty_objects,
            remove_empty_arrays,
        )
    });
}

// ================================================================================================
// Utility Functions
// ================================================================================================

/// Convert tuple-based replacement patterns to the internal vector format
/// This converts the intuitive tuple format to the internal representation used by replacement functions
/// Optimized to use string references instead of cloning to reduce memory allocations
#[inline]
pub(crate) fn convert_tuples_to_patterns(tuples: &[(String, String)]) -> Vec<&str> {
    let mut patterns = Vec::with_capacity(tuples.len() * 2);
    for (pattern, replacement) in tuples {
        patterns.push(pattern.as_str());
        patterns.push(replacement.as_str());
    }
    patterns
}

/// Apply lowercase conversion to all keys in the flattened HashMap
/// This function creates a new HashMap with all keys converted to lowercase
/// Optimized with Cow to avoid unnecessary allocations when keys are already lowercase
/// Uses Entry API for potential collision handling
#[inline]
pub(crate) fn apply_lowercase_keys(flattened: FlatMap) -> FlatMap {
    // TIER 6->2 OPTIMIZATION: Early-exit if all keys are already lowercase
    // Avoids expensive HashMap allocation + all value moves when no transformation needed
    // This is critical because most JSON keys are already lowercase in practice
    let needs_lowercasing = flattened
        .keys()
        .any(|key| key.as_bytes().iter().any(|b| b.is_ascii_uppercase()));

    // Fast path: All keys already lowercase, return original map
    if !needs_lowercasing {
        return flattened;
    }

    // Slow path: Some keys need lowercasing, build new map
    let optimal_capacity = calculate_optimal_capacity(flattened.len());
    let mut result = FxHashMap::with_capacity_and_hasher(optimal_capacity, Default::default());

    for (key, value) in flattened {
        // SIMD-optimized lowercase conversion (zero-copy if already lowercase)
        let lowercase_key = to_lowercase(&key);

        let final_key: Arc<str> = match lowercase_key {
            Cow::Borrowed(_) => key, // Key was already lowercase, reuse original Arc
            Cow::Owned(lower) => lower.into(), // Convert lowercase String to Arc<str>
        };

        // Use Entry API to handle potential collisions more efficiently
        match result.entry(final_key) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                // Handle collision by converting to array
                let existing_value = entry.get_mut();
                match existing_value {
                    Value::Array(arr) => {
                        arr.push(value);
                    }
                    _ => {
                        let old_value = std::mem::replace(existing_value, Value::Null);
                        *existing_value = Value::Array(vec![old_value, value]);
                    }
                }
            }
        }
    }
    result
}

/// Estimates optimal HashMap capacity based on expected size and load factor
pub(crate) fn calculate_optimal_capacity(estimated_size: usize) -> usize {
    if estimated_size == 0 {
        return 16; // Minimum reasonable capacity
    }

    // Use a load factor of 0.75 to minimize rehashing while not wasting too much memory
    let target_capacity = (estimated_size as f64 / 0.75).ceil() as usize;

    // Round up to next power of 2 for optimal HashMap performance
    let next_power_of_2 = target_capacity.next_power_of_two();

    // OPTIMIZATION #18: Balance between avoiding rehashes and excessive pre-allocation
    // Cap at 64K entries (2^16) as a middle ground:
    // - Prevents excessive memory allocation for typical use cases
    // - Still allows pre-allocation for moderately large datasets
    // - Very large datasets will rehash once or twice, which is acceptable
    let max_capacity = 65536; // 2^16
    std::cmp::min(next_power_of_2, max_capacity)
}

// ================================================================================================
// Normal Mode Processing
// ================================================================================================

/// Core normal-mode logic for a single JSON string with Cow optimization
/// Applies key/value transformations and filtering recursively without changing structure
#[inline]
pub(crate) fn process_single_json_normal(
    json: &str,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // Parse JSON using optimized SIMD parsing
    let mut value = crate::json_parser::parse_json(json)?;

    // Apply type conversion FIRST (before other transformations)
    if config.auto_convert_types {
        apply_type_conversion_recursive(&mut value);
    }

    // Apply value replacements recursively to all strings
    if config.replacements.has_value_replacements() {
        apply_value_replacements_recursive(&mut value, &config.replacements.value_replacements)?;
    }

    // Apply key transformations (key replacements and lowercase), with collision handling
    if config.replacements.has_key_replacements()
        || config.lowercase_keys
        || config.collision.has_collision_handling()
    {
        value = apply_key_transformations_normal(value, config)?;
    }

    // Apply filtering recursively after replacements and key transformations
    if config.filtering.has_any_filter() {
        filter_nested_value(
            &mut value,
            config.filtering.remove_empty_strings,
            config.filtering.remove_nulls,
            config.filtering.remove_empty_objects,
            config.filtering.remove_empty_arrays,
        );
    }

    // Serialize back to JSON
    Ok(json_parser::to_string(&value)?)
}

/// Recursively apply value replacements to all string values
fn apply_value_replacements_recursive(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    match value {
        Value::Object(map) => {
            for v in map.values_mut() {
                apply_value_replacements_recursive(v, patterns)?;
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                apply_value_replacements_recursive(v, patterns)?;
            }
        }
        _ => {
            // Apply to primitive string values
            apply_value_replacement_patterns(value, patterns)?;
        }
    }
    Ok(())
}

/// Apply key replacements and lowercase to all object keys recursively, with collision handling
fn apply_key_transformations_normal(
    value: Value,
    config: &ProcessingConfig,
) -> Result<Value, JsonToolsError> {
    match value {
        Value::Object(map) => {
            // Transform this level's keys first
            let mut transformed: Map<String, Value> = Map::with_capacity(map.len());
            for (key, v) in map.into_iter() {
                // Recurse into child first
                let v = apply_key_transformations_normal(v, config)?;

                // Apply key replacement patterns
                let new_key: String = if config.replacements.has_key_replacements() {
                    if let Some(repl) =
                        apply_key_replacement_patterns(&key, &config.replacements.key_replacements)?
                    {
                        repl
                    } else {
                        key
                    }
                } else {
                    key
                };

                // Apply lowercase if needed
                let final_key = if config.lowercase_keys {
                    new_key.to_lowercase()
                } else {
                    new_key
                };

                // Insert; we'll handle collisions later
                transformed.insert(final_key, v);
            }

            // Handle key collisions if requested
            let result_map = if config.collision.has_collision_handling() {
                handle_key_collisions_for_unflatten(transformed, config)
            } else {
                transformed
            };

            Ok(Value::Object(result_map))
        }
        Value::Array(mut arr) => {
            for item in &mut arr {
                let v = std::mem::take(item);
                *item = apply_key_transformations_normal(v, config)?;
            }
            Ok(Value::Array(arr))
        }
        other => Ok(other),
    }
}

// ================================================================================================
// Value Replacements for FlatMap
// ================================================================================================

/// Value replacement with regex caching - optimized to use string references
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
pub(crate) fn apply_value_replacements(
    flattened: &mut FlatMap,
    patterns: &[&str],
) -> Result<(), JsonToolsError> {
    if !patterns.len().is_multiple_of(2) {
        return Err(JsonToolsError::invalid_replacement_pattern(
            "Value replacement patterns must be provided in pairs (pattern, replacement)",
        ));
    }

    // Pre-compile all regex patterns (or mark as literal if compilation fails)
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        // Try to compile as regex
        match get_cached_regex(pattern) {
            Ok(regex) => compiled_patterns.push((Some(regex), replacement)),
            Err(_) => compiled_patterns.push((None, replacement)),
        }
    }

    for (_, value) in flattened.iter_mut() {
        if let Value::String(s) = value {
            let mut new_value = Cow::Borrowed(s.as_str());
            let mut changed = false;

            // Apply each compiled pattern
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_value) {
                        new_value =
                            Cow::Owned(regex.replace_all(&new_value, *replacement).to_string());
                        changed = true;
                    }
                } else if new_value.contains(pattern) {
                    new_value = Cow::Owned(new_value.replace(pattern, replacement));
                    changed = true;
                }
            }

            if changed {
                *value = Value::String(new_value.into_owned());
            }
        }
    }

    Ok(())
}

/// Ultra-fast JSON serialization using sonic-rs
///
/// TIER 6->Direct OPTIMIZATION: Serialize FlatMap directly without intermediate conversion
/// The intermediate HashMap<&str, &Value> allocation is removed (saves ~500 cycles for large maps)
/// Note: This works because both Arc<str> and FxHashMap implement Serialize
#[inline]
pub(crate) fn serialize_flattened(flattened: &FlatMap) -> Result<String, crate::json_parser::JsonError> {
    // Direct serialization - no intermediate HashMap needed
    // Arc<str> implements Serialize via Deref to str
    // FxHashMap implements Serialize just like std HashMap
    json_parser::to_string(flattened)
}

// ================================================================================================
// Unflatten-Specific Transformations
// ================================================================================================

/// Apply key replacements for unflattening (works on Map<String, Value>)
/// This version is used when collision handling is NOT enabled for better performance
/// Takes ownership to avoid cloning values
pub(crate) fn apply_key_replacements_for_unflatten(
    obj: Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<Map<String, Value>, JsonToolsError> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        // Use the unified key replacement logic
        let final_key = if let Some(new_key) = apply_key_replacement_patterns(&key, patterns)? {
            new_key
        } else {
            key
        };

        // No clone needed - we own the value!
        new_obj.insert(final_key, value);
    }

    Ok(new_obj)
}

/// Apply value replacements for unflattening (works on Map<String, Value>)
/// Optimized with Cow to avoid unnecessary string allocations
pub(crate) fn apply_value_replacements_for_unflatten(
    obj: &mut Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    for (_, value) in obj.iter_mut() {
        // Use the unified value replacement logic
        apply_value_replacement_patterns(value, patterns)?;
    }
    Ok(())
}

/// Apply lowercase conversion to keys for unflattening
/// Optimized with Cow to avoid unnecessary allocations when keys are already lowercase
pub(crate) fn apply_lowercase_keys_for_unflatten(obj: Map<String, Value>) -> Map<String, Value> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        // SIMD-optimized lowercase conversion (zero-copy if already lowercase)
        let lowercase_key = to_lowercase(&key);

        let final_key = match lowercase_key {
            Cow::Borrowed(_) => key,    // Key was already lowercase, reuse original
            Cow::Owned(lower) => lower, // Key was converted to lowercase
        };

        new_obj.insert(final_key, value);
    }

    new_obj
}

// ================================================================================================
// Filtering Functions
// ================================================================================================

/// Helper function to check if a value should be filtered out based on criteria
/// Consolidates the filtering logic used by both objects and arrays
#[inline]
pub(crate) fn should_filter_value(
    v: &Value,
    remove_empty_strings: bool,
    remove_nulls: bool,
    remove_empty_objects: bool,
    remove_empty_arrays: bool,
) -> bool {
    if remove_empty_strings {
        if let Some(s) = v.as_str() {
            if s.is_empty() {
                return true;
            }
        }
    }
    if remove_nulls && v.is_null() {
        return true;
    }
    if remove_empty_objects {
        if let Some(obj) = v.as_object() {
            if obj.is_empty() {
                return true;
            }
        }
    }
    if remove_empty_arrays {
        if let Some(arr) = v.as_array() {
            if arr.is_empty() {
                return true;
            }
        }
    }
    false
}

/// Recursively filter nested JSON values based on the specified criteria
/// This function removes empty strings, nulls, empty objects, and empty arrays from nested JSON structures
/// OPTIMIZATION #11: Early exit when no filters are enabled
pub(crate) fn filter_nested_value(
    value: &mut Value,
    remove_empty_strings: bool,
    remove_nulls: bool,
    remove_empty_objects: bool,
    remove_empty_arrays: bool,
) {
    // OPTIMIZATION: Early exit if no filters enabled - avoid recursing through entire tree
    if !remove_empty_strings && !remove_nulls && !remove_empty_objects && !remove_empty_arrays {
        return;
    }

    match value {
        Value::Object(ref mut obj) => {
            // First, recursively filter all nested values
            for (_, v) in obj.iter_mut() {
                filter_nested_value(
                    v,
                    remove_empty_strings,
                    remove_nulls,
                    remove_empty_objects,
                    remove_empty_arrays,
                );
            }

            // Then remove keys that match our filtering criteria
            obj.retain(|_, v| {
                !should_filter_value(
                    v,
                    remove_empty_strings,
                    remove_nulls,
                    remove_empty_objects,
                    remove_empty_arrays,
                )
            });
        }
        Value::Array(ref mut arr) => {
            // First, recursively filter all nested values
            for item in arr.iter_mut() {
                filter_nested_value(
                    item,
                    remove_empty_strings,
                    remove_nulls,
                    remove_empty_objects,
                    remove_empty_arrays,
                );
            }

            // Then remove array elements that match our filtering criteria
            arr.retain(|v| {
                !should_filter_value(
                    v,
                    remove_empty_strings,
                    remove_nulls,
                    remove_empty_objects,
                    remove_empty_arrays,
                )
            });
        }
        _ => {
            // For primitive values (strings, numbers, booleans, null), no filtering needed
            // The filtering will be handled by the parent container
        }
    }
}

/// Helper function to determine if a value should be included based on filtering criteria
/// This ensures consistent filtering logic across both flatten and unflatten operations
/// OPTIMIZATION #13: Force inline for hot path
#[inline(always)]
pub(crate) fn should_include_value(
    value: &Value,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> bool {
    // Check for empty strings
    if remove_empty_string_values {
        if let Some(s) = value.as_str() {
            if s.is_empty() {
                return false;
            }
        }
    }

    // Check for nulls
    if remove_null_values && value.is_null() {
        return false;
    }

    // Check for empty objects
    if remove_empty_dict {
        if let Some(obj) = value.as_object() {
            if obj.is_empty() {
                return false;
            }
        }
    }

    // Check for empty arrays
    if remove_empty_list {
        if let Some(arr) = value.as_array() {
            if arr.is_empty() {
                return false;
            }
        }
    }

    true
}

// ================================================================================================
// Collision Handling
// ================================================================================================

/// Handle key collisions in a flattened map
///
/// This function processes a HashMap to handle cases where multiple keys would collide
/// after key replacements and transformations. It supports two strategies:
///
/// Only supported strategy: `handle_key_collision` to collect values into arrays for duplicate keys
pub(crate) fn handle_key_collisions(mut flattened: FlatMap, handle_key_collision: bool) -> FlatMap {
    // If option is disabled, return as-is
    if !handle_key_collision {
        return flattened;
    }

    // OPTIMIZATION: Fast path - check for duplicates with early exit
    // Common case: no duplicates exist, so avoid expensive grouping operation
    let items: Vec<_> = flattened.drain().collect();
    let mut seen_keys = FxHashMap::with_capacity_and_hasher(items.len(), Default::default());
    let mut has_duplicates = false;

    for (key, _) in &items {
        if seen_keys.insert(key, ()).is_some() {
            has_duplicates = true;
            break; // Early exit as soon as we find one duplicate
        }
    }

    // Fast path: no duplicates found, reconstruct and return
    if !has_duplicates {
        return items.into_iter().collect();
    }

    // Slow path: duplicates exist, use full grouping logic
    let key_groups = group_items_by_key(items.into_iter());
    let mut result = FxHashMap::with_capacity_and_hasher(key_groups.len(), Default::default());

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected: collect values into array
            if let Some((final_key, array_value)) = apply_collision_handling(key, values, None) {
                result.insert(final_key, array_value);
            }
        }
    }

    result
}

/// Handle key collisions for unflattening operations
///
/// This function processes a Map<String, Value> (flattened object) to handle cases where
/// multiple keys would collide after key replacements and transformations.
/// Only supported strategy: collect values into arrays when enabled.
pub(crate) fn handle_key_collisions_for_unflatten(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Map<String, Value> {
    // If option is disabled, return as-is
    if !config.collision.handle_collisions {
        return flattened_obj;
    }

    // OPTIMIZATION: Fast path - check for duplicates with early exit
    // Common case: no duplicates exist, so avoid expensive grouping operation
    let items: Vec<_> = flattened_obj.into_iter().collect();
    let mut seen_keys = FxHashMap::with_capacity_and_hasher(items.len(), Default::default());
    let mut has_duplicates = false;

    for (key, _) in &items {
        if seen_keys.insert(key, ()).is_some() {
            has_duplicates = true;
            break; // Early exit as soon as we find one duplicate
        }
    }

    // Fast path: no duplicates found, reconstruct and return
    if !has_duplicates {
        return items.into_iter().collect();
    }

    // Slow path: duplicates exist, use full grouping logic
    let key_groups = group_items_by_key(items.into_iter());
    let mut result = Map::new();

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected: collect values into array, with filtering
            if let Some((final_key, array_value)) =
                apply_collision_handling(key, values, Some(&config.filtering))
            {
                result.insert(final_key, array_value);
            }
        }
    }

    result
}

/// Apply key replacements with collision handling for flattening operations
///
/// This function combines key replacement and collision detection with performance optimizations
/// including regex pre-compilation, early exit checks, and efficient string handling.
/// It properly handles cases where multiple keys would map to the same result after replacement.
pub(crate) fn apply_key_replacements_with_collision_handling(
    flattened: FlatMap,
    patterns: &[&str],
    config: &ProcessingConfig,
) -> Result<FlatMap, JsonToolsError> {
    if patterns.is_empty() {
        return Ok(flattened);
    }

    if !patterns.len().is_multiple_of(2) {
        return Err(JsonToolsError::invalid_replacement_pattern(
            "Patterns must be provided in pairs (find, replace)",
        ));
    }

    // Pre-compile all regex patterns to avoid repeated compilation
    // Patterns are treated as regex. If compilation fails, fall back to literal matching.
    // Store (regex, pattern_literal, replacement) so inner loops never need patterns.chunks(2)
    let mut compiled_patterns: Vec<(Option<Arc<regex::Regex>>, &str, &str)> =
        Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        // Try to compile as regex
        match get_cached_regex(pattern) {
            Ok(regex) => compiled_patterns.push((Some(regex), pattern, replacement)),
            Err(_) => compiled_patterns.push((None, pattern, replacement)),
        }
    }

    // TIER 6->3 OPTIMIZATION: Early-exit check without value cloning
    // Check if any replacements are needed BEFORE cloning values (critical for performance!)
    // This avoids expensive value cloning when no keys match patterns
    if !config.collision.handle_collisions {
        // FAST PATH: Check if any key needs replacement (no value cloning)
        let needs_replacement = flattened.keys().any(|key| {
            compiled_patterns
                .iter()
                .any(|(compiled_regex, pattern, _)| {
                    if let Some(regex) = compiled_regex {
                        regex.is_match(key)
                    } else {
                        key.contains(*pattern)
                    }
                })
        });

        // Early exit if no replacements needed - avoids value cloning entirely
        if !needs_replacement {
            return Ok(flattened);
        }

        // SLOW PATH: Replacements needed, build new map with value cloning
        let mut new_flattened =
            FxHashMap::with_capacity_and_hasher(flattened.len(), Default::default());

        for (old_key, value) in flattened {
            let mut new_key = Cow::Borrowed(old_key.as_ref());

            // Apply each compiled pattern
            for (compiled_regex, pattern, replacement) in &compiled_patterns {
                if let Some(regex) = compiled_regex {
                    if let Cow::Owned(s) = regex.replace_all(&new_key, *replacement) {
                        new_key = Cow::Owned(s);
                    }
                } else if new_key.contains(*pattern) {
                    new_key = Cow::Owned(new_key.replace(*pattern, replacement));
                }
            }

            // Reuse existing Arc if key unchanged (avoids string copy + Arc allocation)
            let key_arc: Arc<str> = match new_key {
                Cow::Owned(s) => Arc::from(s),
                Cow::Borrowed(_) => old_key,
            };
            new_flattened.insert(key_arc, value);
        }

        return Ok(new_flattened);
    }

    // OPTIMIZATION #19: Single-pass collision handling using HashMap::entry API
    // Instead of 3 separate passes (key_mapping, target_groups, result), we build
    // the result directly in one iteration using the entry API to handle collisions
    let flattened_len = flattened.len();
    let mut result: FlatMap =
        FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());

    for (original_key, value) in flattened {
        let mut new_key = Cow::Borrowed(original_key.as_ref());

        // Apply all key replacement patterns using pre-compiled patterns
        for (compiled_regex, pattern, replacement) in &compiled_patterns {
            if let Some(regex) = compiled_regex {
                if let Cow::Owned(s) = regex.replace_all(&new_key, *replacement) {
                    new_key = Cow::Owned(s);
                }
            } else if new_key.contains(*pattern) {
                new_key = Cow::Owned(new_key.replace(*pattern, replacement));
            }
        }

        // Apply filtering before inserting
        let should_include = should_include_value(
            &value,
            config.filtering.remove_empty_strings,
            config.filtering.remove_nulls,
            config.filtering.remove_empty_objects,
            config.filtering.remove_empty_arrays,
        );

        if !should_include {
            continue;
        }

        // Reuse existing Arc if key unchanged (avoids string copy + Arc allocation)
        let new_key_arc: Arc<str> = match new_key {
            Cow::Owned(s) => Arc::from(s),
            Cow::Borrowed(_) => original_key,
        };
        match result.entry(new_key_arc) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                // No collision - insert value directly
                entry.insert(value);
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                // Collision detected - convert to array or append
                let existing = entry.get_mut();
                match existing {
                    Value::Array(arr) => {
                        // Already an array from previous collision - append
                        arr.push(value);
                    }
                    _ => {
                        // First collision - convert existing value to array with both values
                        let old_value = std::mem::replace(existing, Value::Null);
                        *existing = Value::Array(vec![old_value, value]);
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Apply key replacements with collision handling for unflattening operations
///
/// This function combines key replacement and collision detection for Map<String, Value>
/// to properly handle cases where multiple keys would map to the same result after replacement.
pub(crate) fn apply_key_replacements_unflatten_with_collisions(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Result<Map<String, Value>, JsonToolsError> {
    if config.replacements.key_replacements.is_empty() {
        return Ok(flattened_obj);
    }

    // First pass: apply replacements and track what each original key maps to
    let flattened_len = flattened_obj.len();
    let mut key_mapping: FxHashMap<String, String> =
        FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());
    let mut original_values: FxHashMap<String, Value> =
        FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());

    for (original_key, value) in flattened_obj {
        // Apply all key replacement patterns using Cow to avoid allocation if no replacement
        let mut new_key = Cow::Borrowed(original_key.as_ref());

        // Apply all key replacement patterns
        // Patterns are treated as regex. If compilation fails, fall back to literal matching.
        for (find, replace) in &config.replacements.key_replacements {
            // Try to compile as regex first
            match get_cached_regex(find) {
                Ok(regex) => {
                    if let Cow::Owned(s) = regex.replace_all(&new_key, replace) {
                        new_key = Cow::Owned(s);
                    }
                }
                Err(_) => {
                    // Failed to compile as regex - fall back to literal replacement
                    if new_key.contains(find) {
                        new_key = Cow::Owned(new_key.replace(find, replace));
                    }
                }
            }
        }

        let new_key_string = new_key.into_owned();
        // OPTIMIZATION: Insert original_values first with owned original_key,
        // then clone for key_mapping (avoids cloning new_key_string)
        let original_key_clone = original_key.clone();
        original_values.insert(original_key, value);
        key_mapping.insert(original_key_clone, new_key_string);
    }

    // Second pass: group by target key to detect collisions
    // OPTIMIZATION: Consume key_mapping to avoid cloning target_key
    let mut target_groups: FxHashMap<String, Vec<String>> =
        FxHashMap::with_capacity_and_hasher(key_mapping.len(), Default::default());
    for (original_key, target_key) in key_mapping {
        target_groups
            .entry(target_key)
            .or_default()
            .push(original_key);
    }

    // Third pass: build result with collision handling
    let mut result = Map::with_capacity(target_groups.len());

    for (target_key, original_keys) in target_groups {
        if original_keys.len() == 1 {
            // No collision
            let original_key = &original_keys[0];
            let value = original_values.remove(original_key).unwrap();
            result.insert(target_key, value);
        } else {
            // Collision detected: Only supported strategy is collecting into arrays
            let mut values = Vec::with_capacity(original_keys.len());
            for original_key in &original_keys {
                let value = original_values.remove(original_key).unwrap();

                // Apply filtering to values before adding to collision array
                let should_include = should_include_value(
                    &value,
                    config.filtering.remove_empty_strings,
                    config.filtering.remove_nulls,
                    config.filtering.remove_empty_objects,
                    config.filtering.remove_empty_arrays,
                );

                if should_include {
                    values.push(value);
                }
            }

            // Only create the array if we have values after filtering
            if !values.is_empty() {
                result.insert(target_key, Value::Array(values));
            }
            // If all values were filtered out, don't insert anything
        }
    }

    Ok(result)
}

// ================================================================================================
// Lowercase Utility
// ================================================================================================

/// TIER 2-3 OPTIMIZATION: Lowercase conversion with fast uppercase detection
///
/// Optimizations:
/// - Explicit range check (b'A'..=b'Z') instead of is_ascii_uppercase() for better vectorization
/// - Compiler auto-vectorizes the range check for medium/long strings
/// - Returns Borrowed (zero-copy) if already lowercase (most common case)
/// - Only allocates for uppercase conversion (rare case)
#[inline]
pub(crate) fn to_lowercase(s: &str) -> Cow<'_, str> {
    let bytes = s.as_bytes();

    let has_uppercase = bytes.iter().any(|b| b.is_ascii_uppercase());

    if has_uppercase {
        Cow::Owned(s.to_lowercase())
    } else {
        // Zero-copy fast path: already lowercase
        Cow::Borrowed(s)
    }
}
