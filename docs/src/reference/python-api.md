# Python API Reference

```python
import json_tools_rs
```

## JSONTools

The main builder class for all JSON operations. All configuration methods return `self` for chaining; only `.execute()` and `.execute_to_output()` trigger processing.

### Construction

```python
tools = json_tools_rs.JSONTools()
```

Creates a new `JSONTools` instance with all default settings. The instance is reusable -- you can call `.execute()` multiple times with different inputs.

### Operation Modes

Exactly one mode must be set before calling `.execute()`. Calling a mode method replaces any previously set mode.

#### `.flatten()`

```python
tools.flatten() -> JSONTools
```

Set the operation to flatten nested JSON into dot-separated (or custom separator) keys.

```python
import json_tools_rs as jt

result = jt.JSONTools().flatten().execute({"a": {"b": {"c": 1}}})
# {"a.b.c": 1}
```

#### `.unflatten()`

```python
tools.unflatten() -> JSONTools
```

Set the operation to reconstruct nested JSON from flat, separator-delimited keys.

```python
result = jt.JSONTools().unflatten().execute({"a.b.c": 1})
# {"a": {"b": {"c": 1}}}
```

#### `.normal()`

```python
tools.normal() -> JSONTools
```

Set the operation to apply transformations (filtering, replacements, type conversion) without changing the nesting structure.

```python
result = jt.JSONTools().normal().lowercase_keys(True).execute({"Name": "Alice"})
# {"name": "Alice"}
```

### Configuration Methods

All configuration methods return `self` for chaining.

#### `.separator(sep)`

```python
tools.separator(sep: str) -> JSONTools
```

Set the key separator for flatten/unflatten operations.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `sep` | `str` | `"."` | Non-empty string used to join/split nested keys |

**Raises:** `ValueError` if `sep` is an empty string.

```python
result = jt.JSONTools().flatten().separator("::").execute({"a": {"b": 1}})
# {"a::b": 1}
```

#### `.lowercase_keys(flag)`

```python
tools.lowercase_keys(flag: bool) -> JSONTools
```

Convert all keys to lowercase after processing.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable or disable lowercase key conversion |

```python
result = jt.JSONTools().flatten().lowercase_keys(True).execute({"User": {"Name": "Alice"}})
# {"user.name": "Alice"}
```

#### `.remove_empty_strings(flag)`

```python
tools.remove_empty_strings(flag: bool) -> JSONTools
```

Remove key-value pairs where the value is an empty string `""`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable or disable empty string removal |

```python
result = jt.JSONTools().flatten().remove_empty_strings(True).execute({"a": "", "b": "hello"})
# {"b": "hello"}
```

#### `.remove_nulls(flag)`

```python
tools.remove_nulls(flag: bool) -> JSONTools
```

Remove key-value pairs where the value is `None` / `null`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable or disable null removal |

```python
result = jt.JSONTools().flatten().remove_nulls(True).execute({"a": None, "b": 1})
# {"b": 1}
```

#### `.remove_empty_objects(flag)`

```python
tools.remove_empty_objects(flag: bool) -> JSONTools
```

Remove key-value pairs where the value is an empty object `{}`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable or disable empty object removal |

#### `.remove_empty_arrays(flag)`

```python
tools.remove_empty_arrays(flag: bool) -> JSONTools
```

Remove key-value pairs where the value is an empty array `[]`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable or disable empty array removal |

#### `.key_replacement(find, replace)`

```python
tools.key_replacement(find: str, replace: str) -> JSONTools
```

Add a key replacement pattern. Patterns are **literal (exact substring match) by
default**; wrap a pattern in `r'...'` (e.g. `"r'^user_'"`) to use standard regex
syntax instead. A malformed `r'...'` pattern is silently treated as "no match" rather
than raising an error. Multiple replacements can be chained.

| Parameter | Type | Description |
|-----------|------|-------------|
| `find` | `str` | Literal string, or `r'...'`-wrapped regex pattern, to match in keys |
| `replace` | `str` | Replacement string (supports regex capture groups like `$1` when `find` is a regex) |

```python
result = (jt.JSONTools()
    .flatten()
    .key_replacement("r'^user_'", "")
    .key_replacement("r'_name$'", "_id")
    .execute({"user_name": "Alice"}))
# {"id": "Alice"}
```

#### `.value_replacement(find, replace)`

```python
tools.value_replacement(find: str, replace: str) -> JSONTools
```

Add a value replacement pattern. Works the same as key replacements (literal by
default, `r'...'` for regex) but applies to string values.

| Parameter | Type | Description |
|-----------|------|-------------|
| `find` | `str` | Literal string, or `r'...'`-wrapped regex pattern, to match in values |
| `replace` | `str` | Replacement string |

```python
result = (jt.JSONTools()
    .flatten()
    .value_replacement("@example.com", "@company.org")
    .execute({"email": "user@example.com"}))
# {"email": "user@company.org"}
```

#### `.exclude_key(pattern)`

```python
tools.exclude_key(pattern: str) -> JSONTools
```

Drop any key -- and its entire value/subtree -- whose name contains `pattern`.
Literal (exact substring match) by default; wrap in `r'...'` for regex, matching
`key_replacement`'s convention. Additive -- call once per keyword to exclude
multiple. Checked against the full dot-path in flatten/unflatten mode, and per key
at each nesting level in normal mode; matching a container key drops its entire
subtree without walking it. Array elements are never matched (no key name to check).

| Parameter | Type | Description |
|-----------|------|-------------|
| `pattern` | `str` | Literal string, or `r'...'`-wrapped regex pattern, to match against key names |

```python
result = (jt.JSONTools()
    .flatten()
    .exclude_key("crypto")
    .execute({"user": {"name": "John", "crypto_wallet": {"coin": "BTC"}}}))
# {"user.name": "John"}
```

#### `.exclude_value(pattern)`

```python
tools.exclude_value(pattern: str) -> JSONTools
```

Drop a key-value pair whose value contains `pattern`. Same literal/`r'...'`
convention as `exclude_key`. Additive. Only ever applies to scalar leaf values
(strings/numbers/booleans/null) -- containers have no single value to check.
Checked against the final value *after* any configured `value_replacement`/
`auto_convert_types` have run. A no-op at the document root.

**Unflatten-specific note**: string values are matched against their JSON-serialized
form (including surrounding quotes), not the unescaped logical text. Literal
patterns are unaffected; a regex with anchors needs `r'^"admin"$'` rather than
`r'^admin$'` to match a value that's exactly `"admin"`.

| Parameter | Type | Description |
|-----------|------|-------------|
| `pattern` | `str` | Literal string, or `r'...'`-wrapped regex pattern, to match against values |

```python
result = (jt.JSONTools()
    .flatten()
    .exclude_value("banned")
    .execute({"user": {"name": "John", "status": "banned"}}))
# {"user.name": "John"}
```

#### `.handle_key_collision(flag)`

```python
tools.handle_key_collision(flag: bool) -> JSONTools
```

When enabled, keys that would collide after transformations (e.g., after lowercasing) are collected into arrays instead of overwriting each other.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable collision handling |

```python
result = (jt.JSONTools()
    .flatten()
    .lowercase_keys(True)
    .handle_key_collision(True)
    .execute({"Name": "Alice", "name": "Bob"}))
# {"name": ["Alice", "Bob"]}
```

#### `.auto_convert_types(flag)`

```python
tools.auto_convert_types(flag: bool) -> JSONTools
```

Automatically convert string values to their native types:

- **Numbers**: `"123"` -> `123`, `"1,234.56"` -> `1234.56`, `"$99.99"` -> `99.99`, `"1e5"` -> `100000`
- **Booleans**: `"true"` / `"TRUE"` / `"True"` -> `true`, `"false"` / `"FALSE"` / `"False"` -> `false`
- **Nulls**: `"null"` / `"None"` -> `null`

If conversion fails, the original string is kept. No errors are raised on conversion failure.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag` | `bool` | `False` | Enable automatic type conversion |

```python
result = (jt.JSONTools()
    .flatten()
    .auto_convert_types(True)
    .execute({"id": "123", "price": "1,234.56", "active": "true"}))
# {"id": 123, "price": 1234.56, "active": true}
```

#### `.convert_dates(enable, normalize_to_utc=None, assume_utc_for_naive=None)` / `.convert_nulls(enable, extra_tokens=None)` / `.convert_booleans(enable, extra_true_tokens=None, extra_false_tokens=None)` / `.convert_numbers(enable, currency=None, percent=None, basis_points=None, suffixes=None, fractions=None, radix=None)`

```python
tools.convert_dates(enable: bool, normalize_to_utc: bool | None = None, assume_utc_for_naive: bool | None = None) -> JSONTools
tools.convert_nulls(enable: bool, extra_tokens: list[str] | None = None) -> JSONTools
tools.convert_booleans(enable: bool, extra_true_tokens: list[str] | None = None, extra_false_tokens: list[str] | None = None) -> JSONTools
tools.convert_numbers(enable: bool, currency: bool | None = None, percent: bool | None = None, basis_points: bool | None = None, suffixes: bool | None = None, fractions: bool | None = None, radix: bool | None = None) -> JSONTools
```

Independent, per-category alternative to `.auto_convert_types()`: enable/customize
dates, nulls, booleans, and numbers separately instead of all-or-nothing.
`auto_convert_types(True)` only flips each category's on/off switch and preserves
customization already set via these methods -- call order doesn't reset it. A kwarg
left as `None` on a later call also preserves whatever a previous call set (it's not
reset to the built-in default).

| Method | Kwarg | Default | Description |
|--------|-------|---------|-------------|
| `convert_dates` | `normalize_to_utc` | `True` | Normalize recognized dates/datetimes to UTC; `False` leaves them unchanged |
| `convert_dates` | `assume_utc_for_naive` | `True` | Append `Z` to timezone-less datetimes; `False` leaves them unchanged |
| `convert_nulls` | `extra_tokens` | `[]` | Additional strings recognized as null, beyond the built-in list (additive) |
| `convert_booleans` | `extra_true_tokens` / `extra_false_tokens` | `[]` | Additional true/false strings, beyond the built-in lists (additive) |
| `convert_numbers` | `currency` | `True` | Currency symbol/code/credit-debit-suffix stripping |
| `convert_numbers` | `percent` | `True` | `%`/permille/per-ten-thousand suffix parsing |
| `convert_numbers` | `basis_points` | `True` | Text basis-point suffixes (`"25bps"`) |
| `convert_numbers` | `suffixes` | `True` | K/M/B/T magnitude suffixes |
| `convert_numbers` | `fractions` | `True` | Fractions (`"1/2"`) |
| `convert_numbers` | `radix` | `True` | Hex/binary/octal literals (`"0x1A"`) |

Plain integers/decimals, scientific notation, and thousands-separator cleanup are
always applied when `convert_numbers` is enabled, regardless of the other kwargs.

```python
result = (jt.JSONTools()
    .flatten()
    .convert_dates(True, assume_utc_for_naive=False)
    .convert_nulls(True, extra_tokens=["missing"])
    .execute({"d": "2024-01-15T10:30:00", "a": "missing"}))
# {"d": "2024-01-15T10:30:00", "a": None}
```

#### `.parallel_threshold(n)`

```python
tools.parallel_threshold(n: int) -> JSONTools
```

Set the minimum batch size to trigger parallel processing. Batches smaller than this are processed sequentially to avoid thread-spawning overhead.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n` | `int` | `100` | Minimum batch size for parallelism |

Default can be overridden with the `JSON_TOOLS_PARALLEL_THRESHOLD` environment variable.

```python
tools = jt.JSONTools().flatten().parallel_threshold(50)
```

#### `.num_threads(n)`

```python
tools.num_threads(n: int | None) -> JSONTools
```

Set the number of threads used for parallel processing. Pass `None` (or omit the call) to use the system default.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n` | `int \| None` | `None` (CPU count) | Number of worker threads |

Default can be overridden with the `JSON_TOOLS_NUM_THREADS` environment variable.

```python
tools = jt.JSONTools().flatten().num_threads(4)
```

#### `.nested_parallel_threshold(n)`

```python
tools.nested_parallel_threshold(n: int) -> JSONTools
```

Set the minimum number of keys/items within a single JSON document to trigger nested (intra-document) parallelism. Only objects or arrays exceeding this count are parallelized internally.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n` | `int` | `100` | Minimum keys/items for nested parallelism |

Default can be overridden with the `JSON_TOOLS_NESTED_PARALLEL_THRESHOLD` environment variable.

```python
tools = jt.JSONTools().flatten().nested_parallel_threshold(200)
```

#### `.max_array_index(n)`

```python
tools.max_array_index(n: int) -> JSONTools
```

Set the maximum array index allowed during unflattening. This is a DoS protection: a malicious key like `"items.999999999"` would otherwise allocate a massive array.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `n` | `int` | `100000` | Maximum array index |

Default can be overridden with the `JSON_TOOLS_MAX_ARRAY_INDEX` environment variable.

### Execution Methods

#### `.execute(input)`

```python
tools.execute(input) -> str | dict | list[str] | list[dict] | DataFrame | Series
```

Execute the configured operation. The return type mirrors the input type:

| Input Type | Output Type |
|------------|-------------|
| `str` | `str` (JSON string) |
| `dict` | `dict` (Python dictionary) |
| `list[str]` | `list[str]` |
| `list[dict]` | `list[dict]` |
| `pandas.DataFrame` | `pandas.DataFrame` |
| `pandas.Series` | `pandas.Series` |
| `polars.DataFrame` | `polars.DataFrame` |
| `polars.Series` | `polars.Series` |
| `pyarrow.Table` | `pyarrow.Table` |
| `pyarrow.ChunkedArray` | `pyarrow.Array` (reconstructed via `pyarrow.array()`, not re-chunked) |
| `pyspark.sql.DataFrame` | `list[dict]` (no `SparkSession` round-trip available for reconstruction; falls back to a plain list) |

**Raises:** `JsonToolsError` if no mode is set, input is invalid, or processing fails.

```python
# String input -> string output
result = jt.JSONTools().flatten().execute('{"a": {"b": 1}}')
assert isinstance(result, str)

# Dict input -> dict output
result = jt.JSONTools().flatten().execute({"a": {"b": 1}})
assert isinstance(result, dict)

# Batch string input -> batch string output
results = jt.JSONTools().flatten().execute(['{"a": 1}', '{"b": 2}'])
assert isinstance(results, list) and isinstance(results[0], str)

# Batch dict input -> batch dict output
results = jt.JSONTools().flatten().execute([{"a": {"b": 1}}, {"c": {"d": 2}}])
assert isinstance(results, list) and isinstance(results[0], dict)
```

#### `.execute_to_output(input)`

```python
tools.execute_to_output(input) -> JsonOutput
```

Execute the operation but return a `JsonOutput` wrapper instead of native Python types. Useful when you need to inspect whether the result is single or multiple before extracting.

**Note:** DataFrame and Series inputs are not supported with `execute_to_output()`. Use `.execute()` for those types.

| Parameter | Type | Description |
|-----------|------|-------------|
| `input` | `str`, `dict`, `list[str]`, `list[dict]` | JSON data to process |

```python
output = jt.JSONTools().flatten().execute_to_output('{"a": {"b": 1}}')
if output.is_single:
    print(output.get_single())
elif output.is_multiple:
    for item in output.get_multiple():
        print(item)
```

## JsonOutput

Output wrapper returned by `.execute_to_output()`. Provides typed access to results.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `.is_single` | `bool` | `True` if the result contains a single JSON string |
| `.is_multiple` | `bool` | `True` if the result contains multiple JSON strings |

### Methods

#### `.get_single()`

```python
output.get_single() -> str
```

Extract the single JSON string result.

**Raises:** `ValueError` if the result is multiple.

#### `.get_multiple()`

```python
output.get_multiple() -> list[str]
```

Extract the list of JSON string results.

**Raises:** `ValueError` if the result is single.

#### `.to_python()`

```python
output.to_python() -> str | list[str]
```

Convert to native Python type: returns `str` for single results, `list[str]` for multiple results.

### String Representations

`str(output)` returns the JSON string (single) or a list representation (multiple).
`repr(output)` returns `JsonOutput.Single('...')` or `JsonOutput.Multiple([...])`.

## DataFrame and Series Support

JSON Tools RS natively supports Pandas, Polars, PyArrow, and PySpark DataFrames and Series. Detection is performed via duck typing -- no explicit imports are required.

### Pandas DataFrame

Each **row** is serialized to a JSON object (column names become keys) and processed as a whole document -- so flattening finds nested structure in columns holding actual nested Python objects (dicts/lists), not columns holding pre-serialized JSON-text strings (a string column's value is just a JSON string scalar, with nothing to flatten inside it).

```python
import pandas as pd
import json_tools_rs as jt

df = pd.DataFrame({"user": [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25},
]})

tools = jt.JSONTools().flatten().separator(".")

# Each row -> {"user": {"name": ..., "age": ...}} -> flattened
result_df = tools.execute(df)
# Returns a DataFrame with flattened columns: "user.name", "user.age"
```

### Pandas Series

```python
series = pd.Series([
    '{"a": {"b": 1}}',
    '{"a": {"b": 2}}',
])

result_series = jt.JSONTools().flatten().execute(series)
# Returns a Series of flattened JSON strings
```

### Polars DataFrame

Like Pandas, this flattens a column of nested `Struct` values -- a column of JSON-text strings round-trips unchanged, since there's no nested structure inside a string scalar for `.flatten()` to find.

```python
import polars as pl

df = pl.DataFrame({
    "user": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]
})

result_df = jt.JSONTools().flatten().execute(df)
# result_df columns: ["user.name", "user.age"]
```

### Polars Series

```python
series = pl.Series("data", [
    '{"a": {"b": 1}}',
    '{"a": {"b": 2}}',
])

result_series = jt.JSONTools().flatten().execute(series)
```

### PyArrow Table

Same rule as Pandas/Polars: flatten a `struct`-typed column, not a plain string column holding JSON text.

```python
import pyarrow as pa

table = pa.table({
    "user": pa.array([{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}])
})

result_table = jt.JSONTools().flatten().execute(table)
# result_table columns: ["user.name", "user.age"]
```

### PySpark DataFrame

`.execute(df)` collects the DataFrame to the driver via `toPandas()`, runs it through the same row-is-a-JSON-object pipeline as Pandas (so a `StructType` column flattens; a plain string column does not), and -- because there's no `SparkSession` available inside the native call to rebuild a distributed `DataFrame` -- returns a plain **`list[dict]`**, not a new `pyspark.sql.DataFrame`.

```python
from pyspark.sql import SparkSession, Row

spark = SparkSession.builder.getOrCreate()
df = spark.createDataFrame([
    Row(user=Row(name="Alice", age=30)),
    Row(user=Row(name="Bob", age=25)),
])

result = jt.JSONTools().flatten().execute(df)
# result is a list[dict], e.g. [{"user.name": "Alice", "user.age": 30}, ...]
```

## JsonToolsError

Exception class for all errors raised by JSON Tools RS.

```python
import json_tools_rs as jt

try:
    result = jt.JSONTools().flatten().execute("not valid json")
except jt.JsonToolsError as e:
    print(f"Error: {e}")
    # Error: Failed to process JSON string: [E001] JSON parsing failed: ...
```

Error messages embed a machine-readable code (`E001`-`E008`) in square brackets. Note that the Python bindings prepend their own context before the underlying Rust message (e.g. `"Failed to process JSON string: "`, `"Failed to process Python dict: "`), so the code is not always the very first characters of `str(e)` -- check for `"[E00x]"` as a substring rather than a prefix. See [Error Codes](./error-codes.md) for the full reference.

### Error Codes Quick Reference

| Code | Name | Common Cause |
|------|------|-------------|
| `E001` | `JsonParseError` | Invalid JSON input |
| `E002` | `RegexError` | Bad regex in key/value replacement |
| `E003` | `InvalidReplacementPattern` | Malformed replacement pair |
| `E004` | `InvalidJsonStructure` | Wrong JSON shape for the operation |
| `E005` | `ConfigurationError` | No mode set before `.execute()` |
| `E006` | `BatchProcessingError` | Error in one item during batch processing |
| `E007` | `InputValidationError` | Unsupported input type |
| `E008` | `SerializationError` | Internal serialization failure |

### Handling Specific Errors

```python
import json_tools_rs as jt

try:
    result = jt.JSONTools().execute({"a": 1})  # No mode set
except jt.JsonToolsError as e:
    msg = str(e)
    if "[E005]" in msg:
        print("Forgot to call .flatten() or .unflatten()")
    elif "[E001]" in msg:
        print("Invalid JSON input")
```

## Complete Example

```python
import json_tools_rs as jt

# Build once, reuse many times
tools = (jt.JSONTools()
    .flatten()
    .separator("::")
    .lowercase_keys(True)
    .remove_nulls(True)
    .remove_empty_strings(True)
    .key_replacement("r'^user_'", "")
    .auto_convert_types(True)
    .parallel_threshold(50)
    .num_threads(4)
)

# Single dict
result = tools.execute({"User_Name": "Alice", "User_Age": "30"})
# {"name": "Alice", "age": 30}

# Batch of dicts (processed in parallel if >= 50 items)
results = tools.execute([{"data": str(i)} for i in range(1000)])

# JSON string
result = tools.execute('{"User_Name": "Alice", "nested": {"User_Age": "30"}}')

# DataFrame -- each row becomes {"User_Name": ..., "User_Age": ...} (column names
# are the JSON keys); see "DataFrame and Series Support" above for why a column of
# nested dict/struct values flattens but a column of JSON-text strings does not
import pandas as pd
df = pd.DataFrame({
    "User_Name": ["Alice", "Bob"],
    "User_Age": ["30", "25"],
})
df_result = tools.execute(df)
```
