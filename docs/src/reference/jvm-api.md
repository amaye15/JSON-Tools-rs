# JVM API Reference

```java
import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;
import io.github.amaye15.jsontoolsrs.JsonToolsException;
```

Maven coordinates (published on tagged releases):

```xml
<dependency>
  <groupId>io.github.amaye15</groupId>
  <artifactId>json-tools-rs-spark</artifactId>
  <version>0.9.5</version>
</dependency>
```

The JVM bindings are a thin JNI shim (`src/jvm.rs`, built via the opt-in `jvm` Cargo
feature) over the same Rust `JSONTools` core used by the Python bindings, with full
feature parity. They're built primarily for use as Apache Spark UDFs -- see
[Setting Up on Databricks](../guide/databricks-setup.md) and
[`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
for the Spark-specific usage tiers (row UDF vs. batched `mapPartitions` transform).
This page covers the underlying Java API those tiers are built on.

## JsonTools

`JsonTools` is a pure-Java fluent builder mirroring the Rust `JSONTools` builder. It
accumulates configuration in Java only; nothing crosses into native code until
`.build()` is called. Unset fields are omitted from the serialized config entirely, so
the underlying Rust `JSONTools::new()` defaults apply -- this class never hardcodes
its own copy of a default value.

### Construction

```java
JsonTools builder = JsonTools.builder();
```

### Operation Modes

Exactly one must be called before `.build()`.

| Method | Description |
|--------|-------------|
| `.flatten()` | Flatten nested JSON into separator-delimited keys |
| `.unflatten()` | Reconstruct nested JSON from flat, separator-delimited keys |
| `.normal()` | Apply transformations without changing the nesting structure |

### Configuration Methods

All methods return `this` for chaining.

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.separator(sep)` | `String` | `"."` | Key separator for flatten/unflatten |
| `.lowercaseKeys(value)` | `boolean` | `false` | Convert all keys to lowercase |
| `.removeEmptyStrings(value)` | `boolean` | `false` | Filter out `""` values |
| `.removeNulls(value)` | `boolean` | `false` | Filter out `null` values |
| `.removeEmptyObjects(value)` | `boolean` | `false` | Filter out `{}` values |
| `.removeEmptyArrays(value)` | `boolean` | `false` | Filter out `[]` values |
| `.keyReplacement(find, replace)` | `String, String` | -- | Add a key replacement pattern (literal by default, `r'...'` for regex); repeatable |
| `.valueReplacement(find, replace)` | `String, String` | -- | Add a value replacement pattern (literal by default, `r'...'` for regex); repeatable |
| `.handleKeyCollision(value)` | `boolean` | `false` | Collect colliding keys into arrays |
| `.autoConvertTypes(value)` | `boolean` | `false` | Auto-convert string values to native types |
| `.parallelThreshold(n)` | `int` | `100` | Min batch size for parallel processing |
| `.numThreads(n)` | `int` | CPU count | Thread count for parallelism |
| `.nestedParallelThreshold(n)` | `int` | `100` | Min keys/items for intra-document parallelism |
| `.maxArrayIndex(n)` | `int` | `100_000` | Max array index during unflattening (DoS protection) |

Defaults for `parallelThreshold`, `nestedParallelThreshold`, `numThreads`, and
`maxArrayIndex` can be overridden process-wide via the same environment variables as
the Rust/Python bindings (`JSON_TOOLS_PARALLEL_THRESHOLD`, etc.) -- see
[Performance Tuning](../resources/performance.md) -- since they resolve on the Rust
side regardless of which binding calls in.

See [Key & Value Replacements](../guide/replacements.md) for the `r'...'` literal-vs-regex
convention, which applies identically here (patterns are passed through as plain
strings and resolved entirely on the Rust side).

### `.build()`

```java
JsonToolsHandle build()
```

Serializes the accumulated configuration to a small, deterministic JSON blob (fixed
field order, unset fields omitted) and constructs the native handle from it via one
JNI call (`nativeCreate`). The caller owns the returned handle.

## JsonToolsHandle

```java
public final class JsonToolsHandle implements AutoCloseable
```

Owns one native `JSONTools` handle. The underlying Rust value has no interior
mutability and is immutable after construction, so concurrent `execute`/`executeBatch`
calls from multiple threads on the same handle are safe with no external locking.

| Method | Signature | Description |
|--------|-----------|--------------|
| `execute(json)` | `String execute(String json)` | Process a single JSON document |
| `executeBatch(jsonArray)` | `String[] executeBatch(String[] jsonArray)` | Process a batch in one native call, using the same Rayon-parallel batch path as `execute(Vec<String>)` in Rust |
| `close()` | `void close()` | Frees the native handle. Using the handle afterward is undefined behavior. |

A public constructor, `JsonToolsHandle(String configJson)`, is also available for code
that already has a raw config JSON string (e.g. the Spark `BatchTransform` tier, or a
caller on the other side of the `spark._jvm` escape hatch) and doesn't need to go
through the `JsonTools` fluent builder to get one.

```java
try (JsonToolsHandle tools = JsonTools.builder()
        .flatten()
        .separator("::")
        .keyReplacement("r'^admin_'", "")
        .removeNulls(true)
        .build()) {
    String result = tools.execute("{\"admin_name\": \"Jane\", \"age\": null}");
    // result: {"name":"Jane"}

    String[] batch = tools.executeBatch(new String[] {
        "{\"admin_a\": 1}", "{\"admin_b\": 2}"
    });
}
// tools.close() is called automatically at the end of the try-with-resources block
```

`close()` is idempotent (safe to call more than once) and is the only supported way to
free the native handle -- there is no finalizer. Use try-with-resources, as above, for
a handle you own outright. Two lifecycle patterns are used internally for Spark UDFs
that don't fit a simple try-with-resources scope: a process-wide cache of shared,
never-closed handles for the row-UDF tier (`NativeHandleCache`), and one handle per
partition closed via a Spark `TaskContext` completion listener for the batched
`mapPartitions` tier (`BatchTransform`) -- see their source for the pattern if you're
writing a custom UDF against this library.

## Error Handling

```java
public class JsonToolsException extends RuntimeException
```

Every native entry point is routed through a panic-catching guard on the Rust side:
any error *or* panic in native code surfaces as a thrown `JsonToolsException`, never
an unwind across the JNI boundary (which would be undefined behavior) and never a
silent `null`/default return.

The exception's message is the Rust error's `Display` text, which embeds the same
bracketed `[E00x]` machine-readable code used by the Rust and Python bindings -- see
[Error Codes](./error-codes.md) for the full E001-E008 reference. Unlike the Python
bindings (which prepend their own `"Failed to process ...: "` context before the
code), the JVM message is the Rust `Display` text unmodified, so `[E00x]` is at the
start of `getMessage()`.

```java
try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
    String result = tools.execute("not valid json");
} catch (JsonToolsException e) {
    System.err.println(e.getMessage());
    // [E001] JSON parsing failed: ...
    if (e.getMessage().startsWith("[E001]")) {
        // handle a parse error specifically
    }
}
```

`JsonToolsException` is an unchecked (`RuntimeException` subclass) exception, so
methods that call `execute()`/`executeBatch()`/`build()` don't need to declare it in a
`throws` clause.

## Complete Example

```java
import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;
import io.github.amaye15.jsontoolsrs.JsonToolsException;

public class Example {
    public static void main(String[] args) {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .separator("::")
                .lowercaseKeys(true)
                .removeNulls(true)
                .removeEmptyStrings(true)
                .keyReplacement("r'^user_'", "")
                .autoConvertTypes(true)
                .parallelThreshold(50)
                .numThreads(4)
                .build()) {

            // Single document
            String result = tools.execute("{\"User_Name\": \"Alice\", \"User_Age\": \"30\"}");

            // Batch
            String[] batch = tools.executeBatch(new String[] {
                "{\"a\": \"1\"}", "{\"b\": \"2\"}"
            });
        } catch (JsonToolsException e) {
            System.err.println("json-tools-rs error: " + e.getMessage());
        }
    }
}
```

For Spark-specific usage (row UDFs registered via `spark.udf.registerJavaFunction`,
and the higher-throughput batched `mapPartitions` transform), see
[`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
and [Setting Up on Databricks](../guide/databricks-setup.md).
