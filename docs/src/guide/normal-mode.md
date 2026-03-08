# Normal Mode

Normal mode applies transformations (filtering, replacements, type conversion) without flattening or unflattening the JSON structure.

## Usage

```rust
let result = JSONTools::new()
    .normal()
    .lowercase_keys(true)
    .remove_nulls(true)
    .remove_empty_strings(true)
    .auto_convert_types(true)
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .normal()
    .lowercase_keys(True)
    .remove_nulls(True)
    .remove_empty_strings(True)
    .auto_convert_types(True)
    .execute(data)
)
```

## When to Use Normal Mode

Use `.normal()` when you want to:

- Clean data without changing its structure
- Apply key transformations (lowercase, replacements) to top-level keys only
- Filter out unwanted values while preserving nesting
- Convert string types without flattening

## Example

```python
import json_tools_rs as jt

data = {
    "User_Name": "alice@example.com",
    "User_Age": "",
    "User_Active": "true",
    "User_Score": None,
}

result = (jt.JSONTools()
    .normal()
    .lowercase_keys(True)
    .key_replacement("^user_", "")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(True)
    .remove_nulls(True)
    .execute(data)
)
# {'name': 'alice@company.org', 'active': 'true'}
```

All features available in `.flatten()` and `.unflatten()` modes also work in `.normal()` mode, except the actual flattening/unflattening operation itself.
