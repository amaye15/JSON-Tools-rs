# Quick Start (JVM / Spark)

The JVM bindings expose the same `JSONTools` engine as a Java library, built primarily
for use as **Apache Spark UDFs** -- e.g. inside a **Databricks Lakeflow Declarative
Pipeline** (formerly Delta Live Tables). They're backed by a thin JNI shim over the
same Rust core the Python bindings use, so they have full feature parity: regex and
literal key/value replacement, empty-value filtering, key casing, and type
conversion -- not just flattening.

> Lakeflow Declarative Pipelines only support Python/SQL for pipeline
> *definitions*, so using this binding isn't "write the pipeline in Scala" -- it's
> "expose the Rust core as a JVM-native library that a Python pipeline calls into,"
> via `spark.udf.registerJavaFunction` or a `spark._jvm` escape hatch. See
> [`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
> in the repository for the full Databricks integration walkthrough (distribution,
> both usage tiers, tuning) -- this page just covers the basics.

If your need is just "flatten nested JSON into columns" with no custom key/value
transforms, check whether Spark's built-in `VARIANT` type + `variant_explode()` /
`FLATTEN()` already covers it before reaching for this binding -- it needs no custom
native library at all. This binding earns its keep for the transform features
`VARIANT` doesn't have.

## Building

There's no published artifact yet (see [Installation](./installation.md)) -- build
from source:

```bash
# From the repo root: build the native library, then the Java project
cargo build --release --features jvm
cd jvm && mvn package
```

This produces `jvm/target/json-tools-rs-spark-<version>.jar`. CI
(`.github/workflows/jvm-ci.yml`) builds a version of this jar bundling both
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

Register the default-config UDF directly from a Python pipeline:

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

- [`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md) --
  full build, Databricks distribution, both UDF tiers, tuning guidance.
- [Key & Value Replacements](../guide/replacements.md) -- the `r'...'` pattern convention.
- [Rust API Reference](../reference/rust-api.md) -- the underlying `JSONTools` builder
  every method on the Java `JsonTools` builder maps to 1:1.
