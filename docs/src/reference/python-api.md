# Python API Reference

```python
import json_tools_rs
```

## JSONTools

The main builder class. All methods return `self` for chaining except `.execute()`.

### Construction

```python
tools = json_tools_rs.JSONTools()
```

### Operation Modes

| Method | Description |
|--------|-------------|
| `.flatten()` | Flatten nested JSON |
| `.unflatten()` | Reconstruct nested JSON |
| `.normal()` | Transform without changing structure |

### Configuration Methods

| Method | Argument | Description |
|--------|----------|-------------|
| `.separator(sep)` | `str` | Key separator (default: `"."`) |
| `.lowercase_keys(flag)` | `bool` | Lowercase all keys |
| `.remove_empty_strings(flag)` | `bool` | Remove `""` values |
| `.remove_nulls(flag)` | `bool` | Remove `None` values |
| `.remove_empty_objects(flag)` | `bool` | Remove `{}` values |
| `.remove_empty_arrays(flag)` | `bool` | Remove `[]` values |
| `.key_replacement(find, replace)` | `str, str` | Key pattern replacement |
| `.value_replacement(find, replace)` | `str, str` | Value pattern replacement |
| `.handle_key_collision(flag)` | `bool` | Collect collisions into lists |
| `.auto_convert_types(flag)` | `bool` | Auto-convert string types |
| `.parallel_threshold(n)` | `int` | Min batch size for parallelism |
| `.num_threads(n)` | `int` | Thread count |
| `.nested_parallel_threshold(n)` | `int` | Min entries for nested parallelism |

### Execution

```python
result = tools.execute(input)
```

#### Input Types and Corresponding Output Types

| Input | Output |
|-------|--------|
| `str` | `str` |
| `dict` | `dict` |
| `list[str]` | `list[str]` |
| `list[dict]` | `list[dict]` |
| `pandas.DataFrame` | `pandas.DataFrame` |
| `polars.DataFrame` | `polars.DataFrame` |
| `pyarrow.Table` | `pyarrow.Table` |
| `pyspark.sql.DataFrame` | `pyspark.sql.DataFrame` |
| `pandas.Series` | `pandas.Series` |
| `polars.Series` | `polars.Series` |
| `pyarrow.ChunkedArray` | `pyarrow.ChunkedArray` |

## JsonToolsError

Exception class for all errors.

```python
try:
    result = tools.execute(invalid_data)
except json_tools_rs.JsonToolsError as e:
    print(f"Error: {e}")
```

## JsonOutput

Output wrapper (used internally; `.execute()` returns native Python types directly in most cases).

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
    .key_replacement("regex:^user_", "")
    .auto_convert_types(True)
    .parallel_threshold(50)
    .num_threads(4)
)

# Single dict
result = tools.execute({"User_Name": "Alice", "User_Age": "30"})

# Batch of dicts
results = tools.execute([{"data": i} for i in range(1000)])

# DataFrame
import pandas as pd
df_result = tools.execute(pd.DataFrame([{"a": {"b": 1}}, {"a": {"b": 2}}]))
```
