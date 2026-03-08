# Error Codes

All errors include a machine-readable code accessible via `.error_code()` (Rust) or in the error message (Python).

| Code | Name | Description |
|------|------|-------------|
| `E001` | `JsonParseError` | Invalid JSON input. The input string could not be parsed as valid JSON. |
| `E002` | `RegexError` | Invalid regex pattern in a key or value replacement. |
| `E003` | `InvalidReplacementPattern` | Malformed replacement pattern string. |
| `E004` | `InvalidJsonStructure` | JSON structure is valid but not suitable for the operation (e.g., unflattening non-object JSON). |
| `E005` | `ConfigurationError` | Operation mode not set. Call `.flatten()`, `.unflatten()`, or `.normal()` before `.execute()`. |
| `E006` | `BatchProcessingError` | An error occurred while processing one or more items in a batch. |
| `E007` | `InputValidationError` | Input validation failed (e.g., unsupported input type). |
| `E008` | `SerializationError` | Failed to serialize the output back to JSON. |

## Rust Error Handling

```rust
use json_tools_rs::{JSONTools, JsonToolsError};

match JSONTools::new().flatten().execute(input) {
    Ok(result) => { /* success */ }
    Err(e) => {
        eprintln!("[{}] {}", e.error_code(), e);
        // [E001] JSON parse error: expected value at line 1 column 1
    }
}
```

## Python Error Handling

```python
import json_tools_rs as jt

try:
    result = jt.JSONTools().flatten().execute("not valid json")
except jt.JsonToolsError as e:
    print(f"Error: {e}")
    # Error: [E001] JSON parse error: ...
```

## Common Errors

### E005: No mode set

```python
# Wrong: no mode set
tools = jt.JSONTools().execute(data)  # Raises E005

# Correct: set a mode first
tools = jt.JSONTools().flatten().execute(data)
```

### E001: Invalid JSON

```python
# Wrong: not valid JSON
tools = jt.JSONTools().flatten().execute("hello world")  # Raises E001

# Correct: valid JSON string
tools = jt.JSONTools().flatten().execute('{"key": "value"}')
```
