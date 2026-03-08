# Rust API Reference

Full API documentation is available on [docs.rs](https://docs.rs/json-tools-rs).

## JSONTools

The main builder struct for all JSON operations.

### Construction

```rust
let tools = JSONTools::new();
```

### Operation Modes

| Method | Description |
|--------|-------------|
| `.flatten()` | Flatten nested JSON into dot-separated keys |
| `.unflatten()` | Reconstruct nested JSON from flat keys |
| `.normal()` | Apply transformations without changing structure |

### Configuration Methods

All methods return `Self` for chaining.

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.separator(sep)` | `&str` | `"."` | Key separator for flatten/unflatten |
| `.lowercase_keys(flag)` | `bool` | `false` | Convert all keys to lowercase |
| `.remove_empty_strings(flag)` | `bool` | `false` | Filter out `""` values |
| `.remove_nulls(flag)` | `bool` | `false` | Filter out `null` values |
| `.remove_empty_objects(flag)` | `bool` | `false` | Filter out `{}` values |
| `.remove_empty_arrays(flag)` | `bool` | `false` | Filter out `[]` values |
| `.key_replacement(find, replace)` | `&str, &str` | -- | Add a key replacement pattern |
| `.value_replacement(find, replace)` | `&str, &str` | -- | Add a value replacement pattern |
| `.handle_key_collision(flag)` | `bool` | `false` | Collect colliding keys into arrays |
| `.auto_convert_types(flag)` | `bool` | `false` | Auto-convert string types |
| `.parallel_threshold(n)` | `usize` | `10` | Min batch size for parallelism |
| `.num_threads(n)` | `Option<usize>` | `None` (CPU count) | Thread count |
| `.nested_parallel_threshold(n)` | `usize` | `1000` | Min entries for nested parallelism |

### Execution

```rust
fn execute<I: Into<JsonInput<'a>>>(&self, input: I) -> Result<JsonOutput, JsonToolsError>
```

Accepts:
- `&str` -- single JSON string
- `&[&str]` -- batch of JSON strings
- `Vec<String>` -- batch of owned JSON strings

### Full Example

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .remove_nulls(true)
    .auto_convert_types(true)
    .execute(json)?;

match result {
    JsonOutput::Single(s) => println!("Single: {}", s),
    JsonOutput::Multiple(v) => println!("Batch: {} items", v.len()),
}
```

## JsonOutput

Output enum from `execute()`.

| Variant | Description |
|---------|-------------|
| `JsonOutput::Single(String)` | Single JSON result |
| `JsonOutput::Multiple(Vec<String>)` | Batch results |

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.into_single()` | `String` | Extract single result (panics on Multiple) |
| `.into_multiple()` | `Vec<String>` | Extract batch results (panics on Single) |
| `.into_vec()` | `Vec<String>` | Always returns a Vec (wraps Single in vec) |

## JsonToolsError

Comprehensive error type with machine-readable error codes.

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.error_code()` | `&'static str` | Machine-readable code (e.g., `"E001"`) |

See [Error Codes](./error-codes.md) for the full list.
