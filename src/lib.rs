//! # JSON Tools RS
//!
//! A Rust library for advanced JSON manipulation, including flattening nested JSON structures
//! with configurable filtering and replacement options.
//!
//! ## Features
//!
//! - Flatten nested JSON structures using dot notation
//! - Remove empty values (strings, objects, arrays, null values)
//! - Replace keys and values using literal strings or regex patterns
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust
//! use json_tools_rs::flatten_json;
//!
//! let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
//! let result = test_flatten_json_with_params(json, true, true, false, false, None, None, None, false).unwrap();
//! // Result: {"user.name": "John"}
//! ```

use regex::Regex;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Python bindings module
#[cfg(feature = "python")]
pub mod python;

/// Input type for JSON flattening operations
#[derive(Debug, Clone)]
pub enum JsonInput<'a> {
    /// Single JSON string
    Single(&'a str),
    /// Multiple JSON strings
    Multiple(&'a [&'a str]),
}

impl<'a> From<&'a str> for JsonInput<'a> {
    fn from(json: &'a str) -> Self {
        JsonInput::Single(json)
    }
}

impl<'a> From<&'a String> for JsonInput<'a> {
    fn from(json: &'a String) -> Self {
        JsonInput::Single(json.as_str())
    }
}

impl<'a> From<&'a [&'a str]> for JsonInput<'a> {
    fn from(json_list: &'a [&'a str]) -> Self {
        JsonInput::Multiple(json_list)
    }
}

impl<'a> From<Vec<&'a str>> for JsonInput<'a> {
    fn from(json_list: Vec<&'a str>) -> Self {
        JsonInput::Multiple(json_list.leak())
    }
}

/// Output type for JSON flattening operations
#[derive(Debug, Clone)]
pub enum JsonOutput {
    /// Single flattened JSON string
    Single(String),
    /// Multiple flattened JSON strings
    Multiple(Vec<String>),
}

impl JsonOutput {
    /// Extract single result, panicking if multiple results
    pub fn into_single(self) -> String {
        match self {
            JsonOutput::Single(result) => result,
            JsonOutput::Multiple(_) => panic!("Expected single result but got multiple"),
        }
    }

    /// Extract multiple results, panicking if single result
    pub fn into_multiple(self) -> Vec<String> {
        match self {
            JsonOutput::Single(_) => panic!("Expected multiple results but got single"),
            JsonOutput::Multiple(results) => results,
        }
    }

    /// Get results as vector (single result becomes vec with one element)
    pub fn into_vec(self) -> Vec<String> {
        match self {
            JsonOutput::Single(result) => vec![result],
            JsonOutput::Multiple(results) => results,
        }
    }
}

/// Custom error type for JSON flattening operations
#[derive(Debug)]
pub enum FlattenError {
    /// Error parsing JSON input
    JsonParseError(simd_json::Error),
    /// Error compiling or using regex patterns
    RegexError(regex::Error),
    /// Invalid replacement pattern configuration
    InvalidReplacementPattern(String),
    /// Error processing batch item with index
    BatchError {
        index: usize,
        error: Box<FlattenError>,
    },
}

impl fmt::Display for FlattenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlattenError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            FlattenError::RegexError(e) => write!(f, "Regex error: {}", e),
            FlattenError::InvalidReplacementPattern(msg) => {
                write!(f, "Invalid replacement pattern: {}", msg)
            }
            FlattenError::BatchError { index, error } => {
                write!(f, "Error processing JSON at index {}: {}", index, error)
            }
        }
    }
}

impl Error for FlattenError {}

impl From<simd_json::Error> for FlattenError {
    fn from(error: simd_json::Error) -> Self {
        FlattenError::JsonParseError(error)
    }
}

impl From<regex::Error> for FlattenError {
    fn from(error: regex::Error) -> Self {
        FlattenError::RegexError(error)
    }
}

/// Flattens a nested JSON structure into a single-level structure with dot-notation keys
///
/// # Arguments
///
/// * `json` - A JSON string to be flattened
/// * `remove_empty_string_values` - If true, remove key-value pairs where the value is an empty string `""`
/// * `remove_null_values` - If true, remove key-value pairs where the value is `null`
/// * `remove_empty_dict` - If true, remove key-value pairs where the value is an empty object `{}`
/// * `remove_empty_list` - If true, remove key-value pairs where the value is an empty array `[]`
/// * `key_replacements` - Optional vector of replacement patterns for keys. Each pattern can be either a literal string or a regex pattern (prefixed with "regex:")
/// * `value_replacements` - Optional vector of replacement patterns for values. Each pattern can be either a literal string or a regex pattern (prefixed with "regex:")
/// * `lower_case_keys` - If true, convert all uppercase characters in the flattened JSON keys to lowercase. This transformation is applied after flattening and regex transformations but before value replacements.
///
/// # Returns
///
/// Returns a `Result` containing the flattened JSON as a string, or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The input JSON is invalid
/// - Regex patterns are malformed
/// - Replacement patterns are not provided in pairs
///
/// # Examples
///
/// ## Basic flattening
/// ```rust
/// use json_tools_rs::flatten_json;
///
/// let json = r#"{"a": {"b": {"c": 1}}}"#;
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, None, false).unwrap();
/// // Result: {"a.b.c": 1}
/// ```
///
/// ## Removing empty values
/// ```rust
/// use json_tools_rs::flatten_json;
///
/// let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
/// let result = test_flatten_json_with_params(json, true, true, false, false, None, None, None, false).unwrap();
/// // Result: {"user.name": "John"}
/// ```
///
/// ## Key replacement with regex
/// ```rust
/// use json_tools_rs::flatten_json;
///
/// let json = r#"{"user_name": "John", "admin_role": "super"}"#;
/// let key_replacements = Some(vec![("regex:^(user|admin)_".to_string(), "".to_string())]);
/// let result = test_flatten_json_with_params(json, false, false, false, false, key_replacements, None, None, false).unwrap();
/// // Result: {"name": "John", "role": "super"}
/// ```
///
/// ## Lowercase key conversion
/// ```rust
/// use json_tools_rs::flatten_json;
///
/// let json = r#"{"User": {"Name": "John", "Email": "john@example.com"}}"#;
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, None, true).unwrap();
/// // Result: {"user.name": "John", "user.email": "john@example.com"}
/// ```
/// Unified flatten_json function that accepts either single or multiple JSON strings
///
/// This function can process either a single JSON string or multiple JSON strings,
/// automatically detecting the input type and returning the appropriate output format.
/// It flattens nested JSON structures using configurable key separators, including comprehensive
/// array flattening with numeric indices.
///
/// # Arguments
///
/// * `json_input` - Either a single JSON string (&str) or multiple JSON strings (&[&str])
/// * `remove_empty_string_values` - If true, remove key-value pairs where the value is an empty string `""`
/// * `remove_null_values` - If true, remove key-value pairs where the value is `null`
/// * `remove_empty_dict` - If true, remove key-value pairs where the value is an empty object `{}`
/// * `remove_empty_list` - If true, remove key-value pairs where the value is an empty array `[]`
/// * `key_replacements` - Optional vector of (pattern, replacement) tuples for keys
/// * `value_replacements` - Optional vector of (pattern, replacement) tuples for values
/// * `separator` - Optional key separator (defaults to "." if None)
///
/// # Array Flattening
///
/// Arrays are flattened using numeric indices in dot notation:
/// - `{"items": [1, 2, 3]}` becomes `{"items.0": 1, "items.1": 2, "items.2": 3}`
/// - `{"users": [{"name": "John"}]}` becomes `{"users.0.name": "John"}`
/// - `{"matrix": [[1, 2], [3, 4]]}` becomes `{"matrix.0.0": 1, "matrix.0.1": 2, "matrix.1.0": 3, "matrix.1.1": 4}`
///
/// # Returns
///
/// Returns `JsonOutput::Single(String)` for single JSON input or `JsonOutput::Multiple(Vec<String>)` for multiple JSON inputs.
///
/// # Examples
///
/// ## Basic object flattening
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json = r#"{"user": {"name": "John", "age": 30}}"#;
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, None, false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"user.name": "John", "user.age": 30}
/// ```
///
/// ## Array flattening
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json = r#"{"items": [1, 2, {"nested": "value"}], "matrix": [[1, 2], [3, 4]]}"#;
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, None, false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"items.0": 1, "items.1": 2, "items.2.nested": "value", "matrix.0.0": 1, "matrix.0.1": 2, "matrix.1.0": 3, "matrix.1.1": 4}
/// ```
///
/// ## Multiple JSON strings
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json_list = vec![
///     r#"{"user1": {"name": "Alice"}}"#,
///     r#"{"user2": {"name": "Bob"}}"#,
/// ];
/// let result = test_flatten_json_with_params(&json_list[..], false, false, false, false, None, None, None, false).unwrap();
/// match result {
///     JsonOutput::Single(_) => unreachable!(),
///     JsonOutput::Multiple(results) => {
///         for (i, flattened) in results.iter().enumerate() {
///             println!("Result {}: {}", i + 1, flattened);
///         }
///     }
/// }
/// ```
///
/// ## Filtering and array handling
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json = r#"{"data": [{"name": "John", "email": ""}, null, [], {"name": "Jane"}]}"#;
/// let result = test_flatten_json_with_params(json, true, true, false, true, None, None, None, false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"data.0.name": "John", "data.3.name": "Jane"} (empty strings, nulls, and empty arrays removed)
/// ```
///
/// ## Key and value replacements with new tuple format
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json = r#"{"user_name": "John", "user_email": "john@example.com"}"#;
///
/// // New intuitive tuple format: (pattern, replacement)
/// let key_replacements = Some(vec![
///     ("regex:^user_".to_string(), "person_".to_string()),
/// ]);
/// let value_replacements = Some(vec![
///     ("regex:@example\\.com".to_string(), "@company.org".to_string()),
/// ]);
///
/// let result = test_flatten_json_with_params(json, false, false, false, false, key_replacements, value_replacements, None, false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"person_name": "John", "person_email": "john@company.org"}
/// ```
///
/// ## Custom separators
/// ```rust
/// use json_tools_rs::{flatten_json, JsonOutput};
///
/// let json = r#"{"user": {"profile": {"name": "John"}}}"#;
///
/// // Using underscore separator
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, Some("_"), false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"user_profile_name": "John"}
///
/// // Using double colon separator
/// let result = test_flatten_json_with_params(json, false, false, false, false, None, None, Some("::"), false).unwrap();
/// match result {
///     JsonOutput::Single(flattened) => println!("Result: {}", flattened),
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// // Result: {"user::profile::name": "John"}
/// ```
/// JSON Flattener with builder pattern for easy configuration
///
/// This is the main interface for flattening JSON data. It provides a fluent
/// builder API that makes it easy to configure all flattening options.
///
/// # Examples
///
/// ```rust
/// use json_tools_rs::JsonFlattener;
///
/// // Basic flattening
/// let result = JsonFlattener::new()
///     .flatten(r#"{"user": {"name": "John"}}"#)?;
///
/// // Advanced configuration
/// let result = JsonFlattener::new()
///     .remove_empty_strings(true)
///     .remove_nulls(true)
///     .separator("_")
///     .lowercase_keys(true)
///     .key_replacement("regex:^user_", "")
///     .value_replacement("@example.com", "@company.org")
///     .flatten(json)?;
/// ```
#[derive(Debug, Clone)]
pub struct JsonFlattener {
    /// Remove keys with empty string values
    remove_empty_string_values: bool,
    /// Remove keys with null values
    remove_null_values: bool,
    /// Remove keys with empty object values
    remove_empty_dict: bool,
    /// Remove keys with empty array values
    remove_empty_list: bool,
    /// Key replacement patterns (find, replace)
    key_replacements: Vec<(String, String)>,
    /// Value replacement patterns (find, replace)
    value_replacements: Vec<(String, String)>,
    /// Separator for nested keys (default: ".")
    separator: String,
    /// Convert all keys to lowercase
    lower_case_keys: bool,
}

impl Default for JsonFlattener {
    fn default() -> Self {
        Self {
            remove_empty_string_values: false,
            remove_null_values: false,
            remove_empty_dict: false,
            remove_empty_list: false,
            key_replacements: Vec::new(),
            value_replacements: Vec::new(),
            separator: ".".to_string(),
            lower_case_keys: false,
        }
    }
}

impl JsonFlattener {
    /// Create a new JSON flattener with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Remove keys with empty string values
    pub fn remove_empty_strings(mut self, value: bool) -> Self {
        self.remove_empty_string_values = value;
        self
    }

    /// Remove keys with null values
    pub fn remove_nulls(mut self, value: bool) -> Self {
        self.remove_null_values = value;
        self
    }

    /// Remove keys with empty object values
    pub fn remove_empty_objects(mut self, value: bool) -> Self {
        self.remove_empty_dict = value;
        self
    }

    /// Remove keys with empty array values
    pub fn remove_empty_arrays(mut self, value: bool) -> Self {
        self.remove_empty_list = value;
        self
    }

    /// Add a key replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn key_replacement<F: Into<String>, R: Into<String>>(mut self, find: F, replace: R) -> Self {
        self.key_replacements.push((find.into(), replace.into()));
        self
    }

    /// Add a value replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn value_replacement<F: Into<String>, R: Into<String>>(mut self, find: F, replace: R) -> Self {
        self.value_replacements.push((find.into(), replace.into()));
        self
    }

    /// Set the separator for nested keys
    pub fn separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.separator = separator.into();
        self
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(mut self, value: bool) -> Self {
        self.lower_case_keys = value;
        self
    }

    /// Flatten the JSON input
    ///
    /// # Arguments
    /// * `json_input` - JSON input as string, &str, or slice of strings
    ///
    /// # Returns
    /// * `JsonOutput` - Single flattened JSON string or multiple results
    pub fn flatten<'a, T>(self, json_input: T) -> Result<JsonOutput, Box<dyn Error>>
    where
        T: Into<JsonInput<'a>>,
    {
        flatten_json_with_params(
            json_input,
            self.remove_empty_string_values,
            self.remove_null_values,
            self.remove_empty_dict,
            self.remove_empty_list,
            if self.key_replacements.is_empty() { None } else { Some(self.key_replacements) },
            if self.value_replacements.is_empty() { None } else { Some(self.value_replacements) },
            Some(&self.separator),
            self.lower_case_keys,
        )
    }
}

/// Convenience function for simple JSON flattening with default settings
///
/// For more advanced configuration, use `JsonFlattener::new()` with the builder pattern.
///
/// # Examples
///
/// ```rust
/// use json_tools_rs::flatten_json;
///
/// let result = flatten_json(r#"{"user": {"name": "John"}}"#)?;
/// ```
#[inline]
pub fn flatten_json<'a, T>(json_input: T) -> Result<JsonOutput, Box<dyn Error>>
where
    T: Into<JsonInput<'a>>,
{
    JsonFlattener::new().flatten(json_input)
}

/// Backward compatibility function with individual parameters
///
/// This function maintains the original API for existing code.
/// For new code, prefer using `JsonFlattener::new()` with the builder pattern.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn flatten_json_with_params<'a, T>(
    json_input: T,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<Vec<(String, String)>>,
    value_replacements: Option<Vec<(String, String)>>,
    separator: Option<&str>,
    lower_case_keys: bool,
) -> Result<JsonOutput, Box<dyn Error>>
where
    T: Into<JsonInput<'a>>,
{
    let separator = separator.unwrap_or(".");
    let input = json_input.into();

    match input {
        JsonInput::Single(json) => {
            let result = process_single_json(
                json,
                remove_empty_string_values,
                remove_null_values,
                remove_empty_dict,
                remove_empty_list,
                key_replacements.as_deref(),
                value_replacements.as_deref(),
                separator,
                lower_case_keys,
            )?;
            Ok(JsonOutput::Single(result))
        }
        JsonInput::Multiple(json_list) => {
            let mut results = Vec::with_capacity(json_list.len());

            for (index, json) in json_list.iter().enumerate() {
                match process_single_json(
                    json,
                    remove_empty_string_values,
                    remove_null_values,
                    remove_empty_dict,
                    remove_empty_list,
                    key_replacements.as_deref(),
                    value_replacements.as_deref(),
                    separator,
                    lower_case_keys,
                ) {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        return Err(Box::new(FlattenError::BatchError {
                            index,
                            error: Box::new(match e.downcast::<FlattenError>() {
                                Ok(flatten_err) => *flatten_err,
                                Err(other_err) => FlattenError::InvalidReplacementPattern(format!(
                                    "Unknown error: {}",
                                    other_err
                                )),
                            }),
                        }));
                    }
                }
            }

            Ok(JsonOutput::Multiple(results))
        }
    }
}

/// Convenience function that uses the default "." separator
#[inline]
pub fn flatten_json_default<'a, T>(
    json_input: T,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<Vec<(String, String)>>,
    value_replacements: Option<Vec<(String, String)>>,
) -> Result<JsonOutput, Box<dyn Error>>
where
    T: Into<JsonInput<'a>>,
{
    flatten_json_with_params(
        json_input,
        remove_empty_string_values,
        remove_null_values,
        remove_empty_dict,
        remove_empty_list,
        key_replacements,
        value_replacements,
        None,  // Use default "." separator
        false, // Default to not lowercasing keys
    )
}

/// Core flattening logic for a single JSON string
#[inline]
#[allow(clippy::too_many_arguments)]
fn process_single_json(
    json: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<&[(String, String)]>,
    value_replacements: Option<&[(String, String)]>,
    separator: &str,
    lower_case_keys: bool,
) -> Result<String, Box<dyn Error>> {
    // Parse the input JSON using simd-json for better performance
    let mut json_bytes = json.as_bytes().to_vec();
    let value: Value = simd_json::serde::from_slice(&mut json_bytes)?;

    // Estimate capacity based on JSON size to reduce reallocations
    let estimated_capacity = estimate_flattened_size(&value);
    // Use a larger initial capacity to reduce rehashing
    let initial_capacity = (estimated_capacity * 4) / 3; // Account for load factor
    let mut flattened = HashMap::with_capacity(initial_capacity);

    // Flatten the JSON structure with ultra-optimized key building
    // Pre-allocate string capacity based on estimated max key length
    let max_key_length = estimate_max_key_length(&value);
    let mut builder = FastStringBuilder::with_capacity_and_separator(max_key_length, separator);
    flatten_value_ultra_optimized(&value, &mut builder, &mut flattened);

    // Apply key replacements if provided
    if let Some(key_tuples) = key_replacements {
        // Convert tuple format to the internal vector format
        let key_patterns = convert_tuples_to_patterns(key_tuples);
        flattened = apply_key_replacements_optimized(flattened, &key_patterns)?;
    }

    // Apply lowercase conversion to keys if requested
    if lower_case_keys {
        flattened = apply_lowercase_to_keys(flattened);
    }

    // Apply value replacements if provided
    if let Some(value_tuples) = value_replacements {
        // Convert tuple format to the internal vector format
        let value_patterns = convert_tuples_to_patterns(value_tuples);
        apply_value_replacements_optimized(&mut flattened, &value_patterns)?;
    }

    // Apply filtering AFTER replacements to catch newly created empty values
    // This ensures that values replaced with empty strings are properly removed
    if remove_null_values || remove_empty_string_values || remove_empty_dict || remove_empty_list {
        flattened.retain(|_, v| {
            // Optimize for the most common case (strings) first
            if remove_empty_string_values {
                if let Some(s) = v.as_str() {
                    if s.is_empty() {
                        return false;
                    }
                }
            }

            // Second most common case
            if remove_null_values && v.is_null() {
                return false;
            }

            // Less common cases
            if remove_empty_dict {
                if let Some(obj) = v.as_object() {
                    if obj.is_empty() {
                        return false;
                    }
                }
            }
            if remove_empty_list {
                if let Some(arr) = v.as_array() {
                    if arr.is_empty() {
                        return false;
                    }
                }
            }
            true
        });
    }

    // Convert back to JSON string using fast serialization
    serialize_flattened_fast(&flattened).map_err(|e| Box::new(e) as Box<dyn Error>)
}

/// Convert tuple-based replacement patterns to the internal vector format
/// This converts the intuitive tuple format to the internal representation used by replacement functions
#[inline]
fn convert_tuples_to_patterns(tuples: &[(String, String)]) -> Vec<String> {
    let mut patterns = Vec::with_capacity(tuples.len() * 2);
    for (pattern, replacement) in tuples {
        patterns.push(pattern.clone());
        patterns.push(replacement.clone());
    }
    patterns
}

/// Apply lowercase conversion to all keys in the flattened HashMap
/// This function creates a new HashMap with all keys converted to lowercase
#[inline]
fn apply_lowercase_to_keys(flattened: HashMap<String, Value>) -> HashMap<String, Value> {
    let mut result = HashMap::with_capacity(flattened.len());
    for (key, value) in flattened {
        result.insert(key.to_lowercase(), value);
    }
    result
}

/// Legacy flatten function - kept for reference and comparison
#[allow(dead_code)]
fn flatten_value(value: &Value, prefix: String, result: &mut HashMap<String, Value>) {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                // Keep empty objects as they might be filtered later
                result.insert(prefix, Value::Object(Map::new()));
            } else {
                for (key, val) in obj {
                    let new_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    flatten_value(val, new_key, result);
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                // Keep empty arrays as they might be filtered later
                result.insert(prefix, Value::Array(vec![]));
            } else {
                for (index, val) in arr.iter().enumerate() {
                    let new_key = if prefix.is_empty() {
                        index.to_string()
                    } else {
                        format!("{}.{}", prefix, index)
                    };
                    flatten_value(val, new_key, result);
                }
            }
        }
        _ => {
            result.insert(prefix, value.clone());
        }
    }
}

/// Legacy key replacement function - kept for reference and comparison
#[allow(dead_code)]
fn apply_key_replacements(
    mut flattened: HashMap<String, Value>,
    patterns: &[String],
) -> Result<HashMap<String, Value>, FlattenError> {
    if patterns.len() % 2 != 0 {
        return Err(FlattenError::InvalidReplacementPattern(
            "Key replacement patterns must be provided in pairs (pattern, replacement)".to_string(),
        ));
    }

    let mut new_flattened = HashMap::new();

    for (old_key, value) in flattened.drain() {
        let mut new_key = old_key;

        // Apply each replacement pattern pair
        for chunk in patterns.chunks(2) {
            let pattern = &chunk[0];
            let replacement = &chunk[1];

            if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                let regex = Regex::new(regex_pattern)?;
                new_key = regex.replace_all(&new_key, replacement).to_string();
            } else {
                new_key = new_key.replace(pattern, replacement);
            }
        }

        new_flattened.insert(new_key, value);
    }

    Ok(new_flattened)
}

/// Legacy value replacement function - kept for reference and comparison
#[allow(dead_code)]
fn apply_value_replacements(
    flattened: &mut HashMap<String, Value>,
    patterns: &[String],
) -> Result<(), FlattenError> {
    if patterns.len() % 2 != 0 {
        return Err(FlattenError::InvalidReplacementPattern(
            "Value replacement patterns must be provided in pairs (pattern, replacement)"
                .to_string(),
        ));
    }

    for (_, value) in flattened.iter_mut() {
        if let Value::String(s) = value {
            let mut new_value = s.clone();

            // Apply each replacement pattern pair
            for chunk in patterns.chunks(2) {
                let pattern = &chunk[0];
                let replacement = &chunk[1];

                if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                    let regex = Regex::new(regex_pattern)?;
                    new_value = regex.replace_all(&new_value, replacement).to_string();
                } else {
                    new_value = new_value.replace(pattern, replacement);
                }
            }

            *value = Value::String(new_value);
        }
    }

    Ok(())
}

/// Estimates the flattened size to pre-allocate HashMap capacity
fn estimate_flattened_size(value: &Value) -> usize {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                1
            } else {
                obj.iter().map(|(_, v)| estimate_flattened_size(v)).sum()
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                1
            } else {
                arr.iter().map(estimate_flattened_size).sum()
            }
        }
        _ => 1,
    }
}

/// Estimates the maximum key length for string pre-allocation
fn estimate_max_key_length(value: &Value) -> usize {
    fn estimate_depth_and_width(value: &Value, current_depth: usize) -> (usize, usize) {
        match value {
            Value::Object(obj) => {
                if obj.is_empty() {
                    (current_depth, 0)
                } else {
                    let max_key_len = obj.keys().map(|k| k.len()).max().unwrap_or(0);
                    let (max_child_depth, max_child_width) = obj
                        .values()
                        .map(|v| estimate_depth_and_width(v, current_depth + 1))
                        .fold((current_depth, max_key_len), |(max_d, max_w), (d, w)| {
                            (max_d.max(d), max_w.max(w))
                        });
                    (max_child_depth, max_child_width)
                }
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    (current_depth, 0)
                } else {
                    let max_index_len = (arr.len() - 1).to_string().len();
                    let (max_child_depth, max_child_width) = arr
                        .iter()
                        .map(|v| estimate_depth_and_width(v, current_depth + 1))
                        .fold((current_depth, max_index_len), |(max_d, max_w), (d, w)| {
                            (max_d.max(d), max_w.max(w))
                        });
                    (max_child_depth, max_child_width)
                }
            }
            _ => (current_depth, 0),
        }
    }

    let (max_depth, max_width) = estimate_depth_and_width(value, 0);
    // Estimate: max_depth * (max_width + 1 for dot) + some buffer
    max_depth * (max_width + 1) + 50
}

/// Optimized recursive flattening with reduced string allocations and better capacity management
/// Used in performance comparison tests
#[allow(dead_code)]
#[inline]
fn flatten_value_optimized(
    value: &Value,
    prefix: &mut String,
    result: &mut HashMap<String, Value>,
) {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                result.insert(prefix.clone(), Value::Object(Map::new()));
            } else {
                let prefix_len = prefix.len();
                let needs_dot = !prefix.is_empty();

                for (key, val) in obj {
                    // Reserve capacity for the new key to avoid reallocations
                    let needed_capacity = prefix_len + if needs_dot { 1 } else { 0 } + key.len();
                    if prefix.capacity() < needed_capacity {
                        prefix.reserve(needed_capacity - prefix.len());
                    }

                    if needs_dot {
                        prefix.push('.');
                    }
                    prefix.push_str(key);
                    flatten_value_optimized(val, prefix, result);
                    prefix.truncate(prefix_len);
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                result.insert(prefix.clone(), Value::Array(vec![]));
            } else {
                let prefix_len = prefix.len();
                let needs_dot = !prefix.is_empty();

                for (index, val) in arr.iter().enumerate() {
                    // Reserve capacity for the new key first
                    let index_len = if index < 10 {
                        1
                    } else {
                        index.to_string().len()
                    };
                    let needed_capacity = prefix_len + if needs_dot { 1 } else { 0 } + index_len;
                    if prefix.capacity() < needed_capacity {
                        prefix.reserve(needed_capacity - prefix.len());
                    }

                    if needs_dot {
                        prefix.push('.');
                    }

                    // Fast path for single digits - avoid string allocation
                    if index < 10 {
                        prefix.push(char::from(b'0' + index as u8));
                    } else {
                        prefix.push_str(&index.to_string());
                    }

                    flatten_value_optimized(val, prefix, result);
                    prefix.truncate(prefix_len);
                }
            }
        }
        _ => {
            result.insert(prefix.clone(), value.clone());
        }
    }
}

/// Optimized key replacement with regex caching and in-place operations
fn apply_key_replacements_optimized(
    mut flattened: HashMap<String, Value>,
    patterns: &[String],
) -> Result<HashMap<String, Value>, FlattenError> {
    if patterns.len() % 2 != 0 {
        return Err(FlattenError::InvalidReplacementPattern(
            "Key replacement patterns must be provided in pairs (pattern, replacement)".to_string(),
        ));
    }

    // Pre-compile all regex patterns to avoid repeated compilation
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = &chunk[0];
        let replacement = &chunk[1];

        if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
            let regex = Regex::new(regex_pattern)?;
            compiled_patterns.push((Some(regex), replacement));
        } else {
            compiled_patterns.push((None, replacement));
        }
    }

    // Check if any keys need replacement to avoid unnecessary allocation
    let needs_replacement = flattened.keys().any(|key| {
        for (i, chunk) in patterns.chunks(2).enumerate() {
            let pattern = &chunk[0];
            let (compiled_regex, _) = &compiled_patterns[i];

            if let Some(regex) = compiled_regex {
                if regex.is_match(key) {
                    return true;
                }
            } else if key.contains(pattern) {
                return true;
            }
        }
        false
    });

    if !needs_replacement {
        return Ok(flattened);
    }

    let mut new_flattened = HashMap::with_capacity(flattened.len());

    for (old_key, value) in flattened.drain() {
        let mut new_key = Cow::Borrowed(old_key.as_str());

        // Apply each compiled pattern
        for (i, chunk) in patterns.chunks(2).enumerate() {
            let pattern = &chunk[0];
            let (compiled_regex, replacement) = &compiled_patterns[i];

            if let Some(regex) = compiled_regex {
                if regex.is_match(&new_key) {
                    new_key = Cow::Owned(
                        regex
                            .replace_all(&new_key, replacement.as_str())
                            .to_string(),
                    );
                }
            } else if new_key.contains(pattern) {
                new_key = Cow::Owned(new_key.replace(pattern, replacement));
            }
        }

        new_flattened.insert(new_key.into_owned(), value);
    }

    Ok(new_flattened)
}

/// Optimized value replacement with regex caching
fn apply_value_replacements_optimized(
    flattened: &mut HashMap<String, Value>,
    patterns: &[String],
) -> Result<(), FlattenError> {
    if patterns.len() % 2 != 0 {
        return Err(FlattenError::InvalidReplacementPattern(
            "Value replacement patterns must be provided in pairs (pattern, replacement)"
                .to_string(),
        ));
    }

    // Pre-compile all regex patterns
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = &chunk[0];
        let replacement = &chunk[1];

        if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
            let regex = Regex::new(regex_pattern)?;
            compiled_patterns.push((Some(regex), replacement));
        } else {
            compiled_patterns.push((None, replacement));
        }
    }

    for (_, value) in flattened.iter_mut() {
        if let Value::String(s) = value {
            let mut new_value = Cow::Borrowed(s.as_str());
            let mut changed = false;

            // Apply each compiled pattern
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = &chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_value) {
                        new_value = Cow::Owned(
                            regex
                                .replace_all(&new_value, replacement.as_str())
                                .to_string(),
                        );
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

/// Ultra-fast JSON serialization with aggressive optimizations
#[inline]
fn serialize_flattened_fast(
    flattened: &HashMap<String, Value>,
) -> Result<String, simd_json::Error> {
    let estimated_size = estimate_json_size_optimized(flattened);
    let mut result = String::with_capacity(estimated_size);
    result.push('{');

    let mut first = true;
    for (key, value) in flattened {
        if !first {
            result.push(',');
        }
        first = false;

        // Ultra-fast key serialization
        result.push('"');
        result.push_str(&escape_json_string(key));
        result.push_str("\":");

        // Ultra-optimized value serialization
        serialize_value_ultra_fast(value, &mut result)?;
    }

    result.push('}');
    Ok(result)
}

/// Ultra-fast value serialization with branch prediction optimization
#[inline]
fn serialize_value_ultra_fast(value: &Value, result: &mut String) -> Result<(), simd_json::Error> {
    match value {
        // Most common case first for better branch prediction
        Value::String(s) => {
            result.push('"');
            result.push_str(&escape_json_string(s));
            result.push('"');
        }
        Value::Number(n) => {
            // Optimized number serialization
            if let Some(i) = n.as_i64() {
                // Fast integer path
                if (0..10).contains(&i) {
                    // Ultra-fast path for single digits
                    result.push(char::from(b'0' + i as u8));
                } else if (0..100).contains(&i) {
                    // Fast path for two digits
                    let tens = i / 10;
                    let ones = i % 10;
                    result.push(char::from(b'0' + tens as u8));
                    result.push(char::from(b'0' + ones as u8));
                } else if (0..1000).contains(&i) {
                    // Fast path for three digits
                    let hundreds = i / 100;
                    let tens = (i % 100) / 10;
                    let ones = i % 10;
                    result.push(char::from(b'0' + hundreds as u8));
                    result.push(char::from(b'0' + tens as u8));
                    result.push(char::from(b'0' + ones as u8));
                } else {
                    use std::fmt::Write;
                    write!(result, "{}", i).unwrap();
                }
            } else if let Some(f) = n.as_f64() {
                use std::fmt::Write;
                write!(result, "{}", f).unwrap();
            } else {
                use std::fmt::Write;
                write!(result, "{}", n).unwrap();
            }
        }
        Value::Bool(true) => {
            result.push_str("true");
        }
        Value::Bool(false) => {
            result.push_str("false");
        }
        Value::Null => {
            result.push_str("null");
        }
        _ => {
            // For complex values, fall back to simd_json
            let value_str = simd_json::serde::to_string(value)?;
            result.push_str(&value_str);
        }
    }
    Ok(())
}

/// Ultra-optimized JSON size estimation with better accuracy
#[inline]
fn estimate_json_size_optimized(flattened: &HashMap<String, Value>) -> usize {
    let mut size = 2; // Opening and closing braces
    let len = flattened.len();

    if len == 0 {
        return 2;
    }

    // Pre-calculate comma overhead
    size += len - 1; // Commas between entries

    for (key, value) in flattened {
        size += key.len() + 3; // Key + quotes + colon

        // More accurate value size estimation
        size += match value {
            Value::String(s) => {
                // Account for potential escaping
                let base_len = s.len() + 2; // String + quotes
                let escape_chars = s
                    .bytes()
                    .filter(|&b| matches!(b, b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 0x08 | 0x0C))
                    .count();
                base_len + escape_chars
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if (0..10).contains(&i) {
                        1
                    } else if (-9..100).contains(&i) {
                        2
                    } else if (-99..1000).contains(&i) {
                        3
                    } else {
                        20
                    } // Conservative for large numbers
                } else {
                    20 // Conservative for floats
                }
            }
            Value::Bool(true) => 4,  // "true"
            Value::Bool(false) => 5, // "false"
            Value::Null => 4,        // "null"
            _ => 50,                 // Conservative estimate for complex values
        };
    }

    // Add 10% buffer for safety
    size + (size / 10)
}

/// Ultra-fast JSON string escaping with optimized byte-level operations
#[inline]
fn escape_json_string(s: &str) -> Cow<str> {
    let bytes = s.as_bytes();

    // Ultra-fast path: scan for escape characters using byte operations
    let mut needs_escape = false;
    for &byte in bytes {
        if matches!(byte, b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 0x08 | 0x0C) {
            needs_escape = true;
            break;
        }
    }

    if !needs_escape {
        return Cow::Borrowed(s);
    }

    // Optimized escaping with pre-allocated capacity
    let mut result = String::with_capacity(s.len() + (s.len() >> 2)); // 25% extra capacity

    for &byte in bytes {
        match byte {
            b'"' => result.push_str("\\\""),
            b'\\' => result.push_str("\\\\"),
            b'\n' => result.push_str("\\n"),
            b'\r' => result.push_str("\\r"),
            b'\t' => result.push_str("\\t"),
            0x08 => result.push_str("\\b"),
            0x0C => result.push_str("\\f"),
            _ => result.push(byte as char),
        }
    }
    Cow::Owned(result)
}

/// Cached separator information for ultra-fast operations
#[derive(Clone)]
struct SeparatorCache {
    separator: &'static str,         // Static reference for common separators
    separator_owned: Option<String>, // Owned string for custom separators
    is_single_char: bool,            // True if separator is a single character
    single_char: Option<char>,       // The character if single-char separator
    length: usize,                   // Pre-computed length
    is_common: bool,                 // True if it's a common separator (., _, ::, /, -)
}

impl SeparatorCache {
    #[inline]
    fn new(separator: &str) -> Self {
        // Check for common static separators to avoid heap allocations
        let (static_sep, is_common) = match separator {
            "." => (".", true),
            "_" => ("_", true),
            "::" => ("::", true),
            "/" => ("/", true),
            "-" => ("-", true),
            "|" => ("|", true),
            _ => ("", false),
        };

        let is_single_char = separator.len() == 1;
        let single_char = if is_single_char {
            separator.chars().next()
        } else {
            None
        };

        Self {
            separator: if is_common { static_sep } else { "" },
            separator_owned: if is_common {
                None
            } else {
                Some(separator.to_string())
            },
            is_single_char,
            single_char,
            length: separator.len(),
            is_common,
        }
    }

    #[inline]
    fn append_to_buffer(&self, buffer: &mut String) {
        if self.is_single_char {
            // Ultra-fast path for single characters - direct byte manipulation
            let ch = self.single_char.unwrap();
            // Compile-time optimization for the most common separators
            match ch {
                '.' => buffer.push('.'), // Most common case
                '_' => buffer.push('_'), // Second most common
                '/' => buffer.push('/'), // Third most common
                '|' => buffer.push('|'), // Fourth most common
                '-' => buffer.push('-'), // Fifth most common
                _ => buffer.push(ch),    // Fallback for other single chars
            }
        } else if self.is_common {
            // Fast path for common static multi-char separators
            match self.separator {
                "::" => buffer.push_str("::"),        // Most common multi-char
                _ => buffer.push_str(self.separator), // Other static separators
            }
        } else {
            // Fallback for custom separators
            buffer.push_str(self.separator_owned.as_ref().unwrap());
        }
    }

    #[inline]
    fn reserve_capacity_for_append(&self, buffer: &mut String, additional_content_len: usize) {
        // Pre-calculate total capacity needed to avoid multiple reallocations
        let needed_capacity = buffer.len() + self.length + additional_content_len;
        if buffer.capacity() < needed_capacity {
            buffer.reserve(needed_capacity - buffer.len());
        }
    }
}

/// High-performance string builder with advanced caching and optimization
struct FastStringBuilder {
    buffer: String,
    stack: Vec<usize>, // Stack of prefix lengths for efficient truncation
    separator_cache: SeparatorCache, // Cached separator information
}

impl FastStringBuilder {
    #[inline]
    fn with_capacity_and_separator(capacity: usize, separator: &str) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            stack: Vec::with_capacity(32), // Reasonable depth for most JSON
            separator_cache: SeparatorCache::new(separator),
        }
    }

    #[inline]
    fn push_level(&mut self) {
        self.stack.push(self.buffer.len());
    }

    #[inline]
    fn pop_level(&mut self) {
        if let Some(len) = self.stack.pop() {
            self.buffer.truncate(len);
        }
    }

    #[inline]
    fn append_key(&mut self, key: &str, needs_separator: bool) {
        if needs_separator {
            // Pre-allocate capacity to avoid reallocations
            self.separator_cache
                .reserve_capacity_for_append(&mut self.buffer, key.len());
            self.separator_cache.append_to_buffer(&mut self.buffer);
        } else {
            // Reserve capacity for just the key
            if self.buffer.capacity() < self.buffer.len() + key.len() {
                self.buffer.reserve(key.len());
            }
        }
        self.buffer.push_str(key);
    }

    #[inline]
    fn append_index(&mut self, index: usize, needs_separator: bool) {
        // Pre-calculate index string length for capacity optimization
        let index_len = if index < 10 {
            1
        } else if index < 100 {
            2
        } else if index < 1000 {
            3
        } else if index < 10000 {
            4
        } else {
            // For very large indices, calculate the length
            (index as f64).log10().floor() as usize + 1
        };

        if needs_separator {
            self.separator_cache
                .reserve_capacity_for_append(&mut self.buffer, index_len);
            self.separator_cache.append_to_buffer(&mut self.buffer);
        } else {
            // Reserve capacity for just the index
            if self.buffer.capacity() < self.buffer.len() + index_len {
                self.buffer.reserve(index_len);
            }
        }

        // Ultra-fast path for single digits
        if index < 10 {
            self.buffer.push(char::from(b'0' + index as u8));
        } else if index < 100 {
            // Fast path for two digits
            let tens = index / 10;
            let ones = index % 10;
            self.buffer.push(char::from(b'0' + tens as u8));
            self.buffer.push(char::from(b'0' + ones as u8));
        } else {
            // Fallback for larger numbers
            use std::fmt::Write;
            write!(self.buffer, "{}", index).unwrap();
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        &self.buffer
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// Ultra-optimized flattening using FastStringBuilder and aggressive inlining
#[inline]
fn flatten_value_ultra_optimized(
    value: &Value,
    builder: &mut FastStringBuilder,
    result: &mut HashMap<String, Value>,
) {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                result.insert(builder.as_str().to_string(), Value::Object(Map::new()));
            } else {
                let needs_dot = !builder.is_empty();
                for (key, val) in obj {
                    builder.push_level();
                    builder.append_key(key, needs_dot);
                    flatten_value_ultra_optimized(val, builder, result);
                    builder.pop_level();
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                result.insert(builder.as_str().to_string(), Value::Array(vec![]));
            } else {
                let needs_dot = !builder.is_empty();
                for (index, val) in arr.iter().enumerate() {
                    builder.push_level();
                    builder.append_index(index, needs_dot);
                    flatten_value_ultra_optimized(val, builder, result);
                    builder.pop_level();
                }
            }
        }
        _ => {
            result.insert(builder.as_str().to_string(), value.clone());
        }
    }
}

// Helper function for tests that need the old parameter-based API
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub fn test_flatten_json_with_params<'a, T>(
    json_input: T,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<Vec<(String, String)>>,
    value_replacements: Option<Vec<(String, String)>>,
    separator: Option<&str>,
    lower_case_keys: bool,
) -> Result<JsonOutput, Box<dyn Error>>
where
    T: Into<JsonInput<'a>>,
{
    let mut flattener = JsonFlattener::new()
        .remove_empty_strings(remove_empty_string_values)
        .remove_nulls(remove_null_values)
        .remove_empty_objects(remove_empty_dict)
        .remove_empty_arrays(remove_empty_list)
        .lowercase_keys(lower_case_keys);

    if let Some(sep) = separator {
        flattener = flattener.separator(sep);
    }

    if let Some(key_reps) = key_replacements {
        for (find, replace) in key_reps {
            flattener = flattener.key_replacement(find, replace);
        }
    }

    if let Some(val_reps) = value_replacements {
        for (find, replace) in val_reps {
            flattener = flattener.value_replacement(find, replace);
        }
    }

    flattener.flatten(json_input)
}