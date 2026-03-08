# Key Collision Handling

When key replacements or transformations cause multiple keys to map to the same output key, collision handling determines what happens.

## Enabling Collision Handling

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("regex:(User|Admin)_", "")
    .handle_key_collision(true)
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("regex:(User|Admin)_", "")
    .handle_key_collision(True)
    .execute(data)
)
```

## How It Works

With `.handle_key_collision(true)`, when two keys collide after transformation, their values are collected into an array:

```json
// Input
{"User_name": "John", "Admin_name": "Jane"}

// With key_replacement("regex:(User|Admin)_", "") + handle_key_collision(true)
// Output
{"name": ["John", "Jane"]}
```

Without collision handling, the last value wins (overwrites previous values).

## Collision with Filtering

Collision handling respects filters. If a colliding value would be filtered out (e.g., empty string with `.remove_empty_strings(true)`), it is excluded from the collected array:

```json
// Input
{"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}

// With remove_empty_strings(true) + handle_key_collision(true)
// Output
{"name": ["John", "Bob"], "guest_name": "Bob"}
```

## Works with Both Modes

Collision handling works during both `.flatten()` and `.unflatten()` operations.
