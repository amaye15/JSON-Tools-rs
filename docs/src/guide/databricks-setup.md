# Setting Up on Databricks

**Short version:** to run json-tools-rs *inside* a Lakeflow Declarative Pipeline
(formerly Delta Live Tables), use the **Python bindings** as a `pandas_udf` --
validated and shown below. The JVM bindings (`jvm/`) are for Databricks Jobs and
notebooks on classic compute, or any other Spark environment outside a Lakeflow
pipeline -- they cannot be attached to pipeline compute at all, for reasons explained
below. There is a newer, **untested** potential path via Unity Catalog Scala/Java
UDFs -- see [below](#a-newer-untested-path-unity-catalog-scalajava-udfs) -- but the
Python `pandas_udf` approach is the one to actually use today.

## Why the JVM bindings can't run inside a pipeline

This isn't a limitation of this library specifically -- it's a hard platform
restriction. Databricks' own documentation states plainly: pipelines support only SQL
and Python, and JVM libraries cannot be installed on pipeline compute (serverless or
classic-backed) at all; doing so "causes unpredictable behavior." There is no jar,
init script, or configuration that works around this -- a jar-based UDF can never be
registered from inside a pipeline's Python code, because the class is never on that
JVM's classpath to begin with.

This restriction is specific to Databricks' managed Lakeflow Pipelines product. The
JVM bindings work completely normally in a regular Databricks Job or notebook on a
classic cluster, or on any other Spark deployment (self-managed, EMR, etc.) where you
control cluster libraries -- see [`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
for that path.

## A newer, untested path: Unity Catalog Scala/Java UDFs

Databricks GA'd [Scala and Java UDFs in Unity
Catalog](https://docs.databricks.com/aws/en/udf/scala-java-uc) on July 16, 2026 -- a
different registration mechanism from the cluster-library restriction described
above. A jar uploaded to a Unity Catalog Volume is registered directly as a catalog
function:

```sql
CREATE OR REPLACE FUNCTION my_catalog.my_schema.flatten_json(config STRING, json STRING)
RETURNS STRING
LANGUAGE JAVA
DETERMINISTIC
ENVIRONMENT (
  java_dependencies = '["/Volumes/my_catalog/my_schema/jars/json-tools-rs-spark-0.9.13.jar"]',
  environment_version = '4')
HANDLER 'io.github.amaye15.jsontoolsrs.SomeHandler.flatten';
```

...and executes in an isolated sandbox with no active Spark session. Databricks'
docs state this is supported on "all compute types, including serverless notebooks
and jobs, SQL warehouses, and Spark Declarative Pipelines on Lakeflow" -- an explicit
claim, not an inference. If it holds for this library, it would mean JNI-level
throughput becomes usable inside a Lakeflow pipeline for the first time, without the
upstream-Job workaround described [below](#feeding-a-pipeline-from-a-jvm-processed-table).

**This has not been tested against this library, there is no adapter class for it in
this repo, and it is not a currently supported or documented path** -- treat it as a
lead worth investigating, not a working recipe. Two concrete things stand between
"the docs say it's supported" and "this actually works here":

1. **The HANDLER must be Spark-free.** The docs require a plain `public static`
   method with no Spark API usage or dependency, which rules out
   `FlattenUDF`/`UnflattenUDF`/`BatchTransform` (they implement Spark's
   `UDF1`/`MapPartitionsFunction` interfaces) but fits
   `JsonToolsHandle.execute(String) -> String` well -- that class already has zero
   Spark imports. A new, thin adapter class exposing it as a static method (the
   `SomeHandler.flatten` placeholder above) would need to be written; it does not
   exist yet.
2. **Native library loading inside the sandbox is undocumented.**
   `JsonToolsHandle` loads the compiled Rust library via JNI
   (`NativeLibraryLoader`'s extract-to-temp-file-then-`System.load` pattern).
   Databricks' UC UDF docs describe the sandbox's isolation in real detail (no
   Spark session, no workspace/volume access at runtime, Lakeguard-based container
   isolation) but say nothing -- neither permitting nor forbidding --
   about `System.load()`/JNI specifically. This is the deciding unknown, and it can
   only be settled by actually registering a handler and calling it in a real
   workspace.

If you try this and it works, that's worth reporting back -- it would be a real
capability unlock worth documenting properly and adding a validated example for, the
same way the `pandas_udf` pattern below was validated before being written down.

## Using the Python bindings inside a Lakeflow Declarative Pipeline

Python packages -- including ones backed by a compiled native extension, like this
one -- are a fully supported pipeline dependency (unlike JVM jars). Wrapping the
Python bindings in a [`pandas_udf`](https://spark.apache.org/docs/latest/api/python/reference/pyspark.sql/api/pyspark.sql.functions.pandas_udf.html)
gives you the same properties the JVM bindings' batched `BatchTransform` tier does --
one native call per Arrow-vectorized batch instead of per row -- while running as a
genuinely distributed Python UDF across executors, not something collected to the
driver. This was validated directly (not assumed) against a real Spark session
running the pattern below before writing it here.

### 1. Add the dependency

From the pipeline editor: **Settings → Pipeline environment → Edit environment → Add
dependency**, then enter `json-tools-rs` (already published to PyPI -- see
[Installation](../getting-started/installation.md)). Alternatively, build a wheel
locally (`maturin build --release --features python`) and install it from a Unity
Catalog Volume path instead, the same way pipeline dependencies support installing a
wheel from a volume.

### 2. Define the UDF

Build the `JSONTools` instance once at module scope, not inside the UDF function body
-- it's reusable across calls (the same instance can call `.execute()` repeatedly),
and the underlying regex/pattern cache is process-wide, so there's no benefit to
reconstructing it per batch:

```python
import json_tools_rs as jt
import pandas as pd
from pyspark.sql.functions import pandas_udf
from pyspark.sql.types import StringType

_flatten_tools = (
    jt.JSONTools()
    .flatten()
    .separator("::")
    .remove_nulls(True)
    .key_replacement("r'^admin_'", "")
)


@pandas_udf(StringType())
def flatten_json(payload: pd.Series) -> pd.Series:
    return pd.Series(_flatten_tools.execute(payload.tolist()))
```

### 3. Use it in a pipeline table

```python
import dlt
from pyspark.sql.functions import col

@dlt.table
def flattened_events():
    return (
        dlt.read_stream("raw_events")
        .withColumn("flattened_payload", flatten_json(col("payload")))
    )
```

That's it -- no jar, no cluster library configuration, no `spark._jvm` escape hatch.
It works identically whether the pipeline runs on serverless or classic compute,
since it's an ordinary Python dependency as far as Databricks is concerned.

Malformed input raises `json_tools_rs.JsonToolsError` inside the UDF, which fails the
task the same way any Python UDF exception does -- wrap the `.execute()` call in a
`try`/`except` inside the UDF function if you'd rather emit `None` for bad rows than
fail the pipeline update, or use a
[Lakeflow expectation](https://docs.databricks.com/aws/en/ldp/expectations) to
quarantine rows that fail a validity check upstream of the UDF.

## Databricks Jobs and notebooks (classic compute)

Outside a Lakeflow pipeline -- a plain notebook cell, or a notebook/Python/JAR task
in a Databricks Job, running on a classic all-purpose or job cluster -- the JVM
bindings work directly, including `spark.udf.registerJavaFunction` and the
higher-throughput `BatchTransform`. See
[`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
for the full walkthrough: uploading the jar to a Unity Catalog Volume, attaching it
as a cluster library, and both UDF tiers.

## Feeding a pipeline from a JVM-processed table

If you specifically want the JVM bindings' performance characteristics (e.g. the
batched `mapPartitions` transform) upstream of a pipeline, run that processing as a
separate Databricks Job that writes a Delta table, then have the pipeline read that
table as its source -- no JVM code runs inside the pipeline itself, only inside the
upstream Job. See `jvm/README.md`'s "Feeding a Lakeflow Pipeline" section for the
worked example.
