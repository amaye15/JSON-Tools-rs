# DataFrame & Series Support

The Python bindings natively support DataFrame and Series objects from popular data libraries, with perfect type preservation.

> **Note:** the `.execute(df)` convenience shown here collects the DataFrame to the
> driver, processes it through the Rust engine, and reconstructs a new DataFrame --
> fine for smaller data, but not how you want to process a large distributed Spark
> dataset. For genuinely distributed, per-partition processing (including from
> *inside* a Databricks Lakeflow Declarative Pipeline, where this is the only
> supported approach -- see [Setting Up on Databricks](./databricks-setup.md)), wrap
> the Python bindings in a `pandas_udf` instead, so each executor runs its own share
> of the work. The [JVM / Spark bindings](../getting-started/quickstart-jvm.md) are a
> separate, JVM-native alternative for Databricks Jobs/notebooks on classic compute
> (not usable inside a pipeline at all -- see that page for why).

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
