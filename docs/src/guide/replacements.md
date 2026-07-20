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

## Key Exclusion

Unlike `key_replacement` (which renames matched text within a key), `exclude_key` drops
the entire key -- and its whole value/subtree -- from the output. Matching a container
key removes everything under it, without those nested keys needing to match themselves:

```rust
let json = r#"{"user": {"name": "John", "crypto_wallet": {"coin": "BTC", "balance": 100}}}"#;
let result = JSONTools::new()
    .flatten()
    .exclude_key("crypto")  // Literal
    .exclude_key("r'^secret_'")  // Regex
    .execute(json)?;
// Output: {"user.name": "John"}
```

```python
result = (jt.JSONTools()
    .flatten()
    .exclude_key("crypto")
    .execute(data)
)
```

Works identically in `.flatten()`, `.unflatten()`, and `.normal()` mode: checked against
the full dot-path in flatten/unflatten mode, and per key at each nesting level in normal
mode. Additive -- call it once per keyword to exclude multiple. Array elements are never
matched, since they have no key name to check.

## Value Exclusion

`exclude_value` is `exclude_key`'s counterpart: it drops a key-value pair based on the
**value**'s content instead of the key's name.

```rust
let json = r#"{"user": {"name": "John", "status": "banned"}}"#;
let result = JSONTools::new()
    .flatten()
    .exclude_value("banned")  // Literal
    .exclude_value("r'^flag_'")  // Regex
    .execute(json)?;
// Output: {"user.name": "John"}
```

```python
result = (jt.JSONTools()
    .flatten()
    .exclude_value("banned")
    .execute(data)
)
```

Unlike `exclude_key`, this only ever applies to **scalar leaf values** (strings, numbers,
booleans, `null`) -- containers have no single value to check, so an object or array is
never itself excluded; only its individual scalar leaves can be. The check runs *after*
any configured `value_replacement`/`auto_convert_types` have run, so a value that only
matches after being replaced or converted is still caught. It's a no-op at the document
root, since there's no parent key to drop the value from.

**Unflatten-specific note**: string values are matched against their JSON-serialized
form, including the surrounding quotes -- not the unescaped logical text. A literal
pattern is unaffected by this (quotes don't change substring matching), but a regex with
anchors needs to account for them: use `r'^"admin"$'`, not `r'^admin$'`, to match a value
that's exactly `"admin"` in `.unflatten()` mode.

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
| `"r'\d+'"` | Regex: digit sequences |

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
