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
| `"2024-01-15T10:30:00"` | `"2024-01-15T10:30:00Z"` (naive, assumed UTC -- `Z` appended, no shift) |

### Nulls

| Input | Output |
|-------|--------|
| `"null"`, `"NULL"`, `"Null"` | `null` |
| `"nil"`, `"NIL"`, `"Nil"` | `null` |
| `"none"`, `"NONE"`, `"None"` | `null` |
| `"N/A"`, `"n/a"` | `null` |
| `"NA"`, `"na"` | `null` |

### Booleans

| Input | Output |
|-------|--------|
| `"true"`, `"TRUE"`, `"True"` | `true` |
| `"false"`, `"FALSE"`, `"False"` | `false` |
| `"yes"`, `"YES"`, `"Yes"` | `true` |
| `"no"`, `"NO"`, `"No"` | `false` |
| `"on"`, `"ON"`, `"On"` | `true` |
| `"off"`, `"OFF"`, `"Off"` | `false` |
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
| Currency | `"$1,234.56"`, `"EUR 999"` | `1234.56`, `999` |
| Percentages | `"50%"`, `"12.5%"` | `50`, `12.5` |
| Scientific | `"1e5"`, `"1.23e-4"` | `100000`, `0.000123` |
| Basis points | `"50bps"`, `"100 bp"` | `0.005`, `0.01` |
| Suffixes | `"1K"`, `"2.5M"`, `"5B"` | `1000`, `2500000`, `5000000000` |

> Note: a 3-letter currency code (`USD`, `EUR`, `GBP`, etc.) is only stripped when
> followed by a space -- `"EUR 999"` converts to `999`, but `"EUR999"` (no space) does
> not match and is left as the original string, to avoid misinterpreting things like
> alphanumeric product codes.

> Note: 64-bit integer strings (e.g. Snowflake/Discord/database bigint IDs, commonly
> 17-19 digits) convert losslessly -- `"999999999999999999"` becomes the JSON integer
> `999999999999999999`, not a precision-corrupted `f64` approximation.

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
//   "discount": 15,
//   "active": true,
//   "created": "2024-01-15T05:30:00Z",
//   "status": null,
//   "name": "Product"
// }
```
