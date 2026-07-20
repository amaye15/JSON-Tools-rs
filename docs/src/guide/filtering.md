# Filtering

Remove unwanted values during flattening, unflattening, or `.normal()` processing.
Filtering is applied recursively at every level of nesting.

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

## Examples

### Easy: drop nulls

```python
import json_tools_rs as jt

data = {"name": "Alice", "middle_name": None}
result = jt.JSONTools().flatten().remove_nulls(True).execute(data)
# {'name': 'Alice'}
```

### Medium: all four filters together

```python
data = {"name": "John", "bio": "", "age": None, "tags": [], "metadata": {}, "city": "NYC"}

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

### Hard: cascading removal in `.normal()` mode

Filtering runs bottom-up: a nested object's own children are filtered *first*, and if
that leaves the object empty, `remove_empty_objects` removes it too -- even if it
wasn't `{}` in the original input. This cascades all the way to the root in a single
pass, so an object can vanish for a reason nowhere near itself, once every leaf inside
it has been filtered away:

```python
data = {
    "user": {"name": "Alice", "middle_name": "", "nickname": None},
    "session": {"token": "", "meta": {}},
    "tags": [],
}

result = (jt.JSONTools()
    .normal()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .remove_empty_objects(True)
    .remove_empty_arrays(True)
    .execute(data)
)
# {'user': {'name': 'Alice'}}
```

`session` was never empty in the input -- but once `token` (`""`) and `meta` (`{}`) are
both filtered out of it, `session` itself becomes `{}` and is removed on the same pass,
one level up. `tags` (already `[]`) is removed directly. `user` survives because
`name` is non-empty, even though its two siblings were filtered away.
