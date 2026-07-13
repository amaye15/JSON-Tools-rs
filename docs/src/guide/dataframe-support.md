# DataFrame & Series Support

The Python bindings natively support DataFrame and Series objects from popular data libraries, with perfect type preservation.

> **Note:** PySpark support here means the Python bindings can accept/return a
> PySpark DataFrame directly (useful when driving a Spark job from a Python script
> that has the Python bindings installed). For running *inside* a distributed Spark
> job as a native UDF -- without requiring the Python bindings on every executor,
> e.g. from a Databricks Lakeflow Declarative Pipeline -- see the
> [JVM / Spark bindings](../getting-started/quickstart-jvm.md) instead.

## Supported Libraries

| Library | DataFrame | Series |
|---------|-----------|--------|
| Pandas | Yes | Yes |
| Polars | Yes | Yes |
| PyArrow | Yes (Table) | Yes (Array) |
| PySpark | Yes | -- |

## Usage

### Pandas DataFrame

```python
import json_tools_rs as jt
import pandas as pd

df = pd.DataFrame([
    {"user": {"name": "Alice", "age": 30}},
    {"user": {"name": "Bob", "age": 25}},
])

result = jt.JSONTools().flatten().execute(df)
print(type(result))  # <class 'pandas.core.frame.DataFrame'>
print(result.columns.tolist())  # ['user.name', 'user.age']
```

### Polars DataFrame

```python
import json_tools_rs as jt
import polars as pl

df = pl.DataFrame({
    "data": ['{"user": {"name": "Alice"}}', '{"user": {"name": "Bob"}}']
})

result = jt.JSONTools().flatten().execute(df)
print(type(result))  # <class 'polars.DataFrame'>
```

### Pandas Series

```python
import json_tools_rs as jt
import pandas as pd

series = pd.Series(['{"a": {"b": 1}}', '{"c": {"d": 2}}'])
result = jt.JSONTools().flatten().execute(series)
print(type(result))  # <class 'pandas.core.series.Series'>
```

## How It Works

1. **Detection**: The library uses duck typing to detect DataFrame/Series objects (checks for `.to_dict()`, `.to_list()`, etc.)
2. **Extraction**: Rows are extracted as JSON strings or dicts
3. **Processing**: Each row is processed through the Rust engine (with automatic parallelism for large DataFrames)
4. **Reconstruction**: Results are reconstructed into the original DataFrame/Series type using O(1) constructor calls

## All Features Apply

DataFrames and Series support all the same features as regular input:

```python
tools = (jt.JSONTools()
    .flatten()
    .separator("::")
    .lowercase_keys(True)
    .remove_nulls(True)
    .auto_convert_types(True)
    .parallel_threshold(50)
)

result = tools.execute(large_dataframe)
```
