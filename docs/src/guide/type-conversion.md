# Automatic Type Conversion

When `.auto_convert_types(true)` is enabled, string values are automatically converted to their appropriate types.

## Enabling

```rust
let result = JSONTools::new()
    .flatten()
    .auto_convert_types(true)
    .execute(json)?;
```

```python
result = jt.JSONTools().flatten().auto_convert_types(True).execute(data)
```

## Conversion Rules

Conversions are applied in priority order: **dates -> nulls -> booleans -> numbers**.

### Dates (ISO-8601)

Date strings are detected and normalized to UTC:

| Input | Output |
|-------|--------|
| `"2024-01-15"` | `"2024-01-15"` (kept as-is, not a number) |
| `"2024-01-15T10:30:00+05:00"` | `"2024-01-15T05:30:00Z"` (UTC normalized) |
| `"2024-01-15T10:30:00Z"` | `"2024-01-15T10:30:00Z"` |
| `"2024-01-15T10:30:00"` | `"2024-01-15T10:30:00"` (naive, kept as-is) |

### Nulls

| Input | Output |
|-------|--------|
| `"null"`, `"NULL"` | `null` |
| `"nil"`, `"NIL"` | `null` |
| `"none"`, `"NONE"` | `null` |
| `"N/A"`, `"n/a"` | `null` |

### Booleans

| Input | Output |
|-------|--------|
| `"true"`, `"TRUE"`, `"True"` | `true` |
| `"false"`, `"FALSE"`, `"False"` | `false` |
| `"yes"`, `"YES"` | `true` |
| `"no"`, `"NO"` | `false` |
| `"on"`, `"ON"` | `true` |
| `"off"`, `"OFF"` | `false` |
| `"y"`, `"Y"` | `true` |
| `"n"`, `"N"` | `false` |

> Note: `"1"` and `"0"` are treated as numbers, not booleans.

### Numbers

| Format | Input | Output |
|--------|-------|--------|
| Basic integers | `"123"` | `123` |
| Decimals | `"45.67"` | `45.67` |
| Negative | `"-10"` | `-10` |
| US thousands | `"1,234.56"` | `1234.56` |
| EU thousands | `"1.234,56"` | `1234.56` |
| Space separators | `"1 234.56"` | `1234.56` |
| Currency | `"$1,234.56"`, `"EUR999"` | `1234.56`, `999` |
| Percentages | `"50%"`, `"12.5%"` | `50.0`, `12.5` |
| Scientific | `"1e5"`, `"1.23e-4"` | `100000`, `0.000123` |
| Basis points | `"50bps"`, `"100 bp"` | `0.005`, `0.01` |
| Suffixes | `"1K"`, `"2.5M"`, `"5B"` | `1000`, `2500000`, `5000000000` |

### Non-Convertible Strings

Strings that don't match any pattern are left as-is:

```json
{"name": "Alice", "code": "ABC"} -> {"name": "Alice", "code": "ABC"}
```

## Full Example

```rust
let json = r#"{
    "id": "123",
    "price": "$1,234.56",
    "discount": "15%",
    "active": "yes",
    "created": "2024-01-15T10:30:00+05:00",
    "status": "N/A",
    "name": "Product"
}"#;

let result = JSONTools::new()
    .flatten()
    .auto_convert_types(true)
    .execute(json)?;

// {
//   "id": 123,
//   "price": 1234.56,
//   "discount": 15.0,
//   "active": true,
//   "created": "2024-01-15T05:30:00Z",
//   "status": null,
//   "name": "Product"
// }
```
