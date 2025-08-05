//! Python bindings for JSON Tools RS
//!
//! This module provides Python bindings for the unified JSONTools API
//! using PyO3. It exposes the complete JSONTools builder pattern API to Python
//! with support for both flattening and unflattening operations, collision handling,
//! and all advanced features.

#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;

#[cfg(feature = "python")]
use crate::{JSONTools, JsonOutput};

#[cfg(feature = "python")]
pyo3::create_exception!(
    json_tools_rs,
    JsonToolsError,
    pyo3::exceptions::PyException,
    "Python exception for JSON Tools operations"
);

/// Python wrapper for JsonOutput enum
#[cfg(feature = "python")]
#[pyclass(name = "JsonOutput")]
#[derive(Debug, Clone)]
pub struct PyJsonOutput {
    inner: JsonOutput,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyJsonOutput {
    /// Check if this is a single result
    #[getter]
    fn is_single(&self) -> bool {
        matches!(self.inner, JsonOutput::Single(_))
    }

    /// Check if this is a multiple result
    #[getter]
    fn is_multiple(&self) -> bool {
        matches!(self.inner, JsonOutput::Multiple(_))
    }

    /// Get the single result (raises ValueError if multiple)
    fn get_single(&self) -> PyResult<String> {
        match &self.inner {
            JsonOutput::Single(result) => Ok(result.clone()),
            JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                "Result contains multiple JSON strings, use get_multiple() instead",
            )),
        }
    }

    /// Get the multiple results (raises ValueError if single)
    fn get_multiple(&self) -> PyResult<Vec<String>> {
        match &self.inner {
            JsonOutput::Single(_) => Err(PyValueError::new_err(
                "Result contains single JSON string, use get_single() instead",
            )),
            JsonOutput::Multiple(results) => Ok(results.clone()),
        }
    }

    /// Get the result as a Python object (string for single, list for multiple)
    fn to_python(&self, py: Python) -> PyResult<PyObject> {
        match &self.inner {
            JsonOutput::Single(result) => Ok(result.into_pyobject(py)?.into_any().unbind()),
            JsonOutput::Multiple(results) => Ok(results.into_pyobject(py)?.into_any().unbind()),
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            JsonOutput::Single(result) => format!("JsonOutput.Single('{}')", result),
            JsonOutput::Multiple(results) => format!("JsonOutput.Multiple({:?})", results),
        }
    }

    fn __str__(&self) -> String {
        match &self.inner {
            JsonOutput::Single(result) => result.clone(),
            JsonOutput::Multiple(results) => format!("{:?}", results),
        }
    }
}

#[cfg(feature = "python")]
impl From<JsonOutput> for PyJsonOutput {
    fn from(output: JsonOutput) -> Self {
        PyJsonOutput { inner: output }
    }
}

#[cfg(feature = "python")]
impl PyJsonOutput {
    /// Helper method to create PyJsonOutput from Rust JsonOutput
    pub fn from_rust_output(output: JsonOutput) -> Self {
        PyJsonOutput { inner: output }
    }
}

/// Python JSONTools class - the unified API for JSON manipulation
///
/// This is the single entry point for all JSON operations in Python, providing both
/// flattening and unflattening capabilities with advanced features like collision handling,
/// filtering, and comprehensive transformations. It mirrors the Rust JSONTools API exactly.
///
/// # Input/Output Type Mapping
/// - str input → str output (JSON string)
/// - dict input → dict output (Python dictionary)
/// - list[str] input → list[str] output (list of JSON strings)
/// - list[dict] input → list[dict] output (list of Python dictionaries)
/// - Mixed list preserves original types in output
///
/// # Examples
///
/// ```python
/// import json_tools_rs
///
/// # Basic flattening
/// result = json_tools_rs.JSONTools().flatten().execute('{"a": {"b": 1}}')
/// print(result)  # '{"a.b": 1}' (string)
///
/// # Basic unflattening
/// result = json_tools_rs.JSONTools().unflatten().execute('{"a.b": 1}')
/// print(result)  # '{"a": {"b": 1}}' (string)
///
/// # Advanced configuration with collision handling
/// tools = (json_tools_rs.JSONTools()
///     .flatten()
///     .separator("::")
///     .remove_empty_strings(True)
///     .remove_nulls(True)
///     .lowercase_keys(True)
///     .key_replacement("regex:(User|Admin|Guest)_", "")
///     .handle_key_collision(True))
///
/// result = tools.execute({"User_name": "John", "Admin_name": "", "Guest_name": "Bob"})
/// print(result)  # {"name": ["John", "Bob"]} (dict, empty string filtered out)
///
/// # Collision avoidance strategy
/// tools = (json_tools_rs.JSONTools()
///     .flatten()
///     .key_replacement("regex:(User|Admin|Guest)_", "")
///     .avoid_key_collision(True))
///
/// result = tools.execute({"User_name": "John", "Admin_name": "Jane"})
/// print(result)  # {"name.0": "John", "name.1": "Jane"} (dict)
///
/// # Batch processing with type preservation
/// str_list = ['{"a": 1}', '{"b": 2}']
/// results = json_tools_rs.JSONTools().flatten().execute(str_list)
/// print(results)  # ['{"a": 1}', '{"b": 2}'] (list of strings)
/// ```
#[cfg(feature = "python")]
#[pyclass(name = "JSONTools")]
pub struct PyJSONTools {
    inner: JSONTools,
}

#[cfg(feature = "python")]
impl Default for PyJSONTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyJSONTools {
    /// Create a new JSONTools instance with default settings
    #[new]
    pub fn new() -> Self {
        Self {
            inner: JSONTools::new(),
        }
    }

    /// Configure for flattening operations
    pub fn flatten(slf: PyRef<'_, Self>) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().flatten(),
        }
    }

    /// Configure for unflattening operations
    pub fn unflatten(slf: PyRef<'_, Self>) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().unflatten(),
        }
    }

    /// Set the separator for nested keys (default: ".")
    pub fn separator(slf: PyRef<'_, Self>, separator: String) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().separator(separator),
        }
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().lowercase_keys(value),
        }
    }

    /// Remove keys with empty string values
    pub fn remove_empty_strings(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().remove_empty_strings(value),
        }
    }

    /// Remove keys with null values
    pub fn remove_nulls(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().remove_nulls(value),
        }
    }

    /// Remove keys with empty object values
    pub fn remove_empty_objects(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().remove_empty_objects(value),
        }
    }

    /// Remove keys with empty array values
    pub fn remove_empty_arrays(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().remove_empty_arrays(value),
        }
    }

    /// Add a key replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn key_replacement(slf: PyRef<'_, Self>, find: String, replace: String) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().key_replacement(find, replace),
        }
    }

    /// Add a value replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn value_replacement(
        slf: PyRef<'_, Self>,
        find: String,
        replace: String,
    ) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().value_replacement(find, replace),
        }
    }

    /// Enable collision avoidance strategy
    ///
    /// When key transformations result in duplicate keys, this strategy appends
    /// index suffixes to make keys unique (e.g., "name.0", "name.1", "name.2").
    ///
    /// # Arguments
    /// * `value` - Whether to enable collision avoidance
    pub fn avoid_key_collision(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().avoid_key_collision(value),
        }
    }

    /// Enable collision handling strategy
    ///
    /// When key transformations result in duplicate keys, this strategy collects
    /// all values into arrays (e.g., "name": ["John", "Jane", "Bob"]).
    /// Filtering is applied during collision resolution.
    ///
    /// # Arguments
    /// * `value` - Whether to enable collision handling
    pub fn handle_key_collision(slf: PyRef<'_, Self>, value: bool) -> PyJSONTools {
        PyJSONTools {
            inner: slf.inner.clone().handle_key_collision(value),
        }
    }

    /// Execute the configured JSON operation
    ///
    /// This method executes the configured operation (flatten or unflatten) with all
    /// the specified transformations, collision handling, and filtering options.
    ///
    /// # Arguments
    /// * `json_input` - JSON input as:
    ///   - str: JSON string
    ///   - dict: Python dictionary (will be serialized to JSON)
    ///   - list[str]: List of JSON strings
    ///   - list[dict]: List of Python dictionaries (will be serialized to JSON)
    ///
    /// # Returns
    /// * str input → str output (processed JSON string)
    /// * dict input → dict output (processed Python dictionary)
    /// * list[str] input → list[str] output (list of processed JSON strings)
    /// * list[dict] input → list[dict] output (list of processed Python dictionaries)
    pub fn execute(&self, json_input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = json_input.py();
        let json_module = py.import("json")?;

        // Try to handle as single input first
        if let Ok(json_str) = json_input.extract::<String>() {
            // Single JSON string → return JSON string
            let result = self.inner.clone().execute(json_str.as_str()).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to process JSON string: {}", e))
            })?;

            match result {
                JsonOutput::Single(processed) => Ok(processed.into_pyobject(py)?.into_any().unbind()),
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single JSON input",
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyDict>() {
            // Single Python dictionary → return Python dictionary
            let json_str: String = json_module
                .getattr("dumps")?
                .call1((json_input,))?
                .extract()?;

            let result = self.inner.clone().execute(json_str.as_str()).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to process Python dict: {}", e))
            })?;

            match result {
                JsonOutput::Single(processed) => {
                    // Parse the processed JSON string back to a Python dict
                    let processed_dict = json_module.getattr("loads")?.call1((processed,))?;
                    Ok(processed_dict.unbind())
                }
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single dict input",
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyList>() {
            // Handle list input - could be batch processing or single JSON array
            let list = json_input.downcast::<pyo3::types::PyList>()?;

            // Check if this is valid batch processing (only JSON strings and/or dicts)
            // Empty lists are treated as empty batch
            if list.is_empty() {
                // Empty list - return empty batch result
                return Ok(Vec::<String>::new().into_pyobject(py)?.into_any().unbind());
            }

            let mut has_json_strings = false;
            let mut has_dicts = false;
            let mut has_other_types = false;

            for item in list.iter() {
                if item.extract::<String>().is_ok() {
                    has_json_strings = true;
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    has_dicts = true;
                } else {
                    has_other_types = true;
                }
            }

            // Only allow batch processing if all items are JSON strings or dicts
            if (has_json_strings || has_dicts) && !has_other_types {
                // This is batch processing - list of JSON strings or list of dicts
                let mut json_strings = Vec::new();
                let mut input_types = Vec::new();

                for item in list.iter() {
                    if let Ok(json_str) = item.extract::<String>() {
                        // Item is a JSON string
                        json_strings.push(json_str);
                        input_types.push("str");
                    } else if item.is_instance_of::<pyo3::types::PyDict>() {
                        // Item is a Python dictionary - serialize to JSON
                        let json_str: String =
                            json_module.getattr("dumps")?.call1((item,))?.extract()?;
                        json_strings.push(json_str);
                        input_types.push("dict");
                    }
                }

                // Process the list of JSON strings (batch processing)
                let json_refs: Vec<&str> = json_strings.iter().map(|s| s.as_str()).collect();
                let result = self.inner.clone().execute(&json_refs[..]).map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON list: {}", e))
                })?;

                match result {
                    JsonOutput::Single(_) => Err(PyValueError::new_err(
                        "Unexpected single result for multiple input",
                    )),
                    JsonOutput::Multiple(processed_list) => {
                        // Determine output type based on input types
                        let all_strings = input_types.iter().all(|&t| t == "str");
                        let all_dicts = input_types.iter().all(|&t| t == "dict");

                        if all_strings {
                            // list[str] input → list[str] output
                            Ok(processed_list.into_pyobject(py)?.into_any().unbind())
                        } else if all_dicts {
                            // list[dict] input → list[dict] output
                            let mut dict_results = Vec::new();
                            for processed_json in processed_list {
                                let dict_result =
                                    json_module.getattr("loads")?.call1((processed_json,))?;
                                dict_results.push(dict_result);
                            }
                            Ok(dict_results.into_pyobject(py)?.into_any().unbind())
                        } else {
                            // Mixed input → preserve original types
                            let mut mixed_results = Vec::new();
                            for (processed_json, &input_type) in
                                processed_list.iter().zip(input_types.iter())
                            {
                                if input_type == "str" {
                                    mixed_results.push(processed_json.into_pyobject(py)?.into_any().unbind());
                                } else {
                                    let dict_result =
                                        json_module.getattr("loads")?.call1((processed_json,))?;
                                    mixed_results.push(dict_result.unbind());
                                }
                            }
                            Ok(mixed_results.into_pyobject(py)?.into_any().unbind())
                        }
                    }
                }
            } else {
                // Invalid list - contains types other than JSON strings or dicts
                Err(PyValueError::new_err(
                    "List items must be either JSON strings or Python dictionaries",
                ))
            }
        } else {
            Err(PyValueError::new_err(
                "json_input must be a JSON string, Python dict, list of JSON strings, or list of Python dicts"
            ))
        }
    }

    /// Execute the configured operation and return a JsonOutput object
    ///
    /// This method returns the full JsonOutput object for advanced use cases
    /// where you need to check the result type or handle both single and multiple
    /// results in a unified way.
    ///
    /// # Arguments
    /// * `json_input` - JSON input as:
    ///   - str: JSON string
    ///   - dict: Python dictionary (will be serialized to JSON)
    ///   - list[str]: List of JSON strings
    ///   - list[dict]: List of Python dictionaries (will be serialized to JSON)
    ///
    /// # Returns
    /// * `PyJsonOutput` - JsonOutput object with is_single/is_multiple methods
    pub fn execute_to_output(&self, json_input: &Bound<'_, PyAny>) -> PyResult<PyJsonOutput> {
        let py = json_input.py();

        // Try to handle as single input first
        if let Ok(json_str) = json_input.extract::<String>() {
            // Single JSON string
            let result = self.inner.clone().execute(json_str.as_str()).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to process JSON string: {}", e))
            })?;
            Ok(PyJsonOutput::from_rust_output(result))
        } else if json_input.is_instance_of::<pyo3::types::PyDict>() {
            // Single Python dictionary - serialize to JSON first
            let json_module = py.import("json")?;
            let json_str: String = json_module
                .getattr("dumps")?
                .call1((json_input,))?
                .extract()?;

            let result = self.inner.clone().execute(json_str.as_str()).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to process Python dict: {}", e))
            })?;
            Ok(PyJsonOutput::from_rust_output(result))
        } else if json_input.is_instance_of::<pyo3::types::PyList>() {
            // Handle list input - could be batch processing or single JSON array
            let list = json_input.downcast::<pyo3::types::PyList>()?;

            // Check if this is valid batch processing (only JSON strings and/or dicts)
            // Empty lists are treated as empty batch
            if list.is_empty() {
                // Empty list - return empty batch result
                return Ok(PyJsonOutput::from_rust_output(JsonOutput::Multiple(vec![])));
            }

            let mut has_json_strings = false;
            let mut has_dicts = false;
            let mut has_other_types = false;

            for item in list.iter() {
                if item.extract::<String>().is_ok() {
                    has_json_strings = true;
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    has_dicts = true;
                } else {
                    has_other_types = true;
                }
            }

            // Only allow batch processing if all items are JSON strings or dicts
            if (has_json_strings || has_dicts) && !has_other_types {
                // This is batch processing
                let mut json_strings = Vec::new();

                for item in list.iter() {
                    if let Ok(json_str) = item.extract::<String>() {
                        // Item is a JSON string
                        json_strings.push(json_str);
                    } else if item.is_instance_of::<pyo3::types::PyDict>() {
                        // Item is a Python dictionary - serialize to JSON
                        let json_module = py.import("json")?;
                        let json_str: String =
                            json_module.getattr("dumps")?.call1((item,))?.extract()?;
                        json_strings.push(json_str);
                    }
                }

                // Process the list of JSON strings
                let json_refs: Vec<&str> = json_strings.iter().map(|s| s.as_str()).collect();
                let result = self.inner.clone().execute(&json_refs[..]).map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON list: {}", e))
                })?;
                Ok(PyJsonOutput::from_rust_output(result))
            } else {
                // Invalid list - contains types other than JSON strings or dicts
                Err(PyValueError::new_err(
                    "List items must be either JSON strings or Python dictionaries",
                ))
            }
        } else {
            Err(PyValueError::new_err(
                "json_input must be a JSON string, Python dict, list of JSON strings, or list of Python dicts"
            ))
        }
    }
}










/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn json_tools_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the unified JSONTools class
    m.add_class::<PyJSONTools>()?;

    // Add the JsonOutput class for results
    m.add_class::<PyJsonOutput>()?;

    // Add the custom exception
    m.add(
        "JsonToolsError",
        m.py().get_type::<JsonToolsError>(),
    )?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "JSON Tools RS Contributors")?;
    m.add(
        "__description__",
        "Python bindings for JSON Tools RS - Unified JSON manipulation library with advanced collision handling and filtering",
    )?;

    Ok(())
}
