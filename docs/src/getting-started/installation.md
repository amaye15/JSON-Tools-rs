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
