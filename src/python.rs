//! Python bindings for JSON Tools RS
//!
//! This module provides Python bindings for the unified JSONTools API
//! using PyO3. It exposes the complete JSONTools builder pattern API to Python
//! with support for both flattening and unflattening operations, collision handling,
//! and all advanced features.

#[cfg(feature = "python")]
use pyo3::exceptions::{PyRuntimeError, PyValueError};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::sync::PyOnceLock;
#[cfg(feature = "python")]
use pyo3::types::{PyBytes, PyDict, PyList, PyModule, PyString};

#[cfg(feature = "python")]
use std::mem;
#[cfg(feature = "python")]
use std::sync::Mutex;

#[cfg(feature = "python")]
use indexmap::{IndexMap, IndexSet};

#[cfg(feature = "python")]
use crate::{JSONTools, JsonOutput};

#[cfg(feature = "python")]
pyo3::create_exception!(
    json_tools_rs,
    JsonToolsError,
    pyo3::exceptions::PyException,
    "Python exception for JSON Tools operations"
);

/// Lock the inner config mutex, converting poison errors to Python exceptions.
#[cfg(feature = "python")]
#[inline]
fn lock_config(mutex: &Mutex<JSONTools>) -> PyResult<std::sync::MutexGuard<'_, JSONTools>> {
    mutex
        .lock()
        .map_err(|e| PyRuntimeError::new_err(format!("internal config lock poisoned: {e}")))
}

/// JSON (de)serialization callables, resolved once per process and cached:
/// `orjson` (a hard dependency -- see `pyproject.toml`; 5-10x faster than
/// stdlib for the dict<->str conversion this module performs on every
/// dict-shaped call) with the stdlib `json` module kept alongside as a
/// per-call fallback for the inputs orjson can't handle (integers beyond
/// 64-bit range, arbitrary type subclasses, ...) -- retrying with stdlib on
/// an orjson error preserves stdlib's behavior and error surfaces for exactly
/// those inputs instead of changing them.
#[cfg(feature = "python")]
struct JsonCallables {
    /// `orjson.dumps`.
    dumps: Py<PyAny>,
    /// Kwargs for the dumps call: `{"option": orjson.OPT_NON_STR_KEYS}`, so
    /// int/float/bool/None dict keys are coerced to strings exactly like
    /// stdlib `json.dumps` does by default.
    dumps_kwargs: Py<PyDict>,
    /// stdlib `json.dumps` -- fallback when orjson rejects an input.
    stdlib_dumps: Py<PyAny>,
    /// `orjson.loads`.
    loads: Py<PyAny>,
    /// stdlib `json.loads` -- fallback: orjson rejects some JSON stdlib
    /// accepts (integers beyond 64-bit range).
    stdlib_loads: Py<PyAny>,
}

#[cfg(feature = "python")]
static JSON_CALLABLES: PyOnceLock<JsonCallables> = PyOnceLock::new();

/// Resolve (once) and return the cached JSON callables. Caching the bound
/// callables saves the per-call `sys.modules` lookup + `getattr` +
/// bound-method allocation that `py.import("json")?.call_method1(...)` costs.
#[cfg(feature = "python")]
fn json_callables(py: Python<'_>) -> PyResult<&'static JsonCallables> {
    JSON_CALLABLES.get_or_try_init(py, || {
        let json_mod = py.import("json")?;
        let stdlib_dumps = json_mod.getattr("dumps")?.unbind();
        let stdlib_loads = json_mod.getattr("loads")?.unbind();
        let orjson = py.import("orjson")?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("option", orjson.getattr("OPT_NON_STR_KEYS")?)?;
        Ok(JsonCallables {
            dumps: orjson.getattr("dumps")?.unbind(),
            dumps_kwargs: kwargs.unbind(),
            stdlib_dumps,
            loads: orjson.getattr("loads")?.unbind(),
            stdlib_loads,
        })
    })
}

/// Serialize a Python object to a JSON string using `orjson`, falling back to
/// Python's own (C-accelerated) `json` module for inputs orjson rejects --
/// never a Rust-side serde traversal.
///
/// Benchmarked against the previous `depythonize` + `sonic-rs` serialize
/// approach: for flat dicts the old approach was marginally faster (~25%),
/// but for the nested data this library's `.flatten()`/`.unflatten()` exist to
/// handle -- and for `list[dict]`/DataFrame batches, which are the realistic
/// call shape -- `depythonize` was 5-30% slower for a single nested dict, and
/// 2-3x slower for DataFrame rows, than just letting CPython's own hand-tuned
/// C `json` module do the conversion. orjson beats stdlib by a further 5-10x
/// on the same call shape.
#[cfg(feature = "python")]
#[inline]
fn py_dumps(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<String> {
    let callables = json_callables(py)?;
    let result = match callables
        .dumps
        .bind(py)
        .call((obj,), Some(callables.dumps_kwargs.bind(py)))
    {
        Ok(out) => out,
        // orjson rejects inputs stdlib handles (ints beyond 64-bit, exotic
        // subclasses, ...) -- retry with stdlib so those inputs keep working
        // exactly as before, including stdlib's error messages when the
        // input is genuinely unserializable.
        Err(_) => callables.stdlib_dumps.bind(py).call1((obj,))?,
    };
    // orjson returns `bytes`; stdlib returns `str`.
    if let Ok(bytes) = result.cast::<PyBytes>() {
        std::str::from_utf8(bytes.as_bytes())
            .map(str::to_owned)
            .map_err(|e| {
                PyValueError::new_err(format!("JSON serializer produced non-UTF-8 output: {e}"))
            })
    } else {
        result.extract::<String>()
    }
}

/// True if `s` contains a run of >= 19 consecutive ASCII digits -- the only
/// way an integer literal outside 64-bit range can appear (i64::MAX has 19
/// digits). Used to guard orjson's `loads`, which silently parses such
/// integers as lossy floats where stdlib preserves them exactly (Python ints
/// are arbitrary-precision). Deliberately conservative: long digit runs
/// inside strings or float literals also match and just take the stdlib path.
///
/// Samples every 19th byte instead of scanning all of them: any 19-digit run
/// must contain a sampled position (consecutive samples are <= 19 apart), and
/// a sampled digit expands to its full run to measure the real length, with
/// the next sample resuming past the run's known non-digit end.
#[cfg(feature = "python")]
#[inline]
fn may_contain_big_int(s: &[u8]) -> bool {
    const RUN: usize = 19;
    let n = s.len();
    let mut i = 0;
    while i < n {
        if s[i].is_ascii_digit() {
            let mut start = i;
            while start > 0 && s[start - 1].is_ascii_digit() {
                start -= 1;
            }
            let mut end = i + 1;
            while end < n && s[end].is_ascii_digit() {
                end += 1;
            }
            if end - start >= RUN {
                return true;
            }
            // `end` is a known non-digit (or out of bounds), so the next run
            // starts at end+1 at the earliest and sampling end+RUN leaves at
            // most RUN-1 unsampled positions before it -- too short to hide
            // a qualifying run.
            i = end + RUN;
        } else {
            i += RUN;
        }
    }
    false
}

/// Deserialize a JSON string into a Python object using `orjson`, falling
/// back to Python's own `json` module for inputs orjson rejects. See
/// `py_dumps` for the rationale -- this is the output-side half of the same
/// design.
///
/// Documents that may contain integers beyond 64-bit range always take the
/// stdlib path: orjson parses those as lossy floats with no error to hook a
/// fallback on, and this library guarantees integer precision end-to-end.
#[cfg(feature = "python")]
#[inline]
fn py_loads<'py>(py: Python<'py>, json_str: &str) -> PyResult<Bound<'py, PyAny>> {
    let callables = json_callables(py)?;
    if !may_contain_big_int(json_str.as_bytes()) {
        // On an unexpected orjson failure over our own valid output, fall
        // through to stdlib so its behavior/error surface is what the caller
        // sees, exactly as before the accelerator existed.
        if let Ok(obj) = callables.loads.bind(py).call1((json_str,)) {
            return Ok(obj);
        }
    }
    callables.stdlib_loads.bind(py).call1((json_str,))
}

// =============================================================================
// DataFrame and Series Support Types
// =============================================================================

/// Type of DataFrame library detected via duck-typing.
///
/// Detection uses `type(obj).__module__` and `type(obj).__name__` to identify
/// the DataFrame variant without importing the library, falling back to
/// checking for `to_dict()` or `to_json()` methods for generic compatibility.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataFrameType {
    /// pandas.DataFrame — uses `to_dict("records")` for row extraction
    Pandas,
    /// polars.DataFrame — uses `to_dicts()` for row extraction
    Polars,
    /// pyarrow.Table — uses `to_pydict()` for columnar extraction
    PyArrow,
    /// pyspark.sql.DataFrame — uses `toJSON().collect()` for distributed extraction
    PySpark,
    /// Any object with `to_dict()` or `to_json()` methods
    Generic,
}

/// Type of Series library detected via duck-typing.
///
/// Similar to `DataFrameType`, uses module/name introspection to identify
/// the Series variant, falling back to `to_list()` / `tolist()` for generics.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeriesType {
    /// pandas.Series — uses `tolist()` for value extraction
    Pandas,
    /// polars.Series — uses `to_list()` for value extraction
    Polars,
    /// pyarrow.Array or pyarrow.ChunkedArray — uses `to_pylist()`
    PyArrow,
    /// pyspark.sql.Column (rare) — uses `collect()` for distributed extraction
    PySpark,
    /// Any object with `to_list()` or `tolist()` methods
    Generic,
}

/// Unified data structure type for detection.
///
/// Wraps either a DataFrame or Series detection result so the processing
/// pipeline can handle both types uniformly before dispatching to
/// type-specific extraction logic.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataStructureType {
    /// A DataFrame-like object (tabular rows/columns)
    DataFrame(DataFrameType),
    /// A Series-like object (single column/array of values)
    Series(SeriesType),
}

/// Target DataFrame library for `PyJSONTools::execute`'s `normalise=True` path.
///
/// Distinct from `DataFrameType`/`SeriesType` above (which describe *detected*
/// input): this describes the *requested or resolved output* -- every processed
/// row becomes one row of the resulting table regardless of what shape the input
/// was (a bare `dict` produces a 1-row table just like a `DataFrame` produces an
/// N-row one). `Generic` has no equivalent here -- there's no single library to
/// reconstruct against for a duck-typed "has to_dict()" object.
#[cfg(feature = "python")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormaliseTarget {
    Pandas,
    Polars,
    PyArrow,
    PySpark,
}

#[cfg(feature = "python")]
impl NormaliseTarget {
    /// Parse a user-supplied `target=` string. Errors list the valid values
    /// rather than falling back silently -- an explicit target is a promise to
    /// the caller that exactly this backend will be used.
    fn parse(s: &str) -> PyResult<Self> {
        match s {
            "pandas" => Ok(Self::Pandas),
            "polars" => Ok(Self::Polars),
            "pyarrow" => Ok(Self::PyArrow),
            "pyspark" => Ok(Self::PySpark),
            other => Err(JsonToolsError::new_err(format!(
                "Unknown normalise target {other:?}; expected one of \"pandas\", \"polars\", \"pyarrow\", \"pyspark\""
            ))),
        }
    }

    /// Also the pip/import name for all four -- reused for both.
    fn name(self) -> &'static str {
        match self {
            Self::Pandas => "pandas",
            Self::Polars => "polars",
            Self::PyArrow => "pyarrow",
            Self::PySpark => "pyspark",
        }
    }

    fn from_dataframe_type(t: DataFrameType) -> Option<Self> {
        match t {
            DataFrameType::Pandas => Some(Self::Pandas),
            DataFrameType::Polars => Some(Self::Polars),
            DataFrameType::PyArrow => Some(Self::PyArrow),
            DataFrameType::PySpark => Some(Self::PySpark),
            DataFrameType::Generic => None,
        }
    }

    fn from_series_type(t: SeriesType) -> Option<Self> {
        match t {
            SeriesType::Pandas => Some(Self::Pandas),
            SeriesType::Polars => Some(Self::Polars),
            SeriesType::PyArrow => Some(Self::PyArrow),
            SeriesType::PySpark => Some(Self::PySpark),
            SeriesType::Generic => None,
        }
    }
}

/// Python wrapper for JsonOutput enum
#[cfg(feature = "python")]
#[pyclass(name = "JsonOutput", module = "json_tools_rs", frozen, from_py_object)]
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
    fn to_python(&self, py: Python) -> PyResult<Py<PyAny>> {
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
/// # Performance Optimization
/// Uses Mutex for interior mutability to avoid cloning the entire JSONTools struct
/// on every builder method call. This provides 30-50% performance improvement over
/// the previous clone-based approach while maintaining thread safety for Python's GIL.
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
///     .key_replacement("(User|Admin|Guest)_", "")
///     .handle_key_collision(True))
///
/// result = tools.execute({"User_name": "John", "Admin_name": "", "Guest_name": "Bob"})
/// print(result)  # {"name": ["John", "Bob"]} (dict, empty string filtered out)
///
///
/// # Batch processing with type preservation
/// str_list = ['{"a": 1}', '{"b": 2}']
/// results = json_tools_rs.JSONTools().flatten().execute(str_list)
/// print(results)  # ['{"a": 1}', '{"b": 2}'] (list of strings)
/// ```
#[cfg(feature = "python")]
#[pyclass(name = "JSONTools", module = "json_tools_rs")]
pub struct PyJSONTools {
    // Use Mutex for interior mutability - allows mutation through shared reference
    // This eliminates the need to clone JSONTools on every builder method call
    // Mutex is required for thread safety (PyO3 requires Sync)
    inner: Mutex<JSONTools>,
}

#[cfg(feature = "python")]
impl Default for PyJSONTools {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// DataFrame and Series Detection Functions
// =============================================================================

/// Detect if the input is a DataFrame or Series
#[cfg(feature = "python")]
fn detect_data_structure(obj: &Bound<'_, PyAny>) -> PyResult<Option<DataStructureType>> {
    // Try DataFrame detection first
    if let Some(df_type) = detect_dataframe_type(obj)? {
        return Ok(Some(DataStructureType::DataFrame(df_type)));
    }

    // Try Series detection
    if let Some(series_type) = detect_series_type(obj)? {
        return Ok(Some(DataStructureType::Series(series_type)));
    }

    // Neither DataFrame nor Series
    Ok(None)
}

/// Detect DataFrame type using duck typing (no imports)
#[cfg(feature = "python")]
fn detect_dataframe_type(obj: &Bound<'_, PyAny>) -> PyResult<Option<DataFrameType>> {
    // Get module and class name for specific detection
    let module = obj
        .getattr("__class__")?
        .getattr("__module__")?
        .extract::<String>()
        .unwrap_or_default();

    let class_name = obj
        .getattr("__class__")?
        .getattr("__name__")?
        .extract::<String>()
        .unwrap_or_default();

    // Check for PyArrow Table (class name is "Table", not "DataFrame")
    if module.starts_with("pyarrow") && class_name == "Table" {
        return Ok(Some(DataFrameType::PyArrow));
    }

    // Check for DataFrame-like methods
    let has_to_dict = obj.hasattr("to_dict")?;
    let has_columns = obj.hasattr("columns")?;

    if !has_to_dict && !has_columns {
        return Ok(None); // Not a DataFrame
    }

    // Check if it's actually a DataFrame class
    if class_name != "DataFrame" {
        return Ok(None);
    }

    // Detect specific library
    if module.starts_with("pandas") {
        Ok(Some(DataFrameType::Pandas))
    } else if module.starts_with("polars") {
        Ok(Some(DataFrameType::Polars))
    } else if module.contains("pyspark.sql") {
        Ok(Some(DataFrameType::PySpark))
    } else if has_to_dict || obj.hasattr("to_json")? {
        // Generic DataFrame-like object
        Ok(Some(DataFrameType::Generic))
    } else {
        Ok(None)
    }
}

/// Detect Series type using duck typing (no imports)
#[cfg(feature = "python")]
fn detect_series_type(obj: &Bound<'_, PyAny>) -> PyResult<Option<SeriesType>> {
    // Get module and class name for specific detection
    let module = obj
        .getattr("__class__")?
        .getattr("__module__")?
        .extract::<String>()
        .unwrap_or_default();

    let class_name = obj
        .getattr("__class__")?
        .getattr("__name__")?
        .extract::<String>()
        .unwrap_or_default();

    // Check for PyArrow Array/ChunkedArray (various class names: Array, ChunkedArray, Int64Array, etc.)
    if module.starts_with("pyarrow")
        && (class_name.contains("Array") || obj.hasattr("to_pylist")?)
    {
        return Ok(Some(SeriesType::PyArrow));
    }

    // Check for Series-like methods
    let has_to_list = obj.hasattr("to_list")? || obj.hasattr("tolist")?;
    let has_dtype = obj.hasattr("dtype")?;

    if !has_to_list && !has_dtype {
        return Ok(None); // Not a Series
    }

    // Check if it's actually a Series class
    if class_name != "Series" {
        return Ok(None);
    }

    // Detect specific library
    if module.starts_with("pandas") {
        Ok(Some(SeriesType::Pandas))
    } else if module.starts_with("polars") {
        Ok(Some(SeriesType::Polars))
    } else if module.contains("pyspark") {
        Ok(Some(SeriesType::PySpark))
    } else if has_to_list {
        // Generic Series-like object
        Ok(Some(SeriesType::Generic))
    } else {
        Ok(None)
    }
}

// =============================================================================
// DataFrame Conversion Functions
// =============================================================================

/// Split newline-delimited JSON (JSONL/NDJSON) text into individual record
/// strings, filtering blank lines. Handles both a trailing newline after the
/// last record and an entirely empty/all-whitespace result for a zero-row
/// DataFrame (pandas gives `"\n"`, polars gives `""` for an empty frame --
/// both correctly yield an empty `Vec` here).
#[cfg(feature = "python")]
fn split_ndjson(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

/// Convert a DataFrame to per-row JSON strings.
///
/// Prefers each library's own native JSON export (pandas `to_json`, polars
/// `write_ndjson`) over `to_dict()`/`to_dicts()` + per-row Python<->Rust
/// conversion: benchmarked at ~2-3x faster for realistic row counts, because
/// `to_dict`/`to_dicts` already pay to construct a full Python object graph
/// (one dict per row, each field touched via the DataFrame library's own
/// Python-level iteration) before any conversion to JSON even starts, while
/// the native JSON writers go straight from the DataFrame's internal
/// columnar storage to bytes. PyArrow has no equivalent native JSON writer,
/// so it still goes through `to_pylist()` + per-item conversion (still
/// faster than the old `depythonize`-based path -- see `py_dumps`'s doc
/// comment for why).
#[cfg(feature = "python")]
fn dataframe_to_json_strings(
    df: &Bound<'_, PyAny>,
    df_type: DataFrameType,
) -> PyResult<Vec<String>> {
    let py = df.py();

    match df_type {
        DataFrameType::Pandas => {
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("orient", "records")?;
            kwargs.set_item("lines", true)?;
            let text: String = df.call_method("to_json", (), Some(&kwargs))?.extract()?;
            Ok(split_ndjson(&text))
        }

        DataFrameType::Polars => {
            let text: String = df.call_method0("write_ndjson")?.extract()?;
            Ok(split_ndjson(&text))
        }

        DataFrameType::PyArrow => {
            let records = df.call_method0("to_pylist")?;
            let list = records.cast::<pyo3::types::PyList>()?;
            pylist_to_json_strings(py, list)
        }

        DataFrameType::PySpark => {
            // No native line-delimited JSON writer reachable without a SparkSession
            // round-trip; convert to pandas first and reuse its native path.
            let pandas_df = df.call_method0("toPandas")?;
            dataframe_to_json_strings(&pandas_df, DataFrameType::Pandas)
        }

        DataFrameType::Generic => {
            // Prefer a pandas-style to_json(orient='records', lines=True) if the
            // object actually accepts that signature; fall back to to_dict() +
            // per-row conversion for anything that doesn't.
            if df.hasattr("to_json")? {
                let kwargs = pyo3::types::PyDict::new(py);
                kwargs.set_item("orient", "records")?;
                kwargs.set_item("lines", true)?;
                if let Ok(result) = df.call_method("to_json", (), Some(&kwargs)) {
                    if let Ok(text) = result.extract::<String>() {
                        return Ok(split_ndjson(&text));
                    }
                }
            }
            if df.hasattr("to_dict")? {
                let kwargs = pyo3::types::PyDict::new(py);
                kwargs.set_item("orient", "records")?;
                let records = df.call_method("to_dict", (), Some(&kwargs))?;
                let list = records.cast::<pyo3::types::PyList>()?;
                pylist_to_json_strings(py, list)
            } else {
                Err(JsonToolsError::new_err(
                    "Generic DataFrame must have to_dict() method",
                ))
            }
        }
    }
}

/// Rows sampled to decide whether a column holds JSON-string values worth
/// auto-expanding -- bounded regardless of DataFrame size, so detection stays
/// cheap. A column that's null (or otherwise never a string) in every sampled
/// row won't be detected -- a safe fallback (stays a plain column, not a
/// crash), not a guarantee of detecting every genuinely-JSON column.
#[cfg(feature = "python")]
const JSON_COLUMN_DETECTION_SAMPLE_SIZE: usize = 20;

/// Detect top-level keys, across a sample of row-JSON strings, that hold a JSON
/// *string* value which itself parses as a JSON object or array -- not any
/// scalar, since a string that happens to parse as a bare number/bool/null
/// isn't a column-expansion candidate. Array is included deliberately: a
/// JSON-string-encoded array should expand into indexed sub-columns the same
/// way an already-list-typed cell does today. A key must parse successfully in
/// *every* sampled row where it appears as a string -- conservative: any
/// failure in the sample disqualifies the whole column from auto-expansion, no
/// partial/mixed result.
#[cfg(feature = "python")]
fn detect_json_string_columns(rows: &[String]) -> Vec<String> {
    let sample_size = rows.len().min(JSON_COLUMN_DETECTION_SAMPLE_SIZE);
    let mut seen: IndexMap<String, usize> = IndexMap::new();
    let mut parsed_ok: IndexMap<String, usize> = IndexMap::new();

    // RawValue-based, same reasoning as `splice_row`: this sample is bounded to
    // (at most) 20 rows regardless of DataFrame size, but a large embedded
    // payload (thousands of keys, github.com/amaye15/JSON-Tools-rs/issues/31)
    // makes even 20 full `Value`-tree parses genuinely slow -- measured ~130ms
    // of a ~650ms total for a 100-row/4,000-key case before this change, nearly
    // a fifth of the whole call for a step that's supposed to be the cheap part.
    for row in &rows[..sample_size] {
        let Ok(fields) =
            serde_json::from_str::<IndexMap<String, &serde_json::value::RawValue>>(row)
        else {
            continue;
        };
        for (key, raw) in &fields {
            let text = raw.get();
            if !text.starts_with('"') {
                continue;
            }
            let Ok(inner) = serde_json::from_str::<String>(text) else {
                continue;
            };
            *seen.entry(key.clone()).or_insert(0) += 1;
            let is_object_or_array = serde_json::from_str::<&serde_json::value::RawValue>(&inner)
                .is_ok()
                && matches!(inner.as_bytes().first(), Some(b'{') | Some(b'['));
            if is_object_or_array {
                *parsed_ok.entry(key.clone()).or_insert(0) += 1;
            }
        }
    }

    seen.into_iter()
        .filter(|(key, count)| parsed_ok.get(key).copied().unwrap_or(0) == *count)
        .map(|(key, _)| key)
        .collect()
}

/// Splice `target_keys`' string values into parsed nested JSON within a single
/// row. Returns `None` if the row doesn't parse as a JSON object at all, or if
/// none of `target_keys` actually needed splicing in this row -- both cases
/// left unchanged by the caller (cheaper than a no-op reserialize). Increments
/// `failure_counts[key]` for any target key present as a string in this row
/// whose value fails to re-parse as an object/array here (the sample that
/// drove detection can still be wrong for a specific later row).
///
/// Deliberately avoids building a full `serde_json::Value` tree for the target
/// field's content (github.com/amaye15/JSON-Tools-rs/issues/31): an earlier
/// version of this function parsed the target string into a full `Value` tree
/// and reserialized the whole row, both O(field count) in the target content --
/// expensive and wasteful for a large embedded payload (e.g. a JSON-string
/// column expanding into thousands of columns), since the core flatten engine
/// parses the same content again moments later regardless. `RawValue` captures
/// each field's exact source byte span without recursing into it, so both the
/// outer row and the target field's validity check stay cheap regardless of
/// how large the target content is; reconstruction copies every OTHER field's
/// bytes verbatim (no reserialization -- also strictly safer than the old
/// approach, e.g. no risk of a `Number` being reformatted through a parse/
/// format round-trip). Benchmarked: ~13x faster on a 200-row/500-key case.
#[cfg(feature = "python")]
fn splice_row(
    row: &str,
    target_keys: &[String],
    failure_counts: &mut IndexMap<String, usize>,
) -> Option<String> {
    let fields: IndexMap<String, &serde_json::value::RawValue> = serde_json::from_str(row).ok()?;

    // Pass 1: decide which target keys actually need splicing, without yet
    // committing to a reconstruction -- preserves the fast path for a row
    // where the target column is null/absent (returns None: the caller reuses
    // the original row string byte-for-byte, zero allocation here).
    let mut substitutions: IndexMap<&str, String> = IndexMap::new();
    let mut changed = false;
    for key in target_keys {
        let Some(raw) = fields.get(key.as_str()) else {
            continue;
        };
        let text = raw.get();
        if !text.starts_with('"') {
            continue; // not a string-typed field (null/number/object/array/bool)
        }
        let Ok(inner) = serde_json::from_str::<String>(text) else {
            continue;
        };
        // Validate-then-classify, in that order: a successful RawValue parse is
        // what guarantees the first byte is one of JSON's single-ASCII-byte
        // leading tokens, safe to inspect directly only after validation.
        match serde_json::from_str::<&serde_json::value::RawValue>(&inner) {
            Ok(_) if matches!(inner.as_bytes().first(), Some(b'{') | Some(b'[')) => {
                substitutions.insert(key.as_str(), inner);
                changed = true;
            }
            _ => {
                // Matches the previous implementation exactly: a valid-but-
                // scalar value and a genuinely invalid one both count as a
                // "failure" for this column (neither is splice-worthy).
                *failure_counts.entry(key.clone()).or_insert(0) += 1;
            }
        }
    }
    if !changed {
        return None;
    }

    // Pass 2: reconstruct the row, splicing `substitutions` for detected keys
    // and copying every other field's exact source bytes verbatim.
    let mut out = String::with_capacity(row.len() + 64);
    out.push('{');
    for (i, (key, raw)) in fields.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&serde_json::to_string(key).ok()?);
        out.push(':');
        match substitutions.get(key.as_str()) {
            Some(inner) => out.push_str(inner),
            None => out.push_str(raw.get()),
        }
    }
    out.push('}');
    Some(out)
}

/// Apply `detect_json_string_columns`, then splice each detected key's per-row
/// string value into genuine nested JSON across all rows -- so `flatten()`
/// expands it exactly the same way an already-dict/struct-typed column does
/// today, with no Python-level parse or dict-object-graph construction (the
/// overhead github.com/amaye15/JSON-Tools-rs/issues/30 reports paying via a
/// manual `orjson.loads()` workaround). A row that fails to re-parse despite
/// its column being detected keeps its original string value for that row only
/// (best-effort, not a hard error -- this is heuristic auto-detection, not a
/// guarantee); if any row failed for a given column, emits one aggregated
/// Python warning per column naming the failure count, so this stays loud
/// instead of silently producing a column that's structurally different for
/// just that one row. Rows with none of the detected columns present pass
/// through completely unchanged (zero parse/reserialize cost) -- the common
/// case for a DataFrame with no JSON-string columns at all.
#[cfg(feature = "python")]
fn expand_json_string_columns(py: Python<'_>, rows: Vec<String>) -> PyResult<Vec<String>> {
    let target_keys = detect_json_string_columns(&rows);
    if target_keys.is_empty() {
        return Ok(rows);
    }

    let mut failure_counts: IndexMap<String, usize> = IndexMap::new();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let spliced = splice_row(&row, &target_keys, &mut failure_counts);
        out.push(spliced.unwrap_or(row));
    }

    if !failure_counts.is_empty() {
        let warnings = py.import("warnings")?;
        for (key, count) in &failure_counts {
            warnings.call_method1(
                "warn",
                (format!(
                    "JSON-string column expansion: {count} row(s) in column \"{key}\" looked \
                     like JSON in a sample but failed to parse and were left as their \
                     original string value",
                ),),
            )?;
        }
    }

    Ok(out)
}

/// Convert a Python list of dicts to per-item JSON strings via Python's own
/// `json` module (see `py_dumps`'s doc comment).
#[cfg(feature = "python")]
fn pylist_to_json_strings(
    py: Python<'_>,
    list: &Bound<'_, pyo3::types::PyList>,
) -> PyResult<Vec<String>> {
    let mut strings = Vec::with_capacity(list.len());
    for item in list.iter() {
        let json_str = py_dumps(py, &item)
            .map_err(|e| JsonToolsError::new_err(format!("Failed to convert record: {}", e)))?;
        strings.push(json_str);
    }
    Ok(strings)
}

// =============================================================================
// Series Conversion Functions
// =============================================================================

/// Convert Series to Python list
#[cfg(feature = "python")]
fn series_to_list<'py>(
    series: &Bound<'py, PyAny>,
    series_type: SeriesType,
) -> PyResult<Bound<'py, pyo3::types::PyList>> {
    match series_type {
        SeriesType::Pandas => {
            // Try to_list() first, fallback to tolist()
            if series.hasattr("to_list")? {
                let list = series.call_method0("to_list")?;
                Ok(list.cast::<pyo3::types::PyList>()?.clone())
            } else {
                let list = series.call_method0("tolist")?;
                Ok(list.cast::<pyo3::types::PyList>()?.clone())
            }
        }

        SeriesType::Polars => {
            // Polars uses to_list()
            let list = series.call_method0("to_list")?;
            Ok(list.cast::<pyo3::types::PyList>()?.clone())
        }

        SeriesType::PyArrow => {
            // PyArrow Arrays use to_pylist()
            let list = series.call_method0("to_pylist")?;
            Ok(list.cast::<pyo3::types::PyList>()?.clone())
        }

        SeriesType::PySpark => {
            // PySpark doesn't have Series, but if it exists, convert via pandas
            let pandas_series = series.call_method0("toPandas")?;
            let list = pandas_series.call_method0("tolist")?;
            Ok(list.cast::<pyo3::types::PyList>()?.clone())
        }

        SeriesType::Generic => {
            // Try to_list() first, fallback to tolist()
            if series.hasattr("to_list")? {
                let list = series.call_method0("to_list")?;
                Ok(list.cast::<pyo3::types::PyList>()?.clone())
            } else if series.hasattr("tolist")? {
                let list = series.call_method0("tolist")?;
                Ok(list.cast::<pyo3::types::PyList>()?.clone())
            } else {
                Err(JsonToolsError::new_err(
                    "Generic Series must have to_list() or tolist() method",
                ))
            }
        }
    }
}

/// Helper macro to reduce boilerplate in PyJSONTools builder methods.
/// Each builder method follows the same pattern: lock the mutex, take the inner
/// JSONTools, apply a builder method, store the result back, and return `slf`.
#[cfg(feature = "python")]
macro_rules! py_builder_method {
    ($slf:expr, $tools:ident, $body:expr) => {{
        let mut guard = lock_config(&$slf.inner)?;
        let $tools = mem::take(&mut *guard);
        *guard = $body;
        drop(guard);
        Ok($slf)
    }};
}

#[cfg(feature = "python")]
#[pymethods]
impl PyJSONTools {
    /// Create a new JSONTools instance with default settings.
    ///
    /// Returns a new builder that can be configured with flatten(), unflatten(),
    /// or normal() mode, optional transformations, and then executed.
    ///
    /// Example:
    ///     >>> tools = JSONTools()
    ///     >>> result = tools.flatten().execute({"user": {"name": "John"}})
    #[new]
    #[pyo3(text_signature = "()")]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(JSONTools::new()),
        }
    }

    /// Set operation mode to flatten nested JSON into dot-separated keys.
    ///
    /// Example:
    ///     >>> result = JSONTools().flatten().execute({"a": {"b": 1}})
    ///     >>> result == {"a.b": 1}
    #[pyo3(text_signature = "($self)")]
    #[inline]
    pub fn flatten(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.flatten())
    }

    /// Set operation mode to unflatten dot-separated keys back into nested JSON.
    ///
    /// Example:
    ///     >>> result = JSONTools().unflatten().execute({"a.b": 1})
    ///     >>> result == {"a": {"b": 1}}
    #[pyo3(text_signature = "($self)")]
    #[inline]
    pub fn unflatten(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.unflatten())
    }

    /// Set operation mode to normal (apply transformations without flatten/unflatten).
    ///
    /// In normal mode, key/value replacements, filtering, and type conversion
    /// are applied recursively without changing the nesting structure.
    #[pyo3(text_signature = "($self)")]
    #[inline]
    pub fn normal(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.normal())
    }

    /// Set the separator for nested keys (default: ".").
    ///
    /// Args:
    ///     separator: Non-empty string to use between nested key segments.
    ///
    /// Raises:
    ///     ValueError: If separator is empty.
    #[pyo3(text_signature = "($self, separator)")]
    #[inline]
    pub fn separator(slf: PyRef<'_, Self>, separator: String) -> PyResult<PyRef<'_, Self>> {
        if separator.is_empty() {
            return Err(PyValueError::new_err("Separator cannot be empty"));
        }
        py_builder_method!(slf, tools, tools.separator(separator))
    }

    /// Enable or disable lowercase key conversion.
    ///
    /// Args:
    ///     value: True to convert all keys to lowercase.
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn lowercase_keys(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.lowercase_keys(value))
    }

    /// Enable or disable removal of keys with empty string values.
    ///
    /// Args:
    ///     value: True to remove keys whose values are empty strings ("").
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn remove_empty_strings(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.remove_empty_strings(value))
    }

    /// Enable or disable removal of keys with null values.
    ///
    /// Args:
    ///     value: True to remove keys whose values are null/None.
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn remove_nulls(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.remove_nulls(value))
    }

    /// Enable or disable removal of keys with empty object values ({}).
    ///
    /// Args:
    ///     value: True to remove keys whose values are empty objects.
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn remove_empty_objects(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.remove_empty_objects(value))
    }

    /// Enable or disable removal of keys with empty array values ([]).
    ///
    /// Args:
    ///     value: True to remove keys whose values are empty arrays.
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn remove_empty_arrays(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.remove_empty_arrays(value))
    }

    /// Add a key replacement pattern (regex or literal fallback).
    ///
    /// Args:
    ///     find: Regex pattern to match against keys. Falls back to literal
    ///         string replacement if regex compilation fails.
    ///     replace: Replacement string.
    #[pyo3(text_signature = "($self, find, replace)")]
    #[inline]
    pub fn key_replacement(
        slf: PyRef<'_, Self>,
        find: String,
        replace: String,
    ) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.key_replacement(find, replace))
    }

    /// Add a value replacement pattern (regex or literal fallback).
    ///
    /// Args:
    ///     find: Regex pattern to match against values. Falls back to literal
    ///         string replacement if regex compilation fails.
    ///     replace: Replacement string.
    #[pyo3(text_signature = "($self, find, replace)")]
    #[inline]
    pub fn value_replacement(
        slf: PyRef<'_, Self>,
        find: String,
        replace: String,
    ) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.value_replacement(find, replace))
    }

    /// Exclude any key (and its entire value/subtree) whose name contains `pattern`.
    ///
    /// Literal substring match by default; wrap in `r'...'` for regex, matching
    /// key_replacement's convention. Additive -- call once per keyword to exclude
    /// multiple. Matching a container key drops its entire subtree.
    ///
    /// Args:
    ///     pattern: Substring (or r'...'-wrapped regex) to match against key names.
    #[pyo3(text_signature = "($self, pattern)")]
    #[inline]
    pub fn exclude_key(slf: PyRef<'_, Self>, pattern: String) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.exclude_key(pattern))
    }

    /// Drop a key-value pair whose value contains `pattern`.
    ///
    /// Literal substring match by default; wrap in `r'...'` for regex, matching
    /// exclude_key's convention. Additive -- call once per pattern to exclude
    /// multiple. Only applies to scalar leaf values (strings/numbers/booleans/null).
    ///
    /// Args:
    ///     pattern: Substring (or r'...'-wrapped regex) to match against values.
    #[pyo3(text_signature = "($self, pattern)")]
    #[inline]
    pub fn exclude_value(slf: PyRef<'_, Self>, pattern: String) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.exclude_value(pattern))
    }

    /// Enable collision handling by collecting duplicate keys into arrays.
    ///
    /// When key transformations produce duplicate keys, values are collected
    /// into arrays (e.g., "name": ["John", "Jane"]).
    ///
    /// Args:
    ///     value: True to enable collision handling.
    #[pyo3(text_signature = "($self, value)")]
    #[inline]
    pub fn handle_key_collision(slf: PyRef<'_, Self>, value: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.handle_key_collision(value))
    }

    /// Enable automatic type conversion from strings to numbers and booleans
    ///
    /// When enabled, the library will attempt to convert string values:
    /// - Numbers: "123" → 123, "1,234.56" → 1234.56, "$99.99" → 99.99
    /// - Booleans: "true"/"TRUE"/"True" → true, "false"/"FALSE"/"False" → false
    ///
    /// If conversion fails, the original string value is kept.
    ///
    /// # Arguments
    /// * `enable` - Whether to enable automatic type conversion
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// result = jt.JSONTools().flatten().auto_convert_types(True).execute({"id": "123", "active": "true"})
    /// print(result)  # {'id': 123, 'active': True}
    /// ```
    ///
    #[pyo3(text_signature = "($self, enable)")]
    #[inline]
    pub fn auto_convert_types(slf: PyRef<'_, Self>, enable: bool) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.auto_convert_types(enable))
    }

    /// Enable or disable date/datetime string conversion independently of the other
    /// type-conversion categories (nulls, booleans, numbers).
    ///
    /// # Arguments
    /// * `enable` - Whether to enable date/datetime conversion
    /// * `normalize_to_utc` - If set, configure whether recognized dates/datetimes
    ///   are normalized to UTC (default: True). When False, a recognized date is
    ///   left unchanged but is still protected from being misread as a number.
    /// * `assume_utc_for_naive` - If set, configure whether timezone-less datetimes
    ///   (e.g. "2024-01-15T10:30:00") are assumed to be UTC and get a `Z` appended
    ///   (default: True). When False, naive datetimes are left unchanged.
    ///
    /// A second call without `normalize_to_utc`/`assume_utc_for_naive` preserves
    /// whatever those were set to by a previous call -- only `enable` is always
    /// applied.
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// tools = jt.JSONTools().flatten().convert_dates(True, assume_utc_for_naive=False)
    /// ```
    #[pyo3(signature = (enable, normalize_to_utc=None, assume_utc_for_naive=None))]
    #[pyo3(text_signature = "($self, enable, normalize_to_utc=None, assume_utc_for_naive=None)")]
    pub fn convert_dates(
        slf: PyRef<'_, Self>,
        enable: bool,
        normalize_to_utc: Option<bool>,
        assume_utc_for_naive: Option<bool>,
    ) -> PyResult<PyRef<'_, Self>> {
        let mut guard = lock_config(&slf.inner)?;
        let tools = mem::take(&mut *guard);
        let mut cfg = tools.date_conversion().clone();
        cfg.enabled = enable;
        if let Some(v) = normalize_to_utc {
            cfg.normalize_to_utc = v;
        }
        if let Some(v) = assume_utc_for_naive {
            cfg.assume_utc_for_naive = v;
        }
        *guard = tools.convert_dates_config(cfg);
        drop(guard);
        Ok(slf)
    }

    /// Enable or disable null-string conversion independently of the other
    /// type-conversion categories (dates, booleans, numbers).
    ///
    /// # Arguments
    /// * `enable` - Whether to enable null-string conversion
    /// * `extra_tokens` - If set, a list of additional strings to recognize as null,
    ///   beyond the built-in list ("null", "NULL", "nil", "none", "N/A", "NA", etc.).
    ///   Additive only -- replaces any previously-set extra token list, but never
    ///   narrows the built-in list. Matched exactly (case-sensitive).
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// tools = jt.JSONTools().flatten().convert_nulls(True, extra_tokens=["missing"])
    /// ```
    #[pyo3(signature = (enable, extra_tokens=None))]
    #[pyo3(text_signature = "($self, enable, extra_tokens=None)")]
    pub fn convert_nulls(
        slf: PyRef<'_, Self>,
        enable: bool,
        extra_tokens: Option<Vec<String>>,
    ) -> PyResult<PyRef<'_, Self>> {
        let mut guard = lock_config(&slf.inner)?;
        let tools = mem::take(&mut *guard);
        let mut cfg = tools.null_conversion().clone();
        cfg.enabled = enable;
        if let Some(tokens) = extra_tokens {
            cfg.extra_tokens = tokens.into();
        }
        *guard = tools.convert_nulls_config(cfg);
        drop(guard);
        Ok(slf)
    }

    /// Enable or disable boolean-string conversion independently of the other
    /// type-conversion categories (dates, nulls, numbers).
    ///
    /// # Arguments
    /// * `enable` - Whether to enable boolean-string conversion
    /// * `extra_true_tokens` - If set, additional strings recognized as `true`,
    ///   beyond the built-in list ("true", "yes", "on", "y", etc.). Replaces any
    ///   previously-set list.
    /// * `extra_false_tokens` - If set, additional strings recognized as `false`,
    ///   beyond the built-in list ("false", "no", "off", "n", etc.). Replaces any
    ///   previously-set list.
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// tools = jt.JSONTools().flatten().convert_booleans(
    ///     True, extra_true_tokens=["si"], extra_false_tokens=["nope"]
    /// )
    /// ```
    #[pyo3(signature = (enable, extra_true_tokens=None, extra_false_tokens=None))]
    #[pyo3(text_signature = "($self, enable, extra_true_tokens=None, extra_false_tokens=None)")]
    pub fn convert_booleans(
        slf: PyRef<'_, Self>,
        enable: bool,
        extra_true_tokens: Option<Vec<String>>,
        extra_false_tokens: Option<Vec<String>>,
    ) -> PyResult<PyRef<'_, Self>> {
        let mut guard = lock_config(&slf.inner)?;
        let tools = mem::take(&mut *guard);
        let mut cfg = tools.boolean_conversion().clone();
        cfg.enabled = enable;
        if let Some(tokens) = extra_true_tokens {
            cfg.extra_true_tokens = tokens.into();
        }
        if let Some(tokens) = extra_false_tokens {
            cfg.extra_false_tokens = tokens.into();
        }
        *guard = tools.convert_booleans_config(cfg);
        drop(guard);
        Ok(slf)
    }

    /// Enable or disable numeric-string conversion independently of the other
    /// type-conversion categories (dates, nulls, booleans).
    ///
    /// Plain integers/decimals, scientific notation, and thousands-separator
    /// cleanup are always applied when `enable` is True; the remaining sub-formats
    /// can each be disabled independently.
    ///
    /// # Arguments
    /// * `enable` - Whether to enable numeric-string conversion
    /// * `currency` - If set, configure currency symbol/code/credit-debit-suffix
    ///   stripping (default: True)
    /// * `percent` - If set, configure `%`/permille/per-ten-thousand suffix parsing
    ///   (default: True)
    /// * `basis_points` - If set, configure text basis-point suffix parsing, e.g.
    ///   "25bps" (default: True)
    /// * `suffixes` - If set, configure K/M/B/T magnitude suffix parsing (default: True)
    /// * `fractions` - If set, configure fraction parsing, e.g. "1/2" (default: True)
    /// * `radix` - If set, configure hex/binary/octal literal parsing (default: True)
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// tools = jt.JSONTools().flatten().convert_numbers(True, currency=False)
    /// ```
    #[pyo3(signature = (
        enable,
        currency=None,
        percent=None,
        basis_points=None,
        suffixes=None,
        fractions=None,
        radix=None
    ))]
    #[pyo3(
        text_signature = "($self, enable, currency=None, percent=None, basis_points=None, suffixes=None, fractions=None, radix=None)"
    )]
    #[allow(clippy::too_many_arguments)]
    pub fn convert_numbers(
        slf: PyRef<'_, Self>,
        enable: bool,
        currency: Option<bool>,
        percent: Option<bool>,
        basis_points: Option<bool>,
        suffixes: Option<bool>,
        fractions: Option<bool>,
        radix: Option<bool>,
    ) -> PyResult<PyRef<'_, Self>> {
        let mut guard = lock_config(&slf.inner)?;
        let tools = mem::take(&mut *guard);
        let mut cfg = tools.number_conversion().clone();
        cfg.enabled = enable;
        if let Some(v) = currency {
            cfg.currency = v;
        }
        if let Some(v) = percent {
            cfg.percent = v;
        }
        if let Some(v) = basis_points {
            cfg.basis_points = v;
        }
        if let Some(v) = suffixes {
            cfg.suffixes = v;
        }
        if let Some(v) = fractions {
            cfg.fractions = v;
        }
        if let Some(v) = radix {
            cfg.radix = v;
        }
        *guard = tools.convert_numbers_config(cfg);
        drop(guard);
        Ok(slf)
    }

    /// Set the minimum batch size for parallel processing
    ///
    /// When processing multiple JSON documents, this threshold determines when to use
    /// parallel processing. Batches smaller than this threshold will be processed sequentially
    /// to avoid the overhead of thread spawning.
    ///
    /// Default: 10 items (can be overridden with JSON_TOOLS_PARALLEL_THRESHOLD environment variable)
    ///
    /// # Arguments
    /// * `threshold` - Minimum number of items in a batch to trigger parallel processing
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// # Only use parallelism for batches of 50+ items
    /// tools = jt.JSONTools().flatten().parallel_threshold(50)
    /// results = tools.execute([...])  # Large batch will use parallel processing
    /// ```
    ///
    #[pyo3(text_signature = "($self, threshold)")]
    #[inline]
    pub fn parallel_threshold(slf: PyRef<'_, Self>, threshold: usize) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.parallel_threshold(threshold))
    }

    /// Configure the number of threads for parallel processing
    ///
    /// By default, the number of logical CPUs is used. This method allows you to override
    /// that behavior for specific workloads or resource constraints.
    ///
    /// # Arguments
    /// * `num_threads` - Number of threads to use (None = use system default)
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// # Limit to 4 threads for resource-constrained environments
    /// tools = jt.JSONTools().flatten().num_threads(4)
    /// # Or use None to auto-detect (default)
    /// tools = jt.JSONTools().flatten().num_threads(None)
    /// ```
    ///
    #[pyo3(text_signature = "($self, num_threads)")]
    #[inline]
    pub fn num_threads(
        slf: PyRef<'_, Self>,
        num_threads: Option<usize>,
    ) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.num_threads(num_threads))
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
    /// * `threshold` - Minimum number of keys/items to trigger nested parallelism
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Example
    /// ```python
    /// import json_tools_rs as jt
    /// # Only parallelize objects/arrays with 200+ items
    /// tools = jt.JSONTools().flatten().nested_parallel_threshold(200)
    /// result = tools.execute(large_json)  # Large nested structures will use parallel processing
    /// ```
    ///
    #[pyo3(text_signature = "($self, threshold)")]
    #[inline]
    pub fn nested_parallel_threshold(
        slf: PyRef<'_, Self>,
        threshold: usize,
    ) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.nested_parallel_threshold(threshold))
    }

    /// Set the maximum array index allowed during unflattening (DoS protection)
    ///
    /// Prevents malicious flattened keys like "items.999999999" from causing
    /// excessive memory allocation. Keys with array indices exceeding this
    /// limit will produce an error during unflattening.
    ///
    /// Default: 100,000
    #[pyo3(text_signature = "($self, max)")]
    #[inline]
    pub fn max_array_index(slf: PyRef<'_, Self>, max: usize) -> PyResult<PyRef<'_, Self>> {
        py_builder_method!(slf, tools, tools.max_array_index(max))
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
    /// * When `normalise=True`: always a wide DataFrame (one column per flattened
    ///   key), regardless of input shape -- see `execute_normalise` for details.
    ///
    /// # Performance
    /// Uses interior mutability to avoid cloning JSONTools - only clones for execute() call
    #[pyo3(signature = (json_input, normalise=false, target=None))]
    #[pyo3(text_signature = "($self, json_input, normalise=False, target=None)")]
    pub fn execute(
        &self,
        json_input: &Bound<'_, PyAny>,
        normalise: bool,
        target: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let py = json_input.py();

        if normalise {
            return self.execute_normalise(json_input, target);
        }
        if target.is_some() {
            return Err(JsonToolsError::new_err(
                "target is only valid when normalise=True",
            ));
        }

        // An exactly-typed str/dict/list can never be a DataFrame/Series, so
        // skip the duck-typing detection (several getattr/hasattr chains) for
        // the common input shapes -- it's pure per-call overhead there.
        // Subclasses and everything else still take the full detection path.
        let is_exact_common_type = json_input.is_exact_instance_of::<PyString>()
            || json_input.is_exact_instance_of::<PyDict>()
            || json_input.is_exact_instance_of::<PyList>();

        // Check for DataFrame or Series first (before other type checks)
        if !is_exact_common_type {
            match detect_data_structure(json_input)? {
                Some(DataStructureType::DataFrame(df_type)) => {
                    return self.execute_dataframe(json_input, df_type);
                }
                Some(DataStructureType::Series(series_type)) => {
                    return self.execute_series(json_input, series_type);
                }
                None => {
                    // Fall through to existing type checks
                }
            }
        }

        // Fast path: single JSON string → return JSON string
        if let Ok(json_str) = json_input.extract::<String>() {
            // TIER 6→3 OPTIMIZATION: Take ownership instead of cloning
            // Saves 1K-10K cycles by avoiding deep clone of entire JSONTools config
            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_str.as_str());
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON string: {}", e))
                })?;

            match result {
                JsonOutput::Single(processed) => {
                    Ok(processed.into_pyobject(py)?.into_any().unbind())
                }
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single JSON input",
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyDict>() {
            // Serialize via Python's own `json` module -- see `py_dumps`'s doc comment
            // for why this beats the generic serde-based `depythonize` for the nested
            // data this library's flatten/unflatten exist to handle.
            let json_str = py_dumps(py, json_input).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to convert Python dict: {}", e))
            })?;

            // Process with Rust tools (release GIL)
            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_str.as_str());
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process Python dict: {}", e))
                })?;

            match result {
                JsonOutput::Single(processed) => {
                    let python_dict = py_loads(py, &processed).map_err(|e| {
                        JsonToolsError::new_err(format!("Failed to convert to Python: {}", e))
                    })?;
                    Ok(python_dict.unbind())
                }
                JsonOutput::Multiple(_) => Err(PyValueError::new_err(
                    "Unexpected multiple results for single dict input",
                )),
            }
        } else if json_input.is_instance_of::<pyo3::types::PyList>() {
            // Handle list input - batch processing of JSON strings and/or dicts
            let list = json_input.cast::<pyo3::types::PyList>()?;

            if list.is_empty() {
                return Ok(Vec::<String>::new().into_pyobject(py)?.into_any().unbind());
            }

            let mut json_strings: Vec<String> = Vec::with_capacity(list.len());
            let mut is_str_flags: Vec<bool> = Vec::with_capacity(list.len());
            let mut has_other_types = false;

            for item in list.iter() {
                if let Ok(json_str) = item.extract::<String>() {
                    json_strings.push(json_str);
                    is_str_flags.push(true);
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    let json_str = py_dumps(py, &item).map_err(|e| {
                        JsonToolsError::new_err(format!("Failed to convert dict in list: {}", e))
                    })?;
                    json_strings.push(json_str);
                    is_str_flags.push(false);
                } else {
                    has_other_types = true;
                    break;
                }
            }

            if has_other_types {
                return Err(PyValueError::new_err(
                    "List items must be either JSON strings or Python dictionaries",
                ));
            }

            // TIER 6→3 OPTIMIZATION: Take ownership instead of cloning
            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_strings);
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON list: {}", e))
                })?;

            match result {
                JsonOutput::Single(_) => Err(PyValueError::new_err(
                    "Unexpected single result for multiple input",
                )),
                JsonOutput::Multiple(processed_list) => {
                    // Determine output shape and transform accordingly
                    let all_strings = is_str_flags.iter().all(|&b| b);
                    let all_dicts = is_str_flags.iter().all(|&b| !b);

                    if all_strings {
                        Ok(processed_list.into_pyobject(py)?.into_any().unbind())
                    } else if all_dicts {
                        let mut dict_results: Vec<Py<PyAny>> =
                            Vec::with_capacity(processed_list.len());
                        for processed_json in processed_list {
                            let python_dict = py_loads(py, &processed_json).map_err(|e| {
                                JsonToolsError::new_err(format!(
                                    "Failed to convert to Python dict: {}",
                                    e
                                ))
                            })?;
                            dict_results.push(python_dict.unbind());
                        }
                        Ok(dict_results.into_pyobject(py)?.into_any().unbind())
                    } else {
                        let mut mixed_results: Vec<Py<PyAny>> =
                            Vec::with_capacity(processed_list.len());
                        for (processed_json, is_str) in processed_list.into_iter().zip(is_str_flags)
                        {
                            if is_str {
                                mixed_results
                                    .push(processed_json.into_pyobject(py)?.into_any().unbind());
                            } else {
                                let python_dict = py_loads(py, &processed_json).map_err(|e| {
                                    JsonToolsError::new_err(format!(
                                        "Failed to convert to Python dict: {}",
                                        e
                                    ))
                                })?;
                                mixed_results.push(python_dict.unbind());
                            }
                        }
                        Ok(mixed_results.into_pyobject(py)?.into_any().unbind())
                    }
                }
            }
        } else {
            Err(PyValueError::new_err(
                "json_input must be a JSON string, Python dict, list of JSON strings, or list of Python dicts",
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
    ///
    /// # Performance
    /// Uses interior mutability to avoid cloning JSONTools - only clones for execute() call
    #[pyo3(text_signature = "($self, json_input)")]
    pub fn execute_to_output(&self, json_input: &Bound<'_, PyAny>) -> PyResult<PyJsonOutput> {
        let py = json_input.py();

        // Note: DataFrames/Series are not supported in execute_to_output()
        // Use execute() instead for DataFrame/Series support

        // Single JSON string
        if let Ok(json_str) = json_input.extract::<String>() {
            // TIER 6→3: Take ownership instead of cloning (10K-50K cycles saved)
            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_str.as_str());
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON string: {}", e))
                })?;
            return Ok(PyJsonOutput::from_rust_output(result));
        }

        // Single Python dictionary - serialize via Python's own `json` module (see
        // `py_dumps`'s doc comment)
        if json_input.is_instance_of::<pyo3::types::PyDict>() {
            let json_str = py_dumps(py, json_input).map_err(|e| {
                JsonToolsError::new_err(format!("Failed to convert Python dict: {}", e))
            })?;

            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_str.as_str());
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process Python dict: {}", e))
                })?;
            return Ok(PyJsonOutput::from_rust_output(result));
        }

        // List input - batch processing or single JSON array
        if json_input.is_instance_of::<pyo3::types::PyList>() {
            let list = json_input.cast::<pyo3::types::PyList>()?;

            if list.is_empty() {
                return Ok(PyJsonOutput::from_rust_output(JsonOutput::Multiple(vec![])));
            }

            let mut json_strings: Vec<String> = Vec::with_capacity(list.len());

            for item in list.iter() {
                if let Ok(json_str) = item.extract::<String>() {
                    json_strings.push(json_str);
                } else if item.is_instance_of::<pyo3::types::PyDict>() {
                    let json_str = py_dumps(py, &item).map_err(|e| {
                        JsonToolsError::new_err(format!("Failed to convert dict in list: {}", e))
                    })?;
                    json_strings.push(json_str);
                } else {
                    return Err(PyValueError::new_err(
                        "List items must be either JSON strings or Python dictionaries",
                    ));
                }
            }

            // Process the list of JSON strings directly
            // TIER 6→3: Take ownership instead of cloning (10K-50K cycles saved)
            let result = py
                .detach(|| {
                    let mut guard = lock_config(&self.inner)?;
                    let tools = mem::take(&mut *guard);
                    let result = tools.execute(json_strings);
                    *guard = tools;
                    result
                })
                .map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to process JSON list: {}", e))
                })?;
            return Ok(PyJsonOutput::from_rust_output(result));
        }

        Err(PyValueError::new_err(
            "json_input must be a JSON string, Python dict, DataFrame, Series, list of JSON strings, or list of Python dicts",
        ))
    }

    /// Serialize this instance's configuration to a JSON string, from which an
    /// equivalent, independent instance can be rebuilt via
    /// `JSONTools.from_config_json(...)`.
    ///
    /// This is the mechanism behind pickle support (`__reduce__`, below) --
    /// useful directly too for shipping a configured `JSONTools` across any
    /// other process boundary that can't pickle a native extension object,
    /// e.g. a PySpark `mapInPandas` partition function, which should close
    /// over this string (not `self`) and call `from_config_json` fresh inside
    /// each partition.
    #[pyo3(text_signature = "($self)")]
    pub fn to_config_json(&self) -> PyResult<String> {
        let guard = lock_config(&self.inner)?;
        Ok(guard.to_config_json())
    }

    /// Reconstruct a `JSONTools` instance from a JSON string previously
    /// produced by `to_config_json()`. See that method's doc for why this
    /// pair exists.
    #[staticmethod]
    #[pyo3(text_signature = "(config_json)")]
    pub fn from_config_json(config_json: &str) -> PyResult<Self> {
        let tools = crate::config_json::build_tools(config_json)
            .map_err(|e| JsonToolsError::new_err(format!("Invalid configuration: {e}")))?;
        Ok(Self {
            inner: Mutex::new(tools),
        })
    }

    /// Pickle support (`pickle.dumps`/`pickle.loads`, and by extension
    /// cloudpickle-based closure capture, e.g. inside a PySpark UDF or
    /// `mapInPandas` function) -- see github.com/amaye15/JSON-Tools-rs/issues/29.
    ///
    /// A `PyJSONTools` wraps a native Rust object with no Python `__dict__`,
    /// so the standard `__reduce__` protocol is used instead of `__getstate__`/
    /// `__setstate__`: returns `(from_config_json, (config_json_string,))`.
    /// Unpickling calls `JSONTools.from_config_json(config_json_string)`,
    /// which reconstructs an independent, equivalently-configured instance --
    /// `from_config_json` is a `#[staticmethod]` on a module-registered class,
    /// so pickle can serialize the callable itself by reference
    /// (`json_tools_rs.JSONTools.from_config_json`), the same way it already
    /// handles any other class/static method reference.
    pub fn __reduce__<'py>(slf: &Bound<'py, Self>) -> PyResult<(Bound<'py, PyAny>, (String,))> {
        let config_json = slf.borrow().to_config_json()?;
        let ctor = slf.get_type().getattr("from_config_json")?;
        Ok((ctor, (config_json,)))
    }
}

// =============================================================================
// DataFrame Reconstruction Functions
// =============================================================================

/// Reconstruct DataFrame from list of dicts (with fallback to list)
#[cfg(feature = "python")]
fn reconstruct_dataframe(
    py: Python,
    df_type: DataFrameType,
    processed_dicts: Vec<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    match df_type {
        DataFrameType::Pandas => reconstruct_pandas_df(py, processed_dicts),
        DataFrameType::Polars => reconstruct_polars_df(py, processed_dicts),
        DataFrameType::PyArrow => reconstruct_pyarrow_table(py, processed_dicts),
        DataFrameType::PySpark => {
            // PySpark reconstruction would need SparkSession - fallback to list for now
            Ok(processed_dicts.into_pyobject(py)?.into_any().unbind())
        }
        DataFrameType::Generic => {
            // Generic - just return list of dicts
            Ok(processed_dicts.into_pyobject(py)?.into_any().unbind())
        }
    }
}

/// Reconstruct pandas DataFrame
#[cfg(feature = "python")]
fn reconstruct_pandas_df(py: Python, records: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = records.into_pyobject(py)?.into_any().unbind();

    match py.import("pandas") {
        Ok(pandas) => match pandas.call_method1("DataFrame", (py_list.clone_ref(py),)) {
            Ok(df) => Ok(df.unbind()),
            Err(_) => Ok(py_list),
        },
        Err(_) => Ok(py_list),
    }
}

/// Reconstruct polars DataFrame
#[cfg(feature = "python")]
fn reconstruct_polars_df(py: Python, records: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = records.into_pyobject(py)?.into_any().unbind();

    match py.import("polars") {
        Ok(polars) => match polars.call_method1("DataFrame", (py_list.clone_ref(py),)) {
            Ok(df) => Ok(df.unbind()),
            Err(_) => Ok(py_list),
        },
        Err(_) => Ok(py_list),
    }
}

/// Reconstruct PyArrow Table
#[cfg(feature = "python")]
fn reconstruct_pyarrow_table(py: Python, records: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = records.into_pyobject(py)?.into_any().unbind();

    match py.import("pyarrow") {
        Ok(pyarrow) => {
            let table_class = pyarrow.getattr("Table")?;
            match table_class.call_method1("from_pylist", (py_list.clone_ref(py),)) {
                Ok(table) => Ok(table.unbind()),
                Err(_) => Ok(py_list),
            }
        }
        Err(_) => Ok(py_list),
    }
}

// =============================================================================
// Series Reconstruction Functions
// =============================================================================

/// Reconstruct Series from list (with fallback to list)
#[cfg(feature = "python")]
fn reconstruct_series(
    py: Python,
    series_type: SeriesType,
    processed_items: Vec<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    match series_type {
        SeriesType::Pandas => reconstruct_pandas_series(py, processed_items),
        SeriesType::Polars => reconstruct_polars_series(py, processed_items),
        SeriesType::PyArrow => reconstruct_pyarrow_array(py, processed_items),
        SeriesType::PySpark => {
            // PySpark doesn't have Series - fallback to list
            Ok(processed_items.into_pyobject(py)?.into_any().unbind())
        }
        SeriesType::Generic => {
            // Generic - just return list
            Ok(processed_items.into_pyobject(py)?.into_any().unbind())
        }
    }
}

/// Reconstruct pandas Series
#[cfg(feature = "python")]
fn reconstruct_pandas_series(py: Python, items: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = items.into_pyobject(py)?.into_any().unbind();

    match py.import("pandas") {
        Ok(pandas) => match pandas.call_method1("Series", (py_list.clone_ref(py),)) {
            Ok(series) => Ok(series.unbind()),
            Err(_) => Ok(py_list),
        },
        Err(_) => Ok(py_list),
    }
}

/// Reconstruct polars Series
#[cfg(feature = "python")]
fn reconstruct_polars_series(py: Python, items: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = items.into_pyobject(py)?.into_any().unbind();

    match py.import("polars") {
        Ok(polars) => match polars.call_method1("Series", (py_list.clone_ref(py),)) {
            Ok(series) => Ok(series.unbind()),
            Err(_) => Ok(py_list),
        },
        Err(_) => Ok(py_list),
    }
}

/// Reconstruct PyArrow Array
#[cfg(feature = "python")]
fn reconstruct_pyarrow_array(py: Python, items: Vec<Py<PyAny>>) -> PyResult<Py<PyAny>> {
    // Convert to Python list once — O(1) refcount clone instead of O(N) item clones
    let py_list = items.into_pyobject(py)?.into_any().unbind();

    match py.import("pyarrow") {
        Ok(pyarrow) => match pyarrow.call_method1("array", (py_list.clone_ref(py),)) {
            Ok(array) => Ok(array.unbind()),
            Err(_) => Ok(py_list),
        },
        Err(_) => Ok(py_list),
    }
}

// =============================================================================
// Normalise: Wide DataFrame Reconstruction (execute(..., normalise=True))
// =============================================================================
//
// Unlike the lenient `reconstruct_pandas_df`/`reconstruct_polars_df`/
// `reconstruct_pyarrow_table` above (used by plain `execute(df)`, which silently
// falls back to a list of dicts if the target library isn't installed), everything
// in this section is strict: `normalise=True` is an explicit request for a real
// DataFrame, so a missing library or an un-normaliseable row is a clear
// `JsonToolsError`, never a silent fallback.

/// Convert a Python list whose items are each either a JSON string or a Python
/// dict into per-item JSON strings. The same mixed-item handling `execute()`'s
/// list branch and `execute_series` already do inline, factored out here since
/// `extract_normalise_json_strings` needs it for both list input and
/// Series-derived lists.
#[cfg(feature = "python")]
fn mixed_pylist_to_json_strings(py: Python<'_>, list: &Bound<'_, PyList>) -> PyResult<Vec<String>> {
    let mut json_strings = Vec::with_capacity(list.len());
    for item in list.iter() {
        if let Ok(json_str) = item.extract::<String>() {
            json_strings.push(json_str);
        } else if item.is_instance_of::<PyDict>() {
            let json_str = py_dumps(py, &item)
                .map_err(|e| JsonToolsError::new_err(format!("Failed to convert item: {e}")))?;
            json_strings.push(json_str);
        } else {
            return Err(PyValueError::new_err(
                "List/Series items must be either JSON strings or Python dictionaries",
            ));
        }
    }
    Ok(json_strings)
}

/// Coerce any shape `execute()` accepts into a uniform `Vec<String>` of JSON rows
/// for `normalise=True` -- a bare `str`/`dict` becomes a single-row `Vec`, matching
/// how plain `execute()` treats those as one opaque unit. Also returns the
/// detected input structure (when the input was itself a live DataFrame/Series),
/// used by `resolve_normalise_target` to default the output to the input's own
/// backend.
#[cfg(feature = "python")]
fn extract_normalise_json_strings(
    json_input: &Bound<'_, PyAny>,
) -> PyResult<(Vec<String>, Option<DataStructureType>)> {
    let py = json_input.py();

    if let Some(structure) = detect_data_structure(json_input)? {
        let json_strings = match structure {
            DataStructureType::DataFrame(df_type) => {
                let rows = dataframe_to_json_strings(json_input, df_type)?;
                // Unconditional here (no is_flatten_mode() check needed): the only
                // caller, execute_normalise, already hard-requires flatten mode
                // before reaching this function at all -- see its doc comment.
                expand_json_string_columns(py, rows)?
            }
            DataStructureType::Series(series_type) => {
                let list = series_to_list(json_input, series_type)?;
                mixed_pylist_to_json_strings(py, &list)?
            }
        };
        return Ok((json_strings, Some(structure)));
    }

    if let Ok(json_str) = json_input.extract::<String>() {
        return Ok((vec![json_str], None));
    }

    if json_input.is_instance_of::<PyDict>() {
        let json_str = py_dumps(py, json_input)
            .map_err(|e| JsonToolsError::new_err(format!("Failed to convert Python dict: {e}")))?;
        return Ok((vec![json_str], None));
    }

    if json_input.is_instance_of::<PyList>() {
        let list = json_input.cast::<PyList>()?;
        let json_strings = mixed_pylist_to_json_strings(py, list)?;
        return Ok((json_strings, None));
    }

    Err(PyValueError::new_err(
        "json_input must be a JSON string, Python dict, list of JSON strings/dicts, DataFrame, or Series",
    ))
}

/// Resolve the effective `NormaliseTarget`: an explicit `target=` wins outright;
/// otherwise an input that was itself a live DataFrame/Series of a known backend
/// keeps that backend; otherwise try pandas -> polars -> pyarrow, first installed
/// wins. `pyspark` is deliberately excluded from this last bare-JSON-input
/// fallback -- creating a SparkSession-dependent result as a silent default for a
/// plain string/dict input would be surprising, so pyspark is only reachable via
/// an explicit `target="pyspark"` or when the input itself was already a
/// live PySpark object.
#[cfg(feature = "python")]
fn resolve_normalise_target(
    py: Python<'_>,
    detected: Option<DataStructureType>,
    requested: Option<NormaliseTarget>,
) -> PyResult<NormaliseTarget> {
    if let Some(target) = requested {
        return Ok(target);
    }

    let from_input = match detected {
        Some(DataStructureType::DataFrame(t)) => NormaliseTarget::from_dataframe_type(t),
        Some(DataStructureType::Series(t)) => NormaliseTarget::from_series_type(t),
        None => None,
    };
    if let Some(target) = from_input {
        return Ok(target);
    }

    for candidate in [
        NormaliseTarget::Pandas,
        NormaliseTarget::Polars,
        NormaliseTarget::PyArrow,
    ] {
        if py.import(candidate.name()).is_ok() {
            return Ok(candidate);
        }
    }

    Err(JsonToolsError::new_err(
        "normalise=True could not auto-detect a target DataFrame library: none of pandas, \
         polars, pyarrow are installed. Pass target=\"pandas\"|\"polars\"|\"pyarrow\"|\"pyspark\" \
         explicitly, or install one of these libraries.",
    ))
}

/// Strict presence check for `normalise=True`'s target library -- see this
/// section's header comment for why this doesn't fall back silently like the
/// lenient `reconstruct_*_df` helpers do.
#[cfg(feature = "python")]
fn require_importable<'py>(
    py: Python<'py>,
    target: NormaliseTarget,
) -> PyResult<Bound<'py, PyModule>> {
    py.import(target.name()).map_err(|_| {
        JsonToolsError::new_err(format!(
            "normalise(target=\"{}\") requires the '{}' package to be installed",
            target.name(),
            target.name()
        ))
    })
}

/// Auto-discover the active PySpark session for `target="pyspark"`. No `spark=`
/// parameter is offered -- the caller is expected to already be inside a Spark
/// driver/notebook with a session created (`SparkSession.builder.getOrCreate()`).
#[cfg(feature = "python")]
fn require_active_spark_session(py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
    let pyspark_sql = py.import("pyspark.sql").map_err(|_| {
        JsonToolsError::new_err(
            "normalise(target=\"pyspark\") requires the 'pyspark' package to be installed",
        )
    })?;
    let session_cls = pyspark_sql.getattr("SparkSession")?;
    let active = session_cls.call_method0("getActiveSession")?;
    if active.is_none() {
        return Err(JsonToolsError::new_err(
            "normalise(target=\"pyspark\") requires an active SparkSession \
             (SparkSession.getActiveSession() returned None); create one first, \
             e.g. SparkSession.builder.getOrCreate()",
        ));
    }
    Ok(active)
}

/// Flattened rows reshaped into columns for `normalise=True`, keyed by
/// first-seen field order, with every column's length equal to the row count
/// (a row missing a given key contributes `None` for it).
///
/// Values are left as plain `None` for missing cells -- pandas/polars/pyarrow
/// all handle an all-`None` column on their own without crashing (`object`,
/// `Null`, and `null`-typed respectively; confirmed empirically). PySpark is
/// the one target that needs special handling, done entirely within
/// `reconstruct_pyspark_normalise` via an explicit schema -- see its doc
/// comment for why.
#[cfg(feature = "python")]
struct NormaliseColumns<'py> {
    /// Column name -> per-row values, in first-seen key order.
    columns: IndexMap<String, Vec<Bound<'py, PyAny>>>,
}

/// Parse each processed (flattened) JSON string into a row, union every row's
/// keys in first-seen order, and null-fill any row missing a given key -- shared
/// by all four `normalise` targets so their union/null-fill behavior is
/// *provably* consistent by construction, rather than relying on three separate
/// third-party constructors' own implicit (and independently version-dependent)
/// dict-list union behavior.
#[cfg(feature = "python")]
fn union_and_columnarize<'py>(
    py: Python<'py>,
    processed: Vec<String>,
) -> PyResult<NormaliseColumns<'py>> {
    let n_rows = processed.len();
    let mut rows: Vec<Bound<'py, PyDict>> = Vec::with_capacity(n_rows);
    let mut key_order: IndexSet<String> = IndexSet::new();

    for (idx, json_str) in processed.iter().enumerate() {
        let value = py_loads(py, json_str).map_err(|e| {
            JsonToolsError::new_err(format!("Failed to parse flattened row {idx}: {e}"))
        })?;
        let dict = value
            .cast::<PyDict>()
            .map_err(|_| {
                JsonToolsError::new_err(format!(
                    "normalise=True requires every flattened row to be a JSON object; \
                     row {idx} produced a different type instead"
                ))
            })?
            .clone();
        for key in dict.keys() {
            key_order.insert(key.extract::<String>()?);
        }
        rows.push(dict);
    }

    let mut columns: IndexMap<String, Vec<Bound<'py, PyAny>>> =
        IndexMap::with_capacity(key_order.len());

    for key in &key_order {
        let mut col: Vec<Bound<'py, PyAny>> = Vec::with_capacity(n_rows);
        let mut any_list = false;
        for row in &rows {
            let value = match row.get_item(key)? {
                Some(v) => {
                    if v.is_instance_of::<PyList>() {
                        any_list = true;
                    }
                    v
                }
                None => py.None().into_bound(py),
            };
            col.push(value);
        }
        // `handle_key_collision(True)` produces a list-valued cell only for rows
        // where a collision actually occurred that row; other rows for the same
        // key stay scalar. A column mixing list and scalar values is rejected by
        // pyarrow (`ArrowInvalid: cannot mix list and non-list, non-null values`)
        // and polars (`TypeError: unexpected value while building Series of type
        // List(...)`) -- confirmed empirically, not hypothetical. Once any row
        // makes a column list-valued, wrap every other non-null cell in that
        // column into a single-element list too, so the column is uniformly
        // list-typed across all four targets instead of failing on two of them.
        if any_list {
            for cell in col.iter_mut() {
                if !cell.is_none() && !cell.is_instance_of::<PyList>() {
                    *cell = PyList::new(py, [cell.clone()])?.into_any();
                }
            }
        }
        columns.insert(key.clone(), col);
    }

    Ok(NormaliseColumns { columns })
}

/// Reconstruct a pandas DataFrame from unioned/null-filled columns. Only called
/// after `require_importable` has confirmed pandas is present.
#[cfg(feature = "python")]
fn reconstruct_pandas_normalise(
    py: Python<'_>,
    cols: &NormaliseColumns<'_>,
) -> PyResult<Py<PyAny>> {
    let pandas = py.import("pandas")?;
    let data = PyDict::new(py);
    for (key, values) in &cols.columns {
        data.set_item(key, PyList::new(py, values.iter().cloned())?)?;
    }
    let df = pandas.call_method1("DataFrame", (data,))?;
    Ok(df.unbind())
}

/// Reconstruct a polars DataFrame from unioned/null-filled columns. Only called
/// after `require_importable` has confirmed polars is present.
#[cfg(feature = "python")]
fn reconstruct_polars_normalise(
    py: Python<'_>,
    cols: &NormaliseColumns<'_>,
) -> PyResult<Py<PyAny>> {
    let polars = py.import("polars")?;
    let data = PyDict::new(py);
    for (key, values) in &cols.columns {
        data.set_item(key, PyList::new(py, values.iter().cloned())?)?;
    }
    let df = polars.call_method1("DataFrame", (data,))?;
    Ok(df.unbind())
}

/// Reconstruct a PyArrow Table from unioned/null-filled columns. Only called
/// after `require_importable` has confirmed pyarrow is present.
#[cfg(feature = "python")]
fn reconstruct_pyarrow_normalise(
    py: Python<'_>,
    cols: &NormaliseColumns<'_>,
) -> PyResult<Py<PyAny>> {
    let pyarrow = py.import("pyarrow")?;
    let data = PyDict::new(py);
    for (key, values) in &cols.columns {
        data.set_item(key, PyList::new(py, values.iter().cloned())?)?;
    }
    let table = pyarrow
        .getattr("Table")?
        .call_method1("from_pydict", (data,))?;
    Ok(table.unbind())
}

/// Infer a single column's PySpark type from its first non-`None` value
/// (columns are already uniformly typed by this point -- see
/// `union_and_columnarize`'s list-collision-uniformity handling -- except for
/// genuinely mixed-type columns arising from different rows' same key holding
/// different JSON leaf types across documents, which is a pre-existing,
/// out-of-scope edge case shared with pandas/polars/pyarrow's own type
/// inference). An all-`None` column (no non-`None` value found) defaults to
/// `StringType`, matching the other three targets' own harmless defaults for
/// the same case (pandas `object`, polars `Null`, pyarrow `null`).
#[cfg(feature = "python")]
fn infer_spark_type<'py>(
    types_mod: &Bound<'py, PyModule>,
    values: &[Bound<'py, PyAny>],
) -> PyResult<Bound<'py, PyAny>> {
    for v in values {
        if v.is_none() {
            continue;
        }
        // bool before int: Python's bool is an int subclass.
        if v.is_instance_of::<pyo3::types::PyBool>() {
            return types_mod.getattr("BooleanType")?.call0();
        }
        if v.is_instance_of::<pyo3::types::PyInt>() {
            return types_mod.getattr("LongType")?.call0();
        }
        if v.is_instance_of::<pyo3::types::PyFloat>() {
            return types_mod.getattr("DoubleType")?.call0();
        }
        if v.is_instance_of::<PyList>() {
            let element_type = types_mod.getattr("StringType")?.call0()?;
            return types_mod.getattr("ArrayType")?.call1((element_type,));
        }
        // str, and anything else JSON can produce as a leaf -> StringType.
        return types_mod.getattr("StringType")?.call0();
    }
    types_mod.getattr("StringType")?.call0()
}

/// Reconstruct a real PySpark DataFrame by reusing the pandas reconstruction
/// above for the actual data, then handing it to Spark's own Arrow-optimized
/// `SparkSession.createDataFrame(pandas.DataFrame, schema)` bridge (Arrow
/// conversion on by default since Spark 3.0 via
/// `spark.sql.execution.arrow.pyspark.enabled`) -- the idiomatic, "native" way
/// to get driver-side tabular data into a real distributed DataFrame.
///
/// The schema is passed explicitly rather than left for Spark to infer, which
/// is not optional polish: confirmed empirically that inference is unreliable
/// on the *non*-Arrow fallback path Spark silently takes when pyarrow isn't
/// installed (a real, reachable configuration -- pyspark does not depend on
/// pyarrow). An all-`None` column corrupted to `StructType([])` (empty
/// struct) instead of a null string column, and separately, pandas's nullable
/// `"string"` dtype's `pd.NA` sentinel (an earlier version of this function
/// used it for exactly this all-`None` case) serialized as the *literal
/// string* `"<NA>"` on that same fallback path instead of a real null --
/// silent data corruption, not a crash, so it would not have been obvious
/// without checking actual cell values. Plain Python `None` plus an explicit
/// schema was verified correct on both the Arrow and non-Arrow paths, with
/// and without pyarrow installed, for both all-`None` and mixed-value
/// columns -- schema-driven construction sidesteps inference on either path
/// entirely rather than depending on either one behaving correctly.
///
/// Requires pandas importable (a safe assumption in any real PySpark
/// environment -- PySpark's own Arrow optimizations, and this project's
/// `pandas_udf` docs example, already assume it).
///
/// Used by both `execute_normalise` (`target="pyspark"`) and, since
/// github.com/amaye15/JSON-Tools-rs/issues/31, plain `execute_dataframe` for
/// PySpark input -- the latter no longer falls back to a plain list of dicts.
#[cfg(feature = "python")]
fn reconstruct_pyspark_normalise(
    py: Python<'_>,
    spark: &Bound<'_, PyAny>,
    cols: &NormaliseColumns<'_>,
) -> PyResult<Py<PyAny>> {
    let types_mod = py.import("pyspark.sql.types")?;

    if cols.columns.is_empty() {
        let empty_schema = types_mod.getattr("StructType")?.call0()?;
        let empty_rows: Vec<Py<PyAny>> = Vec::new();
        let df = spark.call_method1("createDataFrame", (empty_rows, empty_schema))?;
        return Ok(df.unbind());
    }

    py.import("pandas").map_err(|_| {
        JsonToolsError::new_err(
            "Reconstructing a PySpark DataFrame (via execute() or \
             normalise(target=\"pyspark\")) requires pandas to be installed -- it's \
             used internally to build the DataFrame handed to Spark's \
             Arrow-optimized createDataFrame() bridge",
        )
    })?;

    let struct_field_cls = types_mod.getattr("StructField")?;
    let mut fields: Vec<Bound<'_, PyAny>> = Vec::with_capacity(cols.columns.len());
    for (key, values) in &cols.columns {
        let field_type = infer_spark_type(&types_mod, values)?;
        fields.push(struct_field_cls.call1((key, field_type, true))?);
    }
    let schema = types_mod.getattr("StructType")?.call1((fields,))?;

    let pandas_df = reconstruct_pandas_normalise(py, cols)?;
    let df = spark.call_method1("createDataFrame", (pandas_df, schema))?;
    Ok(df.unbind())
}

// =============================================================================
// PyJSONTools Helper Methods (DataFrame and Series Processing)
// =============================================================================

#[cfg(feature = "python")]
impl PyJSONTools {
    /// Process DataFrame through existing pipeline
    fn execute_dataframe(
        &self,
        df: &Bound<'_, PyAny>,
        df_type: DataFrameType,
    ) -> PyResult<Py<PyAny>> {
        let py = df.py();

        // Step 1: Convert DataFrame directly to per-row JSON strings (native
        // to_json/write_ndjson where available -- see `dataframe_to_json_strings`'s
        // doc comment)
        let mut json_strings = dataframe_to_json_strings(df, df_type)?;

        // Step 1.5: In flatten mode only, auto-expand any column holding JSON
        // *strings* (not already dicts/structs) the same way a dict-typed column
        // already expands -- see `expand_json_string_columns`'s doc comment
        // (github.com/amaye15/JSON-Tools-rs/issues/30). Gated on flatten mode
        // specifically: `dataframe_to_json_strings`'s own contract is "native
        // per-row JSON text, unmodified," so `.normal()`/`.unflatten()` DataFrame
        // processing is intentionally untouched by this.
        if lock_config(&self.inner)?.is_flatten_mode() {
            json_strings = expand_json_string_columns(py, json_strings)?;
        }

        // Step 2: Process through existing pipeline (releases GIL)
        let result = py
            .detach(|| {
                let mut guard = lock_config(&self.inner)?;
                let tools = mem::take(&mut *guard);
                let result = tools.execute(json_strings); // Batch processing
                *guard = tools;
                result
            })
            .map_err(|e| JsonToolsError::new_err(format!("Failed to process DataFrame: {}", e)))?;

        // Step 3: Reconstruct DataFrame from results
        match result {
            JsonOutput::Multiple(processed_list) => match df_type {
                // PySpark gets a genuine, schema-driven Spark DataFrame back --
                // reuses the exact machinery built for normalise(target="pyspark")
                // (github.com/amaye15/JSON-Tools-rs/issues/31) instead of the old
                // dict-list-then-fallback-to-list behavior.
                DataFrameType::PySpark => {
                    let spark = require_active_spark_session(py)?;
                    let columns = union_and_columnarize(py, processed_list)?;
                    reconstruct_pyspark_normalise(py, &spark, &columns)
                }
                _ => {
                    // Convert JSON strings back to Python dicts
                    let mut processed_dicts: Vec<Py<PyAny>> =
                        Vec::with_capacity(processed_list.len());
                    for json_str in processed_list {
                        let py_dict = py_loads(py, &json_str).map_err(|e| {
                            JsonToolsError::new_err(format!("Failed to convert to Python: {}", e))
                        })?;
                        processed_dicts.push(py_dict.unbind());
                    }

                    // Reconstruct DataFrame (with fallback to list)
                    reconstruct_dataframe(py, df_type, processed_dicts)
                }
            },
            JsonOutput::Single(_) => Err(PyValueError::new_err(
                "Unexpected single result for DataFrame input",
            )),
        }
    }

    /// Process Series through existing list pipeline (REUSE existing code!)
    fn execute_series(
        &self,
        series: &Bound<'_, PyAny>,
        series_type: SeriesType,
    ) -> PyResult<Py<PyAny>> {
        let py = series.py();

        // Step 1: Convert Series to Python list
        let list = series_to_list(series, series_type)?;

        // Step 2: Process using EXISTING list handling code (copy from execute() method)
        let mut json_strings: Vec<String> = Vec::with_capacity(list.len());
        let mut is_str_flags: Vec<bool> = Vec::with_capacity(list.len());
        let mut has_other_types = false;

        for item in list.iter() {
            if let Ok(json_str) = item.extract::<String>() {
                json_strings.push(json_str);
                is_str_flags.push(true);
            } else if item.is_instance_of::<pyo3::types::PyDict>() {
                let json_str = py_dumps(py, &item).map_err(|e| {
                    JsonToolsError::new_err(format!("Failed to convert dict in series: {}", e))
                })?;
                json_strings.push(json_str);
                is_str_flags.push(false);
            } else {
                has_other_types = true;
                break;
            }
        }

        if has_other_types {
            return Err(PyValueError::new_err(
                "Series items must be either JSON strings or Python dictionaries",
            ));
        }

        // Step 3: Process through existing pipeline (releases GIL)
        let result = py
            .detach(|| {
                let mut guard = lock_config(&self.inner)?;
                let tools = mem::take(&mut *guard);
                let result = tools.execute(json_strings);
                *guard = tools;
                result
            })
            .map_err(|e| JsonToolsError::new_err(format!("Failed to process Series: {}", e)))?;

        // Step 4: Reconstruct Series from results
        match result {
            JsonOutput::Multiple(processed_list) => {
                // Type preservation: convert back to appropriate format
                let all_strings = is_str_flags.iter().all(|&b| b);
                let all_dicts = is_str_flags.iter().all(|&b| !b);

                let processed_items: Vec<Py<PyAny>> = if all_strings {
                    // All strings - convert to list of strings
                    processed_list
                        .into_iter()
                        .map(|s| {
                            s.into_pyobject(py)
                                .map(|o| o.into_any().unbind())
                                .map_err(|e| {
                                    JsonToolsError::new_err(format!(
                                        "Failed to convert string to Python object: {}",
                                        e
                                    ))
                                })
                        })
                        .collect::<PyResult<Vec<_>>>()?
                } else if all_dicts {
                    // All dicts - convert to list of dicts
                    let mut dict_results = Vec::with_capacity(processed_list.len());
                    for processed_json in processed_list {
                        let python_dict = py_loads(py, &processed_json).map_err(|e| {
                            JsonToolsError::new_err(format!(
                                "Failed to convert to Python dict: {}",
                                e
                            ))
                        })?;
                        dict_results.push(python_dict.unbind());
                    }
                    dict_results
                } else {
                    // Mixed results - convert each based on type
                    let mut mixed_results = Vec::with_capacity(processed_list.len());
                    for (processed_json, is_str) in processed_list.into_iter().zip(is_str_flags) {
                        if is_str {
                            mixed_results
                                .push(processed_json.into_pyobject(py)?.into_any().unbind());
                        } else {
                            let python_dict = py_loads(py, &processed_json).map_err(|e| {
                                JsonToolsError::new_err(format!(
                                    "Failed to convert to Python: {}",
                                    e
                                ))
                            })?;
                            mixed_results.push(python_dict.unbind());
                        }
                    }
                    mixed_results
                };

                // Reconstruct Series (with fallback to list)
                reconstruct_series(py, series_type, processed_items)
            }
            JsonOutput::Single(_) => Err(PyValueError::new_err(
                "Unexpected single result for Series input",
            )),
        }
    }

    /// Backing implementation for `execute(json_input, normalise=True, target=...)`.
    /// See the "Normalise" section above for the shared helpers this wires
    /// together.
    fn execute_normalise(
        &self,
        json_input: &Bound<'_, PyAny>,
        target: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let py = json_input.py();

        if !lock_config(&self.inner)?.is_flatten_mode() {
            return Err(JsonToolsError::new_err(
                "normalise=True requires .flatten() mode -- unflattened/nested JSON \
                 can't produce clean scalar columns for a wide DataFrame",
            ));
        }

        let requested_target = target.as_deref().map(NormaliseTarget::parse).transpose()?;

        let (json_strings, detected) = extract_normalise_json_strings(json_input)?;
        let resolved_target = resolve_normalise_target(py, detected, requested_target)?;

        // For pyspark, resolve the active session up front (before doing any
        // processing work) so a missing session fails fast with a clear error.
        let spark_session = if resolved_target == NormaliseTarget::PySpark {
            Some(require_active_spark_session(py)?)
        } else {
            require_importable(py, resolved_target)?;
            None
        };

        // Process through the Rust engine (releases GIL) -- same mem::take/restore
        // idiom `execute_dataframe`/`execute_series` above use to avoid cloning.
        let result = py
            .detach(|| {
                let mut guard = lock_config(&self.inner)?;
                let tools = mem::take(&mut *guard);
                let result = tools.execute(json_strings);
                *guard = tools;
                result
            })
            .map_err(|e| JsonToolsError::new_err(format!("Failed to process JSON: {}", e)))?;

        let processed_list = match result {
            JsonOutput::Multiple(processed_list) => processed_list,
            JsonOutput::Single(single) => vec![single],
        };

        let columns = union_and_columnarize(py, processed_list)?;

        match resolved_target {
            NormaliseTarget::Pandas => reconstruct_pandas_normalise(py, &columns),
            NormaliseTarget::Polars => reconstruct_polars_normalise(py, &columns),
            NormaliseTarget::PyArrow => reconstruct_pyarrow_normalise(py, &columns),
            NormaliseTarget::PySpark => {
                let spark = spark_session.expect("resolved to PySpark target above");
                reconstruct_pyspark_normalise(py, &spark, &columns)
            }
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
    m.add("JsonToolsError", m.py().get_type::<JsonToolsError>())?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "JSON Tools RS Contributors")?;
    m.add(
        "__description__",
        "Python bindings for JSON Tools RS - Unified JSON manipulation library with advanced collision handling and filtering",
    )?;

    Ok(())
}

#[cfg(all(test, feature = "python"))]
mod marshal_tests {
    use super::may_contain_big_int;

    /// Straightforward reference implementation: full scan for a digit run
    /// of >= 19. The sampled version must agree with this everywhere.
    fn naive(s: &[u8]) -> bool {
        let mut run = 0usize;
        for &b in s {
            if b.is_ascii_digit() {
                run += 1;
                if run >= 19 {
                    return true;
                }
            } else {
                run = 0;
            }
        }
        false
    }

    #[test]
    fn big_int_guard_edges() {
        assert!(!may_contain_big_int(b""));
        assert!(!may_contain_big_int(b"{}"));
        // 18 digits: largest run that must NOT trigger
        assert!(!may_contain_big_int(b"123456789012345678"));
        // 19 digits: must trigger, at string start, middle, and end
        assert!(may_contain_big_int(b"1234567890123456789"));
        assert!(may_contain_big_int(b"{\"big\": 1234567890123456789}"));
        assert!(may_contain_big_int(
            b"xxxxxxxxxxxxxxxxxxxxx1234567890123456789"
        ));
        // i64::MAX itself (19 digits) is conservatively flagged -- fine
        assert!(may_contain_big_int(b"9223372036854775807"));
        // runs split by separators never combine
        assert!(!may_contain_big_int(b"123456789.123456789e12"));
        // long digit runs inside strings count too (conservative by design)
        assert!(may_contain_big_int(b"{\"id\": \"12345678901234567890\"}"));
    }

    #[test]
    fn big_int_guard_matches_naive_scan() {
        // Deterministic xorshift; digit-heavy alphabet so long runs actually
        // occur, exercising run expansion straddling sample points.
        let mut state = 0x9e3779b97f4a7c15u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for len in [0usize, 1, 17, 18, 19, 20, 37, 38, 57, 200, 1000] {
            for _ in 0..500 {
                let bytes: Vec<u8> = (0..len)
                    .map(|_| match next() % 4 {
                        0 => b'a',
                        1 => b',',
                        _ => b'0' + (next() % 10) as u8,
                    })
                    .collect();
                assert_eq!(
                    may_contain_big_int(&bytes),
                    naive(&bytes),
                    "mismatch on {:?}",
                    String::from_utf8_lossy(&bytes)
                );
            }
        }
    }
}
