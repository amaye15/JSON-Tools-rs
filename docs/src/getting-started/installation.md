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

Python 3.8+ is supported.

## JVM / Spark (Java)

No published artifact yet -- build from source:

```bash
cargo build --release --features jvm
cd jvm && mvn package
```

Produces a jar at `jvm/target/json-tools-rs-spark-<version>.jar`. CI builds a version
of this jar bundling `linux-x86_64` and `linux-aarch64` native libraries (standard
Databricks compute and Graviton instances) as a downloadable artifact on every push.
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
