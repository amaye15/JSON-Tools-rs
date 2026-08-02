# Key Collision Handling

When key replacements or transformations cause multiple keys to map to the same output key, collision handling determines what happens.

## Enabling Collision Handling

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("r'(User|Admin)_'", "")
    .handle_key_collision(true)
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("r'(User|Admin)_'", "")
    .handle_key_collision(True)
    .execute(data)
)
```

## How It Works

With `.handle_key_collision(true)`, when two keys collide after transformation, their values are collected into an array:

```json
// Input
{"User_name": "John", "Admin_name": "Jane"}

// With key_replacement("r'(User|Admin)_'", "") + handle_key_collision(true)
// Output
{"name": ["John", "Jane"]}
```

Without collision handling, the last value wins (overwrites previous values).

## Collision with Filtering

Collision handling respects filters. If a colliding value would be filtered out (e.g., empty string with `.remove_empty_strings(true)`), it is excluded from the collected array:

```json
// Input
{"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}

// With key_replacement("r'(User|Admin|Guest)_'", "") + remove_empty_strings(true) + handle_key_collision(true)
// Output
{"name": ["John", "Bob"]}
```

`Admin_name`'s empty-string value is dropped by the filter before collision
resolution ever sees it, so only `John` and `Bob` end up in the array.

## Works with All Modes

Collision handling works during `.flatten()`, `.unflatten()`, and `.normal()` operations.

> Collected array order is not guaranteed to match input order -- collision resolution
> is backed by a hash map internally, so treat the array as an unordered bag of the
> colliding values, not a positional record of which key contributed which value.

## Consistent Shape Across Documents: `always_array_keys`

A key's scalar-vs-array shape from `.handle_key_collision(true)` depends on
whether a collision actually happened *in that specific document*: a key
that collides in some rows of a batch (e.g. because of `.key_replacement()`)
but not others ends up a plain value in some results and an array in
others. That's awkward for anything expecting a stable schema -- including
building a DataFrame column from the results, or [`normalise()`](dataframe-support.md)'s
own column typing, which needs every row's value to already be
array-shaped to resolve a column to `List<T>`.

`.always_array_keys([...])` names flattened key names that must **always**
render as an array, even when only one value is present in a given
document -- independent of `.handle_key_collision()`:

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("r'(User|Admin)_'", "")
    .always_array_keys(["name"])
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("r'(User|Admin)_'", "")
    .always_array_keys(["name"])
    .execute(data)
)
```

```json
// Document with a collision -- same as handle_key_collision(true) alone
{"User_name": "John", "Admin_name": "Jane"}  ->  {"name": ["John", "Jane"]}

// Document with NO collision -- would be a bare string without always_array_keys;
// wrapped into a one-element array instead
{"User_name": "John"}  ->  {"name": ["John"]}
```

A document that doesn't have the key at all is unaffected -- the key stays
absent, it is never injected. Keys not named in `always_array_keys` keep
their ordinary behavior (governed by `.handle_key_collision()`).

Matched against the *final* flattened key name -- the same name
`.handle_key_collision()` resolves collisions on -- and works for all
operations (`.flatten()`, `.unflatten()`, `.normal()`) and for
`normalise()`/DataFrame input, since those build on top of `.flatten()`.

This is also the way to *guarantee* `normalise()` resolves a column to
`List<T>` even when a particular batch happens to have zero collisions for
it -- without `always_array_keys`, `normalise()` only promotes a column to
`List<T>` once at least one row in that batch actually collides.

## Examples

### Easy: two colliding keys

```python
import json_tools_rs as jt

data = {"User_name": "John", "Admin_name": "Jane"}
result = (jt.JSONTools()
    .flatten()
    .key_replacement("r'(User|Admin)_'", "")
    .handle_key_collision(True)
    .execute(data)
)
# {'name': ['John', 'Jane']}  (order not guaranteed)
```

### Medium: collision plus filtering

```python
data = {"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}
result = (jt.JSONTools()
    .flatten()
    .key_replacement("r'(User|Admin|Guest)_'", "")
    .remove_empty_strings(True)
    .handle_key_collision(True)
    .execute(data)
)
# {'name': ['John', 'Bob']}  ("" from Admin_name never reaches the array)
```

### Hard: collision after type conversion, in `.normal()` mode

Type conversion and filtering both run on each value *before* collision resolution
collects it, so the array ends up holding already-converted values, not the original
strings:

```python
data = {"config": {"User_Score": "10", "Admin_Score": "20", "Guest_Score": "30"}}

result = (jt.JSONTools()
    .normal()
    .key_replacement("r'(User|Admin|Guest)_'", "")
    .handle_key_collision(True)
    .auto_convert_types(True)
    .execute(data)
)
# {'config': {'Score': [10, 20, 30]}}
```

All three sibling keys under `config` collapse to `Score`, and each `"10"`/`"20"`/`"30"`
string has already become a JSON number by the time it lands in the array.
