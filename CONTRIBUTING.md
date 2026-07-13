# Contributing to JSON Tools RS

Thank you for your interest in contributing! This guide covers the development workflow for JSON Tools RS.

## Development Setup

### Prerequisites

- **Rust**: 1.80+ (the MSRV). Install via [rustup](https://rustup.rs/).
- **Python**: 3.9+ (for Python bindings development).
- **maturin**: For building Python wheels (`pip install maturin`).
- **JDK 17+ and Maven**: for JVM (Java/Spark) bindings development (see [jvm/](jvm/)).

### Building

```bash
# Build the Rust library
cargo build

# Build with Python bindings
maturin develop --features python

# Build in release mode
cargo build --release

# Build the JVM (Java/Spark) native library -- see jvm/README.md for the full
# build/test workflow, including copying the native lib into Maven's resources tree
cargo build --release --features jvm
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

# JVM tests (after cargo build --release --features jvm; see jvm/README.md)
cd jvm && mvn test
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

### Recording a benchmark

For the full statistical picture, use Criterion (above). For a quick sanity check or to
add a data point to the persistent, cross-commit history, use `bench_quick` instead —
it's a hand-timed harness (no Criterion wait) covering the scenarios in
[BENCHMARKS.md](BENCHMARKS.md)'s "Performance Targets" table:

```bash
cargo run --release --example bench_quick             # human-readable table
cargo run --release --example bench_quick -- --csv    # CSV rows to stdout
```

CI appends real numbers to [`benches/history.csv`](benches/history.csv) (one row per
`(commit, os, arch, threads, operation, scenario, size) -> time_us`) automatically on
every push to `master`, so most of the time you don't need to do anything. To add a
deliberate local snapshot yourself — e.g. before and after a change you're about to
make, on hardware CI doesn't cover:

```bash
cargo run --release --example bench_quick -- --csv >> benches/history.csv
cargo run --release --example bench_compare               # diffs the last two commits recorded
cargo run --release --example bench_compare <old> <new>    # or two specific ones (prefix match ok)
```

`bench_quick --csv` tags each row with the current `git rev-parse --short HEAD` unless
`BENCH_COMMIT` is set (CI sets it to the commit SHA). Only commit `benches/history.csv`
alongside a change if the numbers actually mean something for that change — don't
commit noise from an idle-machine sanity check.

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
4. For performance changes, include before/after benchmark results (`cargo run --release --example bench_compare` after recording both, or Criterion's own before/after comparison).
5. Open a PR with a clear description of the change.

## Architecture Overview

See the [Architecture reference](https://amaye15.github.io/JSON-Tools-rs/reference/architecture.html) in the mdBook guide for details on the tape-based parsing engine, module structure, and parallelism model.
