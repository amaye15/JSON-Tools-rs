# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation with SIMD-accelerated parsing, providing unified flattening and unflattening operations through a clean builder pattern API.

[![Crates.io](https://img.shields.io/crates/v/json-tools-rs.svg)](https://crates.io/crates/json-tools-rs)
[![Documentation](https://docs.rs/json-tools-rs/badge.svg)](https://docs.rs/json-tools-rs)
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
- ⚡ **High Performance**: SIMD-accelerated JSON parsing with FxHashMap and optimized memory allocations
- 🚄 **Parallel Processing**: Built-in Rayon-based parallelism for 3-5x speedup on batch operations (automatic, no configuration needed)
- 🎯 **Complete Roundtrip**: Flatten JSON and unflatten back to original structure with perfect fidelity
- 🧹 **Comprehensive Filtering**: Remove empty strings, nulls, empty objects, and empty arrays (works for both flatten and unflatten)
- 🔄 **Advanced Replacements**: Literal and regex-based key/value replacements using standard Rust regex syntax
- 🛡️ **Collision Handling**: Intelligent `.handle_key_collision(true)` to collect colliding values into arrays
- 📅 **Date Normalization**: Automatic detection and normalization of ISO-8601 dates to UTC
- 🔀 **Automatic Type Conversion**: Convert strings to numbers, booleans, and nulls with `.auto_convert_types(true)`
- 📦 **Batch Processing**: Process single JSON or batches; Python also supports dicts and lists of dicts
- 🐍 **Python Bindings**: Full Python support with perfect type preservation (input type = output type)

## Table of Contents

- [Why JSON Tools RS?](#why-json-tools-rs)
- [Features](#features)
- [Quick Start](#quick-start)
  - [Rust Examples](#rust---unified-jsontools-api)
  - [Python Examples](#python---unified-jsontools-api)
- [Quick Reference](#quick-reference)
- [Installation](#installation)
- [Performance](#performance)
- [API Reference](#api-reference)
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
    .key_replacement("(User|Admin)_", "")
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
| `.key_replacement(find, repl)` | Replace key patterns (regex) | `.key_replacement("user_", "")` |
| `.value_replacement(find, repl)` | Replace value patterns (regex) | `.value_replacement("@old.com", "@new.com")` |
| `.handle_key_collision(bool)` | Collect colliding keys into arrays | `.handle_key_collision(true)` |
| `.auto_convert_types(bool)` | Convert types (nums, bools, dates) | `.auto_convert_types(true)` |
| `.parallel_threshold(n)` | Min batch size for parallelism | `.parallel_threshold(500)` |
| `.num_threads(n)` | Number of threads (default: CPU count) | `.num_threads(Some(4))` |
| `.nested_parallel_threshold(n)` | Nested object parallelism size | `.nested_parallel_threshold(50)` |

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

## Performance

JSON Tools RS uses several techniques to achieve high performance (~2,000+ ops/ms):

* **SIMD-JSON**: Hardware-accelerated parsing.
* **FxHashMap**: Faster hashing for string keys.
* **SmallVec & Cow**: Stack allocation for short paths and zero-copy string handling.
* **First-Byte Filtering**: Rapid rejection of non-convertible strings during type conversion.
* **Parallelism**: Automatic Rayon-based parallelism for batches and large nested objects.

## Changelog

### v0.8.0 (Current)

* **Python Feature Parity**: Added `auto_convert_types`, `parallel_threshold`, `num_threads`, and `nested_parallel_threshold` to Python bindings.
* **Enhanced Type Conversion**: Added support for ISO-8601 dates, currency codes (USD, EUR), basis points (bps), and suffixed numbers (K/M/B).
* **Date Normalization**: Automatic detection and UTC normalization of date strings.
* **Performance**: Optimized string cleaning and number parsing logic.

### v0.7.0

* **Parallel Configuration**: Added methods to control parallel processing thresholds and thread counts.
* **Optimized Initialization**: Improved HashMap allocation strategies.

See [CHANGELOG.md](CHANGELOG.md) for full history.