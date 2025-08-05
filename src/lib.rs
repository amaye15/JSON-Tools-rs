//! # JSON Tools RS
//!
//! A Rust library for advanced JSON manipulation, including flattening and unflattening
//! nested JSON structures with configurable filtering and replacement options.
//!
//! ## Features
//!
//! - **Unified API**: Single `JSONTools` entry point for both flattening and unflattening
//! - **Builder Pattern**: Fluent, chainable API for easy configuration
//! - **Advanced Filtering**: Remove empty values (strings, objects, arrays, null values)
//! - **Pattern Replacements**: Support for literal and regex-based key/value replacements
//! - **High Performance**: SIMD-accelerated JSON parsing with optimized algorithms
//! - **Batch Processing**: Handle single JSON strings or arrays of JSON strings
//! - **Comprehensive Error Handling**: Detailed error messages for debugging
//!
//! ## Quick Start with Unified API
//!
//! ### Flattening JSON
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .separator("::")
//!     .lowercase_keys(true)
//!     .key_replacement("regex:(User|Admin)_", "")
//!     .value_replacement("@example.com", "@company.org")
//!     .remove_empty_strings(true)
//!     .remove_nulls(true)
//!     .remove_empty_objects(true)
//!     .remove_empty_arrays(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"user::name": "John"}
//!         println!("{}", flattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ### Unflattening JSON
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! let flattened = r#"{"user::name": "John", "user::age": 30}"#;
//! let result = JSONTools::new()
//!     .unflatten()
//!     .separator("::")
//!     .lowercase_keys(true)
//!     .key_replacement("regex:(User|Admin)_", "")
//!     .value_replacement("@company.org", "@example.com")
//!     .remove_empty_strings(true)
//!     .remove_nulls(true)
//!     .remove_empty_objects(true)
//!     .remove_empty_arrays(true)
//!     .execute(flattened).unwrap();
//!
//! match result {
//!     JsonOutput::Single(unflattened) => {
//!         // Result: {"user": {"name": "John", "age": 30}}
//!         println!("{}", unflattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!


use regex::Regex;
use rustc_hash::FxHashMap;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::error::Error;
use std::sync::OnceLock;

// Global regex cache for better performance
static REGEX_CACHE: OnceLock<std::sync::Mutex<FxHashMap<String, Regex>>> = OnceLock::new();

fn get_cached_regex(pattern: &str) -> Result<Regex, regex::Error> {
    let cache = REGEX_CACHE.get_or_init(|| std::sync::Mutex::new(FxHashMap::default()));
    let mut cache_guard = cache.lock().unwrap();

    if let Some(regex) = cache_guard.get(pattern) {
        Ok(regex.clone())
    } else {
        let regex = Regex::new(pattern)?;
        cache_guard.insert(pattern.to_string(), regex.clone());
        Ok(regex)
    }
}

// String pool for reusing allocations
thread_local! {
    static STRING_POOL: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::with_capacity(64));
}

#[inline]
fn get_pooled_string() -> String {
    STRING_POOL.with(|pool| {
        pool.borrow_mut().pop().unwrap_or_else(|| String::with_capacity(64))
    })
}

#[inline]
fn return_pooled_string(mut s: String) {
    if s.capacity() <= 512 { // Only pool reasonably sized strings
        s.clear();
        STRING_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            if pool.len() < 32 { // Limit pool size
                pool.push(s);
            }
        });
    }
}

// Python bindings module
#[cfg(feature = "python")]
pub mod python;

// Tests module
#[cfg(test)]
mod tests;

/// Input type for JSON flattening operations with Cow optimization
#[derive(Debug, Clone)]
pub enum JsonInput<'a> {
    /// Single JSON string with Cow for efficient memory usage
    Single(Cow<'a, str>),
    /// Multiple JSON strings
    Multiple(&'a [&'a str]),
}

impl<'a> From<&'a str> for JsonInput<'a> {
    fn from(json: &'a str) -> Self {
        JsonInput::Single(Cow::Borrowed(json))
    }
}

impl<'a> From<&'a String> for JsonInput<'a> {
    fn from(json: &'a String) -> Self {
        JsonInput::Single(Cow::Borrowed(json.as_str()))
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

/// Comprehensive error type for all JSON Tools operations with detailed information and suggestions
#[derive(Debug, thiserror::Error)]
pub enum JsonToolsError {
    /// Error parsing JSON input with detailed context and suggestions
    #[error("JSON parsing failed: {message}\n💡 Suggestion: {suggestion}")]
    JsonParseError {
        message: String,
        suggestion: String,
        #[source]
        source: simd_json::Error,
    },

    /// Error compiling or using regex patterns with helpful suggestions
    #[error("Regex pattern error: {message}\n💡 Suggestion: {suggestion}")]
    RegexError {
        message: String,
        suggestion: String,
        #[source]
        source: regex::Error,
    },

    /// Invalid replacement pattern configuration with detailed guidance
    #[error("Invalid replacement pattern: {message}\n💡 Suggestion: {suggestion}")]
    InvalidReplacementPattern {
        message: String,
        suggestion: String,
    },

    /// Invalid JSON structure for the requested operation
    #[error("Invalid JSON structure: {message}\n💡 Suggestion: {suggestion}")]
    InvalidJsonStructure {
        message: String,
        suggestion: String,
    },

    /// Configuration error when operation mode is not set
    #[error("Operation mode not configured: {message}\n💡 Suggestion: {suggestion}")]
    ConfigurationError {
        message: String,
        suggestion: String,
    },

    /// Error processing batch item with detailed context
    #[error("Batch processing failed at index {index}: {message}\n💡 Suggestion: {suggestion}")]
    BatchProcessingError {
        index: usize,
        message: String,
        suggestion: String,
        #[source]
        source: Box<JsonToolsError>,
    },

    /// Input validation error with helpful guidance
    #[error("Input validation failed: {message}\n💡 Suggestion: {suggestion}")]
    InputValidationError {
        message: String,
        suggestion: String,
    },

    /// Serialization error when converting results back to JSON
    #[error("JSON serialization failed: {message}\n💡 Suggestion: {suggestion}")]
    SerializationError {
        message: String,
        suggestion: String,
        #[source]
        source: simd_json::Error,
    },
}

impl JsonToolsError {
    /// Create a JSON parse error with helpful suggestions
    pub fn json_parse_error(source: simd_json::Error) -> Self {
        let suggestion = "Verify your JSON syntax using a JSON validator. Common issues include: missing quotes around keys or values, trailing commas, unescaped characters, incomplete JSON (missing closing braces or brackets), or invalid escape sequences.";

        JsonToolsError::JsonParseError {
            message: source.to_string(),
            suggestion: suggestion.to_string(),
            source,
        }
    }

    /// Create a regex error with helpful suggestions
    pub fn regex_error(source: regex::Error) -> Self {
        let suggestion = match source {
            regex::Error::Syntax(_) =>
                "Check your regex pattern syntax. Use online regex testers to validate your pattern. Remember to escape special characters like '.', '*', '+', '?', etc.",
            regex::Error::CompiledTooBig(_) =>
                "Your regex pattern is too complex. Try simplifying it or breaking it into multiple smaller patterns.",
            _ => "Verify your regex pattern is valid. Use tools like regex101.com to test and debug your pattern.",
        };

        JsonToolsError::RegexError {
            message: source.to_string(),
            suggestion: suggestion.to_string(),
            source,
        }
    }

    /// Create an invalid replacement pattern error
    pub fn invalid_replacement_pattern(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("pairs") {
            "Replacement patterns must be provided in pairs (pattern, replacement). Ensure you have an even number of arguments."
        } else if msg.contains("regex:") {
            "When using regex patterns, prefix with 'regex:' followed by a valid regex pattern. Example: 'regex:user_.*' to match keys starting with 'user_'."
        } else {
            "Check your replacement pattern configuration. Patterns should be in the format: pattern1, replacement1, pattern2, replacement2, etc."
        };

        JsonToolsError::InvalidReplacementPattern {
            message: msg,
            suggestion: suggestion.to_string(),
        }
    }

    /// Create an invalid JSON structure error
    pub fn invalid_json_structure(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("unflatten") {
            "For unflattening, ensure your JSON is a flat object with dot-separated keys like {'user.name': 'John', 'user.age': 30}."
        } else if msg.contains("object") {
            "The operation requires a JSON object ({}), but received a different type. Check that your input is a valid JSON object."
        } else {
            "Verify that your JSON structure is compatible with the requested operation. Flattening works on nested objects/arrays, unflattening works on flat objects."
        };

        JsonToolsError::InvalidJsonStructure {
            message: msg,
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a configuration error
    pub fn configuration_error(message: impl Into<String>) -> Self {
        JsonToolsError::ConfigurationError {
            message: message.into(),
            suggestion: "Call .flatten() or .unflatten() on your JSONTools instance before calling .execute() to set the operation mode.".to_string(),
        }
    }

    /// Create a batch processing error
    pub fn batch_processing_error(index: usize, source: JsonToolsError) -> Self {
        JsonToolsError::BatchProcessingError {
            index,
            message: format!("Failed to process item at index {}", index),
            suggestion: "Check the JSON at the specified index. All items in a batch must be valid JSON strings or objects.".to_string(),
            source: Box::new(source),
        }
    }

    /// Create an input validation error
    pub fn input_validation_error(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("type") {
            "Ensure your input is a valid JSON string, Python dict, or list of JSON strings/dicts."
        } else if msg.contains("empty") {
            "Provide non-empty input for processing."
        } else {
            "Check that your input format matches the expected type for the operation."
        };

        JsonToolsError::InputValidationError {
            message: msg,
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a serialization error
    pub fn serialization_error(source: simd_json::Error) -> Self {
        JsonToolsError::SerializationError {
            message: source.to_string(),
            suggestion: "This is likely an internal error. The processed data couldn't be serialized back to JSON. Please report this issue.".to_string(),
            source,
        }
    }
}

// Automatic conversion from simd_json::Error
impl From<simd_json::Error> for JsonToolsError {
    fn from(error: simd_json::Error) -> Self {
        JsonToolsError::json_parse_error(error)
    }
}

// Automatic conversion from regex::Error
impl From<regex::Error> for JsonToolsError {
    fn from(error: regex::Error) -> Self {
        JsonToolsError::regex_error(error)
    }
}

/// Operation mode for the unified JSONTools API
#[derive(Debug, Clone, PartialEq)]
enum OperationMode {
    /// Flatten JSON structures
    Flatten,
    /// Unflatten JSON structures
    Unflatten,
}

/// Unified JSON Tools API with builder pattern for both flattening and unflattening operations
///
/// This is the unified interface for all JSON manipulation operations.
/// It provides a single entry point for all JSON manipulation operations with a consistent builder pattern.
///
/// # Examples
///
/// ```rust
/// use json_tools_rs::{JSONTools, JsonOutput};
///
/// // Flattening with advanced configuration
/// let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
/// let result = JSONTools::new()
///     .flatten()
///     .separator("::")
///     .lowercase_keys(true)
///     .key_replacement("regex:(User|Admin)_", "")
///     .key_replacement("Profile::", "")
///     .value_replacement("@example.com", "@company.org")
///     .value_replacement("regex:^super$", "administrator")
///     .remove_empty_strings(true)
///     .remove_nulls(true)
///     .remove_empty_objects(true)
///     .remove_empty_arrays(true)
///     .execute(json).unwrap();
///
/// match result {
///     JsonOutput::Single(flattened) => {
///         // Process flattened JSON
///         println!("{}", flattened);
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
///
/// // Unflattening with the same configuration options
/// let flattened = r#"{"user::name": "John", "user::age": 30}"#;
/// let result = JSONTools::new()
///     .unflatten()
///     .separator("::")
///     .lowercase_keys(true)
///     .remove_empty_strings(true)
///     .remove_nulls(true)
///     .remove_empty_objects(true)
///     .remove_empty_arrays(true)
///     .execute(flattened).unwrap();
///
/// match result {
///     JsonOutput::Single(unflattened) => {
///         // Process unflattened JSON
///         println!("{}", unflattened);
///     }
///     JsonOutput::Multiple(_) => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct JSONTools {
    /// Current operation mode (flatten or unflatten)
    mode: Option<OperationMode>,
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
    /// Avoid key collisions by appending index suffixes
    avoid_key_collision: bool,
    /// Handle key collisions by collecting values into arrays
    handle_key_collision: bool,
}

impl Default for JSONTools {
    fn default() -> Self {
        Self {
            mode: None,
            remove_empty_string_values: false,
            remove_null_values: false,
            remove_empty_dict: false,
            remove_empty_list: false,
            key_replacements: Vec::new(),
            value_replacements: Vec::new(),
            separator: ".".to_string(),
            lower_case_keys: false,
            avoid_key_collision: false,
            handle_key_collision: false,
        }
    }
}

impl JSONTools {
    /// Create a new JSONTools instance with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the operation mode to flatten
    pub fn flatten(mut self) -> Self {
        self.mode = Some(OperationMode::Flatten);
        self
    }

    /// Set the operation mode to unflatten
    pub fn unflatten(mut self) -> Self {
        self.mode = Some(OperationMode::Unflatten);
        self
    }

    /// Set the separator used for nested keys (default: ".")
    pub fn separator<S: Into<Cow<'static, str>>>(mut self, separator: S) -> Self {
        self.separator = separator.into().into_owned();
        self
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(mut self, value: bool) -> Self {
        self.lower_case_keys = value;
        self
    }

    /// Add a key replacement pattern
    ///
    /// Supports both literal strings and regex patterns (prefix with "regex:")
    /// Works for both flatten and unflatten operations
    pub fn key_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        self.key_replacements.push((find.into().into_owned(), replace.into().into_owned()));
        self
    }

    /// Add a value replacement pattern
    ///
    /// Supports both literal strings and regex patterns (prefix with "regex:")
    /// Works for both flatten and unflatten operations
    pub fn value_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        self.value_replacements.push((find.into().into_owned(), replace.into().into_owned()));
        self
    }

    /// Remove keys with empty string values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty string values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty string values
    pub fn remove_empty_strings(mut self, value: bool) -> Self {
        self.remove_empty_string_values = value;
        self
    }

    /// Remove keys with null values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have null values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have null values
    pub fn remove_nulls(mut self, value: bool) -> Self {
        self.remove_null_values = value;
        self
    }

    /// Remove keys with empty object values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty object values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty object values
    pub fn remove_empty_objects(mut self, value: bool) -> Self {
        self.remove_empty_dict = value;
        self
    }

    /// Remove keys with empty array values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty array values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty array values
    pub fn remove_empty_arrays(mut self, value: bool) -> Self {
        self.remove_empty_list = value;
        self
    }

    /// Avoid key collisions by appending index suffixes
    ///
    /// When enabled, if key replacement operations result in duplicate keys,
    /// automatically append an index suffix to make keys unique.
    /// The index uses the configured separator followed by a sequential number (0, 1, 2, etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     // With separator "::" and key replacement that causes collisions
    ///     let json = r#"{"User_name": "John", "Admin_name": "Jane"}"#;
    ///     let result = JSONTools::new()
    ///         .flatten()
    ///         .separator("::")
    ///         .key_replacement("regex:(User|Admin)_", "")
    ///         .avoid_key_collision(true)
    ///         .execute(json)?;
    ///
    ///     // Result: {"name::0": "John", "name::1": "Jane"}
    ///     Ok(())
    /// }
    /// ```
    ///
    /// **Note**: This feature is mutually exclusive with `handle_key_collision`.
    /// If both are enabled, `avoid_key_collision` takes precedence.
    ///
    /// Works for both flatten and unflatten operations.
    pub fn avoid_key_collision(mut self, value: bool) -> Self {
        self.avoid_key_collision = value;
        self
    }

    /// Handle key collisions by collecting values into arrays
    ///
    /// When enabled, instead of avoiding collisions by renaming keys,
    /// collect all values that would have the same key into an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     // Key replacement that causes collisions
    ///     let json = r#"{"User_name": "John", "Admin_name": "Jane"}"#;
    ///     let result = JSONTools::new()
    ///         .flatten()
    ///         .key_replacement("regex:(User|Admin)_", "")
    ///         .handle_key_collision(true)
    ///         .execute(json)?;
    ///
    ///     // Result: {"name": ["John", "Jane"]}
    ///     Ok(())
    /// }
    /// ```
    ///
    /// **Note**: This feature is mutually exclusive with `avoid_key_collision`.
    /// If both are enabled, `avoid_key_collision` takes precedence.
    ///
    /// Works for both flatten and unflatten operations.
    pub fn handle_key_collision(mut self, value: bool) -> Self {
        self.handle_key_collision = value;
        self
    }

    /// Execute the configured operation on the provided JSON input
    ///
    /// This method performs either flattening or unflattening based on the operation mode
    /// set by calling `.flatten()` or `.unflatten()` on the builder.
    ///
    /// # Arguments
    /// * `json_input` - JSON input that can be a single string, multiple strings, or other supported types
    ///
    /// # Returns
    /// * `Result<JsonOutput, Box<dyn Error>>` - The processed JSON result or an error
    ///
    /// # Errors
    /// * Returns an error if no operation mode has been set (must call `.flatten()` or `.unflatten()` first)
    /// * Returns an error if the JSON input is invalid
    /// * Returns an error if processing fails for any other reason
    ///
    /// # Examples
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     // Flatten operation
    ///     let result = JSONTools::new()
    ///         .flatten()
    ///         .separator("::")
    ///         .remove_nulls(true)
    ///         .execute(r#"{"user": {"name": "John", "age": null}}"#)?;
    ///
    ///     // Unflatten operation
    ///     let result = JSONTools::new()
    ///         .unflatten()
    ///         .separator("::")
    ///         .execute(r#"{"user::name": "John"}"#)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn execute<'a, T>(&self, json_input: T) -> Result<JsonOutput, Box<dyn Error>>
    where
        T: Into<JsonInput<'a>>,
    {
        // Ensure operation mode is set
        let mode = self.mode.as_ref().ok_or_else(|| {
            Box::new(JsonToolsError::configuration_error(
                "Operation mode not set. Call .flatten() or .unflatten() before .execute()"
            )) as Box<dyn Error>
        })?;

        let input = json_input.into();

        match mode {
            OperationMode::Flatten => {
                // Use the flattening logic
                let key_replacements = if self.key_replacements.is_empty() {
                    None
                } else {
                    Some(self.key_replacements.as_slice())
                };

                let value_replacements = if self.value_replacements.is_empty() {
                    None
                } else {
                    Some(self.value_replacements.as_slice())
                };

                match input {
                    JsonInput::Single(json_cow) => {
                        let result = process_single_json(
                            &json_cow,
                            self.remove_empty_string_values,
                            self.remove_null_values,
                            self.remove_empty_dict,
                            self.remove_empty_list,
                            key_replacements,
                            value_replacements,
                            &self.separator,
                            self.lower_case_keys,
                            self.avoid_key_collision,
                            self.handle_key_collision,
                        )?;
                        Ok(JsonOutput::Single(result))
                    }
                    JsonInput::Multiple(json_list) => {
                        let mut results = Vec::with_capacity(json_list.len());

                        for (index, json) in json_list.iter().enumerate() {
                            let json_cow = Cow::Borrowed(*json);
                            match process_single_json(
                                &json_cow,
                                self.remove_empty_string_values,
                                self.remove_null_values,
                                self.remove_empty_dict,
                                self.remove_empty_list,
                                key_replacements,
                                value_replacements,
                                &self.separator,
                                self.lower_case_keys,
                                self.avoid_key_collision,
                                self.handle_key_collision,
                            ) {
                                Ok(result) => results.push(result),
                                Err(e) => {
                                    return Err(Box::new(JsonToolsError::batch_processing_error(
                                        index,
                                        JsonToolsError::input_validation_error(format!("Processing failed: {}", e))
                                    )));
                                }
                            }
                        }

                        Ok(JsonOutput::Multiple(results))
                    }
                }
            }
            OperationMode::Unflatten => {
                // Use the unflattening logic
                match input {
                    JsonInput::Single(json_cow) => {
                        let result = process_single_json_for_unflatten(
                            &json_cow,
                            self.remove_empty_string_values,
                            self.remove_null_values,
                            self.remove_empty_dict,
                            self.remove_empty_list,
                            self.key_replacements.as_slice(),
                            self.value_replacements.as_slice(),
                            &self.separator,
                            self.lower_case_keys,
                            self.avoid_key_collision,
                            self.handle_key_collision,
                        )?;
                        Ok(JsonOutput::Single(result))
                    }
                    JsonInput::Multiple(json_list) => {
                        let mut results = Vec::with_capacity(json_list.len());

                        for (index, json) in json_list.iter().enumerate() {
                            let json_cow = Cow::Borrowed(*json);
                            match process_single_json_for_unflatten(
                                &json_cow,
                                self.remove_empty_string_values,
                                self.remove_null_values,
                                self.remove_empty_dict,
                                self.remove_empty_list,
                                self.key_replacements.as_slice(),
                                self.value_replacements.as_slice(),
                                &self.separator,
                                self.lower_case_keys,
                                self.avoid_key_collision,
                                self.handle_key_collision,
                            ) {
                                Ok(result) => results.push(result),
                                Err(e) => {
                                    return Err(Box::new(JsonToolsError::batch_processing_error(
                                        index,
                                        JsonToolsError::input_validation_error(format!("Processing failed: {}", e))
                                    )));
                                }
                            }
                        }

                        Ok(JsonOutput::Multiple(results))
                    }
                }
            }
        }
    }
}

/// Core unflattening logic for a single JSON string with Cow optimization
#[inline]
#[allow(clippy::too_many_arguments)]
fn process_single_json_for_unflatten(
    json: &Cow<str>,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: &[(String, String)],
    value_replacements: &[(String, String)],
    separator: &str,
    lower_case_keys: bool,
    avoid_key_collision: bool,
    handle_key_collision: bool,
) -> Result<String, Box<dyn Error>> {
    // Ultra-optimized SIMD JSON parsing - always use the fastest path
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
            return Err(Box::new(JsonToolsError::invalid_json_structure(
                "Expected object for unflattening"
            )))
        }
    };

    // Apply key and value replacements if specified
    let mut processed_obj = flattened_obj.clone();

    // Apply key replacements with collision detection if provided
    if !key_replacements.is_empty() {
        // Use optimized version when collision handling is disabled for better performance
        if !avoid_key_collision && !handle_key_collision {
            processed_obj = apply_key_replacements_for_unflatten(&processed_obj, key_replacements)?;
        } else {
            processed_obj = apply_key_replacements_unflatten_with_collisions(
                processed_obj,
                key_replacements,
                avoid_key_collision,
                handle_key_collision,
                separator,
                remove_empty_string_values,
                remove_null_values,
                remove_empty_dict,
                remove_empty_list,
            )?;
        }
    }

    // Apply value replacements
    if !value_replacements.is_empty() {
        apply_value_replacements_for_unflatten(&mut processed_obj, value_replacements)?;
    }

    // Apply lowercase conversion if specified
    if lower_case_keys {
        processed_obj = apply_lowercase_keys_for_unflatten(processed_obj);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if (avoid_key_collision || handle_key_collision) && key_replacements.is_empty() {
            processed_obj = handle_key_collisions_for_unflatten(
                processed_obj,
                avoid_key_collision,
                handle_key_collision,
                separator,
                remove_empty_string_values,
                remove_null_values,
                remove_empty_dict,
                remove_empty_list,
            );
        }
    } else if (avoid_key_collision || handle_key_collision) && key_replacements.is_empty() {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        processed_obj = handle_key_collisions_for_unflatten(
            processed_obj,
            avoid_key_collision,
            handle_key_collision,
            separator,
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
        );
    }

    // Perform the actual unflattening
    let mut unflattened = unflatten_object(&processed_obj, separator)?;

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

    // Serialize the result
    Ok(simd_json::serde::to_string(&unflattened)?)
}

/// Core flattening logic for a single JSON string with Cow optimization
#[inline]
#[allow(clippy::too_many_arguments)]
fn process_single_json(
    json: &Cow<str>,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<&[(String, String)]>,
    value_replacements: Option<&[(String, String)]>,
    separator: &str,
    lower_case_keys: bool,
    avoid_key_collision: bool,
    handle_key_collision: bool,
) -> Result<String, Box<dyn Error>> {
    // Ultra-optimized SIMD JSON parsing - always use the fastest path
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

    // Ultra-aggressive capacity estimation for maximum SIMD performance
    let estimated_capacity = estimate_flattened_size(&value);
    // Use extremely aggressive initial capacity to eliminate all rehashing
    let initial_capacity = std::cmp::max(
        estimated_capacity * 4, // Quadruple the estimated capacity for zero rehashing
        256 // Very high minimum capacity for SIMD-friendly operations
    );
    // Use FxHashMap for better performance with string keys
    let mut flattened = FxHashMap::with_capacity_and_hasher(
        initial_capacity,
        Default::default()
    );

    // Flatten the JSON structure with key building
    // Ultra-aggressive string builder capacity for SIMD performance
    let max_key_length = estimate_max_key_length(&value);
    // Use massive extra capacity to ensure zero reallocations for SIMD efficiency
    let builder_capacity = std::cmp::max(max_key_length * 4, 512);
    let mut builder = FastStringBuilder::with_capacity_and_separator(builder_capacity, separator);
    flatten_value(&value, &mut builder, &mut flattened);

    // Apply key replacements with collision detection if provided
    if let Some(key_tuples) = key_replacements {
        // Convert tuple format to the internal vector format
        let key_patterns = convert_tuples_to_patterns(key_tuples);

        // Use the consolidated function that handles both optimized and collision scenarios
        flattened = apply_key_replacements_with_collision_handling(
            flattened,
            &key_patterns,
            avoid_key_collision,
            handle_key_collision,
            separator,
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
        )?;
    }

    // Apply lowercase conversion to keys if requested
    if lower_case_keys {
        flattened = apply_lowercase_keys(flattened);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if (avoid_key_collision || handle_key_collision) && key_replacements.is_none() {
            flattened = handle_key_collisions(flattened, avoid_key_collision, handle_key_collision, separator);
        }
    } else if (avoid_key_collision || handle_key_collision) && key_replacements.is_none() {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        flattened = handle_key_collisions(flattened, avoid_key_collision, handle_key_collision, separator);
    }

    // Apply value replacements if provided
    if let Some(value_tuples) = value_replacements {
        // Convert tuple format to the internal vector format
        let value_patterns = convert_tuples_to_patterns(value_tuples);
        apply_value_replacements(&mut flattened, &value_patterns)?;
    }

    // Apply filtering AFTER replacements to catch newly created empty values
    // This ensures that values replaced with empty strings are properly removed
    if remove_null_values || remove_empty_string_values || remove_empty_dict || remove_empty_list {
        // First pass: filter inside arrays that were created by collision handling
        if handle_key_collision {
            for (_, v) in flattened.iter_mut() {
                if let Some(arr) = v.as_array_mut() {
                    // Filter elements inside collision-created arrays
                    arr.retain(|element| {
                        if remove_empty_string_values {
                            if let Some(s) = element.as_str() {
                                if s.is_empty() {
                                    return false;
                                }
                            }
                        }
                        if remove_null_values && element.is_null() {
                            return false;
                        }
                        if remove_empty_dict {
                            if let Some(obj) = element.as_object() {
                                if obj.is_empty() {
                                    return false;
                                }
                            }
                        }
                        if remove_empty_list {
                            if let Some(inner_arr) = element.as_array() {
                                if inner_arr.is_empty() {
                                    return false;
                                }
                            }
                        }
                        true
                    });
                }
            }
        }

        // Second pass: filter top-level key-value pairs
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



    // Convert back to JSON string using simd-json serialization
    serialize_flattened(&flattened).map_err(|e| Box::new(e) as Box<dyn Error>)
}

/// Convert tuple-based replacement patterns to the internal vector format
/// This converts the intuitive tuple format to the internal representation used by replacement functions
/// Optimized to use string references instead of cloning to reduce memory allocations
#[inline]
fn convert_tuples_to_patterns(tuples: &[(String, String)]) -> Vec<&str> {
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
#[inline]
fn apply_lowercase_keys(flattened: FxHashMap<String, Value>) -> FxHashMap<String, Value> {
    let mut result = FxHashMap::with_capacity_and_hasher(flattened.len(), Default::default());
    for (key, value) in flattened {
        // Use Cow to avoid allocation if the key is already lowercase
        let lowercase_key = if key.chars().any(|c| c.is_uppercase()) {
            Cow::Owned(key.to_lowercase())
        } else {
            Cow::Borrowed(&key)
        };

        let final_key = match lowercase_key {
            Cow::Borrowed(_) => key, // Key was already lowercase, reuse original
            Cow::Owned(lower) => lower, // Key was converted to lowercase
        };

        result.insert(final_key, value);
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



/// Apply value replacements to a single value (for root-level primitives)
/// Optimized with Cow to avoid unnecessary string allocations
fn apply_value_replacements_to_single(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    if let Value::String(s) = value {
        let mut current_value = Cow::Borrowed(s.as_str());
        let mut changed = false;

        for (pattern, replacement) in patterns {
            if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                let regex = get_cached_regex(regex_pattern)?;
                if regex.is_match(&current_value) {
                    current_value = Cow::Owned(regex.replace_all(&current_value, replacement).into_owned());
                    changed = true;
                }
            } else if current_value.contains(pattern) {
                current_value = Cow::Owned(current_value.replace(pattern, replacement));
                changed = true;
            }
        }

        if changed {
            *s = current_value.into_owned();
        }
    }
    Ok(())
}

/// Value replacement with regex caching - optimized to use string references
fn apply_value_replacements(
    flattened: &mut FxHashMap<String, Value>,
    patterns: &[&str],
) -> Result<(), JsonToolsError> {
    if patterns.len() % 2 != 0 {
        return Err(JsonToolsError::invalid_replacement_pattern(
            "Value replacement patterns must be provided in pairs (pattern, replacement)"
        ));
    }

    // Pre-compile all regex patterns
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
            let regex = get_cached_regex(regex_pattern)?;
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
                let pattern = chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_value) {
                        new_value = Cow::Owned(
                            regex
                                .replace_all(&new_value, *replacement)
                                .to_string(),
                        );
                        changed = true;
                    }
                } else if new_value.contains(pattern) {
                    new_value = Cow::Owned(new_value.replace(pattern, *replacement));
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

/// JSON serialization with simd-json integration
#[inline]
fn serialize_flattened(
    flattened: &FxHashMap<String, Value>,
) -> Result<String, simd_json::Error> {
    // Always use SIMD for maximum performance - use simd_json for all serialization
    let json_obj = Value::Object(flattened.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
    simd_json::serde::to_string(&json_obj)
}

/// Value serialization with branch prediction optimization
#[inline]
fn serialize_value(value: &Value, result: &mut String) -> Result<(), simd_json::Error> {
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

/// JSON size estimation with better accuracy
#[inline]
fn estimate_json_size(flattened: &FxHashMap<String, Value>) -> usize {
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
                result.push_str(&format!("\\u{:04x}", b));
            }
            _ => result.push(byte as char),
        }
    }
    Cow::Owned(result)
}

/// Cached separator information for operations with Cow optimization
#[derive(Clone)]
struct SeparatorCache {
    separator: Cow<'static, str>,    // Cow for efficient memory usage
    is_single_char: bool,            // True if separator is a single character
    single_char: Option<char>,       // The character if single-char separator
    length: usize,                   // Pre-computed length
    is_common: bool,                 // True if it's a common separator (., _, ::, /, -)
}

impl SeparatorCache {
    #[inline]
    fn new(separator: &str) -> Self {
        // Check for common static separators to avoid heap allocations
        let (separator_cow, is_common) = match separator {
            "." => (Cow::Borrowed("."), true),
            "_" => (Cow::Borrowed("_"), true),
            "::" => (Cow::Borrowed("::"), true),
            "/" => (Cow::Borrowed("/"), true),
            "-" => (Cow::Borrowed("-"), true),
            "|" => (Cow::Borrowed("|"), true),
            _ => (Cow::Owned(separator.to_string()), false),
        };

        let is_single_char = separator.len() == 1;
        let single_char = if is_single_char {
            separator.chars().next()
        } else {
            None
        };

        Self {
            separator: separator_cow,
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
        } else {
            // Use Cow for efficient string handling
            buffer.push_str(&self.separator);
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
            // Pre-allocate capacity to avoid reallocations with extra buffer
            self.separator_cache
                .reserve_capacity_for_append(&mut self.buffer, key.len() + 8); // Extra buffer for future appends
            self.separator_cache.append_to_buffer(&mut self.buffer);
        } else {
            // Reserve capacity for the key plus some extra buffer
            let needed = key.len() + 16; // Extra buffer for future operations
            if self.buffer.capacity() < self.buffer.len() + needed {
                self.buffer.reserve(needed);
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
    fn into_string(self) -> String {
        self.buffer
    }

    #[inline]
    fn clone_string(&self) -> String {
        self.buffer.clone()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// Flattening using FastStringBuilder and aggressive inlining with optimized key allocation
#[inline]
fn flatten_value(
    value: &Value,
    builder: &mut FastStringBuilder,
    result: &mut FxHashMap<String, Value>,
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
                    flatten_value(val, builder, result);
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
                    flatten_value(val, builder, result);
                    builder.pop_level();
                }
            }
        }
        _ => {
            // Ultra-optimized key creation using string pool
            let key_str = builder.as_str();

            // Use pooled string for better memory reuse
            let mut key = get_pooled_string();
            if key.capacity() < key_str.len() {
                key.reserve(key_str.len() - key.capacity());
            }
            key.push_str(key_str);

            // Optimize value insertion - avoid cloning for simple types
            let optimized_value = match value {
                Value::String(s) => {
                    // For strings, we can potentially optimize further
                    if s.len() < 64 {
                        value.clone() // Small strings are cheap to clone
                    } else {
                        value.clone() // Still need to clone for now
                    }
                }
                Value::Number(_) | Value::Bool(_) | Value::Null => {
                    // These are very cheap to clone
                    value.clone()
                }
                _ => value.clone(),
            };

            // Insert with the pooled key
            result.insert(key, optimized_value);
        }
    }
}

/// Apply key replacements for unflattening (works on Map<String, Value>)
/// This version is used when collision handling is NOT enabled for better performance
/// Optimized with Cow to avoid unnecessary string allocations
fn apply_key_replacements_for_unflatten(
    obj: &Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<Map<String, Value>, JsonToolsError> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        let mut new_key = Cow::Borrowed(key.as_str());
        let mut changed = false;

        // Apply each replacement pattern
        for (pattern, replacement) in patterns {
            if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                // Handle regex replacement
                let regex = get_cached_regex(regex_pattern)?;
                if regex.is_match(&new_key) {
                    new_key = Cow::Owned(regex.replace_all(&new_key, replacement).into_owned());
                    changed = true;
                }
            } else if new_key.contains(pattern) {
                // Handle literal replacement
                new_key = Cow::Owned(new_key.replace(pattern, replacement));
                changed = true;
            }
        }

        let final_key = if changed { new_key.into_owned() } else { key.clone() };
        new_obj.insert(final_key, value.clone());
    }

    Ok(new_obj)
}

/// Apply value replacements for unflattening (works on Map<String, Value>)
/// Optimized with Cow to avoid unnecessary string allocations
fn apply_value_replacements_for_unflatten(
    obj: &mut Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    for (_, value) in obj.iter_mut() {
        if let Value::String(s) = value {
            let mut current_value = Cow::Borrowed(s.as_str());
            let mut changed = false;

            for (pattern, replacement) in patterns {
                if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
                    // Handle regex replacement
                    let regex = get_cached_regex(regex_pattern)?;
                    if regex.is_match(&current_value) {
                        current_value = Cow::Owned(regex.replace_all(&current_value, replacement).into_owned());
                        changed = true;
                    }
                } else if current_value.contains(pattern) {
                    // Handle literal replacement
                    current_value = Cow::Owned(current_value.replace(pattern, replacement));
                    changed = true;
                }
            }

            if changed {
                *s = current_value.into_owned();
            }
        }
    }
    Ok(())
}

/// Apply lowercase conversion to keys for unflattening
/// Optimized with Cow to avoid unnecessary allocations when keys are already lowercase
fn apply_lowercase_keys_for_unflatten(obj: Map<String, Value>) -> Map<String, Value> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        // Use Cow to avoid allocation if the key is already lowercase
        let lowercase_key = if key.chars().any(|c| c.is_uppercase()) {
            Cow::Owned(key.to_lowercase())
        } else {
            Cow::Borrowed(&key)
        };

        let final_key = match lowercase_key {
            Cow::Borrowed(_) => key, // Key was already lowercase, reuse original
            Cow::Owned(lower) => lower, // Key was converted to lowercase
        };

        new_obj.insert(final_key, value);
    }

    new_obj
}

/// Core unflattening algorithm that reconstructs nested JSON from flattened keys
fn unflatten_object(obj: &Map<String, Value>, separator: &str) -> Result<Value, JsonToolsError> {
    let mut result = Map::new();

    // Pre-analyze all keys to determine if paths should be arrays or objects
    let path_types = analyze_path_types(obj, separator);

    for (key, value) in obj {
        set_nested_value_with_types(&mut result, key, value.clone(), separator, &path_types)?;
    }

    Ok(Value::Object(result))
}

/// Analyze all flattened keys to determine whether each path should be an array or object
fn analyze_path_types(obj: &Map<String, Value>, separator: &str) -> FxHashMap<String, bool> {
    let mut path_types = FxHashMap::default(); // true = array, false = object

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
    path_types: &FxHashMap<String, bool>,
) -> Result<(), JsonToolsError> {
    let parts: Vec<&str> = key_path.split(separator).collect();

    if parts.is_empty() {
        return Err(JsonToolsError::invalid_json_structure("Empty key path"));
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
    path_types: &FxHashMap<String, bool>,
) -> Result<(), JsonToolsError> {
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
        _ => Err(JsonToolsError::invalid_json_structure(format!(
            "Cannot navigate into non-object/non-array value at key: {}",
            part
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
    path_types: &FxHashMap<String, bool>,
) -> Result<(), JsonToolsError> {
    if index >= parts.len() {
        return Err(JsonToolsError::invalid_json_structure(
            "Invalid path for array"
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
                _ => Err(JsonToolsError::invalid_json_structure(format!(
                    "Array element at index {} has incompatible type",
                    array_index
                ))),
            }
        }
    } else {
        Err(JsonToolsError::invalid_json_structure(format!(
            "Expected array index but got: {}",
            part
        )))
    }
}

/// Recursively filter nested JSON values based on the specified criteria
/// This function removes empty strings, nulls, empty objects, and empty arrays from nested JSON structures
fn filter_nested_value(
    value: &mut Value,
    remove_empty_strings: bool,
    remove_nulls: bool,
    remove_empty_objects: bool,
    remove_empty_arrays: bool,
) {
    match value {
        Value::Object(ref mut obj) => {
            // First, recursively filter all nested values
            for (_, v) in obj.iter_mut() {
                filter_nested_value(v, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays);
            }

            // Then remove keys that match our filtering criteria
            obj.retain(|_, v| {
                // Check for empty strings
                if remove_empty_strings {
                    if let Some(s) = v.as_str() {
                        if s.is_empty() {
                            return false;
                        }
                    }
                }

                // Check for nulls
                if remove_nulls && v.is_null() {
                    return false;
                }

                // Check for empty objects
                if remove_empty_objects {
                    if let Some(obj) = v.as_object() {
                        if obj.is_empty() {
                            return false;
                        }
                    }
                }

                // Check for empty arrays
                if remove_empty_arrays {
                    if let Some(arr) = v.as_array() {
                        if arr.is_empty() {
                            return false;
                        }
                    }
                }

                true
            });
        }
        Value::Array(ref mut arr) => {
            // First, recursively filter all nested values
            for item in arr.iter_mut() {
                filter_nested_value(item, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays);
            }

            // Then remove array elements that match our filtering criteria
            arr.retain(|v| {
                // Check for empty strings
                if remove_empty_strings {
                    if let Some(s) = v.as_str() {
                        if s.is_empty() {
                            return false;
                        }
                    }
                }

                // Check for nulls
                if remove_nulls && v.is_null() {
                    return false;
                }

                // Check for empty objects
                if remove_empty_objects {
                    if let Some(obj) = v.as_object() {
                        if obj.is_empty() {
                            return false;
                        }
                    }
                }

                // Check for empty arrays
                if remove_empty_arrays {
                    if let Some(arr) = v.as_array() {
                        if arr.is_empty() {
                            return false;
                        }
                    }
                }

                true
            });
        }
        _ => {
            // For primitive values (strings, numbers, booleans, null), no filtering needed
            // The filtering will be handled by the parent container
        }
    }
}

/// Handle key collisions in a flattened map
///
/// This function processes a HashMap to handle cases where multiple keys would collide
/// after key replacements and transformations. It supports two strategies:
///
/// 1. `avoid_key_collision`: Append index suffixes to make keys unique
/// 2. `handle_key_collision`: Collect values into arrays for duplicate keys
///
/// If both options are enabled, `avoid_key_collision` takes precedence.
fn handle_key_collisions(
    mut flattened: FxHashMap<String, Value>,
    avoid_key_collision: bool,
    handle_key_collision: bool,
    separator: &str,
) -> FxHashMap<String, Value> {
    // If neither option is enabled, return as-is
    if !avoid_key_collision && !handle_key_collision {
        return flattened;
    }

    // Group values by key to detect collisions
    let mut key_groups: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    for (key, value) in flattened.drain() {
        key_groups.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut result = FxHashMap::with_capacity_and_hasher(key_groups.len(), Default::default());

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected
            if avoid_key_collision {
                // Strategy 1: Append index suffixes to avoid collisions
                for (index, value) in values.into_iter().enumerate() {
                    let new_key = format!("{}{}{}", key, separator, index);
                    result.insert(new_key, value);
                }
            } else if handle_key_collision {
                // Strategy 2: Collect values into an array
                let array_value = Value::Array(values);
                result.insert(key, array_value);
            }
        }
    }

    result
}

/// Handle key collisions for unflattening operations
///
/// This function processes a Map<String, Value> (flattened object) to handle cases where
/// multiple keys would collide after key replacements and transformations. It supports
/// the same two strategies as the flattening version:
///
/// 1. `avoid_key_collision`: Append index suffixes to make keys unique
/// 2. `handle_key_collision`: Collect values into arrays for duplicate keys
///
/// If both options are enabled, `avoid_key_collision` takes precedence.
fn handle_key_collisions_for_unflatten(
    flattened_obj: Map<String, Value>,
    avoid_key_collision: bool,
    handle_key_collision: bool,
    separator: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> Map<String, Value> {
    // If neither option is enabled, return as-is
    if !avoid_key_collision && !handle_key_collision {
        return flattened_obj;
    }

    // Group values by key to detect collisions
    let mut key_groups: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    for (key, value) in flattened_obj {
        key_groups.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut result = Map::new();

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected
            if avoid_key_collision {
                // Strategy 1: Append index suffixes to avoid collisions
                for (index, value) in values.into_iter().enumerate() {
                    let new_key = format!("{}{}{}", key, separator, index);
                    result.insert(new_key, value);
                }
            } else if handle_key_collision {
                // Strategy 2: Collect values into an array, filtering out unwanted values
                let filtered_values: Vec<Value> = values.into_iter().filter(|value| {
                    should_include_value(
                        value,
                        remove_empty_string_values,
                        remove_null_values,
                        remove_empty_dict,
                        remove_empty_list,
                    )
                }).collect();

                // Only create the array if we have values after filtering
                if !filtered_values.is_empty() {
                    let array_value = Value::Array(filtered_values);
                    result.insert(key, array_value);
                }
                // If all values were filtered out, don't insert anything
            }
        }
    }

    result
}

/// Helper function to determine if a value should be included based on filtering criteria
/// This ensures consistent filtering logic across both flatten and unflatten operations
fn should_include_value(
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

/// Apply key replacements with collision handling for flattening operations
///
/// This function combines key replacement and collision detection with performance optimizations
/// including regex pre-compilation, early exit checks, and efficient string handling.
/// It properly handles cases where multiple keys would map to the same result after replacement.
fn apply_key_replacements_with_collision_handling(
    flattened: FxHashMap<String, Value>,
    patterns: &[&str],
    avoid_key_collision: bool,
    handle_key_collision: bool,
    separator: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> Result<FxHashMap<String, Value>, Box<dyn Error>> {
    if patterns.is_empty() {
        return Ok(flattened);
    }

    if patterns.len() % 2 != 0 {
        return Err(Box::new(JsonToolsError::invalid_replacement_pattern(
            "Patterns must be provided in pairs (find, replace)"
        )));
    }

    // Pre-compile all regex patterns to avoid repeated compilation
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        if let Some(regex_pattern) = pattern.strip_prefix("regex:") {
            let regex = get_cached_regex(regex_pattern)
                .map_err(|e| Box::new(JsonToolsError::regex_error(e)))?;
            compiled_patterns.push((Some(regex), replacement));
        } else {
            compiled_patterns.push((None, replacement));
        }
    }

    // Early exit optimization: check if any keys need replacement to avoid unnecessary allocation
    if !avoid_key_collision && !handle_key_collision {
        // When collision handling is disabled, we can use the optimized path
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

        // Use optimized path for non-collision scenarios with Cow for efficient string handling
        let mut new_flattened = FxHashMap::with_capacity_and_hasher(flattened.len(), Default::default());

        for (old_key, value) in flattened {
            let mut new_key = Cow::Borrowed(old_key.as_str());

            // Apply each compiled pattern
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_key) {
                        new_key = Cow::Owned(
                            regex
                                .replace_all(&new_key, *replacement)
                                .to_string(),
                        );
                    }
                } else if new_key.contains(pattern) {
                    new_key = Cow::Owned(new_key.replace(pattern, *replacement));
                }
            }

            new_flattened.insert(new_key.into_owned(), value);
        }

        return Ok(new_flattened);
    }

    // Collision handling path: apply replacements and track what each original key maps to
    let mut key_mapping: FxHashMap<String, String> = FxHashMap::default();
    let mut original_values: FxHashMap<String, Value> = FxHashMap::default();

    for (original_key, value) in flattened {
        let mut new_key = Cow::Borrowed(original_key.as_str());

        // Apply all key replacement patterns using pre-compiled patterns
        for (i, chunk) in patterns.chunks(2).enumerate() {
            let pattern = chunk[0];
            let (compiled_regex, replacement) = &compiled_patterns[i];

            if let Some(regex) = compiled_regex {
                if regex.is_match(&new_key) {
                    new_key = Cow::Owned(
                        regex
                            .replace_all(&new_key, *replacement)
                            .to_string(),
                    );
                }
            } else if new_key.contains(pattern) {
                new_key = Cow::Owned(new_key.replace(pattern, *replacement));
            }
        }

        key_mapping.insert(original_key.clone(), new_key.into_owned());
        original_values.insert(original_key, value);
    }

    // Second pass: group by target key to detect collisions
    let mut target_groups: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for (original_key, target_key) in &key_mapping {
        target_groups.entry(target_key.clone()).or_insert_with(Vec::new).push(original_key.clone());
    }

    // Third pass: build result with collision handling
    let mut result = FxHashMap::with_capacity_and_hasher(target_groups.len(), Default::default());

    for (target_key, original_keys) in target_groups {
        if original_keys.len() == 1 {
            // No collision
            let original_key = &original_keys[0];
            let value = original_values.remove(original_key).unwrap();
            result.insert(target_key, value);
        } else {
            // Collision detected
            if avoid_key_collision {
                // Strategy 1: Append index suffixes to avoid collisions
                for (index, original_key) in original_keys.iter().enumerate() {
                    let value = original_values.remove(original_key).unwrap();
                    let new_key = format!("{}{}{}", target_key, separator, index);
                    result.insert(new_key, value);
                }
            } else if handle_key_collision {
                // Strategy 2: Collect values into an array, filtering out unwanted values
                let mut values = Vec::new();
                for original_key in &original_keys {
                    let value = original_values.remove(original_key).unwrap();

                    // Apply filtering to values before adding to collision array
                    let should_include = should_include_value(
                        &value,
                        remove_empty_string_values,
                        remove_null_values,
                        remove_empty_dict,
                        remove_empty_list,
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
            } else {
                // No collision handling enabled, use the last value (default behavior)
                let last_original_key = original_keys.last().unwrap();
                let value = original_values.remove(last_original_key).unwrap();
                result.insert(target_key, value);
            }
        }
    }

    Ok(result)
}

/// Apply key replacements with collision handling for unflattening operations
///
/// This function combines key replacement and collision detection for Map<String, Value>
/// to properly handle cases where multiple keys would map to the same result after replacement.
fn apply_key_replacements_unflatten_with_collisions(
    flattened_obj: Map<String, Value>,
    key_replacements: &[(String, String)],
    avoid_key_collision: bool,
    handle_key_collision: bool,
    separator: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    if key_replacements.is_empty() {
        return Ok(flattened_obj);
    }

    // First pass: apply replacements and track what each original key maps to
    let mut key_mapping: FxHashMap<String, String> = FxHashMap::default();
    let mut original_values: FxHashMap<String, Value> = FxHashMap::default();

    for (original_key, value) in flattened_obj {
        let mut new_key = original_key.clone();

        // Apply all key replacement patterns
        for (find, replace) in key_replacements {
            if find.starts_with("regex:") {
                // Handle regex replacement
                let pattern = &find[6..]; // Remove "regex:" prefix
                let regex = get_cached_regex(pattern)
                    .map_err(|e| Box::new(JsonToolsError::regex_error(e)))?;
                new_key = regex.replace_all(&new_key, replace).to_string();
            } else {
                // Handle literal replacement
                new_key = new_key.replace(find, replace);
            }
        }

        key_mapping.insert(original_key.clone(), new_key);
        original_values.insert(original_key, value);
    }

    // Second pass: group by target key to detect collisions
    let mut target_groups: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for (original_key, target_key) in &key_mapping {
        target_groups.entry(target_key.clone()).or_insert_with(Vec::new).push(original_key.clone());
    }

    // Third pass: build result with collision handling
    let mut result = Map::new();

    for (target_key, original_keys) in target_groups {
        if original_keys.len() == 1 {
            // No collision
            let original_key = &original_keys[0];
            let value = original_values.remove(original_key).unwrap();
            result.insert(target_key, value);
        } else {
            // Collision detected
            if avoid_key_collision {
                // Strategy 1: Append index suffixes to avoid collisions
                // Use the provided separator for consistency with flatten operations
                for (index, original_key) in original_keys.iter().enumerate() {
                    let value = original_values.remove(original_key).unwrap();
                    let new_key = format!("{}{}{}", target_key, separator, index);
                    result.insert(new_key, value);
                }
            } else if handle_key_collision {
                // Strategy 2: Collect values into an array, filtering out unwanted values
                let mut values = Vec::new();
                for original_key in &original_keys {
                    let value = original_values.remove(original_key).unwrap();

                    // Apply filtering to values before adding to collision array
                    let should_include = should_include_value(
                        &value,
                        remove_empty_string_values,
                        remove_null_values,
                        remove_empty_dict,
                        remove_empty_list,
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
            } else {
                // No collision handling enabled, use the last value (default behavior)
                let last_original_key = original_keys.last().unwrap();
                let value = original_values.remove(last_original_key).unwrap();
                result.insert(target_key, value);
            }
        }
    }

    Ok(result)
}


