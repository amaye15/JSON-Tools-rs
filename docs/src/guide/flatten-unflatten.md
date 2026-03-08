# Flattening & Unflattening

## Flattening

Flattening converts nested JSON into a flat key-value structure using dot-separated (or custom) keys.

```json
// Input
{"user": {"name": "John", "address": {"city": "NYC", "zip": "10001"}}}

// Output (flattened)
{"user.name": "John", "user.address.city": "NYC", "user.address.zip": "10001"}
```

### Arrays

Arrays are flattened with numeric indices:

```json
// Input
{"users": [{"name": "Alice"}, {"name": "Bob"}]}

// Output
{"users.0.name": "Alice", "users.1.name": "Bob"}
```

### Custom Separators

Use `.separator()` to change the key delimiter:

```rust
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .execute(json)?;
// {"user::name": "John", "user::address::city": "NYC"}
```

```python
result = jt.JSONTools().flatten().separator("::").execute(data)
```

## Unflattening

Unflattening reverses the process, reconstructing nested structures from flat keys.

```json
// Input
{"user.name": "John", "user.address.city": "NYC"}

// Output (unflattened)
{"user": {"name": "John", "address": {"city": "NYC"}}}
```

Numeric keys reconstruct arrays:

```json
// Input
{"users.0.name": "Alice", "users.1.name": "Bob"}

// Output
{"users": [{"name": "Alice"}, {"name": "Bob"}]}
```

## Roundtrip

Flattening and unflattening are perfect inverses. You can flatten data, apply transformations, then unflatten to recover the original structure:

```rust
let original = r#"{"user": {"name": "John", "scores": [10, 20, 30]}}"#;

// Flatten
let flat = JSONTools::new().flatten().execute(original)?;

// Unflatten back
let restored = JSONTools::new().unflatten().execute(
    &flat.into_single()
)?;
// Matches original structure
```

All configuration options (filtering, replacements, collision handling, type conversion) work with both `.flatten()` and `.unflatten()` modes.
