# Quick Start (Python)

The Python bindings provide the same `JSONTools` API with **perfect type matching**: input type equals output type.

## Type Preservation

| Input Type | Output Type |
|-----------|------------|
| `str` | `str` (JSON string) |
| `dict` | `dict` |
| `list[str]` | `list[str]` |
| `list[dict]` | `list[dict]` |
| `DataFrame` | `DataFrame` (Pandas, Polars, PyArrow, PySpark) |
| `Series` | `Series` (Pandas, Polars, PyArrow) |

## Basic Flattening

```python
import json_tools_rs as jt

# Dict input -> dict output
result = jt.JSONTools().flatten().execute({"user": {"name": "John", "age": 30}})
print(result)  # {'user.name': 'John', 'user.age': 30}

# String input -> string output
result = jt.JSONTools().flatten().execute('{"user": {"name": "John"}}')
print(result)  # '{"user.name":"John"}'
```

## Basic Unflattening

```python
import json_tools_rs as jt

result = jt.JSONTools().unflatten().execute({"user.name": "John", "user.age": 30})
print(result)  # {'user': {'name': 'John', 'age': 30}}
```

## Advanced Configuration

```python
import json_tools_rs as jt

tools = (jt.JSONTools()
    .flatten()
    .separator("::")
    .lowercase_keys(True)
    .remove_empty_strings(True)
    .remove_nulls(True)
    .key_replacement("r'^user_'", "")
    .auto_convert_types(True)
)

data = {"User_Name": "Alice", "User_Age": "30", "User_Status": None}
result = tools.execute(data)
print(result)  # {'name': 'Alice', 'age': 30}
```

## Batch Processing

```python
import json_tools_rs as jt

tools = jt.JSONTools().flatten()

# List of dicts -> list of dicts
results = tools.execute([
    {"user": {"name": "Alice"}},
    {"user": {"name": "Bob"}},
])
print(results)  # [{'user.name': 'Alice'}, {'user.name': 'Bob'}]

# List of strings -> list of strings
results = tools.execute(['{"a": {"b": 1}}', '{"c": {"d": 2}}'])
print(results)  # ['{"a.b":1}', '{"c.d":2}']
```

## DataFrame Support

```python
import json_tools_rs as jt
import pandas as pd

df = pd.DataFrame([
    {"user": {"name": "Alice", "age": 30}},
    {"user": {"name": "Bob", "age": 25}},
])

result = jt.JSONTools().flatten().execute(df)
print(type(result))  # <class 'pandas.core.frame.DataFrame'>
# Also works with Polars, PyArrow Tables, and PySpark DataFrames
```

## Normalise: Always Get a DataFrame

Sometimes you don't have a DataFrame to start with -- just a dict, a string, or a
list of records -- but you still want a wide table back. `normalise=True` does
that regardless of input shape, with `target` picking the library (or auto-resolving
if omitted):

```python
import json_tools_rs as jt

tools = jt.JSONTools().flatten()

# A bare dict -> a 1-row DataFrame, no DataFrame input required
df = tools.execute({"user": {"name": "Alice", "age": 30}}, normalise=True)
print(df)
#   user.name  user.age
# 0     Alice        30

# A list of differently-shaped records -> unioned, null-filled columns
data = [{"a": 1, "b": {"x": "hi"}}, {"a": 2, "c": True}]
df = tools.execute(data, normalise=True, target="polars")
```

See [DataFrame & Series Support](../guide/dataframe-support.md#normalise-always-get-a-wide-dataframe)
for the full picture, including composing with the rest of the builder pipeline
and genuine PySpark DataFrame output via `target="pyspark"`.

## Error Handling

```python
import json_tools_rs as jt

try:
    result = jt.JSONTools().flatten().execute("invalid json")
except jt.JsonToolsError as e:
    print(f"Error: {e}")
```
