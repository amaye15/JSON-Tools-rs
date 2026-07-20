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
| PySpark | Yes (input; result comes back as a list of dicts, see below) | -- |

> **PySpark DataFrame type preservation is the one exception to "perfect type
> preservation" above.** A PySpark `DataFrame` is accepted as input (it's converted
> via `.toPandas()` internally, processed, then converted back), but reconstruction
> doesn't rebuild a distributed PySpark `DataFrame` -- there's no `SparkSession`
> available at that point to do so -- so `.execute()` returns a plain Python `list`
> of dicts instead. If you need the result back as a PySpark `DataFrame`, wrap it
> yourself: `spark.createDataFrame(result)`. This is also a reminder that `.execute(df)`
> on a PySpark DataFrame collects the whole thing to the driver first (see the note
> above) -- for a genuinely distributed path, use the `pandas_udf` pattern instead.

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

df = pl.DataFrame([
    {"user": {"name": "Alice", "age": 30}},
    {"user": {"name": "Bob", "age": 25}},
])

result = jt.JSONTools().flatten().execute(df)
print(type(result))  # <class 'polars.dataframe.frame.DataFrame'>
print(result.columns)  # ['user.name', 'user.age']
```

> A column holding pre-serialized JSON *strings* (e.g. `pl.DataFrame({"data": ['{"a":
> 1}', ...]})`) is exported by `write_ndjson` as `{"data": "{\"a\": 1}"}` per row --
> the nested structure is trapped inside a string value, so flattening it is a no-op.
> To flatten nested data with polars, use struct-typed columns (as above), which
> serialize as real nested JSON.

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
4. **Reconstruction**: Results are reconstructed into the original DataFrame/Series type using O(1) constructor calls (except PySpark DataFrames, which come back as a list of dicts -- see the note above)

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

## Examples

### Easy: flatten a Pandas DataFrame

```python
import json_tools_rs as jt
import pandas as pd

df = pd.DataFrame([{"user": {"name": "Alice", "age": 30}}, {"user": {"name": "Bob", "age": 25}}])
result = jt.JSONTools().flatten().execute(df)
# DataFrame with columns ['user.name', 'user.age']
```

### Medium: Polars struct column with filtering

```python
import polars as pl

df = pl.DataFrame([
    {"user": {"name": "Alice", "age": 30, "bio": ""}},
    {"user": {"name": "Bob", "age": None, "bio": "hi"}},
])

result = (jt.JSONTools()
    .flatten()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .execute(df)
)
# shape: (2, 3)
# ┌───────────┬──────────┬──────────┐
# │ user.name ┆ user.age ┆ user.bio │
# ╞═══════════╪══════════╪══════════╡
# │ Alice     ┆ 30       ┆ null     │
# │ Bob       ┆ null     ┆ hi       │
# └───────────┴──────────┴──────────┘
```

Filtering is per-row, but a DataFrame's columns are shared across all rows. Row 0's
`bio` (`""`) was filtered out of *that row*, and row 1's `age` (`null`) was filtered
out of *that row* -- but since each column still exists (some other row still has a
value there), the filtered-out cell shows up as `null` in the reconstructed
DataFrame rather than making the column disappear or shifting columns per row.

### Hard: distributed processing with PySpark via `pandas_udf`

`.execute(df)` collects a DataFrame to the driver first -- fine for the two examples
above, but not for a large distributed Spark dataset. For genuinely distributed,
per-partition processing, wrap the bindings in a `pandas_udf` instead, so each executor
processes its own share of the data with one native call per Arrow-vectorized batch
(not per row):

```python
import json_tools_rs as jt
import pandas as pd
from pyspark.sql.functions import pandas_udf
from pyspark.sql.types import StringType

_tools = (
    jt.JSONTools()
    .flatten()
    .separator("::")
    .remove_nulls(True)
    .key_replacement("r'^admin_'", "")
)

@pandas_udf(StringType())
def flatten_json(payload: pd.Series) -> pd.Series:
    return pd.Series(_tools.execute(payload.tolist()))

spark_df.withColumn("flattened", flatten_json(spark_df["payload"]))
```

Build the `JSONTools` instance once at module scope (it's reusable across calls), not
inside the UDF function body. See [Setting Up on Databricks](./databricks-setup.md)
for the full walkthrough, including why this is the *only* supported approach inside a
Lakeflow Declarative Pipeline.
