# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation with SIMD-accelerated parsing, providing unified flattening and unflattening operations through a clean builder pattern API. Ships with Rust, Python, and JVM (Java/Spark) bindings.

[![PyPI](https://img.shields.io/pypi/v/json-tools-rs.svg)](https://pypi.org/project/json-tools-rs/)
[![Crates.io](https://img.shields.io/crates/v/json-tools-rs.svg)](https://crates.io/crates/json-tools-rs)
[![Maven Central](https://img.shields.io/maven-central/v/io.github.amaye15/json-tools-rs-spark.svg)](https://central.sonatype.com/artifact/io.github.amaye15/json-tools-rs-spark)
[![Documentation](https://docs.rs/json-tools-rs/badge.svg)](https://docs.rs/json-tools-rs)
[![Book](https://img.shields.io/badge/book-GitHub%20Pages-blue)](https://amaye15.github.io/JSON-Tools-rs/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

## Why JSON Tools RS?

JSON Tools RS is designed for developers who need to:
- **Transform nested JSON** into flat structures for databases, CSV exports, or analytics
- **Clean and normalize** JSON data from external APIs or user input
- **Process large batches** of JSON documents efficiently
- **Maintain type safety** with perfect roundtrip support (flatten → unflatten → original)
- **Work with both Rust and Python** using the same consistent API

Unlike simple JSON parsers, JSON Tools RS provides a complete toolkit for JSON transformation with production-ready performance and error handling.

## Features

- 🚀 **Unified API**: Single `JSONTools` entry point for flattening, unflattening, or pass-through transforms (`.normal()`)
- 🔧 **Builder Pattern**: Fluent, chainable API for easy configuration and method chaining
- ⚡ **High Performance**: SIMD-accelerated JSON parsing with FxHashMap, SmallVec stack allocation, and tiered caching
- 🚄 **Parallel Processing**: Built-in Rayon-based parallelism (persistent work-stealing pool) for 3-5x speedup on batch operations and large nested structures
- 🎯 **Complete Roundtrip**: Flatten JSON and unflatten back to original structure with perfect fidelity
- 🧹 **Comprehensive Filtering**: Remove empty strings, nulls, empty objects, and empty arrays (works for both flatten and unflatten)
- 🔄 **Advanced Replacements**: Key/value replacements, literal (exact substring match) by default, or regex by wrapping the pattern in `r'...'`
- 🛡️ **Collision Handling**: Intelligent `.handle_key_collision(true)` to collect colliding values into arrays
- 📅 **Date Normalization**: Automatic detection and normalization of ISO-8601 dates to UTC
- 🔀 **Automatic Type Conversion**: Convert strings to numbers, booleans, and nulls with `.auto_convert_types(true)`
- 📦 **Batch Processing**: Process single JSON or batches; Python also supports dicts and lists of dicts
- 🐍 **Python Bindings**: Full Python support with perfect type preservation (input type = output type)
- 📊 **DataFrame/Series Support**: Native support for Pandas, Polars, PyArrow, and PySpark DataFrames and Series in Python
- ☕ **JVM Bindings**: Java/Spark UDFs (row and batched `mapPartitions` tiers) for Databricks Jobs/notebooks on classic compute and other Spark workloads -- see [`jvm/README.md`](jvm/README.md)

## Table of Contents

- [Why JSON Tools RS?](#why-json-tools-rs)
- [Features](#features)
- [Quick Start](#quick-start)
  - [Rust Examples](#rust---unified-jsontools-api)
  - [Python Examples](#python---unified-jsontools-api)
  - [JVM / Spark Examples](#jvm--spark)
- [Quick Reference](#quick-reference)
- [Installation](#installation)
- [Architecture](#architecture)
- [Performance](#performance)
- [Contributing](#contributing)
- [License](#license)
- [Changelog](#changelog)

## Quick Start

### Rust - Unified JSONTools API

The `JSONTools` struct provides a unified builder pattern API for all JSON manipulation operations. Simply call `.flatten()` or `.unflatten()` to set the operation mode, then chain configuration methods and call `.execute()`.

#### Basic Flattening

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user": {"name": "John", "profile": {"age": 30, "city": "NYC"}}}"#;
let result = JSONTools::new()
    .flatten()
    .execute(json)?;

if let JsonOutput::Single(flattened) = result {
    println!("{}", flattened);
}
// Output: {"user.name": "John", "user.profile.age": 30, "user.profile.city": "NYC"}

```

#### Advanced Flattening with Filtering

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .key_replacement("r'(User|Admin)_'", "")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(true)
    .remove_nulls(true)
    .remove_empty_objects(true)
    .remove_empty_arrays(true)
    .execute(json)?;

if let JsonOutput::Single(flattened) = result {
    println!("{}", flattened);
}
// Output: {"user::name": "John"}

```

#### Automatic Type Conversion

Convert string values to numbers, booleans, dates, and null automatically for data cleaning and normalization.

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{
    "id": "123",
    "price": "$1,234.56",
    "discount": "15%",
    "active": "yes",
    "verified": "1",
    "created": "2024-01-15T10:30:00+05:00",
    "status": "N/A"
}"#;

let result = JSONTools::new()
    .flatten()
    .auto_convert_types(true)
    .execute(json)?;

if let JsonOutput::Single(flattened) = result {
    println!("{}", flattened);
}
// Output: {
//   "id": 123,
//   "price": 1234.56,
//   "discount": 15.0,
//   "active": true,
//   "verified": 1,
//   "created": "2024-01-15T05:30:00Z", // Normalized to UTC
//   "status": null
// }

```

### Python - Unified JSONTools API

The Python bindings provide the same unified `JSONTools` API with **perfect type matching**: input type equals output type.

#### Basic Usage

```python
import json_tools_rs as jt

# Basic flattening - dict input → dict output
result = jt.JSONTools().flatten().execute({"user": {"name": "John", "age": 30}})
print(result)  # {'user.name': 'John', 'user.age': 30}

# Basic unflattening - dict input → dict output
result = jt.JSONTools().unflatten().execute({"user.name": "John", "user.age": 30})
print(result)  # {'user': {'name': 'John', 'age': 30}}

```

#### Advanced Configuration & Parallelism

```python
import json_tools_rs as jt

# Configure tools with parallel processing settings
tools = (jt.JSONTools()
    .flatten()
    .separator("::")
    .lowercase_keys(True)
    .remove_empty_strings(True)
    .parallel_threshold(50)       # Parallelize batches >= 50 items
    .num_threads(4)               # Use 4 threads
    .nested_parallel_threshold(200) # Parallelize large objects
)

# Process a batch of data
batch = [{"data": i} for i in range(100)]
results = tools.execute(batch)

```

#### DataFrame & Series Support

```python
import json_tools_rs as jt
import pandas as pd

# Pandas DataFrame input → Pandas DataFrame output
df = pd.DataFrame([
    {"user": {"name": "Alice", "age": 30}},
    {"user": {"name": "Bob", "age": 25}},
])
result = jt.JSONTools().flatten().execute(df)
print(type(result))  # <class 'pandas.core.frame.DataFrame'>

# Also works with Polars, PyArrow Tables, and PySpark DataFrames
# Series input → Series output (Pandas, Polars, PyArrow)
```

### JVM / Spark

JNI-based Java bindings, mirroring the same `JSONTools` builder, for use as Apache
Spark UDFs -- a simple row UDF and a higher-throughput batched `mapPartitions`
transform. Built for Databricks Jobs/notebooks on classic compute and other Spark
workloads (**not** usable inside a Databricks Lakeflow Declarative Pipeline --
Databricks doesn't permit JVM libraries on pipeline compute at all; use the Python
bindings above, wrapped in a `pandas_udf`, for that case instead).

```java
import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;

try (JsonToolsHandle tools = JsonTools.builder()
        .flatten()
        .separator("::")
        .keyReplacement("r'^admin_'", "")
        .removeNulls(true)
        .build()) {
    String result = tools.execute("{\"admin_name\": \"Jane\", \"age\": null}");
    // {"name":"Jane"}
}
```

See [`jvm/README.md`](jvm/README.md) for the Spark UDF API and
[Setting Up on Databricks](https://amaye15.github.io/JSON-Tools-rs/guide/databricks-setup.html)
for the full deployment walkthrough (both this and the pandas_udf path).

## Quick Reference

### Method Cheat Sheet

| Method | Description | Example |
| --- | --- | --- |
| `.flatten()` | Set operation mode to flatten | `JSONTools::new().flatten()` |
| `.unflatten()` | Set operation mode to unflatten | `JSONTools::new().unflatten()` |
| `.normal()` | Set mode to pass-through (transform only) | `JSONTools::new().normal()` |
| `.separator(sep)` | Set key separator (default: `"."`) | `.separator("::")` |
| `.lowercase_keys(bool)` | Convert keys to lowercase | `.lowercase_keys(true)` |
| `.remove_empty_strings(bool)` | Remove empty string values | `.remove_empty_strings(true)` |
| `.remove_nulls(bool)` | Remove null values | `.remove_nulls(true)` |
| `.remove_empty_objects(bool)` | Remove empty objects `{}` | `.remove_empty_objects(true)` |
| `.remove_empty_arrays(bool)` | Remove empty arrays `[]` | `.remove_empty_arrays(true)` |
| `.key_replacement(find, repl)` | Replace key patterns (literal, or regex via `r'...'`) | `.key_replacement("r'user_'", "")` |
| `.value_replacement(find, repl)` | Replace value patterns (literal, or regex via `r'...'`) | `.value_replacement("@old.com", "@new.com")` |
| `.handle_key_collision(bool)` | Collect colliding keys into arrays | `.handle_key_collision(true)` |
| `.auto_convert_types(bool)` | Convert types (nums, bools, dates) | `.auto_convert_types(true)` |
| `.parallel_threshold(n)` | Min batch size for parallelism | `.parallel_threshold(500)` |
| `.num_threads(n)` | Number of threads (default: CPU count) | `.num_threads(Some(4))` |
| `.nested_parallel_threshold(n)` | Nested object parallelism size | `.nested_parallel_threshold(50)` |
| `.max_array_index(n)` | Max array index for unflatten (DoS protection) | `.max_array_index(100_000)` |

## Automatic Type Conversion

When `.auto_convert_types(true)` is enabled, the library performs smart parsing on string values:

1. **Date & Time (ISO-8601)**:
* Detects date strings to avoid converting them to numbers (e.g., "2024-01-01").
* Normalizes datetimes to UTC.
* Supports offsets (`+05:00`), Z suffix, and naive datetimes.


2. **Numbers**:
* **Basic**: `"123"` → `123`, `"45.67"` → `45.67`
* **Separators**: `"1,234.56"` (US), `"1.234,56"` (EU), `"1 234.56"` (Space)
* **Currency**: `"$123"`, `"€99"`, `"£50"`, `"¥1000"`, `"R$50"`
* **Scientific**: `"1e5"` → `100000`
* **Percentages**: `"50%"` → `50.0`, `"12.5%"` → `12.5`
* **Basis Points**: `"50bps"` → `0.005`, `"100 bp"` → `0.01`
* **Suffixes**: `"1K"`, `"2.5M"`, `"5B"` (Thousand, Million, Billion)


3. **Booleans**:
* `"true"`, `"false"`, `"yes"`, `"no"`, `"on"`, `"off"`, `"y"`, `"n"` (case-insensitive).
* *Note*: `"1"` and `"0"` are treated as numbers, not booleans.


4. **Nulls**:
* `"null"`, `"nil"`, `"none"`, `"N/A"` (case-insensitive) → `null`.



## Installation

### Rust

```bash
cargo add json-tools-rs

```

### Python

```bash
pip install json-tools-rs

```

### JVM / Spark

No published artifact yet -- build from source (`cargo build --release --features
jvm && cd jvm && mvn package`), or download the jar from a `jvm-ci.yml` CI run. See
[`jvm/README.md`](jvm/README.md) for details; a Maven Central release
(`io.github.amaye15:json-tools-rs-spark`) ships automatically on tagged releases.

## Architecture

The codebase is organized into focused, single-responsibility modules:

```
src/
├── lib.rs            Facade: mod declarations + pub use re-exports
├── json_parser.rs    Conditional SIMD parser (sonic-rs on 64-bit, simd-json on 32-bit)
├── types.rs          Core types: JsonInput, JsonOutput
├── error.rs          Error types with codes E001-E008
├── config.rs         Configuration structs and operation modes
├── cache.rs          Tiered caching: regex, key deduplication, phf perfect hash
├── convert.rs        Type conversion: numbers, dates, booleans, nulls (SIMD-optimized)
├── transform.rs      Filtering, key/value replacements, collision handling
├── flatten.rs        Flattening algorithm with Rayon parallelism
├── unflatten.rs      Unflattening with SIMD separator detection
├── builder.rs        Public JSONTools builder API and execute() entry point
├── python.rs         Python bindings via PyO3
├── jvm.rs            JVM bindings via JNI (Java/Spark UDFs, see jvm/)
├── tests.rs          Unit tests
└── main.rs           CLI examples
```

The processing pipeline:
1. **Parse** -- SIMD-accelerated JSON parsing (`json_parser`)
2. **Flatten/Unflatten** -- Recursive traversal with Arc\<str\> key dedup (`flatten`/`unflatten`)
3. **Transform** -- Lowercase, replacements (cached regex), collision handling (`transform`)
4. **Filter** -- Remove empty strings, nulls, empty objects/arrays (`transform`)
5. **Convert** -- Type conversion with first-byte discriminators (`convert`)
6. **Serialize** -- Output to JSON string or native Python types

## Performance

### Benchmark Results

| Benchmark | Time | Description |
|-----------|------|-------------|
| Deep nesting (100 levels) | 8.3 µs | Deeply nested JSON objects |
| Wide objects (1,000 keys) | ~337 µs | Flat objects with many keys |
| Large arrays (5,000 items) | ~2.11 ms | Arrays with many elements |
| Parallel batch (10,000 items) | ~2.61 ms | Batch processing with Rayon |

*Measured on Apple Silicon. Results may vary by platform and data shape.*

### Optimization Techniques

JSON Tools RS uses several techniques to achieve high performance (~2,000+ ops/ms):

* **SIMD-JSON**: Hardware-accelerated parsing via sonic-rs (64-bit) / simd-json (32-bit).
* **SIMD Byte Search**: memchr/memmem for SIMD-accelerated string operations and pattern matching.
* **FxHashMap**: Faster hashing for string keys with rustc-hash.
* **Tiered Caching**: Three-level regex cache (compile-time phf table → thread-local FxHashMap → global `RwLock<FxHashMap>`).
* **SmallVec & Cow**: Stack allocation for depth stacks and number buffers; zero-copy string handling.
* **Arc\<str\> Deduplication**: Shared key storage to minimize allocations in wide/deep JSON.
* **First-Byte Discriminators**: Rapid rejection of non-convertible strings during type conversion.
* **Parallelism**: Rayon's persistent work-stealing thread pool for batch processing and large nested structures (avoids per-call OS thread spawn cost).

## CLI Demo

The crate includes an educational demo binary that showcases library features:

```bash
cargo run
```

This prints progressive examples covering basic flattening, unflattening, custom separators, filtering, replacements, collision handling, type conversion, and batch processing.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, testing, benchmarking, and PR guidelines.

## Changelog

### v0.9.3 (Current)

* **Bug fix**: `flatten` produced invalid JSON for any key containing an escaped character (`\"`, `\\`, control chars) when no key transform was configured -- the default, most common usage.
* **Bug fix**: re-escaping corrupted multi-byte UTF-8 characters (e.g. `café "quoted"` became `cafÃ© \"quoted\"`) whenever a string needed escaping and also contained non-ASCII text -- affected key escaping under `lowercase_keys`/`key_replacement`/collision-handling, value escaping under `value_replacement`, and `unflatten`'s key serialization.
* **Performance**: JSON object keys now use `CompactString` instead of `String` (inlines keys up to 24 bytes, no heap allocation) -- `unflatten` is ~19-22% faster (Criterion). Redundant separator re-scan eliminated in `unflatten`'s tree-building. Regex pattern cache now evicts genuinely least-recently-used entries instead of arbitrary ones. Key/value re-escaping is ~17-22% faster as a side effect of the UTF-8 corruption fix above.

See [CHANGELOG.md](CHANGELOG.md) for full details on all of the above.

### v0.9.2

*(`v0.9.1` was tagged a day earlier but only completed publishing to Maven Central --
a crates.io/PyPI release pipeline bug caused those two to fail before any upload.
Fixed and re-cut as v0.9.2 across all three registries; no code changes beyond the
release pipeline fix itself.)*

* **JVM (Java) bindings** (BREAKING for `key_replacement`/`value_replacement`, see below): new Spark UDF bindings (`jvm/`) with full feature parity, via a JNI shim over the same Rust core -- see [`jvm/README.md`](jvm/README.md).
* **`key_replacement`/`value_replacement` pattern syntax (BREAKING)**: patterns are now literal (exact substring match) by default; wrap in `r'...'` (e.g. `r'^admin_'`) for regex. Previously every pattern was always compiled as regex.
* **Rayon parallelism**: batch processing switched back from `std::thread::scope` (per-call OS thread spawn) to Rayon's persistent work-stealing pool -- measurably faster for small-to-medium batches.
* **`has_escape` scanner bug fix**: escape sequences not adjacent to a quote (`\n`, `\t`, `\r`, `\uXXXX`) were previously invisible to the tape scanner, silently skipping `auto_convert_types`/replacements/`lowercase_keys` for affected strings.
* **crates.io and Maven Central publishing** enabled on tagged releases.

See [CHANGELOG.md](CHANGELOG.md) for full details on all of the above.

### v0.9.0

* **Crossbeam Parallelism**: Migrated from Rayon to Crossbeam for finer-grained parallel control.
* **DataFrame/Series Support**: Native Python support for Pandas, Polars, PyArrow, and PySpark DataFrames and Series.
* **Modular Architecture**: Refactored into 10 focused modules for maintainability (zero API changes).
* **Performance Optimizations**: Eliminated per-entry HashMap in parallel flatten, early-exit discriminators, SIMD literal fallback, thread-local regex cache half-eviction, vectorized `clean_number_string()`.
* **Python Binding Optimizations**: `mem::take` for zero-cost builder mutations, O(1) DataFrame/Series reconstruction.

### v0.8.0

* **Python Feature Parity**: Added `auto_convert_types`, `parallel_threshold`, `num_threads`, and `nested_parallel_threshold` to Python bindings.
* **Enhanced Type Conversion**: Added support for ISO-8601 dates, currency codes (USD, EUR), basis points (bps), and suffixed numbers (K/M/B).
* **Date Normalization**: Automatic detection and UTC normalization of date strings.

See [CHANGELOG.md](CHANGELOG.md) for full history.