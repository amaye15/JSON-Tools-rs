# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation with SIMD-accelerated parsing, including flattening and unflattening nested JSON structures.

## Features

- **🚀 Unified API**: Single `JSONTools` entry point for both flattening and unflattening operations
- **🔧 Builder Pattern**: Fluent, chainable API for easy configuration
- **⚡ High Performance**: SIMD-accelerated JSON parsing with optimized algorithms
- **🎯 Complete Roundtrip Support**: Flatten JSON and unflatten it back to original structure
- **🧹 Comprehensive Filtering**: Remove empty values, nulls, empty objects/arrays
- **🔄 Advanced Replacements**: Support for literal and regex-based key/value replacements
- **⚔️ Key Collision Handling**: Two strategies for handling key conflicts after transformations
  - **Avoid Collisions**: Append index suffixes to make keys unique
  - **Collect Values**: Merge colliding values into arrays with intelligent filtering
- **📦 Batch Processing**: Handle single JSON strings or arrays of JSON strings
- **🐍 Python Bindings**: Full Python support via maturin/PyO3
- **🛡️ Type Safety**: Compile-time checked with comprehensive error handling

## Quick Start

### Rust - Unified API (Recommended)

#### Flattening JSON

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .key_replacement("regex:(User|Admin)_", "")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(true)
    .remove_nulls(true)
    .remove_empty_objects(true)
    .remove_empty_arrays(true)
    .execute(json)?;

match result {
    JsonOutput::Single(flattened) => println!("{}", flattened),
    JsonOutput::Multiple(_) => unreachable!(),
}
// Output: {"user::name": "John"}
```

#### Unflattening JSON

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let flattened = r#"{"user::name": "John", "user::age": 30}"#;
let result = JSONTools::new()
    .unflatten()
    .separator("::")
    .lowercase_keys(true)
    .key_replacement("regex:(User|Admin)_", "")
    .value_replacement("@company.org", "@example.com")
    .remove_empty_strings(true)
    .remove_nulls(true)
    .remove_empty_objects(true)
    .remove_empty_arrays(true)
    .execute(flattened)?;

match result {
    JsonOutput::Single(unflattened) => println!("{}", unflattened),
    JsonOutput::Multiple(_) => unreachable!(),
}
// Output: {"user": {"name": "John", "age": 30}}
```

#### Roundtrip Example

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let original = r#"{"user": {"name": "John", "age": 30}, "items": [1, 2, {"nested": "value"}]}"#;

// Flatten
let flattened = JSONTools::new().flatten().execute(original)?.into_single();
// Unflatten back
let restored = JSONTools::new().unflatten().execute(&flattened)?.into_single();

// original and restored are equivalent JSON structures
assert_eq!(
    serde_json::from_str::<serde_json::Value>(original)?,
    serde_json::from_str::<serde_json::Value>(&restored)?
);
```



### Python

#### Flattening

```python
import json_tools_rs

# Perfect type matching - input type = output type!

# JSON string input → JSON string output
flattener = json_tools_rs.JsonFlattener()
result = flattener.flatten('{"user": {"name": "John", "age": 30}}')
print(result)  # '{"user.name": "John", "user.age": 30}' (str)

# Python dict input → Python dict output (much more convenient!)
result = flattener.flatten({"user": {"name": "John", "age": 30}})
print(result)  # {'user.name': 'John', 'user.age': 30} (dict)

# Advanced configuration
flattener = (json_tools_rs.JsonFlattener()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .separator("_")
    .lowercase_keys(True))

result = flattener.flatten({"User": {"Name": "John", "Email": ""}})
print(result)  # {'user_name': 'John'} (dict)
```

#### Unflattening

```python
import json_tools_rs

# JSON string input → JSON string output
unflattener = json_tools_rs.JsonUnflattener()
result = unflattener.unflatten('{"user.name": "John", "user.age": 30}')
print(result)  # '{"user": {"name": "John", "age": 30}}' (str)

# Python dict input → Python dict output
result = unflattener.unflatten({"user.name": "John", "items.0": "first", "items.1": "second"})
print(result)  # {'user': {'name': 'John'}, 'items': ['first', 'second']} (dict)

# Advanced configuration with builder pattern
unflattener = (json_tools_rs.JsonUnflattener()
    .separator("_")
    .lowercase_keys(True)
    .key_replacement("prefix_", "user_")
    .value_replacement("@company.org", "@example.com"))

result = unflattener.unflatten({"PREFIX_NAME": "john@company.org"})
print(result)  # {'user': {'name': 'john@example.com'}} (dict)

# Roundtrip example
original = {"user": {"name": "John", "age": 30}, "items": [1, 2, {"nested": "value"}]}
flattened = json_tools_rs.JsonFlattener().flatten(original)
restored = json_tools_rs.JsonUnflattener().unflatten(flattened)
assert original == restored  # Perfect roundtrip!
```

#### Batch Processing

```python
# Flattening: List[str] input → List[str] output
results = flattener.flatten(['{"a": 1}', '{"b": 2}'])
print(results)  # ['{"a": 1}', '{"b": 2}'] (list of strings)

# Flattening: List[dict] input → List[dict] output
results = flattener.flatten([{"a": 1}, {"b": 2}])
print(results)  # [{'a': 1}, {'b': 2}] (list of dicts)

# Unflattening: List[str] input → List[str] output
unflattener = json_tools_rs.JsonUnflattener()
results = unflattener.unflatten(['{"a.b": 1}', '{"c.d": 2}'])
print(results)  # ['{"a": {"b": 1}}', '{"c": {"d": 2}}'] (list of strings)

# Unflattening: List[dict] input → List[dict] output
results = unflattener.unflatten([{"a.b": 1}, {"c.d": 2}])
print(results)  # [{'a': {'b': 1}}, {'c': {'d': 2}}] (list of dicts)
```

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
json-tools-rs = "0.1.0"
```

### Python

Install from PyPI (when published):

```bash
pip install json-tools-rs
```

Or build from source:

```bash
git clone https://github.com/amaye15/JSON-Tools-rs.git
cd JSON-Tools-rs
maturin develop --features python
```

## Performance

JSON Tools RS delivers excellent performance across different workloads:

- **Basic flattening**: 2,000+ keys/ms
- **Advanced configuration**: 1,300+ keys/ms  
- **Regex replacements**: 1,800+ keys/ms
- **Batch processing**: 1,900+ keys/ms

## API Reference

### JsonFlattener

The main entry point for all JSON flattening operations. Provides a builder pattern API:

- `remove_empty_strings(bool)` - Remove keys with empty string values
- `remove_nulls(bool)` - Remove keys with null values
- `remove_empty_objects(bool)` - Remove keys with empty object values
- `remove_empty_arrays(bool)` - Remove keys with empty array values
- `key_replacement(find, replace)` - Add key replacement pattern
- `value_replacement(find, replace)` - Add value replacement pattern
- `separator(sep)` - Set separator for nested keys (default: ".")
- `lowercase_keys(bool)` - Convert all keys to lowercase
- `flatten(input)` - Flatten the JSON input

### JsonUnflattener

The companion to JsonFlattener that provides the inverse operation - converting flattened JSON back to nested JSON structure. Provides the same builder pattern API:

- `key_replacement(find, replace)` - Add key replacement pattern (applied before unflattening)
- `value_replacement(find, replace)` - Add value replacement pattern (applied before unflattening)
- `separator(sep)` - Set separator for nested keys (default: ".")
- `lowercase_keys(bool)` - Convert all keys to lowercase before processing
- `unflatten(input)` - Unflatten the JSON input

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
