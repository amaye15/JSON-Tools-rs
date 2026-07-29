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
| PySpark | Yes -- a real, distributed `pyspark.sql.DataFrame` back | -- |

> **PySpark DataFrame reconstruction.** A PySpark `DataFrame` is accepted as input
> (it's converted via `.toPandas()` internally, processed, then converted back) and
> `.execute(df)` reconstructs a genuine PySpark `DataFrame`
> ([#31](https://github.com/amaye15/JSON-Tools-rs/issues/31); earlier versions
> returned a plain Python `list` of dicts here, since there was no `SparkSession`
> reachable at reconstruction time -- an active session is now auto-discovered via
> `SparkSession.getActiveSession()`, the same mechanism `normalise(target="pyspark")`
> already used). This is still a reminder that `.execute(df)` on a PySpark DataFrame
> collects the whole thing to the driver first (see the note above) and only the
> *final* reconstruction step is genuinely distributed (via Spark's own
> Arrow-optimized `SparkSession.createDataFrame(pandas.DataFrame, schema)` bridge) --
> the flatten/processing computation itself is not distributed. For that, use the
> `pandas_udf` pattern further down this page.
>
> `normalise=True` (see [below](#normalise-always-get-a-wide-dataframe)) uses this
> exact same reconstruction mechanism -- the two paths are now consistent with each
> other, not just individually documented.

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
> 1}', ...]})`) also flattens correctly in `.flatten()` mode -- `execute()`
> auto-detects columns holding JSON strings and expands them the same way a
> struct-typed column already does, so `data` becomes `data.a` here too. See
> [Auto-Expanding JSON-String Columns](#auto-expanding-json-string-columns) below
> for the detection rules and caveats.

### Pandas Series

```python
import json_tools_rs as jt
import pandas as pd

series = pd.Series(['{"a": {"b": 1}}', '{"c": {"d": 2}}'])
result = jt.JSONTools().flatten().execute(series)
print(type(result))  # <class 'pandas.core.series.Series'>
```

## Auto-Expanding JSON-String Columns

A DataFrame column that's already a dict/struct expands into flattened columns
automatically -- that's just `.flatten()` finding real nested JSON in the row. A
column holding **pre-serialized JSON strings** (common with data loaded from a
JSON/JSONL file, a database `TEXT`/`JSON` column, or an upstream system that
already serialized a payload) used to stay an opaque string instead, since a
string *value* isn't something `.flatten()` re-parses -- that's not its contract.
`execute()` on a DataFrame in `.flatten()` mode now detects columns holding JSON
strings and expands them the same way, so it "just works" without a manual
pre-parsing step
(see [issue #30](https://github.com/amaye15/JSON-Tools-rs/issues/30)):

```python
import json_tools_rs as jt
import pandas as pd

df = pd.DataFrame({
    "id": [1, 2],
    "payload": ['{"user": {"name": "Alice"}}', '{"user": {"name": "Bob"}}'],
})

result = jt.JSONTools().flatten().execute(df)
print(result)
#    id payload.user.name
# 0   1             Alice
# 1   2               Bob
```

This closes the gap the previous version of this page documented for the polars
`write_ndjson` case above, and applies uniformly to pandas, polars, pyarrow, and
PySpark (PySpark DataFrames convert to pandas internally first, so they get this
for free too).

### Detection rules

- Runs **only in `.flatten()` mode** -- `.unflatten()` and `.normal()` DataFrame
  processing are unaffected; a JSON-string column stays exactly as-is in those
  modes.
- A column is a candidate only if its values, when parsed, are JSON **objects or
  arrays** -- not any scalar. A column of plain strings that happen to parse as a
  bare number/bool/null is never touched, and neither is a column of ordinary text:

  ```python
  df = pd.DataFrame({"id": [1], "notes": ["just some text"]})
  result = jt.JSONTools().flatten().execute(df)
  print(result)
  #    id           notes
  # 0   1  just some text
  ```
- Detection samples the first 20 rows: a column must parse successfully as JSON in
  *every* sampled row where it holds a string value, or the whole column is left
  untouched (conservative -- no partial/mixed expansion). A column that's `None`
  in every one of the first 20 rows won't be detected even if later rows hold real
  JSON -- a known limitation of sample-based detection, not a crash.
- A JSON-string-encoded **array** expands into indexed sub-columns
  (`col.0`, `col.1`, ...) the same way an already-list-typed column does today --
  including for a large array (e.g. a stringified embedding vector with hundreds of
  elements). There's currently no cap on this (unlike `.unflatten()`'s
  `max_array_index`, which guards against reconstructing a huge sparse array from a
  numeric key, not against a real array this large) -- a very wide array column
  will produce that many DataFrame columns.
- A row that fails to re-parse despite its column being detected (malformed JSON
  in just that one row, past the sample) keeps its original string value for that
  row only, and a Python warning is emitted naming the column and how many rows
  were affected -- so this stays visible instead of silently leaving that row's
  data in a different shape than the rest of the column.

## Normalise: Always Get a Wide DataFrame

`.execute()` normally mirrors the input's own type (str→str, dict→dict, DataFrame→
DataFrame). `execute(data, normalise=True)` instead always returns a genuine wide
DataFrame -- one column per flattened key -- no matter what shape `data` is: a bare
JSON string or dict becomes a 1-row DataFrame, a list becomes an N-row one, and an
existing DataFrame/Series gets re-normalised the same way. Requires `.flatten()`
mode (a `JsonToolsError` explains why if it's not set -- unflattened/nested JSON
can't produce clean scalar columns).

### A single record → a 1-row DataFrame

No DataFrame library or wrapping needed on the input side at all -- useful for
turning a single API response or log line straight into a table row:

```python
import json_tools_rs as jt

tools = jt.JSONTools().flatten()

df = tools.execute({"user": {"name": "Alice", "age": 30}}, normalise=True)
print(df)
#   user.name  user.age
# 0     Alice        30
```

### Heterogeneous records → union + null-fill

A list of records that don't all share the same keys gets unioned into one
consistent set of columns, in first-seen order, with `None`/null filling any row
that's missing a given key -- the same union/null-fill behavior the "Medium"
example above shows for an existing DataFrame, just starting from plain
dicts instead:

```python
import json_tools_rs as jt

tools = jt.JSONTools().flatten()
data = [
    {"a": 1, "b": {"x": "hi"}},
    {"a": 2, "c": True},
]
df = tools.execute(data, normalise=True)
print(df)
#    a   b.x     c
# 0  1    hi  None
# 1  2  None  True
```

### Choosing the target library

Pass `target` to pick the library explicitly, or omit it to auto-resolve: an input
that's already a live DataFrame/Series keeps that backend; otherwise pandas → polars
→ pyarrow is tried in order (first installed wins). `target="pyspark"` is never
chosen automatically for bare JSON input -- see [below](#pyspark-a-real-distributed-dataframe-not-a-list).

```python
import json_tools_rs as jt

tools = jt.JSONTools().flatten()
data = [{"a": 1, "b": 2}, {"a": 3, "b": 4}]

pandas_df = tools.execute(data, normalise=True, target="pandas")
polars_df = tools.execute(data, normalise=True, target="polars")
arrow_table = tools.execute(data, normalise=True, target="pyarrow")

print(type(pandas_df), type(polars_df), type(arrow_table))
# <class 'pandas.core.frame.DataFrame'> <class 'polars.dataframe.frame.DataFrame'> <class 'pyarrow.lib.Table'>
```

`target=None`'s auto-resolution also applies when the input is already a live
DataFrame/Series -- useful for re-normalising into a *different* backend than the
one you started with, or for cleaning up something that isn't wide yet:

```python
import json_tools_rs as jt
import pandas as pd

tools = jt.JSONTools().flatten()
pandas_df = pd.DataFrame([{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}])

# target=None here would keep pandas (input's own backend); pass target= to convert
polars_df = tools.execute(pandas_df, normalise=True, target="polars")
print(type(polars_df))  # <class 'polars.dataframe.frame.DataFrame'>
```

### Composing with the rest of the builder pipeline

`normalise` is just the reconstruction step -- every other builder feature still
runs first, exactly as it would for plain `.execute()`:

```python
import json_tools_rs as jt

tools = (
    jt.JSONTools()
    .flatten()
    .separator("::")
    .remove_nulls(True)
    .key_replacement("r'^admin_'", "")
    .auto_convert_types(True)
)

data = [
    {"admin_name": "Jane", "admin_status": None, "count": "42"},
    {"admin_name": "Bob", "count": "7"},
]
df = tools.execute(data, normalise=True, target="pandas")
print(df)
#    name  count
# 0  Jane     42
# 1   Bob      7
```

### PySpark: a real distributed DataFrame, not a list

`target="pyspark"` requires an active `SparkSession` (auto-discovered via
`SparkSession.getActiveSession()`) and is never chosen automatically for bare
JSON input -- only via an explicit `target="pyspark"`, or when the input itself
was already a live PySpark object:

```python
import json_tools_rs as jt
from pyspark.sql import SparkSession

SparkSession.builder.getOrCreate()  # normalise auto-discovers this

tools = jt.JSONTools().flatten()
data = [{"user": {"name": "Alice", "age": 30}}, {"user": {"name": "Bob", "age": 25}}]

spark_df = tools.execute(data, normalise=True, target="pyspark")
from pyspark.sql import DataFrame as SparkDataFrame
print(isinstance(spark_df, SparkDataFrame))  # True -- a real, distributed DataFrame
spark_df.show()
# +---------+--------+
# |user.name|user.age|
# +---------+--------+
# |    Alice|      30|
# |      Bob|      25|
# +---------+--------+
```

Under the hood, the `pyspark` target reuses the exact same pandas reconstruction as
`target="pandas"`, then hands that DataFrame -- plus an explicit `StructType`
schema computed from the data -- to Spark's own Arrow-optimized
`SparkSession.createDataFrame(pandas.DataFrame, schema)` bridge, rather than
letting Spark infer the schema itself. This isn't just style: schema inference
from a pandas DataFrame is unreliable specifically on the non-Arrow fallback path
Spark silently takes when pyarrow isn't installed (pyspark does not depend on
pyarrow) -- an explicit schema sidesteps that entirely. See the note earlier on
this page for what this bridge does and doesn't distribute (the reconstruction,
not the flatten computation itself).

### What happens without `.flatten()` mode

`normalise=True` needs `.flatten()` mode specifically -- `.unflatten()`, `.normal()`,
or no mode set all raise a clear error rather than silently producing columns full
of nested objects:

```python
import json_tools_rs as jt

tools = jt.JSONTools().unflatten()
tools.execute({"a.b": 1}, normalise=True)
# json_tools_rs.JsonToolsError: normalise=True requires .flatten() mode -- unflattened/nested
# JSON can't produce clean scalar columns for a wide DataFrame
```

`target` is only meaningful alongside `normalise=True` -- setting it without also
setting `normalise=True` is rejected too, rather than silently ignored:

```python
tools = jt.JSONTools().flatten()
tools.execute({"a": 1}, target="pandas")  # normalise=True missing
# json_tools_rs.JsonToolsError: target is only valid when normalise=True
```

## How It Works

1. **Detection**: The library uses duck typing to detect DataFrame/Series objects (checks for `.to_dict()`, `.to_list()`, etc.)
2. **Extraction**: Rows are extracted as JSON strings or dicts
3. **JSON-string-column expansion** (`.flatten()` mode only): columns holding JSON strings are detected and spliced into genuine nested JSON in each row -- see [Auto-Expanding JSON-String Columns](#auto-expanding-json-string-columns) above
4. **Processing**: Each row is processed through the Rust engine (with automatic parallelism for large DataFrames)
5. **Reconstruction**: Results are reconstructed into the original DataFrame/Series type -- O(1) constructor calls for pandas/polars/pyarrow, or a schema-driven `SparkSession.createDataFrame(...)` call for PySpark (see the note above)

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
