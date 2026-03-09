use smallvec::SmallVec;
use std::borrow::Cow;

use crate::config::{
    CollisionConfig, FilteringConfig, OperationMode, ProcessingConfig, ReplacementConfig,
};
use crate::error::JsonToolsError;
use crate::flatten::process_single_json;
use crate::transform::process_single_json_normal;
use crate::types::{JsonInput, JsonOutput};
use crate::unflatten::process_single_json_for_unflatten;

// ================================================================================================
// ProcessingConfig::from_json_tools() - lives here to avoid circular dependency
// ================================================================================================

impl ProcessingConfig {
    /// Create a ProcessingConfig from a JSONTools builder instance
    pub fn from_json_tools(tools: &JSONTools) -> Self {
        Self {
            separator: tools.separator.clone(),
            lowercase_keys: tools.lower_case_keys,
            filtering: FilteringConfig {
                remove_empty_strings: tools.remove_empty_string_values,
                remove_nulls: tools.remove_null_values,
                remove_empty_objects: tools.remove_empty_dict,
                remove_empty_arrays: tools.remove_empty_list,
            },
            collision: CollisionConfig {
                handle_collisions: tools.handle_key_collision,
            },
            replacements: ReplacementConfig {
                key_replacements: tools.key_replacements.clone(),
                value_replacements: tools.value_replacements.clone(),
            },
            auto_convert_types: tools.auto_convert_types,
            parallel_threshold: tools.parallel_threshold,
            num_threads: tools.num_threads,
            nested_parallel_threshold: tools.nested_parallel_threshold,
        }
    }
}

// ================================================================================================
// JSONTools Builder Struct
// ================================================================================================

/// Unified JSON Tools API with builder pattern for both flattening and unflattening operations
///
/// This is the unified interface for all JSON manipulation operations.
/// It provides a single entry point for all JSON manipulation operations with a consistent builder pattern.
///
/// Fields are ordered by size for better memory alignment and cache locality:
/// - Large fields (24 bytes each): Vec, String
/// - Medium fields (2 bytes): Option<OperationMode>
/// - Small fields (1 byte each): bool flags
#[derive(Debug, Clone)]
pub struct JSONTools {
    // SmallVec fields (stack-allocated for 0-2 replacements, common case)
    /// Key replacement patterns (find, replace)
    /// Uses SmallVec to avoid heap allocation for 0-2 replacements (90% of use cases)
    key_replacements: SmallVec<[(String, String); 2]>,
    /// Value replacement patterns (find, replace)
    /// Uses SmallVec to avoid heap allocation for 0-2 replacements (90% of use cases)
    value_replacements: SmallVec<[(String, String); 2]>,
    /// Separator for nested keys (default: ".")
    separator: String,

    // Medium fields (8 bytes on 64-bit systems)
    /// Minimum batch size to use parallel processing (default: 10)
    parallel_threshold: usize,
    /// Number of threads for parallel processing (None = use system default)
    num_threads: Option<usize>,
    /// Minimum object/array size for nested parallel processing within a single JSON document
    nested_parallel_threshold: usize,

    // Medium fields (2 bytes)
    /// Current operation mode (flatten or unflatten)
    mode: Option<OperationMode>,

    // Small fields (1 byte each) - grouped together to minimize padding
    /// Remove keys with empty string values
    remove_empty_string_values: bool,
    /// Remove keys with null values
    remove_null_values: bool,
    /// Remove keys with empty object values
    remove_empty_dict: bool,
    /// Remove keys with empty array values
    remove_empty_list: bool,
    /// Convert all keys to lowercase
    lower_case_keys: bool,
    /// Handle key collisions by collecting values into arrays
    handle_key_collision: bool,
    /// Automatically convert string values to numbers and booleans
    auto_convert_types: bool,
}

impl Default for JSONTools {
    fn default() -> Self {
        Self {
            // SmallVec fields - no heap allocation for 0-2 replacements!
            key_replacements: SmallVec::new(),
            value_replacements: SmallVec::new(),
            separator: ".".to_string(),
            // Medium fields
            parallel_threshold: std::env::var("JSON_TOOLS_PARALLEL_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            num_threads: std::env::var("JSON_TOOLS_NUM_THREADS")
                .ok()
                .and_then(|v| v.parse().ok()),
            nested_parallel_threshold: std::env::var("JSON_TOOLS_NESTED_PARALLEL_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            mode: None,
            // Small fields
            remove_empty_string_values: false,
            remove_null_values: false,
            remove_empty_dict: false,
            remove_empty_list: false,
            lower_case_keys: false,
            handle_key_collision: false,
            auto_convert_types: false,
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

    /// Set the operation mode to normal (no flatten/unflatten)
    pub fn normal(mut self) -> Self {
        self.mode = Some(OperationMode::Normal);
        self
    }

    /// Set the separator used for nested keys (default: ".")
    pub fn separator<S: Into<Cow<'static, str>>>(mut self, separator: S) -> Self {
        let sep_cow = separator.into();
        // Only allocate if we need to own the string
        self.separator = match sep_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        self
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(mut self, value: bool) -> Self {
        self.lower_case_keys = value;
        self
    }

    /// Add a key replacement pattern
    ///
    /// Patterns are treated as regex patterns using standard Rust regex syntax.
    /// If a pattern fails to compile as regex, it falls back to literal string replacement.
    /// Works for both flatten and unflatten operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// // Regex pattern (standard Rust regex syntax)
    /// let json = r#"{"user_name": "John", "admin_name": "Jane"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .key_replacement("(user|admin)_", "person_")
    ///     .execute(json).unwrap();
    ///
    /// // Literal pattern (if regex compilation fails)
    /// let result2 = JSONTools::new()
    ///     .flatten()
    ///     .key_replacement("user_", "person_")
    ///     .execute(json).unwrap();
    /// ```
    pub fn key_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        let find_cow = find.into();
        let replace_cow = replace.into();

        // Only allocate when necessary
        let find_string = match find_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        let replace_string = match replace_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };

        self.key_replacements.push((find_string, replace_string));
        self
    }

    /// Add a value replacement pattern
    ///
    /// Patterns are treated as regex patterns using standard Rust regex syntax.
    /// If a pattern fails to compile as regex, it falls back to literal string replacement.
    /// Works for both flatten and unflatten operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// // Regex pattern (standard Rust regex syntax)
    /// let json = r#"{"role": "super", "level": "admin"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .value_replacement("^(super|admin)$", "administrator")
    ///     .execute(json).unwrap();
    ///
    /// // Literal pattern (if regex compilation fails)
    /// let result2 = JSONTools::new()
    ///     .flatten()
    ///     .value_replacement("@example.com", "@company.org")
    ///     .execute(json).unwrap();
    /// ```
    pub fn value_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        let find_cow = find.into();
        let replace_cow = replace.into();

        // Only allocate when necessary
        let find_string = match find_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        let replace_string = match replace_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };

        self.value_replacements.push((find_string, replace_string));
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

    /// Handle key collisions by collecting values into arrays
    ///
    /// When enabled, collect all values that would have the same key into an array.
    /// Works for all operations (flatten, unflatten, normal).
    pub fn handle_key_collision(mut self, value: bool) -> Self {
        self.handle_key_collision = value;
        self
    }

    /// Enable automatic type conversion from strings to numbers and booleans
    ///
    /// When enabled, the library will attempt to convert string values to numbers or booleans:
    /// - **Numbers**: "123" -> 123, "1,234.56" -> 1234.56, "$99.99" -> 99.99, "1e5" -> 100000
    /// - **Booleans**: "true"/"TRUE"/"True" -> true, "false"/"FALSE"/"False" -> false
    ///
    /// If conversion fails, the original string value is kept. No errors are thrown.
    ///
    /// Works for all operations (flatten, unflatten, normal).
    ///
    /// # Example
    /// ```
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// let json = r#"{"id": "123", "price": "1,234.56", "active": "true"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .auto_convert_types(true)
    ///     .execute(json)
    ///     .unwrap();
    ///
    /// match result {
    ///     JsonOutput::Single(output) => {
    ///         // Result: {"id": 123, "price": 1234.56, "active": true}
    ///         assert!(output.contains(r#""id":123"#));
    ///         assert!(output.contains(r#""price":1234.56"#));
    ///         assert!(output.contains(r#""active":true"#));
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn auto_convert_types(mut self, enable: bool) -> Self {
        self.auto_convert_types = enable;
        self
    }

    /// Set the minimum batch size for parallel processing (only available with 'parallel' feature)
    ///
    /// When processing multiple JSON documents, this threshold determines when to use
    /// parallel processing. Batches smaller than this threshold will be processed sequentially
    /// to avoid the overhead of thread spawning.
    ///
    /// Default: 10 items (can be overridden with JSON_TOOLS_PARALLEL_THRESHOLD environment variable)
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum number of items in a batch to trigger parallel processing
    ///
    /// # Example
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .parallel_threshold(50); // Only use parallelism for batches of 50+ items
    /// ```
    pub fn parallel_threshold(mut self, threshold: usize) -> Self {
        self.parallel_threshold = threshold;
        self
    }

    /// Configure the number of threads for parallel processing
    ///
    /// By default, the number of logical CPUs is used. This method allows you to override
    /// that behavior for specific workloads or resource constraints.
    ///
    /// # Arguments
    ///
    /// * `num_threads` - Number of threads to use (None = use system default)
    ///
    /// # Examples
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .num_threads(Some(4)); // Use exactly 4 threads
    /// ```
    pub fn num_threads(mut self, num_threads: Option<usize>) -> Self {
        self.num_threads = num_threads;
        self
    }

    /// Configure the threshold for nested parallel processing within individual JSON documents
    ///
    /// When flattening or unflattening a single large JSON document, this threshold determines
    /// when to parallelize the processing of objects and arrays. Only objects/arrays with more
    /// than this many keys/items will be processed in parallel.
    ///
    /// Default: 100 (can be overridden with JSON_TOOLS_NESTED_PARALLEL_THRESHOLD environment variable)
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum number of keys/items to trigger nested parallelism
    ///
    /// # Examples
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .nested_parallel_threshold(200); // Only parallelize objects/arrays with 200+ items
    /// ```
    pub fn nested_parallel_threshold(mut self, threshold: usize) -> Self {
        self.nested_parallel_threshold = threshold;
        self
    }

    /// Execute the configured operation on the provided JSON input
    ///
    /// This method performs the selected operation based on the mode set by calling
    /// `.flatten()`, `.unflatten()`, or `.normal()`. If no mode was set, an error is returned.
    ///
    /// # Arguments
    /// * `json_input` - JSON input that can be a single string, multiple strings, or other supported types
    ///
    /// # Returns
    /// * `Result<JsonOutput, Box<dyn Error>>` - The processed JSON result or an error
    ///
    /// # Errors
    /// * Returns an error if no operation mode has been set
    /// * Returns an error if the JSON input is invalid
    /// * Returns an error if processing fails for any other reason
    pub fn execute<'a, T>(&self, json_input: T) -> Result<JsonOutput, JsonToolsError>
    where
        T: Into<JsonInput<'a>>,
    {
        let mode = self.mode.as_ref().ok_or_else(|| {
            JsonToolsError::configuration_error(
                "Operation mode not set. Call .flatten(), .unflatten(), or .normal() before .execute()"
            )
        })?;

        let input = json_input.into();
        match mode {
            OperationMode::Flatten => self.execute_flatten(input),
            OperationMode::Unflatten => self.execute_unflatten(input),
            OperationMode::Normal => self.execute_normal(input),
        }
    }

    /// Generic batch processing helper that eliminates code duplication
    /// Processes single or multiple JSON inputs using the provided processor function
    #[inline]
    fn execute_with_processor<'a, F>(
        input: JsonInput<'a>,
        config: &ProcessingConfig,
        processor: F,
    ) -> Result<JsonOutput, JsonToolsError>
    where
        F: Fn(&str, &ProcessingConfig) -> Result<String, JsonToolsError> + Sync + Send,
    {
        match input {
            JsonInput::Single(json_cow) => {
                let result = processor(json_cow.as_ref(), config)?;
                Ok(JsonOutput::Single(result))
            }
            JsonInput::Multiple(json_list) => {
                // Use parallel processing if batch size meets threshold
                if json_list.len() >= config.parallel_threshold {
                    let n_threads = std::thread::available_parallelism()
                        .map(|p| p.get())
                        .unwrap_or(4)
                        .min(json_list.len());
                    let chunk_size = json_list.len().div_ceil(n_threads);

                    // Pre-allocate result slots; each thread writes to its own non-overlapping slice,
                    // preserving input order without sorting or channels.
                    let mut slots: Vec<Option<Result<String, JsonToolsError>>> =
                        (0..json_list.len()).map(|_| None).collect();

                    // &F is Copy (shared ref), so each closure iteration captures its own ptr
                    let proc = &processor;
                    crossbeam::thread::scope(|s| {
                        for (chunk_idx, (inputs, outputs)) in json_list
                            .chunks(chunk_size)
                            .zip(slots.chunks_mut(chunk_size))
                            .enumerate()
                        {
                            let base = chunk_idx * chunk_size;
                            s.spawn(move |_| {
                                for (i, (json, slot)) in
                                    inputs.iter().zip(outputs.iter_mut()).enumerate()
                                {
                                    *slot = Some(proc(json, config).map_err(|e| {
                                        JsonToolsError::batch_processing_error(base + i, e)
                                    }));
                                }
                            });
                        }
                    })
                    .expect("batch processing thread panicked");

                    let results: Result<Vec<_>, _> =
                        slots.into_iter().map(|s| s.unwrap()).collect();
                    return Ok(JsonOutput::Multiple(results?));
                }

                // Sequential processing (default or below threshold)
                let mut results = Vec::with_capacity(json_list.len());
                for (index, json) in json_list.iter().enumerate() {
                    match processor(json, config) {
                        Ok(result) => results.push(result),
                        Err(e) => return Err(JsonToolsError::batch_processing_error(index, e)),
                    }
                }
                Ok(JsonOutput::Multiple(results))
            }
            JsonInput::MultipleOwned(vecs) => {
                // Use parallel processing if batch size meets threshold
                if vecs.len() >= config.parallel_threshold {
                    let n_threads = std::thread::available_parallelism()
                        .map(|p| p.get())
                        .unwrap_or(4)
                        .min(vecs.len());
                    let chunk_size = vecs.len().div_ceil(n_threads);

                    let mut slots: Vec<Option<Result<String, JsonToolsError>>> =
                        (0..vecs.len()).map(|_| None).collect();

                    let proc = &processor;
                    crossbeam::thread::scope(|s| {
                        for (chunk_idx, (inputs, outputs)) in vecs
                            .chunks(chunk_size)
                            .zip(slots.chunks_mut(chunk_size))
                            .enumerate()
                        {
                            let base = chunk_idx * chunk_size;
                            s.spawn(move |_| {
                                for (i, (json_cow, slot)) in
                                    inputs.iter().zip(outputs.iter_mut()).enumerate()
                                {
                                    *slot = Some(proc(json_cow.as_ref(), config).map_err(|e| {
                                        JsonToolsError::batch_processing_error(base + i, e)
                                    }));
                                }
                            });
                        }
                    })
                    .expect("batch processing thread panicked");

                    let results: Result<Vec<_>, _> =
                        slots.into_iter().map(|s| s.unwrap()).collect();
                    return Ok(JsonOutput::Multiple(results?));
                }

                // Sequential processing (default or below threshold)
                let mut results = Vec::with_capacity(vecs.len());
                for (index, json_cow) in vecs.iter().enumerate() {
                    match processor(json_cow.as_ref(), config) {
                        Ok(result) => results.push(result),
                        Err(e) => return Err(JsonToolsError::batch_processing_error(index, e)),
                    }
                }
                Ok(JsonOutput::Multiple(results))
            }
        }
    }

    fn execute_flatten<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json)
    }

    fn execute_unflatten<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json_for_unflatten)
    }

    fn execute_normal<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json_normal)
    }
}
