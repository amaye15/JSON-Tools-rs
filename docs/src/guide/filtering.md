# Filtering

Remove unwanted values during flattening or unflattening.

## Available Filters

| Method | Removes |
|--------|---------|
| `.remove_empty_strings(true)` | `""` empty string values |
| `.remove_nulls(true)` | `null` values |
| `.remove_empty_objects(true)` | `{}` empty objects |
| `.remove_empty_arrays(true)` | `[]` empty arrays |

## Example

```rust
use json_tools_rs::{JSONTools, JsonOutput};

let json = r#"{
    "name": "John",
    "bio": "",
    "age": null,
    "tags": [],
    "metadata": {},
    "city": "NYC"
}"#;

let result = JSONTools::new()
    .flatten()
    .remove_empty_strings(true)
    .remove_nulls(true)
    .remove_empty_arrays(true)
    .remove_empty_objects(true)
    .execute(json)?;

// Result: {"name": "John", "city": "NYC"}
```

```python
import json_tools_rs as jt

data = {
    "name": "John",
    "bio": "",
    "age": None,
    "tags": [],
    "metadata": {},
    "city": "NYC",
}

result = (jt.JSONTools()
    .flatten()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .remove_empty_arrays(True)
    .remove_empty_objects(True)
    .execute(data)
)
# {'name': 'John', 'city': 'NYC'}
```

## Filtering with Unflatten

Filters also work during unflattening, applied after the nested structure is reconstructed:

```rust
let result = JSONTools::new()
    .unflatten()
    .remove_nulls(true)
    .remove_empty_strings(true)
    .execute(flat_json)?;
```

## Combining Filters

All filters can be combined freely. They are applied after the flatten/unflatten operation completes.
