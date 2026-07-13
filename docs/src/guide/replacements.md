# Key & Value Replacements

Replace patterns in keys and/or values using literal strings or regular expressions.

## Key Replacements

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("user_profile_", "")  // Literal
    .key_replacement("r'(User|Admin)_'", "")  // Regex
    .execute(json)?;
```

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("user_profile_", "")
    .key_replacement("r'(User|Admin)_'", "")
    .execute(data)
)
```

## Value Replacements

```rust
let result = JSONTools::new()
    .flatten()
    .value_replacement("@example.com", "@company.org")  // Literal
    .value_replacement("r'^super$'", "administrator")  // Regex
    .execute(json)?;
```

## Regex Syntax

Wrap a pattern in `r'...'` (e.g. `r'^prefix_'`) to use it as a regular expression. Any
pattern *not* wrapped this way is matched as a literal, exact substring -- including
patterns that contain characters that would otherwise be regex metacharacters (`.`, `$`,
`(`, etc.). The regex engine uses standard Rust regex syntax.

| Pattern | Description |
|---------|-------------|
| `"old"` | Literal string replacement |
| `"r'^prefix_'"` | Regex: match start of string |
| `"r'(a\|b)_'"` | Regex: alternation |
| `"r'\\d+'"` | Regex: digit sequences |

A malformed `r'...'` pattern (invalid regex syntax) is silently treated as "no match" for
that pattern rather than raising an error -- test your patterns to confirm they compile as
intended.

## Multiple Replacements

You can chain multiple key and value replacements. They are applied in order:

```rust
let result = JSONTools::new()
    .flatten()
    .key_replacement("prefix_", "")
    .key_replacement("_suffix", "")
    .key_replacement("_", ".")
    .value_replacement("@old.com", "@new.com")
    .value_replacement("r'^admin$'", "administrator")
    .execute(json)?;
```

## Real-World Example

Normalizing an API response:

```rust
let result = JSONTools::new()
    .flatten()
    .separator("::")
    .lowercase_keys(true)
    .key_replacement("r'(api_response|user_data)::'", "")
    .key_replacement("_", ".")
    .value_replacement("@example.com", "@company.org")
    .remove_empty_strings(true)
    .remove_nulls(true)
    .execute(api_response)?;
```
