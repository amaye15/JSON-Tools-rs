# Quick Start (JVM / Spark)

The JVM bindings expose the same `JSONTools` engine as a Java library, built for use
as **Apache Spark UDFs in a Databricks Job or notebook running on classic compute**
(or any other Spark environment where you control cluster libraries). They're backed
by a thin JNI shim over the same Rust core the Python bindings use, so they have full
feature parity: regex and literal key/value replacement, empty-value filtering, key
casing, and type conversion -- not just flattening.

> **This does not work inside a Lakeflow Declarative Pipeline** (formerly Delta Live
> Tables). Databricks' own docs are explicit that pipelines support only SQL and
> Python, and that JVM libraries cannot be attached to pipeline compute at all
> (serverless or classic-backed) -- so a jar-based UDF can never be registered from
> inside a pipeline's Python code. See [Setting Up on
> Databricks](../guide/databricks-setup.md) for what actually works: running this as
> a Databricks Job on a classic cluster, optionally feeding a Lakeflow pipeline
> downstream via a Delta table, or using the Python bindings instead if you need the
> processing to happen genuinely inside a pipeline (Python wheels *are* supported
> there). This page covers the JVM API itself; that guide covers the Databricks
> deployment mechanics.

If your need is just "flatten nested JSON into columns" with no custom key/value
transforms, check whether Spark's built-in `VARIANT` type + `variant_explode()` /
`FLATTEN()` already covers it before reaching for this binding -- it needs no custom
native library at all. This binding earns its keep for the transform features
`VARIANT` doesn't have.

## Installing

Available on Maven Central as `io.github.amaye15:json-tools-rs-spark` -- see
[Installation](./installation.md) for the dependency snippet. The published jar
bundles native libraries for `linux-x86_64` and `linux-aarch64` (standard Databricks
compute and Graviton instances), so no separate native library install is needed on
those platforms.

To build from source instead:

```bash
# From the repo root: build the native library, then the Java project
cargo build --release --features jvm
cd jvm && mvn package
```

This produces `jvm/target/json-tools-rs-spark-<version>.jar`. CI
(`.github/workflows/jvm-ci.yml`) also builds this jar bundling both
`linux-x86_64` and `linux-aarch64` native libraries as a downloadable artifact on
every push.

## Basic Usage (plain Java)

The `JsonTools` fluent builder mirrors the Rust `JSONTools` builder:

```java
import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;

try (JsonToolsHandle tools = JsonTools.builder()
        .flatten()
        .separator("::")
        .keyReplacement("r'^admin_'", "")
        .removeNulls(true)
        .build()) {
    String result = tools.execute("{\"admin_name\": \"Jane\", \"age\": null}");
    // {"name":"Jane"}
}
```

Patterns follow the same `r'...'` convention as every other binding -- see
[Key & Value Replacements](../guide/replacements.md).

## As a Spark Row UDF

Register the default-config UDF in a notebook or Job task running on a classic
cluster that has the jar attached as a library:

```python
from pyspark.sql.types import StringType

spark.udf.registerJavaFunction(
    "flatten_json",
    "io.github.amaye15.jsontoolsrs.spark.FlattenUDF",
    StringType(),
)

df.selectExpr("flatten_json(payload) AS flattened_payload")
```

Malformed input throws (fail-fast), matching the core library and Python bindings'
behavior -- it doesn't silently return `null`. See `jvm/README.md` for custom
configuration (needs the `spark._jvm` escape hatch, since `registerJavaFunction`'s
reflection-based instantiation can't take constructor arguments) and for the
higher-throughput batched `BatchTransform` (`mapPartitions`-based) alternative.

## Where to Go Next

- [Setting Up on Databricks](../guide/databricks-setup.md) -- attaching the jar to a
  cluster, running it from a Job, and feeding a Lakeflow Pipeline downstream.
- [`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md) --
  full build instructions, both UDF tiers, tuning guidance.
- [Key & Value Replacements](../guide/replacements.md) -- the `r'...'` pattern convention.
- [Rust API Reference](../reference/rust-api.md) -- the underlying `JSONTools` builder
  every method on the Java `JsonTools` builder maps to 1:1.
