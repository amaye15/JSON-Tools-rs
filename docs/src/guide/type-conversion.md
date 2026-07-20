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

```java
try (JsonToolsHandle tools = JsonTools.builder().flatten().autoConvertTypes(true).build()) {
    String result = tools.execute(json);
}
```

## Fine-Grained Control

`auto_convert_types(true)` turns on all four categories below (dates, nulls,
booleans, numbers) with their default behavior. For independent control -- enabling
only some categories, or customizing how a category matches -- use the per-category
methods instead. Each accepts a plain `bool` for the common case, plus an optional
customized config for the less common case.

```rust
use json_tools_rs::{JSONTools, DateConversionConfig, NullConversionConfig};

// Only convert numbers; leave dates/nulls/booleans as plain strings.
let result = JSONTools::new()
    .flatten()
    .convert_numbers(true)
    .execute(json)?;

// Customize a category: don't assume UTC for timezone-less datetimes, and
// recognize an extra null token.
let result = JSONTools::new()
    .flatten()
    .convert_dates_config(DateConversionConfig::new().enabled(true).assume_utc_for_naive(false))
    .convert_nulls_config(NullConversionConfig::new().enabled(true).add_extra_token("missing"))
    .execute(json)?;
```

```python
# Only convert booleans.
result = jt.JSONTools().flatten().convert_booleans(True).execute(data)

# Customize with kwargs -- unset kwargs preserve whatever a previous call set.
result = (
    jt.JSONTools()
    .flatten()
    .convert_dates(True, assume_utc_for_naive=False)
    .convert_nulls(True, extra_tokens=["missing"])
    .execute(data)
)
```

```java
try (JsonToolsHandle tools = JsonTools.builder()
        .flatten()
        .convertDates(true)
        .dateAssumeUtcForNaive(false)
        .convertNulls(true)
        .nullExtraToken("missing")
        .build()) {
    String result = tools.execute(json);
}
```

Calling `auto_convert_types(bool)` only ever flips each category's own on/off
switch -- it never resets a category's customization back to its own defaults. So
`.convert_dates_config(...).auto_convert_types(true)` keeps the customization while
turning every category on, regardless of call order.

### Per-Category Reference

| Category | Rust | Python | Java |
|----------|------|--------|------|
| Dates | `.convert_dates(bool)` / `.convert_dates_config(DateConversionConfig)` | `.convert_dates(enable, normalize_to_utc=None, assume_utc_for_naive=None)` | `.convertDates(boolean)` / `.dateNormalizeToUtc(boolean)` / `.dateAssumeUtcForNaive(boolean)` |
| Nulls | `.convert_nulls(bool)` / `.convert_nulls_config(NullConversionConfig)` | `.convert_nulls(enable, extra_tokens=None)` | `.convertNulls(boolean)` / `.nullExtraToken(String)` |
| Booleans | `.convert_booleans(bool)` / `.convert_booleans_config(BooleanConversionConfig)` | `.convert_booleans(enable, extra_true_tokens=None, extra_false_tokens=None)` | `.convertBooleans(boolean)` / `.booleanExtraTrueToken(String)` / `.booleanExtraFalseToken(String)` |
| Numbers | `.convert_numbers(bool)` / `.convert_numbers_config(NumberConversionConfig)` | `.convert_numbers(enable, currency=None, percent=None, basis_points=None, suffixes=None, fractions=None, radix=None)` | `.convertNumbers(boolean)` / `.numberCurrency(boolean)` / `.numberPercent(boolean)` / `.numberBasisPoints(boolean)` / `.numberSuffixes(boolean)` / `.numberFractions(boolean)` / `.numberRadix(boolean)` |

**Dates** (`DateConversionConfig`):
- `normalize_to_utc` (default `true`) -- when `false`, a recognized date/datetime is left byte-for-byte unchanged (still protected from being misread as a number).
- `assume_utc_for_naive` (default `true`) -- when `false`, a timezone-less datetime (e.g. `"2024-01-15T10:30:00"`) is left unchanged instead of getting a `Z` appended.

**Nulls** (`NullConversionConfig`) / **Booleans** (`BooleanConversionConfig`):
- `extra_tokens` / `extra_true_tokens` / `extra_false_tokens` -- additional strings recognized beyond the built-in list. **Additive only**: the built-in list stays active regardless, this only extends it. Matched exactly (case-sensitive) against the *trimmed* value -- consistent with every other category and the built-in lists (e.g. `" 123 "` already converts to `123`), so a token like `"si"` also matches `"si "` (trailing whitespace), not only a byte-for-byte match against the raw string. In Rust, add one token per call (`.add_extra_token("missing")`, matching `key_replacement()`'s idiom); in Python, `extra_tokens=[...]` is bulk-replace (a later call's list replaces, not merges with, an earlier one); in Java, add one token per call (`.nullExtraToken("missing")`, additive like Rust).

**Numbers** (`NumberConversionConfig`) -- plain integers/decimals, scientific
notation, and thousands-separator cleanup are always applied when the category is
enabled (no one asked to disable unambiguous number parsing); the remaining
sub-formats can each be disabled independently since they're more "opinionated"
(each can reinterpret a string that wasn't meant to be a number):
- `currency` (default `true`) -- currency symbol/code/credit-debit-suffix stripping.
- `percent` (default `true`) -- `%`/permille/per-ten-thousand suffix parsing.
- `basis_points` (default `true`) -- text basis-point suffixes (`"25bps"`).
- `suffixes` (default `true`) -- K/M/B/T magnitude suffixes.
- `fractions` (default `true`) -- fractions (`"1/2"`).
- `radix` (default `true`) -- hex/binary/octal literals (`"0x1A"`).

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

## Examples

### Easy: turn it on

```python
import json_tools_rs as jt

data = {"count": "42", "active": "true", "rate": "3.14"}
result = jt.JSONTools().normal().auto_convert_types(True).execute(data)
# {'count': 42, 'active': True, 'rate': 3.14}
```

### Medium: one category, customized

Disable currency stripping specifically, while keeping percent/basis-point/suffix/
fraction/radix parsing and every other category untouched:

```python
data = {"price": "$1,234.56", "big_id": "999999999999999999", "pct": "12.5%"}
result = jt.JSONTools().flatten().convert_numbers(True, currency=False).execute(data)
# {'price': '$1,234.56', 'big_id': 999999999999999999, 'pct': 12.5}
```

`price` is left as the original string (currency parsing is off), `pct` still
converts (percent parsing is unaffected), and `big_id` -- a 19-digit ID -- converts
losslessly to a JSON integer instead of a precision-corrupted `f64`.

### Hard: mixed categories with per-category config

```rust
use json_tools_rs::{JSONTools, DateConversionConfig, NullConversionConfig};

let json = r#"{
    "created": "2024-01-15T10:30:00",
    "status": "missing",
    "score": "1,234.50",
    "code": "EUR999"
}"#;

let result = JSONTools::new()
    .flatten()
    .convert_dates_config(DateConversionConfig::new().enabled(true).assume_utc_for_naive(false))
    .convert_nulls_config(NullConversionConfig::new().enabled(true).add_extra_token("missing"))
    .convert_numbers(true)
    .execute(json)?;

// {
//   "created": "2024-01-15T10:30:00",  // naive datetime left untouched (assume_utc_for_naive: false)
//   "status": null,                    // "missing" recognized via the extra token
//   "score": 1234.5,                   // US thousands separator parsed
//   "code": "EUR999"                   // no space before the 3-letter code -- left as a string
// }
```

Booleans are never enabled here (`auto_convert_types` was not called, and
`convert_booleans` was never turned on), so a value like `"yes"` elsewhere in the same
document would be left as a plain string -- categories are independent switches, not
an all-or-nothing package once you're using the per-category methods.
