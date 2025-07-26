//! Python bindings for JSON Tools RS
//!
//! This module provides Python bindings for the JSON flattening functionality
//! using PyO3. It exposes the `JsonFlattener` builder pattern API to Python.

#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;

#[cfg(feature = "python")]
use crate::{JsonFlattener, JsonOutput};

/// Python exception for JSON flattening errors
#[cfg(feature = "python")]
pyo3::create_exception!(
    json_tools_rs,
    JsonFlattenError,
    pyo3::exceptions::PyException
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
            JsonOutput::Single(result) => Ok(result.to_object(py)),
            JsonOutput::Multiple(results) => Ok(results.to_object(py)),
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



/// Python JsonFlattener class - the unified API for JSON flattening
///
/// This is the single entry point for all JSON flattening operations in Python.
/// It provides a fluent builder pattern API that matches the Rust implementation.
/// The flatten() method accepts various Python types and returns matching output types.
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
/// # JSON string input → JSON string output
/// result = json_tools_rs.JsonFlattener().flatten('{"a": {"b": 1}}')
/// print(result)  # '{"a.b": 1}' (string)
/// print(type(result))  # <class 'str'>
///
/// # Python dict input → Python dict output (more convenient!)
/// result = json_tools_rs.JsonFlattener().flatten({"a": {"b": 1}})
/// print(result)  # {'a.b': 1} (dict)
/// print(type(result))  # <class 'dict'>
///
/// # Advanced configuration with builder pattern
/// flattener = (json_tools_rs.JsonFlattener()
///     .remove_empty_strings(True)
///     .remove_nulls(True)
///     .separator("_")
///     .lowercase_keys(True)
///     .key_replacement("regex:^user_", "")
///     .value_replacement("@example.com", "@company.org"))
///
/// result = flattener.flatten({"user_name": "john@example.com"})
/// print(result)  # {"name": "john@company.org"} (dict)
///
/// # Batch processing with type preservation
/// str_list = ['{"a": 1}', '{"b": 2}']
/// results = json_tools_rs.JsonFlattener().flatten(str_list)
/// print(results)  # ['{"a": 1}', '{"b": 2}'] (list of strings)
///
/// dict_list = [{"a": 1}, {"b": 2}]
/// results = json_tools_rs.JsonFlattener().flatten(dict_list)
/// print(results)  # [{'a': 1}, {'b': 2}] (list of dicts)
/// ```
#[cfg(feature = "python")]
#[pyclass(name = "JsonFlattener")]
pub struct PyJsonFlattener {
    inner: JsonFlattener,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyJsonFlattener {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: JsonFlattener::new(),
        }
    }

    /// Remove keys with empty string values
    pub fn remove_empty_strings(slf: PyRef<'_, Self>, value: bool) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().remove_empty_strings(value),
        }
    }

    /// Remove keys with null values
    pub fn remove_nulls(slf: PyRef<'_, Self>, value: bool) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().remove_nulls(value),
        }
    }

    /// Remove keys with empty object values
    pub fn remove_empty_objects(slf: PyRef<'_, Self>, value: bool) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().remove_empty_objects(value),
        }
    }

    /// Remove keys with empty array values
    pub fn remove_empty_arrays(slf: PyRef<'_, Self>, value: bool) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().remove_empty_arrays(value),
        }
    }

    /// Add a key replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn key_replacement(slf: PyRef<'_, Self>, find: String, replace: String) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().key_replacement(find, replace),
        }
    }

    /// Add a value replacement pattern
    ///
    /// # Arguments
    /// * `find` - Pattern to find (use "regex:" prefix for regex patterns)
    /// * `replace` - Replacement string
    pub fn value_replacement(slf: PyRef<'_, Self>, find: String, replace: String) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().value_replacement(find, replace),
        }
    }

    /// Set the separator for nested keys
    pub fn separator(slf: PyRef<'_, Self>, separator: String) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().separator(separator),
        }
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(slf: PyRef<'_, Self>, value: bool) -> PyJsonFlattener {
        PyJsonFlattener {
            inner: slf.inner.clone().lowercase_keys(value),
        }
    }

    /// Flatten the JSON input
    ///
    /// # Arguments
    /// * `json_input` - JSON input as:
    ///   - str: JSON string
    ///   - dict: Python dictionary (will be serialized to JSON)
    ///   - list[str]: List of JSON strings
    ///   - list[dict]: List of Python dictionaries (will be serialized to JSON)
    ///
    /// # Returns
    /// * str input → str output (flattened JSON string)
    /// * dict input → dict output (flattened Python dictionary)
    /// * list[str] input → list[str] output (list of flattened JSON strings)
    /// * list[dict] input → list[dict] output (list of flattened Python dictionaries)
    pub fn flatten(&self, json_input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        let py = json_input.py();
        let json_module = py.import_bound("json")?;

        // Try to handle as single input first
        if let Ok(json_str) = json_input.extract::<String>() {
            // Single JSON string → return JSON string
            let result = self.inner.clone().flatten(json_str.as_str())
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten JSON string: {}", e)))?;

            match result {
                JsonOutput::Single(flattened) => Ok(flattened.to_object(py)),
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single JSON input"
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyDict>() {
            // Single Python dictionary → return Python dictionary
            let json_str: String = json_module
                .getattr("dumps")?
                .call1((json_input,))?
                .extract()?;

            let result = self.inner.clone().flatten(json_str.as_str())
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten Python dict: {}", e)))?;

            match result {
                JsonOutput::Single(flattened) => {
                    // Parse the flattened JSON string back to a Python dict
                    let flattened_dict = json_module
                        .getattr("loads")?
                        .call1((flattened,))?;
                    Ok(flattened_dict.to_object(py))
                },
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single dict input"
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyList>() {
            // Handle list input - could be list[str] or list[dict]
            let list = json_input.downcast::<pyo3::types::PyList>()?;
            let mut json_strings = Vec::new();
            let mut input_types = Vec::new(); // Track whether each item was str or dict

            for item in list.iter() {
                if let Ok(json_str) = item.extract::<String>() {
                    // Item is a JSON string
                    json_strings.push(json_str);
                    input_types.push("str");
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    // Item is a Python dictionary - serialize to JSON
                    let json_str: String = json_module
                        .getattr("dumps")?
                        .call1((item,))?
                        .extract()?;
                    json_strings.push(json_str);
                    input_types.push("dict");
                } else {
                    return Err(PyValueError::new_err(
                        "List items must be either JSON strings or Python dictionaries"
                    ));
                }
            }

            // Process the list of JSON strings
            let json_refs: Vec<&str> = json_strings.iter().map(|s| s.as_str()).collect();
            let result = self.inner.clone().flatten(&json_refs[..])
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten JSON list: {}", e)))?;

            match result {
                JsonOutput::Single(_) => Err(PyValueError::new_err(
                    "Unexpected single result for multiple input"
                )),
                JsonOutput::Multiple(flattened_list) => {
                    // Determine output type based on input types
                    let all_strings = input_types.iter().all(|&t| t == "str");
                    let all_dicts = input_types.iter().all(|&t| t == "dict");

                    if all_strings {
                        // list[str] input → list[str] output
                        Ok(flattened_list.to_object(py))
                    } else if all_dicts {
                        // list[dict] input → list[dict] output
                        let mut dict_results = Vec::new();
                        for flattened_json in flattened_list {
                            let dict_result = json_module
                                .getattr("loads")?
                                .call1((flattened_json,))?;
                            dict_results.push(dict_result);
                        }
                        Ok(dict_results.to_object(py))
                    } else {
                        // Mixed input → preserve original types
                        let mut mixed_results = Vec::new();
                        for (flattened_json, &input_type) in flattened_list.iter().zip(input_types.iter()) {
                            if input_type == "str" {
                                mixed_results.push(flattened_json.to_object(py));
                            } else {
                                let dict_result = json_module
                                    .getattr("loads")?
                                    .call1((flattened_json,))?;
                                mixed_results.push(dict_result.to_object(py));
                            }
                        }
                        Ok(mixed_results.to_object(py))
                    }
                }
            }
        } else {
            Err(PyValueError::new_err(
                "json_input must be a JSON string, Python dict, list of JSON strings, or list of Python dicts"
            ))
        }
    }

    /// Flatten the JSON input and return a JsonOutput object
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
    pub fn flatten_to_output(&self, json_input: &Bound<'_, PyAny>) -> PyResult<PyJsonOutput> {
        let py = json_input.py();

        // Try to handle as single input first
        if let Ok(json_str) = json_input.extract::<String>() {
            // Single JSON string
            let result = self.inner.clone().flatten(json_str.as_str())
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten JSON string: {}", e)))?;
            Ok(PyJsonOutput::from_rust_output(result))
        } else if json_input.is_instance_of::<pyo3::types::PyDict>() {
            // Single Python dictionary - serialize to JSON first
            let json_module = py.import_bound("json")?;
            let json_str: String = json_module
                .getattr("dumps")?
                .call1((json_input,))?
                .extract()?;

            let result = self.inner.clone().flatten(json_str.as_str())
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten Python dict: {}", e)))?;
            Ok(PyJsonOutput::from_rust_output(result))
        } else if json_input.is_instance_of::<pyo3::types::PyList>() {
            // Handle list input - could be list[str] or list[dict]
            let list = json_input.downcast::<pyo3::types::PyList>()?;
            let mut json_strings = Vec::new();

            for item in list.iter() {
                if let Ok(json_str) = item.extract::<String>() {
                    // Item is a JSON string
                    json_strings.push(json_str);
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    // Item is a Python dictionary - serialize to JSON
                    let json_module = py.import_bound("json")?;
                    let json_str: String = json_module
                        .getattr("dumps")?
                        .call1((item,))?
                        .extract()?;
                    json_strings.push(json_str);
                } else {
                    return Err(PyValueError::new_err(
                        "List items must be either JSON strings or Python dictionaries"
                    ));
                }
            }

            // Process the list of JSON strings
            let json_refs: Vec<&str> = json_strings.iter().map(|s| s.as_str()).collect();
            let result = self.inner.clone().flatten(&json_refs[..])
                .map_err(|e| JsonFlattenError::new_err(format!("Failed to flatten JSON list: {}", e)))?;
            Ok(PyJsonOutput::from_rust_output(result))
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
    // Add the main JsonFlattener class
    m.add_class::<PyJsonFlattener>()?;

    // Add the JsonOutput class for results
    m.add_class::<PyJsonOutput>()?;

    // Add the custom exception
    m.add("JsonFlattenError", m.py().get_type_bound::<JsonFlattenError>())?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "JSON Tools RS Contributors")?;
    m.add(
        "__description__",
        "Python bindings for JSON Tools RS - Advanced JSON manipulation library with unified JsonFlattener API",
    )?;

    Ok(())
}
