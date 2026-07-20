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
    &flat.try_into_single()?
)?;
// Matches original structure
```

All configuration options (filtering, replacements, collision handling, type conversion) work with both `.flatten()` and `.unflatten()` modes.

## Examples

### Easy: flatten a nested object

```rust
let result = JSONTools::new()
    .flatten()
    .execute(r#"{"user": {"name": "Alice", "age": 30}}"#)?;
// {"user.name": "Alice", "user.age": 30}
```

```python
result = jt.JSONTools().flatten().execute({"user": {"name": "Alice", "age": 30}})
# {'user.name': 'Alice', 'user.age': 30}
```

### Medium: arrays of objects, custom separator, round-trip

```python
import json_tools_rs as jt

data = {"users": [{"name": "Alice", "roles": ["admin", "editor"]}, {"name": "Bob", "roles": []}]}

flat = jt.JSONTools().flatten().separator("::").execute(data)
# {'users::0::name': 'Alice', 'users::0::roles::0': 'admin', 'users::0::roles::1': 'editor',
#  'users::1::name': 'Bob', 'users::1::roles': []}
# Note: Bob's empty "roles" array is kept as a literal [] value under its own key --
# only a *non-empty* container gets recursively flattened into per-element keys.

restored = jt.JSONTools().unflatten().separator("::").execute(flat)
# {'users': [{'name': 'Alice', 'roles': ['admin', 'editor']}, {'name': 'Bob', 'roles': []}]}
# Exact round trip, including Bob's empty array.
```

### Hard: flatten -> transform -> unflatten pipeline

Filtering, key/value transforms, and type conversion all run *during* `.flatten()`;
`.unflatten()` only reconstructs structure from whatever flat keys survive, so a single
flatten call can prepare data that a later unflatten call reconstructs with fewer keys
and already-converted types:

```python
import json_tools_rs as jt

data = {
    "Order_ID": "ORD-1001",
    "Customer": {"Name": "Jane Doe", "Email": "jane@old-domain.com"},
    "Items": [
        {"sku": "A1", "qty": "2", "price": "19.99"},
        {"sku": "B2", "qty": "1", "price": "9.5"},
    ],
    "Notes": None,
}

flat = (
    jt.JSONTools()
    .flatten()
    .separator("::")
    .lowercase_keys(True)
    .auto_convert_types(True)
    .remove_nulls(True)
    .execute(data)
)
# {'order_id': 'ORD-1001', 'customer::name': 'Jane Doe',
#  'customer::email': 'jane@old-domain.com', 'items::0::sku': 'A1',
#  'items::0::qty': 2, 'items::0::price': 19.99, 'items::1::sku': 'B2',
#  'items::1::qty': 1, 'items::1::price': 9.5}
# "Notes" is gone entirely -- remove_nulls ran before the flat map was built, so
# there's no "notes" key left for unflatten to see.

restored = jt.JSONTools().unflatten().separator("::").execute(flat)
# {'order_id': 'ORD-1001', 'customer': {'name': 'Jane Doe', 'email': 'jane@old-domain.com'},
#  'items': [{'sku': 'A1', 'qty': 2, 'price': 19.99}, {'sku': 'B2', 'qty': 1, 'price': 9.5}]}
```

The nested shape is fully restored, but `qty`/`price` come back as numbers (not the
original strings) and `notes` never reappears -- unflatten can only rebuild structure
from the keys it's given, it has no memory of what flatten discarded or converted.
