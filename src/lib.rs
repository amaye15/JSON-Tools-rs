//! # JSON Tools RS
//!
//! A Rust library for advanced JSON manipulation, including flattening and unflattening
//! nested JSON structures with configurable filtering and replacement options.
//!
//! ## Features
//!
//! - Flatten nested JSON structures using dot notation
//! - Unflatten flattened JSON back to nested structures
//! - Remove empty values (strings, objects, arrays, null values)
//! - Replace keys and values using literal strings or regex patterns
//! - Comprehensive error handling
//! - Builder pattern API for easy configuration
//!
//! ## Examples
//!
//! ### Flattening JSON
//!
//! ```rust
//! use json_tools_rs::{JsonFlattener, JsonOutput};
//!
//! let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
//! let result = JsonFlattener::new()
//!     .remove_empty_strings(true)
//!     .remove_nulls(true)
//!     .flatten(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"user.name": "John"}
//!         assert!(flattened.contains("user.name"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ### Unflattening JSON
//!
//! ```rust
//! use json_tools_rs::{JsonUnflattener, JsonOutput};
//!
//! let flattened = r#"{"user.name": "John", "user.age": 30, "items.0": "first", "items.1": "second"}"#;
//! let result = JsonUnflattener::new()
//!     .unflatten(flattened).unwrap();
//!
//! match result {
//!     JsonOutput::Single(unflattened) => {
//!         // Result: {"user": {"name": "John", "age": 30}, "items": ["first", "second"]}
//!         assert!(unflattened.contains("\"user\""));
//!         assert!(unflattened.contains("\"items\""));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
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

// Tests module
#[cfg(test)]
mod tests;

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
    /// Invalid JSON structure for operation
    InvalidJson(String),
    /// Error processing batch item with index
    BatchError {
        index: usize,
        error: Box<FlattenError>,
    },
}

impl fmt::Display for FlattenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlattenError::JsonParseError(e) => write!(f, "JSON parse error: {e}"),
            FlattenError::RegexError(e) => write!(f, "Regex error: {e}"),
            FlattenError::InvalidReplacementPattern(msg) => {
                write!(f, "Invalid replacement pattern: {msg}")
            }
            FlattenError::InvalidJson(msg) => {
                write!(f, "Invalid JSON: {msg}")
            }
            FlattenError::BatchError { index, error } => {
                write!(f, "Error processing JSON at index {index}: {error}")
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

/// JSON Flattener with builder pattern for easy configuration
///
/// This is the main interface for flattening JSON data. It provides a fluent
/// builder API that makes it easy to configure all flattening options.
///
/// # Examples
///
/// ```rust
/// use json_tools_rs::{JsonFlattener, JsonOutput};
///
/// // Basic flattening
/// let result = JsonFlattener::new()
///     .flatten(r#"{"user": {"name": "John"}}"#).unwrap();
///
/// match result {
///     JsonOutput::Single(flattened) => {
///         assert!(flattened.contains("user.name"));
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
///
/// // Advanced configuration
/// let json = r#"{"user_email": "john@example.com"}"#;
/// let result = JsonFlattener::new()
///     .remove_empty_strings(true)
///     .remove_nulls(true)
///     .separator("_")
///     .lowercase_keys(true)
///     .key_replacement("regex:^user_", "")
///     .value_replacement("@example.com", "@company.org")
///     .flatten(json).unwrap();
///
/// match result {
///     JsonOutput::Single(flattened) => {
///         assert!(flattened.contains("email"));
///         assert!(flattened.contains("@company.org"));
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
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
    pub fn key_replacement<F: Into<String>, R: Into<String>>(
        mut self,
        find: F,
        replace: R,
    ) -> Self {
        self.key_replacements.push((find.into(), replace.into()));
        self
    }

    /// Add a value replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn value_replacement<F: Into<String>, R: Into<String>>(
        mut self,
        find: F,
        replace: R,
    ) -> Self {
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
        let input = json_input.into();
        let key_replacements = if self.key_replacements.is_empty() {
            None
        } else {
            Some(self.key_replacements)
        };
        let value_replacements = if self.value_replacements.is_empty() {
            None
        } else {
            Some(self.value_replacements)
        };

        match input {
            JsonInput::Single(json) => {
                let result = process_single_json(
                    json,
                    self.remove_empty_string_values,
                    self.remove_null_values,
                    self.remove_empty_dict,
                    self.remove_empty_list,
                    key_replacements.as_deref(),
                    value_replacements.as_deref(),
                    &self.separator,
                    self.lower_case_keys,
                )?;
                Ok(JsonOutput::Single(result))
            }
            JsonInput::Multiple(json_list) => {
                let mut results = Vec::with_capacity(json_list.len());

                for (index, json) in json_list.iter().enumerate() {
                    match process_single_json(
                        json,
                        self.remove_empty_string_values,
                        self.remove_null_values,
                        self.remove_empty_dict,
                        self.remove_empty_list,
                        key_replacements.as_deref(),
                        value_replacements.as_deref(),
                        &self.separator,
                        self.lower_case_keys,
                    ) {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            return Err(Box::new(FlattenError::BatchError {
                                index,
                                error: Box::new(match e.downcast::<FlattenError>() {
                                    Ok(flatten_err) => *flatten_err,
                                    Err(other_err) => FlattenError::InvalidReplacementPattern(
                                        format!("Unknown error: {other_err}"),
                                    ),
                                }),
                            }));
                        }
                    }
                }

                Ok(JsonOutput::Multiple(results))
            }
        }
    }
}

/// JSON Unflattener with builder pattern for easy configuration
///
/// This is the companion interface to JsonFlattener that provides the inverse operation -
/// converting flattened JSON back to nested JSON structure. It provides the same fluent
/// builder API that makes it easy to configure all unflattening options.
///
/// # Examples
///
/// ```rust
/// use json_tools_rs::{JsonUnflattener, JsonOutput};
///
/// // Basic unflattening
/// let flattened = r#"{"user.name": "John", "user.age": 30}"#;
/// let result = JsonUnflattener::new()
///     .unflatten(flattened).unwrap();
///
/// match result {
///     JsonOutput::Single(unflattened) => {
///         // Result: {"user": {"name": "John", "age": 30}}
///         assert!(unflattened.contains("\"user\""));
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
///
/// // Advanced configuration
/// let flattened = r#"{"prefix_email": "john@company.org", "prefix_name": "John"}"#;
/// let result = JsonUnflattener::new()
///     .separator("_")
///     .lowercase_keys(true)
///     .key_replacement("prefix_", "user_")  // Replace prefix
///     .value_replacement("@company.org", "@example.com")  // Reverse replacement
///     .unflatten(flattened).unwrap();
///
/// match result {
///     JsonOutput::Single(unflattened) => {
///         assert!(unflattened.contains("user"));
///         assert!(unflattened.contains("@example.com"));
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct JsonUnflattener {
    /// Key replacement patterns (find, replace) - applied before unflattening
    key_replacements: Vec<(String, String)>,
    /// Value replacement patterns (find, replace) - applied before unflattening
    value_replacements: Vec<(String, String)>,
    /// Separator for nested keys (default: ".")
    separator: String,
    /// Convert all keys to lowercase before processing
    lower_case_keys: bool,
}

impl Default for JsonUnflattener {
    fn default() -> Self {
        Self {
            key_replacements: Vec::new(),
            value_replacements: Vec::new(),
            separator: ".".to_string(),
            lower_case_keys: false,
        }
    }
}

impl JsonUnflattener {
    /// Create a new JSON unflattener with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the separator used for nested keys (default: ".")
    pub fn separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.separator = separator.into();
        self
    }

    /// Convert all keys to lowercase before processing
    pub fn lowercase_keys(mut self, value: bool) -> Self {
        self.lower_case_keys = value;
        self
    }

    /// Add a key replacement pattern
    ///
    /// The pattern can be a literal string or a regex pattern (prefix with "regex:")
    /// This replacement is applied to keys before unflattening
    pub fn key_replacement<F: Into<String>, R: Into<String>>(
        mut self,
        find: F,
        replace: R,
    ) -> Self {
        self.key_replacements.push((find.into(), replace.into()));
        self
    }

    /// Add a value replacement pattern
    ///
    /// The pattern can be a literal string or a regex pattern (prefix with "regex:")
    /// This replacement is applied to values before unflattening
    pub fn value_replacement<F: Into<String>, R: Into<String>>(
        mut self,
        find: F,
        replace: R,
    ) -> Self {
        self.value_replacements.push((find.into(), replace.into()));
        self
    }

    /// Unflatten JSON data using the configured settings
    ///
    /// Accepts the same input types as JsonFlattener and returns the same output format
    pub fn unflatten<'a, T>(&self, json_input: T) -> Result<JsonOutput, Box<dyn Error>>
    where
        T: Into<JsonInput<'a>>,
    {
        let input = json_input.into();

        match input {
            JsonInput::Single(json) => {
                let result = process_single_json_unflatten(
                    json,
                    self.key_replacements.as_slice(),
                    self.value_replacements.as_slice(),
                    &self.separator,
                    self.lower_case_keys,
                )?;
                Ok(JsonOutput::Single(result))
            }
            JsonInput::Multiple(json_list) => {
                let mut results = Vec::with_capacity(json_list.len());

                for (index, json) in json_list.iter().enumerate() {
                    match process_single_json_unflatten(
                        json,
                        self.key_replacements.as_slice(),
                        self.value_replacements.as_slice(),
                        &self.separator,
                        self.lower_case_keys,
                    ) {
                        Ok(result) => results.push(result),
                        Err(e) => {
                            return Err(Box::new(FlattenError::BatchError {
                                index,
                                error: Box::new(match e.downcast::<FlattenError>() {
                                    Ok(flatten_err) => *flatten_err,
                                    Err(other_err) => FlattenError::InvalidReplacementPattern(
                                        format!("Unknown error: {other_err}"),
                                    ),
                                }),
                            }));
                        }
                    }
                }

                Ok(JsonOutput::Multiple(results))
            }
        }
    }
}

/// Core unflattening logic for a single JSON string
#[inline]
#[allow(clippy::too_many_arguments)]
fn process_single_json_unflatten(
    json: &str,
    key_replacements: &[(String, String)],
    value_replacements: &[(String, String)],
    separator: &str,
    lower_case_keys: bool,
) -> Result<String, Box<dyn Error>> {
    // Parse the input JSON using simd-json for better performance
    let mut json_bytes = json.as_bytes().to_vec();
    let flattened: Value = simd_json::serde::from_slice(&mut json_bytes)?;

    // Handle root-level primitives - they should be returned as-is
    match &flattened {
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // For root-level primitives, apply value replacements if any, then return
            let mut single_value = flattened.clone();
            if !value_replacements.is_empty() {
                apply_value_replacements_to_single(&mut single_value, value_replacements)?;
            }
            return Ok(simd_json::serde::to_string(&single_value)?);
        }
        Value::Object(obj) if obj.is_empty() => {
            // Empty object should remain empty object
            return Ok("{}".to_string());
        }
        Value::Array(_) => {
            // Arrays at root level are not valid flattened JSON - convert to empty object
            return Ok("{}".to_string());
        }
        _ => {
            // Continue with normal unflattening for objects with content
        }
    }

    // Extract the flattened object
    let flattened_obj = match flattened {
        Value::Object(obj) => obj,
        _ => {
            return Err(Box::new(FlattenError::InvalidJson(
                "Expected object for unflattening".to_string(),
            )))
        }
    };

    // Apply key and value replacements if specified
    let mut processed_obj = flattened_obj.clone();

    // Apply key replacements first
    if !key_replacements.is_empty() {
        processed_obj = apply_key_replacements_for_unflatten(&processed_obj, key_replacements)?;
    }

    // Apply value replacements
    if !value_replacements.is_empty() {
        apply_value_replacements_for_unflatten(&mut processed_obj, value_replacements)?;
    }

    // Apply lowercase conversion if specified
    if lower_case_keys {
        processed_obj = apply_lowercase_keys_for_unflatten(processed_obj);
    }

    // Perform the actual unflattening
    let unflattened = unflatten_object(&processed_obj, separator)?;

    // Serialize the result
    Ok(simd_json::serde::to_string(&unflattened)?)
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

    // Handle root-level primitives - they should be returned as-is
    match &value {
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // For root-level primitives, apply value replacements if any, then return
            let mut single_value = value.clone();
            if let Some(patterns) = value_replacements {
                apply_value_replacements_to_single(&mut single_value, patterns)?;
            }
            return Ok(simd_json::serde::to_string(&single_value)?);
        }
        Value::Object(obj) if obj.is_empty() => {
            // Empty object should remain empty object
            return Ok("{}".to_string());
        }
        Value::Array(arr) if arr.is_empty() => {
            // Empty array at root level should become empty object
            return Ok("{}".to_string());
        }
        _ => {
            // Continue with normal flattening for objects and arrays with content
        }
    }

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

/// Apply value replacements to a single value (for root-level primitives)
fn apply_value_replacements_to_single(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), FlattenError> {
    if let Value::String(s) = value {
        for (pattern, replacement) in patterns {
            if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                let regex = Regex::new(regex_pattern)?;
                let new_value = regex.replace_all(s, replacement).to_string();
                *s = new_value;
            } else {
                *s = s.replace(pattern, replacement);
            }
        }
    }
    Ok(())
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
                    write!(result, "{i}").unwrap();
                }
            } else if let Some(f) = n.as_f64() {
                use std::fmt::Write;
                write!(result, "{f}").unwrap();
            } else {
                use std::fmt::Write;
                write!(result, "{n}").unwrap();
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
                let mut escape_overhead = 0;
                for &b in s.as_bytes() {
                    match b {
                        b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 0x08 | 0x0C => escape_overhead += 1,
                        b if b < 0x20 => escape_overhead += 5, // \uXXXX format
                        _ => {}
                    }
                }
                base_len + escape_overhead
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
        if matches!(byte, b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 0x08 | 0x0C) || byte < 0x20 {
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
            // Handle other control characters (0x00-0x1F)
            b if b < 0x20 => {
                result.push_str(&format!("\\u{b:04x}"));
            }
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
            write!(self.buffer, "{index}").unwrap();
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

/// Apply key replacements for unflattening (works on Map<String, Value>)
fn apply_key_replacements_for_unflatten(
    obj: &Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<Map<String, Value>, FlattenError> {
    let mut new_obj = Map::new();

    for (key, value) in obj {
        let mut new_key = key.clone();

        // Apply each replacement pattern
        for (pattern, replacement) in patterns {
            if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                // Handle regex replacement
                // Remove "regex:" prefix
                let regex = Regex::new(regex_pattern).map_err(FlattenError::RegexError)?;
                new_key = regex.replace_all(&new_key, replacement).into_owned();
            } else {
                // Handle literal replacement
                new_key = new_key.replace(pattern, replacement);
            }
        }

        new_obj.insert(new_key, value.clone());
    }

    Ok(new_obj)
}

/// Apply value replacements for unflattening (works on Map<String, Value>)
fn apply_value_replacements_for_unflatten(
    obj: &mut Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<(), FlattenError> {
    for (_, value) in obj.iter_mut() {
        if let Value::String(s) = value {
            for (pattern, replacement) in patterns {
                if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                    // Handle regex replacement
                    // Remove "regex:" prefix
                    let regex = Regex::new(regex_pattern).map_err(FlattenError::RegexError)?;
                    *s = regex.replace_all(s, replacement).into_owned();
                } else {
                    // Handle literal replacement
                    *s = s.replace(pattern, replacement);
                }
            }
        }
    }
    Ok(())
}

/// Apply lowercase conversion to keys for unflattening
fn apply_lowercase_keys_for_unflatten(obj: Map<String, Value>) -> Map<String, Value> {
    let mut new_obj = Map::new();

    for (key, value) in obj {
        new_obj.insert(key.to_lowercase(), value);
    }

    new_obj
}

/// Core unflattening algorithm that reconstructs nested JSON from flattened keys
fn unflatten_object(obj: &Map<String, Value>, separator: &str) -> Result<Value, FlattenError> {
    let mut result = Map::new();

    // Pre-analyze all keys to determine if paths should be arrays or objects
    let path_types = analyze_path_types(obj, separator);

    for (key, value) in obj {
        set_nested_value_with_types(&mut result, key, value.clone(), separator, &path_types)?;
    }

    Ok(Value::Object(result))
}

/// Analyze all flattened keys to determine whether each path should be an array or object
fn analyze_path_types(obj: &Map<String, Value>, separator: &str) -> HashMap<String, bool> {
    let mut path_types = HashMap::new(); // true = array, false = object

    // First, collect all the actual parent paths that have children
    let mut parent_paths = std::collections::HashSet::new();

    for key in obj.keys() {
        let parts: Vec<&str> = key.split(separator).collect();

        // Only consider paths that have more than one part (i.e., have children)
        if parts.len() > 1 {
            for i in 0..parts.len() - 1 {
                let parent_path = parts[..=i].join(separator);
                parent_paths.insert(parent_path);
            }
        }
    }

    // Now analyze each parent path to determine if it should be an array or object
    for parent_path in parent_paths {
        let parent_parts: Vec<&str> = parent_path.split(separator).collect();
        let parent_depth = parent_parts.len();

        // Find all child keys for this parent path
        let child_keys: Vec<&str> = obj
            .keys()
            .filter_map(|key| {
                let parts: Vec<&str> = key.split(separator).collect();
                if parts.len() > parent_depth
                    && parts[..parent_depth].join(separator) == parent_path
                {
                    Some(parts[parent_depth])
                } else {
                    None
                }
            })
            .collect();

        // Check if all child keys are valid array indices
        let all_numeric = !child_keys.is_empty()
            && child_keys.iter().all(|&key| {
                // Must be a valid usize and not have leading zeros (except "0" itself)
                key.parse::<usize>().is_ok() && (key == "0" || !key.starts_with('0'))
            });

        // Also check if there are any non-numeric keys
        let has_non_numeric = child_keys
            .iter()
            .any(|&key| key.parse::<usize>().is_err() || (key != "0" && key.starts_with('0')));

        if has_non_numeric {
            path_types.insert(parent_path, false); // object
        } else if all_numeric {
            path_types.insert(parent_path, true); // array
        } else {
            path_types.insert(parent_path, false); // default to object
        }
    }

    path_types
}

/// Set a nested value using pre-analyzed path types to handle conflicts
fn set_nested_value_with_types(
    result: &mut Map<String, Value>,
    key_path: &str,
    value: Value,
    separator: &str,
    path_types: &HashMap<String, bool>,
) -> Result<(), FlattenError> {
    let parts: Vec<&str> = key_path.split(separator).collect();

    if parts.is_empty() {
        return Err(FlattenError::InvalidJson("Empty key path".to_string()));
    }

    if parts.len() == 1 {
        // Simple key, just insert
        result.insert(parts[0].to_string(), value);
        return Ok(());
    }

    // Use the type-aware recursive approach
    set_nested_value_recursive_with_types(result, &parts, 0, value, separator, path_types)
}

/// Recursive helper for setting nested values with type awareness
fn set_nested_value_recursive_with_types(
    current: &mut Map<String, Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &HashMap<String, bool>,
) -> Result<(), FlattenError> {
    let part = parts[index];

    if index == parts.len() - 1 {
        // Last part, insert the value
        current.insert(part.to_string(), value);
        return Ok(());
    }

    // Build the current path to check its type
    let current_path = parts[..=index].join(separator);
    let should_be_array = path_types.get(&current_path).copied().unwrap_or(false);

    // Get or create the nested structure based on the determined type
    let entry = current.entry(part.to_string()).or_insert_with(|| {
        if should_be_array {
            Value::Array(vec![])
        } else {
            Value::Object(Map::new())
        }
    });

    match entry {
        Value::Object(ref mut obj) => set_nested_value_recursive_with_types(
            obj,
            parts,
            index + 1,
            value,
            separator,
            path_types,
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
                    // Continue navigating
                    let next_path = parts[..=index + 1].join(separator);
                    let next_should_be_array = path_types.get(&next_path).copied().unwrap_or(false);

                    if arr[array_index].is_null() {
                        arr[array_index] = if next_should_be_array {
                            Value::Array(vec![])
                        } else {
                            Value::Object(Map::new())
                        };
                    }

                    match &mut arr[array_index] {
                        Value::Object(ref mut obj) => set_nested_value_recursive_with_types(
                            obj,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                        ),
                        Value::Array(ref mut nested_arr) => {
                            set_nested_value_recursive_for_array_with_types(
                                nested_arr,
                                parts,
                                index + 2,
                                value,
                                separator,
                                path_types,
                            )
                        }
                        _ => Err(FlattenError::InvalidJson(format!(
                            "Array element at index {array_index} has incompatible type"
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
                    set_nested_value_recursive_with_types(
                        obj,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                    )
                } else {
                    unreachable!()
                }
            }
        }
        _ => Err(FlattenError::InvalidJson(format!(
            "Cannot navigate into non-object/non-array value at key: {part}"
        ))),
    }
}

/// Recursive helper for setting nested values in arrays with type awareness
fn set_nested_value_recursive_for_array_with_types(
    arr: &mut Vec<Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &HashMap<String, bool>,
) -> Result<(), FlattenError> {
    if index >= parts.len() {
        return Err(FlattenError::InvalidJson(
            "Invalid path for array".to_string(),
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
            let next_path = parts[..=index].join(separator);
            let next_should_be_array = path_types.get(&next_path).copied().unwrap_or(false);

            if arr[array_index].is_null() {
                arr[array_index] = if next_should_be_array {
                    Value::Array(vec![])
                } else {
                    Value::Object(Map::new())
                };
            }

            match &mut arr[array_index] {
                Value::Object(ref mut obj) => set_nested_value_recursive_with_types(
                    obj,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                ),
                Value::Array(ref mut nested_arr) => {
                    set_nested_value_recursive_for_array_with_types(
                        nested_arr,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                    )
                }
                _ => Err(FlattenError::InvalidJson(format!(
                    "Array element at index {array_index} has incompatible type"
                ))),
            }
        }
    } else {
        Err(FlattenError::InvalidJson(format!(
            "Expected array index but got: {part}"
        )))
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
