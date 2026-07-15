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

### Profile-Guided Optimization (PGO)

Published Python wheels for windows and macos are built with PGO: an instrumented
build runs a representative training workload (`scripts/pgo_train.py`, driven through
the actual Python bindings) to collect branch/call-frequency profiles, which are then
fed back into a final optimized build. Locally validated ~5-17% faster across
flatten/unflatten/normal-mode scenarios, both at the pure-Rust level (via
`cargo-pgo` + `examples/bench_quick.rs`) and at the actual wheel level (via `maturin`
+ manual `RUSTFLAGS`). See `.github/workflows/maturin-ci.yml`'s `windows`/`macos` jobs
for the exact CI sequence. `linux`/`musllinux` wheels are **not** PGO'd: those build
inside Docker/QEMU (for manylinux/musllinux compliance and cross-arch targets), which
makes running an instrumented binary on the target architecture during CI meaningfully
harder to wire up safely -- left as a deliberate follow-up, not an oversight.

To reproduce locally (macOS/Linux; adjust venv activation paths on Windows):

```bash
rustup component add llvm-tools-preview

# 1. Instrumented build, installed into a scratch venv
python3 -m venv .venv-pgo
source .venv-pgo/bin/activate
pip install maturin
RUSTFLAGS="-Cprofile-generate=$PWD/pgo-data" maturin develop --release --features python,mimalloc

# 2. Train against realistic workloads through the actual Python bindings
python3 scripts/pgo_train.py

# 3. Merge the collected profiles
PROFDATA="$(rustup which llvm-profdata)"
"$PROFDATA" merge -o pgo-data/merged.profdata pgo-data/*.profraw

# 4. Rebuild the wheel using the merged profile
RUSTFLAGS="-Cprofile-use=$PWD/pgo-data/merged.profdata" maturin build --release --features python,mimalloc --out dist
```

For the CLI binary / core library (not currently published as a standalone binary
anywhere, so this is a local/optional exercise rather than something CI does):

```bash
cargo install cargo-pgo
cargo pgo instrument run -- --release --example bench_quick
cargo pgo optimize build -- --release --example bench_quick
```

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

## Release Process

Pushing a version tag (`git tag vX.Y.Z && git push origin vX.Y.Z`) triggers three
publishes at once, all gated to tag pushes only:

- **crates.io** (`maturin-ci.yml`'s `release` job, `cargo publish`)
- **PyPI** (same job, `maturin-action` upload)
- **Maven Central** (`jvm-ci.yml`'s `release` job, GPG-signed via Sonatype's Central
  Portal -- see [`jvm/pom.xml`](jvm/pom.xml)'s `release` profile)

**Before tagging**, bump the version in all three places and make sure they match --
CI enforces this and will fail the build otherwise:

- `Cargo.toml`'s `[package] version`
- `python/json_tools_rs/__init__.py`'s `__version__` (checked in `rust-ci.yml`'s
  `lint` job)
- `jvm/pom.xml`'s `<version>` (checked in `jvm-ci.yml`'s `package` job, which also
  gates the `release` job)

A GitHub Release with generated notes is created separately by `release.yml`
(also tag-triggered). Both crates.io and Maven Central publishes are **permanent**
(a version can be yanked/deprecated but not deleted) -- there's no `workflow_dispatch`
fallback for either release job, deliberately, so a publish only ever happens from an
actual tag push.

## Architecture Overview

See the [Architecture reference](https://amaye15.github.io/JSON-Tools-rs/reference/architecture.html) in the mdBook guide for details on the tape-based parsing engine, module structure, and parallelism model.
