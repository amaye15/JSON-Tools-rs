# JSON Tools RS

A high-performance Rust library for advanced JSON manipulation, including flattening nested JSON structures with configurable filtering and replacement options. Now with **Python bindings** for easy integration into Python projects!

https://github.com/amaye15/JSON-Tools-rs.git

## 🚀 Performance

This library uses **simd-json** for ultra-fast JSON parsing, providing significant performance improvements over standard JSON libraries:

- **SIMD-accelerated parsing**: Leverages CPU SIMD instructions for faster JSON processing
- **Zero-copy operations**: Minimizes memory allocations during parsing
- **Optimized flattening**: Custom algorithms designed for high-throughput JSON transformation
- **Benchmark results**: Processes over 1,600 keys per millisecond on modern hardware

Perfect for high-volume data processing, ETL pipelines, and real-time JSON transformation tasks.

# Comprehensive Guide to `flatten_json` Function

This guide demonstrates all the different ways to use the `flatten_json` function in both Rust and Python implementations, including basic usage, configuration options, data type handling, error handling, and advanced features.

## Table of Contents
1. [Function Signatures](#function-signatures)
2. [Basic Usage Examples](#basic-usage-examples)
3. [Configuration Options](#configuration-options)
4. [Data Type Handling](#data-type-handling)
5. [Error Handling](#error-handling)
6. [Advanced Features](#advanced-features)
7. [Input/Output Examples](#inputoutput-examples)
8. [Differences Between Rust and Python](#differences-between-rust-and-python)

## Function Signatures

### Rust
```rust
use json_tools_rs::{flatten_json, JsonOutput, FlattenError};

pub fn flatten_json<'a, T>(
    json_input: T,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
    key_replacements: Option<Vec<(String, String)>>,
    value_replacements: Option<Vec<(String, String)>>,
    separator: Option<&str>,
) -> Result<JsonOutput, Box<dyn Error>>
where
    T: Into<JsonInput<'a>>,
```

### Python
```python
from json_tools_rs import flatten_json, JsonOutput, JsonFlattenError
from typing import Union, List, Optional, Tuple

def flatten_json(
    json_input: Union[str, List[str]],
    remove_empty_string_values: bool = False,
    remove_null_values: bool = False,
    remove_empty_dict: bool = False,
    remove_empty_list: bool = False,
    key_replacements: Optional[List[Tuple[str, str]]] = None,
    value_replacements: Optional[List[Tuple[str, str]]] = None,
    separator: Optional[str] = None,
) -> JsonOutput
```

## Basic Usage Examples

### 1. Simple Object Flattening

#### Rust
```rust
use json_tools_rs::{flatten_json, JsonOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = flatten_json(json, false, false, false, false, None, None, None)?;
    
    match result {
        JsonOutput::Single(flattened) => println!("Result: {}", flattened),
        JsonOutput::Multiple(_) => unreachable!(),
    }
    // Output: {"user.name": "John", "user.age": 30}
    
    Ok(())
}
```

#### Python
```python
from json_tools_rs import flatten_json
import json

# Basic flattening
json_str = '{"user": {"name": "John", "age": 30}}'
result = flatten_json(json_str)

flattened = result.get_single()
parsed = json.loads(flattened)
print(parsed)
# Output: {'user.name': 'John', 'user.age': 30}
```

### 2. Array Flattening

#### Rust
```rust
let json = r#"{"items": [1, 2, {"nested": "value"}]}"#;
let result = flatten_json(json, false, false, false, false, None, None, None)?;
// Result: {"items.0": 1, "items.1": 2, "items.2.nested": "value"}
```

#### Python
```python
json_str = '{"items": [1, 2, {"nested": "value"}]}'
result = flatten_json(json_str)
flattened = result.get_single()
# Result: {"items.0": 1, "items.1": 2, "items.2.nested": "value"}
```

### 3. Matrix (Nested Arrays) Flattening

#### Rust
```rust
let json = r#"{"matrix": [[1, 2], [3, 4]]}"#;
let result = flatten_json(json, false, false, false, false, None, None, None)?;
// Result: {"matrix.0.0": 1, "matrix.0.1": 2, "matrix.1.0": 3, "matrix.1.1": 4}
```

#### Python
```python
json_str = '{"matrix": [[1, 2], [3, 4]]}'
result = flatten_json(json_str)
# Result: {"matrix.0.0": 1, "matrix.0.1": 2, "matrix.1.0": 3, "matrix.1.1": 4}
```

## Configuration Options

### 1. Custom Separator

#### Rust
```rust
// Using underscore separator
let json = r#"{"user": {"profile": {"name": "John"}}}"#;
let result = flatten_json(json, false, false, false, false, None, None, Some("_"))?;
// Result: {"user_profile_name": "John"}

// Using double colon separator
let result = flatten_json(json, false, false, false, false, None, None, Some("::"))?;
// Result: {"user::profile::name": "John"}
```

#### Python
```python
# Using underscore separator
json_str = '{"user": {"profile": {"name": "John"}}}'
result = flatten_json(json_str, separator="_")
# Result: {"user_profile_name": "John"}

# Using double colon separator
result = flatten_json(json_str, separator="::")
# Result: {"user::profile::name": "John"}
```

### 2. Filtering Options

#### Remove Empty String Values

##### Rust
```rust
let json = r#"{"name": "John", "email": "", "city": "NYC"}"#;
let result = flatten_json(json, true, false, false, false, None, None, None)?;
// Result: {"name": "John", "city": "NYC"} (email removed)
```

##### Python
```python
json_str = '{"name": "John", "email": "", "city": "NYC"}'
result = flatten_json(json_str, remove_empty_string_values=True)
# Result: {"name": "John", "city": "NYC"} (email removed)
```

#### Remove Null Values

##### Rust
```rust
let json = r#"{"name": "John", "age": null, "city": "NYC"}"#;
let result = flatten_json(json, false, true, false, false, None, None, None)?;
// Result: {"name": "John", "city": "NYC"} (age removed)
```

##### Python
```python
json_str = '{"name": "John", "age": null, "city": "NYC"}'
result = flatten_json(json_str, remove_null_values=True)
# Result: {"name": "John", "city": "NYC"} (age removed)
```

#### Remove Empty Objects

##### Rust
```rust
let json = r#"{"user": {"name": "John"}, "metadata": {}, "settings": {"theme": "dark"}}"#;
let result = flatten_json(json, false, false, true, false, None, None, None)?;
// Result: {"user.name": "John", "settings.theme": "dark"} (metadata removed)
```

##### Python
```python
json_str = '{"user": {"name": "John"}, "metadata": {}, "settings": {"theme": "dark"}}'
result = flatten_json(json_str, remove_empty_dict=True)
# Result: {"user.name": "John", "settings.theme": "dark"} (metadata removed)
```

#### Remove Empty Arrays

##### Rust
```rust
let json = r#"{"items": [1, 2], "empty": [], "tags": ["tag1"]}"#;
let result = flatten_json(json, false, false, false, true, None, None, None)?;
// Result: {"items.0": 1, "items.1": 2, "tags.0": "tag1"} (empty removed)
```

##### Python
```python
json_str = '{"items": [1, 2], "empty": [], "tags": ["tag1"]}'
result = flatten_json(json_str, remove_empty_list=True)
# Result: {"items.0": 1, "items.1": 2, "tags.0": "tag1"} (empty removed)
```

#### Combined Filtering

##### Rust
```rust
let json = r#"{"user": {"name": "John", "email": "", "age": null}, "metadata": {}, "items": []}"#;
let result = flatten_json(json, true, true, true, true, None, None, None)?;
// Result: {"user.name": "John"} (all empty values removed)
```

##### Python
```python
json_str = '{"user": {"name": "John", "email": "", "age": null}, "metadata": {}, "items": []}'
result = flatten_json(
    json_str,
    remove_empty_string_values=True,
    remove_null_values=True,
    remove_empty_dict=True,
    remove_empty_list=True
)
# Result: {"user.name": "John"} (all empty values removed)
```

### 3. Key and Value Replacements

#### Literal String Replacements

##### Rust
```rust
// Key replacements
let json = r#"{"user_name": "John", "user_email": "john@example.com"}"#;
let key_replacements = Some(vec![("user_".to_string(), "person_".to_string())]);
let result = flatten_json(json, false, false, false, false, key_replacements, None, None)?;
// Result: {"person_name": "John", "person_email": "john@example.com"}

// Value replacements
let json = r#"{"email": "john@example.com", "backup_email": "john@example.com"}"#;
let value_replacements = Some(vec![("@example.com".to_string(), "@company.org".to_string())]);
let result = flatten_json(json, false, false, false, false, None, value_replacements, None)?;
// Result: {"email": "john@company.org", "backup_email": "john@company.org"}
```

##### Python
```python
# Key replacements
json_str = '{"user_name": "John", "user_email": "john@example.com"}'
key_replacements = [("user_", "person_")]
result = flatten_json(json_str, key_replacements=key_replacements)
# Result: {"person_name": "John", "person_email": "john@example.com"}

# Value replacements
json_str = '{"email": "john@example.com", "backup_email": "john@example.com"}'
value_replacements = [("@example.com", "@company.org")]
result = flatten_json(json_str, value_replacements=value_replacements)
# Result: {"email": "john@company.org", "backup_email": "john@company.org"}
```

#### Regex Pattern Replacements

##### Rust
```rust
// Regex key replacements
let json = r#"{"user_name": "John", "admin_role": "super"}"#;
let key_replacements = Some(vec![("regex:^(user|admin)_".to_string(), "".to_string())]);
let result = flatten_json(json, false, false, false, false, key_replacements, None, None)?;
// Result: {"name": "John", "role": "super"}

// Regex value replacements with capture groups
let json = r#"{"phone": "+1-555-123-4567"}"#;
let value_replacements = Some(vec![
    ("regex:\\+(\\d)-(\\d{3})-(\\d{3})-(\\d{4})".to_string(), "($2) $3-$4".to_string())
]);
let result = flatten_json(json, false, false, false, false, None, value_replacements, None)?;
// Result: {"phone": "(555) 123-4567"}
```

##### Python
```python
# Regex key replacements
json_str = '{"user_name": "John", "admin_role": "super"}'
key_replacements = [("regex:^(user|admin)_", "")]
result = flatten_json(json_str, key_replacements=key_replacements)
# Result: {"name": "John", "role": "super"}

# Regex value replacements
json_str = '{"email": "user@example.com", "backup": "admin@example.com"}'
value_replacements = [("regex:@example\\.com", "@company.org")]
result = flatten_json(json_str, value_replacements=value_replacements)
# Result: {"email": "user@company.org", "backup": "admin@company.org"}
```

## Data Type Handling

### 1. Different JSON Value Types

#### Rust
```rust
let json = r#"{
    "string": "hello",
    "number": 42,
    "float": 3.14,
    "boolean": true,
    "null_value": null,
    "array": [1, "two", true],
    "object": {"nested": "value"}
}"#;
let result = flatten_json(json, false, false, false, false, None, None, None)?;
// Result: {
//   "string": "hello",
//   "number": 42,
//   "float": 3.14,
//   "boolean": true,
//   "null_value": null,
//   "array.0": 1,
//   "array.1": "two",
//   "array.2": true,
//   "object.nested": "value"
// }
```

#### Python
```python
json_str = '''{
    "string": "hello",
    "number": 42,
    "float": 3.14,
    "boolean": true,
    "null_value": null,
    "array": [1, "two", true],
    "object": {"nested": "value"}
}'''
result = flatten_json(json_str)
# Same result as Rust example
```

### 2. Multiple JSON Strings (Batch Processing)

#### Rust
```rust
let json_list = vec![
    r#"{"user1": {"name": "Alice", "age": 25}}"#,
    r#"{"user2": {"name": "Bob", "age": 30}}"#,
];
let result = flatten_json(&json_list[..], false, false, false, false, None, None, None)?;

match result {
    JsonOutput::Multiple(results) => {
        for (i, flattened) in results.iter().enumerate() {
            println!("Result {}: {}", i, flattened);
        }
    },
    JsonOutput::Single(_) => unreachable!(),
}
// Results: [
//   {"user1.name": "Alice", "user1.age": 25},
//   {"user2.name": "Bob", "user2.age": 30}
// ]
```

#### Python
```python
json_list = [
    '{"user1": {"name": "Alice", "age": 25}}',
    '{"user2": {"name": "Bob", "age": 30}}'
]
result = flatten_json(json_list)

assert result.is_multiple
results = result.get_multiple()
# Results: [
#   '{"user1.name": "Alice", "user1.age": 25}',
#   '{"user2.name": "Bob", "user2.age": 30}'
# ]
```

## Error Handling

### Rust Error Types

```rust
use json_tools_rs::{flatten_json, FlattenError};

// Handle different error types
let invalid_json = r#"{"invalid": json}"#;
match flatten_json(invalid_json, false, false, false, false, None, None, None) {
    Ok(result) => println!("Success: {:?}", result),
    Err(e) => {
        if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
            match flatten_err {
                FlattenError::JsonParseError(json_err) => {
                    eprintln!("JSON parsing failed: {}", json_err);
                },
                FlattenError::RegexError(regex_err) => {
                    eprintln!("Regex compilation failed: {}", regex_err);
                },
                FlattenError::InvalidReplacementPattern(msg) => {
                    eprintln!("Invalid replacement pattern: {}", msg);
                },
                FlattenError::BatchError { index, error } => {
                    eprintln!("Error processing JSON at index {}: {}", index, error);
                },
            }
        }
    }
}

// Invalid regex pattern
let key_replacements = Some(vec![("regex:[invalid".to_string(), "replacement".to_string())]);
let result = flatten_json(r#"{"test": "value"}"#, false, false, false, false, key_replacements, None, None);
assert!(result.is_err()); // Will be RegexError
```

### Python Error Handling

```python
from json_tools_rs import flatten_json, JsonFlattenError

# Handle JSON parsing errors
try:
    result = flatten_json('{"invalid": json}')
except JsonFlattenError as e:
    print(f"JSON flattening failed: {e}")
except ValueError as e:
    print(f"Invalid arguments: {e}")

# Handle invalid input types
try:
    result = flatten_json(123)  # Invalid type
except ValueError as e:
    print(f"Invalid input type: {e}")

# Handle JsonOutput method errors
json_list = ['{"a": 1}', '{"b": 2}']
result = flatten_json(json_list)

try:
    single_result = result.get_single()  # Error: multiple results
except ValueError as e:
    print(f"Method call error: {e}")

# Correct usage
multiple_results = result.get_multiple()
print(f"Multiple results: {multiple_results}")
```

## Advanced Features

### 1. Case-Insensitive Regex Matching

#### Rust
```rust
let json = r#"{"User_Name": "John", "user_email": "john@example.com"}"#;

// Case-sensitive (default) - only matches "user_email"
let key_replacements = Some(vec![("regex:^user_".to_string(), "person_".to_string())]);
let result = flatten_json(json, false, false, false, false, key_replacements, None, None)?;
// Result: {"User_Name": "John", "person_email": "john@example.com"}

// Case-insensitive - matches both
let key_replacements = Some(vec![("regex:(?i)^user_".to_string(), "person_".to_string())]);
let result = flatten_json(json, false, false, false, false, key_replacements, None, None)?;
// Result: {"person_Name": "John", "person_email": "john@example.com"}
```

#### Python
```python
json_str = '{"User_Name": "John", "user_email": "john@example.com"}'

# Case-sensitive (default)
key_replacements = [("regex:^user_", "person_")]
result = flatten_json(json_str, key_replacements=key_replacements)
# Result: {"User_Name": "John", "person_email": "john@example.com"}

# Case-insensitive
key_replacements = [("regex:(?i)^user_", "person_")]
result = flatten_json(json_str, key_replacements=key_replacements)
# Result: {"person_Name": "John", "person_email": "john@example.com"}
```

### 2. Complex Regex Patterns with Capture Groups

#### Rust
```rust
// Reorder and transform keys
let json = r#"{"field_123_name": "John", "field_456_email": "john@example.com"}"#;
let key_replacements = Some(vec![
    ("regex:^field_(\\d+)_(.+)".to_string(), "$2_id_$1".to_string())
]);
let result = flatten_json(json, false, false, false, false, key_replacements, None, None)?;
// Result: {"name_id_123": "John", "email_id_456": "john@example.com"}

// Phone number formatting
let json = r#"{"phone": "+1-555-123-4567"}"#;
let value_replacements = Some(vec![
    ("regex:\\+(\\d)-(\\d{3})-(\\d{3})-(\\d{4})".to_string(), "($2) $3-$4".to_string())
]);
let result = flatten_json(json, false, false, false, false, None, value_replacements, None)?;
// Result: {"phone": "(555) 123-4567"}
```

#### Python
```python
# Same patterns work in Python
json_str = '{"field_123_name": "John", "field_456_email": "john@example.com"}'
key_replacements = [("regex:^field_(\\d+)_(.+)", "$2_id_$1")]
result = flatten_json(json_str, key_replacements=key_replacements)
# Result: {"name_id_123": "John", "email_id_456": "john@example.com"}
```

### 3. Mixing Literal and Regex Patterns

#### Rust
```rust
let json = r#"{"user_name": "John", "temp_data": "test", "old_backup": "file", "status": "active"}"#;

let key_replacements = Some(vec![
    ("user_".to_string(), "person_".to_string()),  // Literal replacement
    ("regex:^(temp|old)_".to_string(), "legacy_".to_string())  // Regex replacement
]);
let value_replacements = Some(vec![
    ("@example.com".to_string(), "@company.org".to_string()),  // Literal replacement
    ("regex:^active$".to_string(), "enabled".to_string())  // Regex replacement
]);

let result = flatten_json(json, false, false, false, false, key_replacements, value_replacements, None)?;
// Result: {"person_name": "John", "legacy_data": "test", "legacy_backup": "file", "status": "enabled"}
```

#### Python
```python
json_str = '{"user_name": "John", "temp_data": "test", "old_backup": "file", "status": "active"}'

key_replacements = [
    ("user_", "person_"),  # Literal replacement
    ("regex:^(temp|old)_", "legacy_")  # Regex replacement
]
value_replacements = [
    ("@example.com", "@company.org"),  # Literal replacement
    ("regex:^active$", "enabled")  # Regex replacement
]

result = flatten_json(json_str, key_replacements=key_replacements, value_replacements=value_replacements)
# Same result as Rust example
```

## Input/Output Examples

### Example 1: E-commerce Product Data

#### Input
```json
{
  "product": {
    "id": 12345,
    "name": "Laptop",
    "details": {
      "brand": "TechCorp",
      "model": "Pro-X1",
      "specs": {
        "cpu": "Intel i7",
        "ram": "16GB",
        "storage": "512GB SSD"
      }
    },
    "pricing": {
      "base_price": 999.99,
      "discount": null,
      "final_price": 999.99
    },
    "availability": {
      "in_stock": true,
      "quantity": 50,
      "warehouses": ["NYC", "LA", "CHI"]
    }
  }
}
```

#### Output (Basic Flattening)
```json
{
  "product.id": 12345,
  "product.name": "Laptop",
  "product.details.brand": "TechCorp",
  "product.details.model": "Pro-X1",
  "product.details.specs.cpu": "Intel i7",
  "product.details.specs.ram": "16GB",
  "product.details.specs.storage": "512GB SSD",
  "product.pricing.base_price": 999.99,
  "product.pricing.discount": null,
  "product.pricing.final_price": 999.99,
  "product.availability.in_stock": true,
  "product.availability.quantity": 50,
  "product.availability.warehouses.0": "NYC",
  "product.availability.warehouses.1": "LA",
  "product.availability.warehouses.2": "CHI"
}
```

#### Output (With Filtering and Replacements)
```rust
// Remove null values and replace keys
let key_replacements = Some(vec![("product.".to_string(), "".to_string())]);
let result = flatten_json(json, false, true, false, false, key_replacements, None, None)?;
```

```json
{
  "id": 12345,
  "name": "Laptop",
  "details.brand": "TechCorp",
  "details.model": "Pro-X1",
  "details.specs.cpu": "Intel i7",
  "details.specs.ram": "16GB",
  "details.specs.storage": "512GB SSD",
  "pricing.base_price": 999.99,
  "pricing.final_price": 999.99,
  "availability.in_stock": true,
  "availability.quantity": 50,
  "availability.warehouses.0": "NYC",
  "availability.warehouses.1": "LA",
  "availability.warehouses.2": "CHI"
}
```

### Example 2: User Profile with Empty Values

#### Input
```json
{
  "user": {
    "personal": {
      "name": "John Doe",
      "email": "john@example.com",
      "phone": "",
      "address": {
        "street": "123 Main St",
        "city": "Anytown",
        "state": "",
        "zip": "12345"
      }
    },
    "preferences": {
      "theme": "dark",
      "notifications": {
        "email": true,
        "sms": false,
        "push": null
      },
      "privacy": {}
    },
    "activity": {
      "last_login": "2023-12-01T10:00:00Z",
      "login_count": 42,
      "recent_actions": []
    }
  }
}
```

#### Output (With All Filtering Options)
```rust
let result = flatten_json(json, true, true, true, true, None, None, None)?;
```

```json
{
  "user.personal.name": "John Doe",
  "user.personal.email": "john@example.com",
  "user.personal.address.street": "123 Main St",
  "user.personal.address.city": "Anytown",
  "user.personal.address.zip": "12345",
  "user.preferences.theme": "dark",
  "user.preferences.notifications.email": true,
  "user.preferences.notifications.sms": false,
  "user.activity.last_login": "2023-12-01T10:00:00Z",
  "user.activity.login_count": 42
}
```

## Differences Between Rust and Python

| Aspect | Rust | Python |
|--------|------|--------|
| **Return Type** | `Result<JsonOutput, Box<dyn Error>>` | `JsonOutput` (raises exceptions on error) |
| **Input Types** | `Into<JsonInput<'a>>` (strings, slices, arrays) | `Union[str, List[str]]` |
| **Error Handling** | `Result` type with `FlattenError` variants | Exceptions (`JsonFlattenError`, `ValueError`) |
| **Memory Management** | Zero-copy operations, compile-time safety | Automatic garbage collection |
| **Performance** | Faster (compiled, zero-cost abstractions) | Slightly slower (Python overhead) |
| **Type Safety** | Compile-time type checking | Runtime type checking with hints |
| **Regex Support** | Full Rust regex crate features | Same regex features via Rust backend |
| **Batch Processing** | Native slice support | List of strings |

### JsonOutput Usage Patterns

**Rust**:
```rust
match result {
    JsonOutput::Single(s) => println!("Single: {}", s),
    JsonOutput::Multiple(v) => println!("Multiple: {:?}", v),
}

// Or use helper methods
let single = result.into_single(); // Panics if multiple
let vec = result.into_vec();       // Always returns Vec<String>
```

**Python**:
```python
# Check type first
if result.is_single:
    single = result.get_single()
elif result.is_multiple:
    multiple = result.get_multiple()

# Universal method
python_result = result.to_python()  # Returns str or List[str]
```

### Error Handling Patterns

**Rust**:
```rust
match flatten_json(json, ...) {
    Ok(JsonOutput::Single(result)) => println!("Success: {}", result),
    Ok(JsonOutput::Multiple(results)) => println!("Batch: {:?}", results),
    Err(e) => {
        if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
            match flatten_err {
                FlattenError::JsonParseError(e) => eprintln!("JSON error: {}", e),
                FlattenError::RegexError(e) => eprintln!("Regex error: {}", e),
                FlattenError::InvalidReplacementPattern(msg) => eprintln!("Pattern error: {}", msg),
                FlattenError::BatchError { index, error } => eprintln!("Batch error at {}: {}", index, error),
            }
        }
    }
}
```

**Python**:
```python
try:
    result = flatten_json(json_input, ...)
    if result.is_single:
        print(f"Success: {result.get_single()}")
    else:
        print(f"Batch: {result.get_multiple()}")
except JsonFlattenError as e:
    print(f"JSON flattening failed: {e}")
except ValueError as e:
    print(f"Invalid arguments: {e}")
```

## Running the Examples

### Rust Examples
```bash
# Compile and run the Rust examples
rustc --extern json_tools_rs rust_examples.rs -o rust_examples
./rust_examples

# Or if you have the project set up:
cargo run --example rust_examples
```

### Python Examples
```bash
# Make sure the package is installed
maturin develop --features python

# Run the Python examples
python python_examples.py
```

Both implementations provide identical core functionality with language-appropriate interfaces and error handling patterns. The Rust version offers better performance and compile-time safety, while the Python version provides easier integration with Python ecosystems and more familiar error handling for Python developers.


## License

This project is licensed under the MIT OR Apache-2.0 license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
