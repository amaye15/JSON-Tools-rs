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
- Apply key transformations (lowercase, replacements), filtering, and type conversion
  recursively at every level of nesting, not just the top level
- Filter out unwanted values while preserving nesting
- Convert string types without flattening

> Key replacement runs *before* `lowercase_keys` in normal mode (the opposite order
> from `.flatten()`, where lowercasing happens first). A pattern like `r'^user_'`
> is matched against the *original-case* key, so it won't match `"User_Name"` --
> use `r'^User_'` (matching the actual input case) or a case-insensitive pattern
> like `r'(?i)^user_'` instead. This ordering difference is easy to trip over when
> porting a `key_replacement` pattern between `.flatten()` and `.normal()`.

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
    .key_replacement("r'^User_'", "")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(True)
    .remove_nulls(True)
    .execute(data)
)
# {'name': 'alice@company.org', 'active': 'true'}
```

All features available in `.flatten()` and `.unflatten()` modes also work in `.normal()` mode, except the actual flattening/unflattening operation itself.

## Examples

### Easy: lowercase keys, structure untouched

```python
import json_tools_rs as jt

data = {"User": {"Name": "Alice"}}
result = jt.JSONTools().normal().lowercase_keys(True).execute(data)
# {'user': {'name': 'Alice'}}
```

### Medium: lowercase + replace + filter (see the example above)

The [example above](#example) combines `lowercase_keys`, `key_replacement`,
`value_replacement`, and two filters on a flat one-level object.

### Hard: cascading filters on deeply nested data

Filters recurse into every level, and an object that becomes empty *after* its own
children are filtered is itself removed on the same pass -- see
[Filtering](./filtering.md#hard-cascading-removal-in-normal-mode) for the full
mechanics. In `.normal()` mode this applies at arbitrary depth, not just one level:

```python
data = {
    "org": {
        "team": {
            "lead": {"name": "Priya", "notes": ""},
            "intern": {"name": "", "notes": None},
        }
    }
}

result = (jt.JSONTools()
    .normal()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .remove_empty_objects(True)
    .execute(data)
)
# {'org': {'team': {'lead': {'name': 'Priya'}}}}
```

`intern` has no surviving fields (`name` is `""`, `notes` is `null`), so it collapses
to `{}` and is removed -- which is exactly the same check that keeps `lead` around,
just applied one level deeper.
