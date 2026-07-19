# Installation

## Rust

Add to your `Cargo.toml`:

```bash
cargo add json-tools-rs
```

Or manually:

```toml
[dependencies]
json-tools-rs = "0.9"
```

## Python

Install from PyPI:

```bash
pip install json-tools-rs
```

Pre-built wheels are available for:

| Platform | Architectures |
|----------|--------------|
| Linux (glibc) | x86_64, x86, aarch64, armv7, ppc64le |
| Linux (musl) | x86_64, x86, aarch64, armv7 |
| macOS | x86_64 (Intel), aarch64 (Apple Silicon) |
| Windows | x64 |

Python 3.9+ is supported.

## JVM / Spark (Java)

Available on Maven Central as `io.github.amaye15:json-tools-rs-spark`:

```xml
<dependency>
    <groupId>io.github.amaye15</groupId>
    <artifactId>json-tools-rs-spark</artifactId>
    <version>0.9.6</version>
</dependency>
```

The published jar bundles native libraries for `linux-x86_64` and `linux-aarch64`
(standard Databricks compute and Graviton instances), so no separate native library
install is needed on those platforms.

To build from source instead (for local development, or a platform other than Linux
x86_64/aarch64):

```bash
cargo build --release --features jvm
cd jvm && mvn package
```

This produces a jar at `jvm/target/json-tools-rs-spark-<version>.jar`. CI
(`.github/workflows/jvm-ci.yml`) also builds this jar bundling `linux-x86_64` and
`linux-aarch64` native libraries as a downloadable artifact on every push.
See [Quick Start (JVM / Spark)](./quickstart-jvm.md) and
[`jvm/README.md`](https://github.com/amaye15/json-tools-rs/blob/master/jvm/README.md)
for the full Databricks integration walkthrough.

## Verify Installation

**Rust:**

```rust
use json_tools_rs::JSONTools;

fn main() {
    let result = JSONTools::new()
        .flatten()
        .execute(r#"{"hello": "world"}"#)
        .unwrap();
    println!("{:?}", result);
}
```

**Python:**

```python
import json_tools_rs as jt

result = jt.JSONTools().flatten().execute({"hello": "world"})
print(result)  # {'hello': 'world'}
```
