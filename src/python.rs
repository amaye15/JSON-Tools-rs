//! Python bindings for JSON Tools RS
//! 
//! This module provides Python bindings for the JSON flattening functionality
//! using PyO3. It exposes the `flatten_json` function and related types to Python.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::exceptions::{PyValueError, PyRuntimeError};
#[cfg(feature = "python")]
use pyo3::types::PyModule;

#[cfg(feature = "python")]
use crate::{flatten_json as rust_flatten_json, JsonOutput, FlattenError};

/// Python exception for JSON flattening errors
#[cfg(feature = "python")]
pyo3::create_exception!(json_tools_rs, JsonFlattenError, pyo3::exceptions::PyException);

/// Convert Rust FlattenError to Python exception
#[cfg(feature = "python")]
fn flatten_error_to_py_err(err: &FlattenError) -> PyErr {
    match err {
        FlattenError::JsonParseError(e) => PyValueError::new_err(format!("JSON parse error: {}", e)),
        FlattenError::RegexError(e) => PyValueError::new_err(format!("Regex error: {}", e)),
        FlattenError::InvalidReplacementPattern(msg) => PyValueError::new_err(format!("Invalid replacement pattern: {}", msg)),
        FlattenError::BatchError { index, error } => PyRuntimeError::new_err(format!("Error processing JSON at index {}: {}", index, error)),
    }
}

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
            JsonOutput::Multiple(_) => Err(PyValueError::new_err("Result contains multiple JSON strings, use get_multiple() instead")),
        }
    }
    
    /// Get the multiple results (raises ValueError if single)
    fn get_multiple(&self) -> PyResult<Vec<String>> {
        match &self.inner {
            JsonOutput::Single(_) => Err(PyValueError::new_err("Result contains single JSON string, use get_single() instead")),
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

/// Flatten JSON with comprehensive options
///
/// Args:
///     json_input: JSON string or list of JSON strings to flatten
///     remove_empty_string_values: Remove keys with empty string values
///     remove_null_values: Remove keys with null values
///     remove_empty_dict: Remove keys with empty object values
///     remove_empty_list: Remove keys with empty array values
///     key_replacements: List of (old, new) tuples for key replacements
///     value_replacements: List of (old, new) tuples for value replacements
///     separator: Custom separator for flattened keys (default: ".")
///     lower_case_keys: Convert all keys to lowercase (default: False)
///
/// Returns:
///     JsonOutput: Object containing flattened JSON result(s)
///
/// Raises:
///     JsonFlattenError: If JSON parsing or processing fails
///     ValueError: If arguments are invalid
///
/// Examples:
///     >>> result = flatten_json('{"a": {"b": 1}}')
///     >>> result.get_single()
///     '{"a.b": 1}'
///
///     >>> result = flatten_json('{"User": {"Name": "John"}}', lower_case_keys=True)
///     >>> result.get_single()
///     '{"user.name": "John"}'
///
///     >>> result = flatten_json(['{"a": 1}', '{"b": 2}'])
///     >>> result.get_multiple()
///     ['{"a": 1}', '{"b": 2}']
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (
    json_input,
    remove_empty_string_values = false,
    remove_null_values = false,
    remove_empty_dict = false,
    remove_empty_list = false,
    key_replacements = None,
    value_replacements = None,
    separator = None,
    lower_case_keys = false
))]
pub fn flatten_json(
    json_input: &PyAny,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<Vec<(String, String)>>,
    value_replacements: Option<Vec<(String, String)>>,
    separator: Option<&str>,
    lower_case_keys: bool,
) -> PyResult<PyJsonOutput> {
    // Handle different input types
    if let Ok(json_str) = json_input.extract::<String>() {
        // Single JSON string
        let result = rust_flatten_json(
            json_str.as_str(),
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
            key_replacements,
            value_replacements,
            separator,
            lower_case_keys,
        ).map_err(|e| {
            if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
                flatten_error_to_py_err(flatten_err)
            } else if let Some(json_err) = e.downcast_ref::<simd_json::Error>() {
                PyValueError::new_err(format!("JSON parse error: {}", json_err))
            } else {
                PyRuntimeError::new_err(format!("Unknown error: {}", e))
            }
        })?;
        
        Ok(PyJsonOutput::from(result))
    } else if let Ok(json_list) = json_input.extract::<Vec<String>>() {
        // Multiple JSON strings
        let json_refs: Vec<&str> = json_list.iter().map(|s| s.as_str()).collect();
        let result = rust_flatten_json(
            &json_refs[..],
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
            key_replacements,
            value_replacements,
            separator,
            lower_case_keys,
        ).map_err(|e| {
            if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
                flatten_error_to_py_err(flatten_err)
            } else if let Some(json_err) = e.downcast_ref::<simd_json::Error>() {
                PyValueError::new_err(format!("JSON parse error: {}", json_err))
            } else {
                PyRuntimeError::new_err(format!("Unknown error: {}", e))
            }
        })?;
        
        Ok(PyJsonOutput::from(result))
    } else {
        Err(PyValueError::new_err("json_input must be a string or list of strings"))
    }
}

/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn json_tools_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(flatten_json, m)?)?;
    m.add_class::<PyJsonOutput>()?;
    m.add("JsonFlattenError", py.get_type::<JsonFlattenError>())?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "JSON Tools RS Contributors")?;
    m.add("__description__", "Python bindings for JSON Tools RS - Advanced JSON manipulation library")?;

    Ok(())
}
