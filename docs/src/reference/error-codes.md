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

match JSONTools::new().flatten().execute("not valid json") {
    Ok(result) => { /* success */ }
    Err(e) => {
        // e.error_code() -> "E001" (bare code, for match arms / logging fields)
        // format!("{e}")  -> "[E001] JSON parsing failed: Invalid literal (`true`,
        //                    `false`, or a `null`) while parsing at line 1 column 4
        //                    ...
        //                    💡 Suggestion: Verify your JSON syntax using a JSON
        //                    validator. ..." (Display already includes the bracketed code)
        eprintln!("{e}");
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
    # Error: Failed to process JSON string: [E001] JSON parsing failed: ...
```

The Python bindings prepend their own context (e.g. `"Failed to process JSON string: "`) before the underlying Rust message, so the `[E00x]` code is embedded in `str(e)` but not necessarily its first characters -- match it as a substring (`"[E001]" in str(e)`), not a prefix.

## JVM Error Handling

```java
import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;
import io.github.amaye15.jsontoolsrs.JsonToolsException;

try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
    String result = tools.execute("not valid json");
} catch (JsonToolsException e) {
    System.err.println(e.getMessage());
    // [E001] JSON parsing failed: ...
}
```

`JsonToolsException` (an unchecked `RuntimeException`) carries the Rust error's `Display` text as its message, unmodified -- unlike the Python bindings, the JVM side adds no extra prefix, so `[E00x]` is at the very start of `getMessage()`. See the [JVM API reference](./jvm-api.md#error-handling) for details.

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
