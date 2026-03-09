# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation with SIMD-accelerated parsing, Crossbeam-based parallelism, and native Python bindings with DataFrame/Series support.

[![Crates.io](https://img.shields.io/crates/v/json-tools-rs.svg)](https://crates.io/crates/json-tools-rs)
[![PyPI](https://img.shields.io/pypi/v/json-tools-rs.svg)](https://pypi.org/project/json-tools-rs/)
[![Documentation](https://docs.rs/json-tools-rs/badge.svg)](https://docs.rs/json-tools-rs)
[![Book](https://img.shields.io/badge/book-GitHub%20Pages-blue)](https://amaye15.github.io/JSON-Tools-rs/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/amaye15/JSON-Tools-rs/blob/master/LICENSE-MIT)

## Why JSON Tools RS?

JSON Tools RS is designed for developers who need to:

- **Transform nested JSON** into flat structures for databases, CSV exports, or analytics
- **Clean and normalize** JSON data from external APIs or user input
- **Process large batches** of JSON documents efficiently
- **Maintain type safety** with perfect roundtrip support (flatten -> unflatten -> original)
- **Work with both Rust and Python** using the same consistent API

## Key Features

- **Unified API** -- Single `JSONTools` entry point for flattening, unflattening, or pass-through transforms
- **Builder Pattern** -- Fluent, chainable API for configuration
- **High Performance** -- SIMD-accelerated parsing, FxHashMap, SmallVec stack allocation, tiered caching (~2,000+ ops/ms)
- **Parallel Processing** -- Crossbeam-based parallelism for 3-5x speedup on batch operations
- **Complete Roundtrip** -- Flatten and unflatten with perfect fidelity
- **Comprehensive Filtering** -- Remove empty strings, nulls, empty objects, empty arrays
- **Advanced Replacements** -- Literal and regex-based key/value replacements
- **Collision Handling** -- Collect colliding values into arrays
- **Automatic Type Conversion** -- Strings to numbers, booleans, dates, and nulls
- **Date Normalization** -- ISO-8601 detection and UTC normalization
- **Batch Processing** -- Single or batch JSON, dicts, lists, DataFrames, and Series
- **Python Bindings** -- Full Python support with perfect type preservation
- **DataFrame/Series Support** -- Pandas, Polars, PyArrow, and PySpark
- **Modular Architecture** -- 10 focused modules for maintainability with zero-overhead abstraction

## Quick Example

**Rust:**

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let result = JSONTools::new()
    .flatten()
    .execute(r#"{"user": {"name": "John", "age": 30}}"#)?;
// {"user.name": "John", "user.age": 30}
```

**Python:**

```python
import json_tools_rs as jt

result = jt.JSONTools().flatten().execute({"user": {"name": "John", "age": 30}})
# {'user.name': 'John', 'user.age': 30}
```
