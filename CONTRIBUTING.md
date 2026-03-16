# Contributing to JSON Tools RS

Thank you for your interest in contributing! This guide covers the development workflow for JSON Tools RS.

## Development Setup

### Prerequisites

- **Rust**: 1.80+ (the MSRV). Install via [rustup](https://rustup.rs/).
- **Python**: 3.9+ (for Python bindings development).
- **maturin**: For building Python wheels (`pip install maturin`).

### Building

```bash
# Build the Rust library
cargo build

# Build with Python bindings
maturin develop --features python

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Rust unit tests + doctests
cargo test
cargo test --doc

# Run examples
cargo run --example basic_usage
cargo run --example advance_usage
cargo run --example test_type_conversion

# Python tests (after maturin develop)
pytest python/tests/tests.py -v
```

### Running Benchmarks

Benchmarks use [Criterion](https://github.com/bheisler/criterion.rs) and produce HTML reports in `target/criterion/`.

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark suite
cargo bench --bench comprehensive_benchmark
cargo bench --bench stress_benchmarks
cargo bench --bench isolation_benchmarks
cargo bench --bench combination_benchmarks
cargo bench --bench realworld_benchmarks
```

### Profiling (macOS)

The project uses [samply](https://github.com/mstange/samply) for profiling on macOS:

```bash
# Build with profiling symbols
cargo bench --profile profiling --bench stress_benchmarks --no-run

# Record a profile (saves Firefox Profiler format)
samply record --save-only -o /tmp/profile.json -- ./target/profiling/deps/stress_benchmarks-* --bench

# View the profile
samply load /tmp/profile.json
```

> **Note**: `flamegraph` requires full Xcode (not CLI tools only). Valgrind does not work on modern macOS.

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy --all-targets --all-features` and address any warnings.
- Pre-commit hooks are configured in `.pre-commit-config.yaml` — install with `pre-commit install`.

## Pull Request Process

1. Create a feature branch from `master`.
2. Make your changes with tests.
3. Ensure `cargo test`, `cargo clippy`, and `cargo fmt --check` pass.
4. For performance changes, include before/after benchmark results.
5. Open a PR with a clear description of the change.

## Architecture Overview

See the [Architecture reference](https://amaye15.github.io/JSON-Tools-rs/reference/architecture.html) in the mdBook guide for details on the tape-based parsing engine, module structure, and parallelism model.
