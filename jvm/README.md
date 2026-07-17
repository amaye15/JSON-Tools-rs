# json-tools-rs-spark

JVM (Java) bindings for [json-tools-rs](../README.md), exposed as Apache Spark UDFs
and a batched `Dataset` transform. Backed by a thin JNI shim over the same Rust core
used by the crate's Python bindings (`../src/jvm.rs`) -- full feature parity (regex
and literal key/value replacement, empty-value filtering, key casing, type
conversion), not just flattening.

Built for use as Apache Spark UDFs in a **Databricks Job or notebook running on
classic compute** (an all-purpose or job cluster with the jar attached as a cluster
library).

> **This does *not* work inside a Lakeflow Declarative Pipeline (formerly Delta Live
> Tables).** Databricks' own docs are explicit: *"pipelines support only SQL and
> Python. You cannot use JVM libraries in a pipeline"* and *"installing JVM libraries
> causes unpredictable behavior."* This applies to every Lakeflow pipeline regardless
> of compute type (serverless or classic-backed) -- there is no way to attach this
> jar to a pipeline's compute, so `spark.udf.registerJavaFunction(...)` called from
> inside a pipeline's Python code will fail with a `ClassNotFoundException` (the class
> was never on that JVM's classpath to begin with). See [Databricks
> Setup](../docs/src/guide/databricks-setup.md) for the two ways to actually get this
> library's functionality into a Databricks data flow, including one that *does*
> feed a Lakeflow pipeline.
>
> If your goal is specifically "call json-tools-rs from inside a Lakeflow pipeline,"
> use the **Python bindings** instead (`pip install json-tools-rs`, already published
> to PyPI) -- Python wheels are supported in a pipeline's Environment settings, unlike
> JVM jars.

If your need is just "flatten nested JSON into columns" with no custom key/value
transforms, check whether Spark's built-in `VARIANT` type + `variant_explode()` /
`FLATTEN()` already covers it -- it's ~8x faster than string-column JSON parsing and
needs no custom native library. This binding earns its keep for the transform
features `VARIANT` doesn't have: regex/literal replacement, filtering, casing.

## Building

```bash
# From the repo root: build the native library
cargo build --release --features jvm

# Copy it into the Maven resources tree for your platform, e.g. on macOS:
mkdir -p jvm/src/main/resources/native/darwin-aarch64
cp target/release/libjson_tools_rs.dylib jvm/src/main/resources/native/darwin-aarch64/

# Then build/test the JVM side
cd jvm
mvn test      # requires JDK 17+ (JDK 25 currently breaks the local Spark tests --
               # see "Known local-JDK issue" below)
mvn package    # produces target/json-tools-rs-spark-<version>.jar
```

CI (`.github/workflows/jvm-ci.yml`) builds and bundles `linux-x86_64` and
`linux-aarch64` native libraries (standard Databricks compute and Graviton instances
respectively) into the packaged jar automatically. Other platforms (macOS, Windows)
are for local development/testing only and are not part of the distributed jar.

### Known local-JDK issue

Very new JDKs (25+) hit `UnsupportedOperationException: getSubject is not supported`
when creating a local `SparkSession`, from Hadoop client code Spark depends on that
hasn't caught up to recent JDK security-manager-removal changes. Databricks itself
runs JDK 17 or 21, so this only affects local test runs -- point `JAVA_HOME` at a JDK
17 or 21 install:

```bash
export JAVA_HOME=$(/usr/libexec/java_home -v 17)   # or your platform's equivalent
```

## Examples

Standalone runnable examples live under [`examples/`](examples/) (kept out of the
default build via a separate Maven profile, so `mvn test`/`mvn package` never compile
or ship them in the jar): [`FeatureByFeature.java`](examples/io/github/amaye15/jsontoolsrs/examples/FeatureByFeature.java)
covers every builder method in isolation, and
[`FeatureCombinations.java`](examples/io/github/amaye15/jsontoolsrs/examples/FeatureCombinations.java)
covers curated multi-feature pipelines. Mirrors `../examples/*.rs` and
`../python/examples/*.py` -- same inputs, same output, across all three languages.

```bash
mvn -P examples compile exec:java \
    -Dexec.mainClass=io.github.amaye15.jsontoolsrs.examples.FeatureByFeature
mvn -P examples compile exec:java \
    -Dexec.mainClass=io.github.amaye15.jsontoolsrs.examples.FeatureCombinations
```

## Distribution to Databricks

**Via CI artifact (any commit):** download the `json-tools-rs-spark-jar` artifact
from a `jvm-ci.yml` CI run (or build it locally per above), then:

1. Upload the jar to a Unity Catalog Volume.
2. Attach it as a cluster library on a **classic** all-purpose or job cluster
   (Compute → your cluster → Libraries → Install new → Volumes). Not applicable to
   Lakeflow Pipeline compute -- see the note above.

**Via Maven Central (tagged releases only):** pushing a git tag (e.g. `v0.9.5`)
triggers `jvm-ci.yml`'s `release` job, which builds, GPG-signs, and publishes
`io.github.amaye15:json-tools-rs-spark` to Maven Central automatically -- live since
v0.9.2. Add it as a normal Maven/Gradle/sbt dependency:

```xml
<dependency>
  <groupId>io.github.amaye15</groupId>
  <artifactId>json-tools-rs-spark</artifactId>
  <version>0.9.5</version>
</dependency>
```

A Maven Central publish is **permanent** (versions can be deprecated but not
deleted) -- the release job deliberately only triggers on an actual tag push (not
`workflow_dispatch`), and depends on the same version-sync check (`Cargo.toml` vs.
`jvm/pom.xml`) that gates the `package` job. See `jvm/pom.xml`'s `release` Maven
profile for what it does (attaches `-sources`/`-javadoc` jars, GPG-signs everything,
publishes via Sonatype's `central-publishing-maven-plugin`) -- everything in that
profile is opt-in and only runs when explicitly activated (`-P release`), so a plain
`mvn test`/`mvn package` never needs a GPG key.

## Usage in a Databricks Job or notebook (classic compute)

This is a PySpark notebook cell or a notebook/Python task in a Databricks Job,
running on a cluster that has the jar attached as a library (see above) --
**not** a Lakeflow Pipeline.

### Tier 1: row UDF (simple, SQL-native)

Default configuration (`.` separator, no replacements/filters), via the standard
`registerJavaFunction` path -- Spark instantiates the class by its no-arg
constructor, so this only supports default config:

```python
from pyspark.sql.types import StringType

spark.udf.registerJavaFunction(
    "flatten_json",
    "io.github.amaye15.jsontoolsrs.spark.FlattenUDF",
    StringType(),
)
# same for unflatten:
spark.udf.registerJavaFunction(
    "unflatten_json",
    "io.github.amaye15.jsontoolsrs.spark.UnflattenUDF",
    StringType(),
)

flattened = spark.table("raw_events").selectExpr(
    "flatten_json(payload) AS flattened_payload"
)
flattened.write.mode("append").saveAsTable("flattened_events")
```

For **custom configuration** (separator, replacements, filters), use the `spark._jvm`
escape hatch -- `registerJavaFunction`'s reflection-based instantiation has no way to
pass constructor arguments from Python:

```python
import json

config_json = json.dumps({
    "mode": "flatten",
    "separator": "::",
    "remove_nulls": True,
    "key_replacements": [["r'^admin_'", ""]],
})
jvm_udf = spark._jvm.io.github.amaye15.jsontoolsrs.spark.FlattenUDF(config_json)
spark._jsparkSession.udf().register(
    "flatten_json_custom",
    jvm_udf,
    spark._jvm.org.apache.spark.sql.types.DataTypes.StringType,  # field, not a method call
)
```

**Malformed input throws `JsonToolsException`** (fail-fast), consistent with how both
the core Rust library and its Python bindings already behave -- it is not silently
swallowed to `null`. Wrap in SQL `TRY(...)` if you want `from_json`-style
null-on-error semantics instead.

### Tier 2: batched transform (higher throughput)

Buffers many rows per native call (via the same rayon-parallel batch path the core
library already uses for list input) instead of one JNI crossing per row. Reached
from Python via the `df._jdf` escape hatch, since a `mapPartitions` transform on a
Java-backed `Dataset` isn't directly callable from plain PySpark DataFrame methods:

`BatchTransform.flattenPartitioned` has an overload that takes a `Dataset<Row>` plus
a column name directly, so the PySpark side only needs to hand across
`df.select(col)._jdf` (a `Dataset<Row>` with one column) rather than assembling a JVM
`Dataset<String>` from Python, which is much fiddlier via plain py4j calls:

```python
from pyspark.sql import DataFrame

config_json = '{"mode":"flatten"}'

result_jdf = spark._jvm.io.github.amaye15.jsontoolsrs.spark.BatchTransform.flattenPartitioned(
    df.select("json_column")._jdf,
    "json_column",
    config_json,
    64,  # batchSize
)
result_df = DataFrame(result_jdf, spark)
```

**Tuning `batchSize`**: the default (64) is deliberately kept below the core
library's own `parallel_threshold` default (100), so out of the box this stays on the
sequential per-batch path inside Rust, leaving Spark's own task-level parallelism
(multiple partitions/tasks running concurrently per executor) as the only parallelism
axis in play. Raising `batchSize` together with `parallel_threshold`/`num_threads` in
the config only pays off for workloads with few partitions and low task concurrency;
otherwise it stacks rayon's intra-batch fan-out on top of Spark's task parallelism on
the same cores.

**Batch-failure diagnostics**: the native batch call fails the whole chunk if any
single row in it is malformed (the underlying Rust batch processor short-circuits on
the first error). `BatchTransform` retries a failed chunk row by row so the resulting
exception identifies the actual malformed row's own JSON error, instead of an opaque
per-chunk index -- this does not change Spark's normal task-failure semantics (the
task still fails and is retried/aborted the same way any `mapPartitions` exception
would), it only makes the failure diagnosable.

## Feeding a Lakeflow Pipeline

Since this library can't run inside a Lakeflow Pipeline directly, the way to get its
output *into* one is to run it upstream as a separate Databricks Job (per above) that
writes a Delta table, then point the pipeline at that table as its source:

```python
# In the Job's notebook/task (classic cluster, jar attached):
spark.table("raw_events") \
    .selectExpr("flatten_json(payload) AS flattened_payload") \
    .write.mode("append").saveAsTable("main.default.flattened_events")
```

```python
# In the Lakeflow Pipeline (separate, Python/SQL only, serverless or classic compute):
@dlt.table
def bronze_events():
    return spark.readStream.table("main.default.flattened_events")
```

Schedule the Job to run before the pipeline (or as an upstream task in the same
Databricks Workflow) so the Delta table is fresh when the pipeline reads it. See
[Databricks Setup](../docs/src/guide/databricks-setup.md) for the full walkthrough of
both this pattern and the simpler Job-only one, plus the Python-bindings alternative
for when the requirement is genuinely "run inside the pipeline itself."

## Configuration reference

`JsonTools` (the fluent builder) mirrors the Rust `JSONTools` builder:
`flatten()`/`unflatten()`/`normal()`, `separator()`, `lowercaseKeys()`,
`keyReplacement(find, replace)`, `valueReplacement(find, replace)`,
`removeEmptyStrings()`, `removeNulls()`, `removeEmptyObjects()`, `removeEmptyArrays()`,
`handleKeyCollision()`, `autoConvertTypes()`, `parallelThreshold()`, `numThreads()`,
`nestedParallelThreshold()`, `maxArrayIndex()`. See the [core library's replacement
pattern docs](../docs/src/guide/replacements.md) for the `r'...'` regex-vs-literal
convention -- it applies identically here (patterns are passed through as plain
strings, resolved entirely on the Rust side).

```java
try (JsonToolsHandle tools = JsonTools.builder()
        .flatten()
        .separator("::")
        .keyReplacement("r'^admin_'", "")
        .removeNulls(true)
        .build()) {
    String result = tools.execute("{\"admin_name\": \"Jane\", \"age\": null}");
}
```
