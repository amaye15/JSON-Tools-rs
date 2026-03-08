# Key & Value Replacements

Replace patterns in keys and/or values using literal strings or regular expressions.

## Key Replacements

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("user_profile_", "")  // Literal
    .key_replacement("regex:(User|Admin)_", "")  // Regex
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("user_profile_", "")
    .key_replacement("regex:(User|Admin)_", "")
    .execute(data)
)
```

## Value Replacements

```rust
let result = JSONTools::new()
    .flatten()
    .value_replacement("@example.com", "@company.org")  // Literal
    .value_replacement("regex:^super$", "administrator")  // Regex
    .execute(json)?;
```

## Regex Syntax

Prefix patterns with `regex:` to use regular expressions. The regex engine uses standard Rust regex syntax.

| Pattern | Description |
|---------|-------------|
| `"old"` | Literal string replacement |
| `"regex:^prefix_"` | Regex: match start of string |
| `"regex:(a\|b)_"` | Regex: alternation |
| `"regex:\\d+"` | Regex: digit sequences |

## Multiple Replacements

You can chain multiple key and value replacements. They are applied in order:

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("prefix_", "")
    .key_replacement("_suffix", "")
    .key_replacement("_", ".")
    .value_replacement("@old.com", "@new.com")
    .value_replacement("regex:^admin$", "administrator")
    .execute(json)?;
```

## Real-World Example

Normalizing an API response:

```rust
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .key_replacement("regex:(api_response|user_data)::", "")
    .key_replacement("_", ".")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(true)
    .remove_nulls(true)
    .execute(api_response)?;
```
