# Quick Start (Rust)

The `JSONTools` struct provides a unified builder pattern API. Call `.flatten()` or `.unflatten()` to set the mode, chain configuration methods, then call `.execute()`.

## Basic Flattening

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user": {"name": "John", "profile": {"age": 30, "city": "NYC"}}}"#;
let result = JSONTools::new()
    .flatten()
    .execute(json)?;

if let JsonOutput::Single(flattened) = result {
    println!("{}", flattened);
}
// {"user.name": "John", "user.profile.age": 30, "user.profile.city": "NYC"}
```

## Basic Unflattening

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user.name": "John", "user.profile.age": 30}"#;
let result = JSONTools::new()
    .unflatten()
    .execute(json)?;

if let JsonOutput::Single(nested) = result {
    println!("{}", nested);
}
// {"user": {"name": "John", "profile": {"age": 30}}}
```

## Advanced Configuration

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .remove_empty_strings(true)
    .remove_nulls(true)
    .execute(json)?;

if let JsonOutput::Single(flattened) = result {
    println!("{}", flattened);
}
// {"user::name": "John"}
```

## Batch Processing

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let batch = vec![
    r#"{"user": {"name": "Alice"}}"#,
    r#"{"user": {"name": "Bob"}}"#,
    r#"{"user": {"name": "Charlie"}}"#,
];

let result = JSONTools::new()
    .flatten()
    .separator("_")
    .execute(batch.as_slice())?;

if let JsonOutput::Multiple(results) = result {
    for r in &results {
        println!("{}", r);
    }
}
// {"user_name": "Alice"}
// {"user_name": "Bob"}
// {"user_name": "Charlie"}
```

## Error Handling

```rust
use json_tools_rs::{JSONTools, JsonToolsError};

match JSONTools::new().flatten().execute("invalid json") {
    Ok(result) => println!("{:?}", result),
    Err(e) => {
        eprintln!("Error [{}]: {}", e.error_code(), e);
        // Error [E001]: JSON parse error: ...
    }
}
```
