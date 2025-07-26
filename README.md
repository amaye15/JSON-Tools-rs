# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation with SIMD-accelerated parsing, including flattening nested JSON structures.

## Features

- **Unified JsonFlattener API**: Single entry point for all JSON flattening operations
- **High Performance**: SIMD-accelerated JSON parsing with optimized algorithms
- **Builder Pattern**: Fluent, chainable API for easy configuration
- **Comprehensive Filtering**: Remove empty values, nulls, empty objects/arrays
- **Advanced Replacements**: Support for literal and regex-based key/value replacements
- **Batch Processing**: Handle single JSON strings or arrays of JSON strings
- **Python Bindings**: Full Python support via maturin/PyO3

## Quick Start

### Rust

```rust
use json_tools_rs::{JsonFlattener, JsonOutput};

let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
let result = JsonFlattener::new()
    .remove_empty_strings(true)
    .remove_nulls(true)
    .flatten(json)?;

match result {
    JsonOutput::Single(flattened) => println!("{}", flattened),
    JsonOutput::Multiple(_) => unreachable!(),
}
// Output: {"user.name": "John"}
```

### Python

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

# List[str] input → List[str] output
results = flattener.flatten(['{"a": 1}', '{"b": 2}'])
print(results)  # ['{"a": 1}', '{"b": 2}'] (list of strings)

# List[dict] input → List[dict] output
results = flattener.flatten([{"a": 1}, {"b": 2}])
print(results)  # [{'a': 1}, {'b': 2}] (list of dicts)

# Mixed types preserve original types
results = flattener.flatten(['{"a": 1}', {"b": 2}])
print(results)  # ['{"a": 1}', {'b': 2}] (mixed list)
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

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
