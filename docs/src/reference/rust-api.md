# Rust API Reference

Full API documentation is available on [docs.rs](https://docs.rs/json-tools-rs).

## JSONTools

The main builder struct for all JSON operations. Uses the owned-self builder pattern -- all configuration methods consume and return `Self` for chaining.

### Construction

```rust
use json_tools_rs::JSONTools;

let tools = JSONTools::new();
```

`JSONTools` implements `Default`, `Debug`, and `Clone`.

### Operation Modes

Exactly one mode must be set before calling `.execute()`.

| Method | Description |
|--------|-------------|
| `.flatten()` | Flatten nested JSON into separator-delimited keys |
| `.unflatten()` | Reconstruct nested JSON from flat, separator-delimited keys |
| `.normal()` | Apply transformations without changing the nesting structure |

```rust
use json_tools_rs::{JSONTools, JsonOutput};

// Flatten
let result = JSONTools::new()
    .flatten()
    .execute(r#"{"a": {"b": 1}}"#)?;

// Unflatten
let result = JSONTools::new()
    .unflatten()
    .execute(r#"{"a.b": 1}"#)?;

// Normal mode -- transformations only
let result = JSONTools::new()
    .normal()
    .lowercase_keys(true)
    .auto_convert_types(true)
    .execute(r#"{"Name": "John", "Age": "30"}"#)?;
```

### Configuration Methods

All methods consume `self` and return `Self` for chaining. Marked `#[must_use]`.

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.separator(sep)` | `impl Into<String>` | `"."` | Key separator for flatten/unflatten |
| `.lowercase_keys(flag)` | `bool` | `false` | Convert all keys to lowercase |
| `.remove_empty_strings(flag)` | `bool` | `false` | Filter out `""` values |
| `.remove_nulls(flag)` | `bool` | `false` | Filter out `null` values |
| `.remove_empty_objects(flag)` | `bool` | `false` | Filter out `{}` values |
| `.remove_empty_arrays(flag)` | `bool` | `false` | Filter out `[]` values |
| `.key_replacement(find, replace)` | `impl Into<String>, impl Into<String>` | -- | Add a key replacement regex pattern |
| `.value_replacement(find, replace)` | `impl Into<String>, impl Into<String>` | -- | Add a value replacement regex pattern |
| `.handle_key_collision(flag)` | `bool` | `false` | Collect colliding keys into arrays |
| `.auto_convert_types(flag)` | `bool` | `false` | Auto-convert string values to native types |
| `.parallel_threshold(n)` | `usize` | `100` | Min batch size for parallel processing |
| `.num_threads(n)` | `Option<usize>` | `None` (CPU count) | Thread count for parallelism |
| `.nested_parallel_threshold(n)` | `usize` | `100` | Min keys/items for intra-document parallelism |
| `.max_array_index(n)` | `usize` | `100_000` | Max array index during unflattening (DoS protection) |

**Note:** `.separator()` panics if given an empty string. Defaults for `parallel_threshold`, `nested_parallel_threshold`, `num_threads`, and `max_array_index` can be overridden via environment variables (see [Performance Tuning](../resources/performance.md)).

### Execution

```rust
pub fn execute<'a, T>(&self, json_input: T) -> Result<JsonOutput, JsonToolsError>
where
    T: Into<JsonInput<'a>>,
```

Accepts any type that implements `Into<JsonInput>`:

| Rust Type | JsonInput Variant |
|-----------|-------------------|
| `&str` | `Single(Cow::Borrowed)` |
| `&String` | `Single(Cow::Borrowed)` |
| `&[&str]` | `Multiple` (borrowing) |
| `Vec<&str>` | `MultipleOwned` |
| `Vec<String>` | `MultipleOwned` |
| `&[String]` | `MultipleOwned` |

**Errors:** Returns `Err(JsonToolsError)` if no mode is set, JSON is invalid, or processing fails.

### Full Example

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let tools = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .remove_nulls(true)
    .remove_empty_strings(true)
    .key_replacement("^user_", "")
    .auto_convert_types(true)
    .parallel_threshold(50)
    .num_threads(Some(4));

// Single document
let result = tools.execute(r#"{"User_Name": "Alice", "User_Age": "30"}"#)?;
match result {
    JsonOutput::Single(s) => println!("{}", s),
    JsonOutput::Multiple(_) => unreachable!(),
}

// Batch processing
let batch: Vec<String> = (0..1000)
    .map(|i| format!(r#"{{"id": "{}"}}"#, i))
    .collect();
let results = tools.execute(batch)?;
match results {
    JsonOutput::Multiple(v) => println!("Processed {} items", v.len()),
    JsonOutput::Single(_) => unreachable!(),
}
```

## JsonInput

Input enum for `execute()`. You rarely construct this directly -- the `From` implementations handle conversion automatically.

```rust
pub enum JsonInput<'a> {
    /// Single JSON string (zero-copy via Cow)
    Single(Cow<'a, str>),
    /// Multiple JSON strings (borrowing)
    Multiple(&'a [&'a str]),
    /// Multiple JSON strings (owned or mixed)
    MultipleOwned(Vec<Cow<'a, str>>),
}
```

### From Implementations

| Source Type | Variant |
|-------------|---------|
| `&str` | `Single(Cow::Borrowed)` |
| `&String` | `Single(Cow::Borrowed)` |
| `&[&str]` | `Multiple` |
| `Vec<&str>` | `MultipleOwned` |
| `Vec<String>` | `MultipleOwned` |
| `&[String]` | `MultipleOwned` |

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let tools = JSONTools::new().flatten();

// All of these work transparently:
let _ = tools.execute(r#"{"a": 1}"#);                      // &str
let s = String::from(r#"{"a": 1}"#);
let _ = tools.execute(&s);                                  // &String
let batch = vec![r#"{"a": 1}"#, r#"{"b": 2}"#];
let _ = tools.execute(batch);                               // Vec<&str>
let owned: Vec<String> = vec![r#"{"a": 1}"#.into()];
let _ = tools.execute(owned);                               // Vec<String>
```

## JsonOutput

Output enum from `execute()`.

```rust
pub enum JsonOutput {
    /// Single JSON result string
    Single(String),
    /// Multiple JSON result strings (batch)
    Multiple(Vec<String>),
}
```

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.into_single()` | `String` | Extract single result. **Panics** on `Multiple`. |
| `.into_multiple()` | `Vec<String>` | Extract batch results. **Panics** on `Single`. |
| `.try_into_single()` | `Result<String, JsonToolsError>` | Non-panicking single extraction |
| `.try_into_multiple()` | `Result<Vec<String>, JsonToolsError>` | Non-panicking batch extraction |
| `.into_vec()` | `Vec<String>` | Always returns a `Vec` (wraps `Single` in a one-element vec) |

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let result = JSONTools::new().flatten().execute(r#"{"a": {"b": 1}}"#)?;

// Pattern matching (recommended)
match result {
    JsonOutput::Single(s) => println!("Single: {}", s),
    JsonOutput::Multiple(v) => println!("Batch of {}", v.len()),
}

// Direct extraction (panics on wrong variant)
let s = JSONTools::new().flatten().execute(r#"{"a": 1}"#)?.into_single();

// Safe extraction (returns Result)
let s = JSONTools::new().flatten().execute(r#"{"a": 1}"#)?.try_into_single()?;

// Always-vec (useful for uniform handling)
let v = JSONTools::new().flatten().execute(r#"{"a": 1}"#)?.into_vec();
assert_eq!(v.len(), 1);
```

## JsonToolsError

Comprehensive error enum with machine-readable error codes (`E001`-`E008`), human-readable messages, and actionable suggestions.

```rust
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum JsonToolsError {
    JsonParseError { .. },           // E001
    RegexError { .. },               // E002
    InvalidReplacementPattern { .. }, // E003
    InvalidJsonStructure { .. },     // E004
    ConfigurationError { .. },       // E005
    BatchProcessingError { .. },     // E006
    InputValidationError { .. },     // E007
    SerializationError { .. },       // E008
}
```

### Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.error_code()` | `&'static str` | Machine-readable code: `"E001"` through `"E008"` |

### Error Handling Example

```rust
use json_tools_rs::{JSONTools, JsonToolsError};

let result = JSONTools::new().flatten().execute("invalid json");

match result {
    Ok(output) => { /* success */ }
    Err(e) => {
        // Machine-readable error code
        match e.error_code() {
            "E001" => eprintln!("JSON parsing error: {}", e),
            "E005" => eprintln!("Configuration error: {}", e),
            "E006" => eprintln!("Batch error: {}", e),
            code => eprintln!("[{}] {}", code, e),
        }

        // Pattern matching for specific handling
        match &e {
            JsonToolsError::JsonParseError { message, suggestion, .. } => {
                eprintln!("Parse failed: {}", message);
                eprintln!("Try: {}", suggestion);
            }
            JsonToolsError::BatchProcessingError { index, source, .. } => {
                eprintln!("Item {} failed: {}", index, source);
            }
            _ => eprintln!("{}", e),
        }
    }
}
```

### Auto-Conversions

`JsonToolsError` implements `From` for common error types:

```rust
// These conversions happen automatically in ? chains:
impl From<json_parser::JsonError> for JsonToolsError { .. }  // -> E001
impl From<regex::Error> for JsonToolsError { .. }            // -> E002
```

See [Error Codes](./error-codes.md) for the full error reference.

## ProcessingConfig

Low-level configuration struct used internally by `JSONTools`. You can construct it directly for advanced use cases, but the `JSONTools` builder is the recommended interface.

```rust
pub struct ProcessingConfig {
    pub separator: String,
    pub lowercase_keys: bool,
    pub filtering: FilteringConfig,
    pub collision: CollisionConfig,
    pub replacements: ReplacementConfig,
    pub auto_convert_types: bool,
    pub parallel_threshold: usize,
    pub num_threads: Option<usize>,
    pub nested_parallel_threshold: usize,
    pub max_array_index: usize,
}
```

### Builder Methods

```rust
use json_tools_rs::ProcessingConfig;

let config = ProcessingConfig::new()
    .separator("::")
    .lowercase_keys(true)
    .filtering(FilteringConfig::new().set_remove_nulls(true))
    .collision(CollisionConfig::new().handle_collisions(true))
    .replacements(
        ReplacementConfig::new()
            .add_key_replacement("^old_", "new_")
    );
```

## FilteringConfig

Configuration for value filtering, stored internally as a bitmask for single-instruction checks on the hot path.

```rust
pub struct FilteringConfig { /* bitmask */ }
```

### Builder Methods

All methods consume and return `Self`.

| Method | Description |
|--------|-------------|
| `.set_remove_empty_strings(bool)` | Filter `""` values |
| `.set_remove_nulls(bool)` | Filter `null` values |
| `.set_remove_empty_objects(bool)` | Filter `{}` values |
| `.set_remove_empty_arrays(bool)` | Filter `[]` values |

### Query Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.remove_empty_strings()` | `bool` | Is empty string filtering enabled? |
| `.remove_nulls()` | `bool` | Is null filtering enabled? |
| `.remove_empty_objects()` | `bool` | Is empty object filtering enabled? |
| `.remove_empty_arrays()` | `bool` | Is empty array filtering enabled? |
| `.has_any_filter()` | `bool` | Is any filter enabled? |

```rust
use json_tools_rs::FilteringConfig;

let filtering = FilteringConfig::new()
    .set_remove_nulls(true)
    .set_remove_empty_strings(true);

assert!(filtering.has_any_filter());
assert!(filtering.remove_nulls());
assert!(!filtering.remove_empty_objects());
```

## CollisionConfig

Configuration for key collision handling.

```rust
pub struct CollisionConfig {
    pub handle_collisions: bool,
}
```

### Builder Methods

| Method | Description |
|--------|-------------|
| `.handle_collisions(bool)` | Enable/disable collision handling |

### Query Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.has_collision_handling()` | `bool` | Is collision handling enabled? |

```rust
use json_tools_rs::CollisionConfig;

let collision = CollisionConfig::new().handle_collisions(true);
assert!(collision.has_collision_handling());
```

## ReplacementConfig

Configuration for key and value replacement patterns. Uses `SmallVec<[(String, String); 2]>` internally to avoid heap allocation for the common case of 0-2 replacements.

```rust
pub struct ReplacementConfig {
    pub key_replacements: SmallVec<[(String, String); 2]>,
    pub value_replacements: SmallVec<[(String, String); 2]>,
}
```

### Builder Methods

| Method | Description |
|--------|-------------|
| `.add_key_replacement(find, replace)` | Add a key replacement regex pattern |
| `.add_value_replacement(find, replace)` | Add a value replacement regex pattern |

### Query Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.has_key_replacements()` | `bool` | Are any key replacements configured? |
| `.has_value_replacements()` | `bool` | Are any value replacements configured? |

```rust
use json_tools_rs::ReplacementConfig;

let replacements = ReplacementConfig::new()
    .add_key_replacement("^user_", "")
    .add_value_replacement("@old\\.com", "@new.com");

assert!(replacements.has_key_replacements());
assert!(replacements.has_value_replacements());
```
