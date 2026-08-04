# Performance & Benchmarks

JSON Tools RS achieves ~2,000+ ops/ms through multiple optimization layers.

## Optimization Techniques

| Technique | Impact |
|-----------|--------|
| **SIMD JSON Parsing** | sonic-rs (64-bit) / simd-json (32-bit) -- used for the root-primitive fallback path; the main flatten/unflatten/normal paths use a custom tape scanner instead (see [Architecture](../reference/architecture.md)) |
| **SIMD Byte Search** | memchr/memmem for fast string operations |
| **FxHashMap** | Fast non-cryptographic hashing via a custom in-tree `FxHasher` (`src/fxhash.rs`), not the `rustc-hash` crate |
| **Multi-Tier Regex Cache** | Compile-time common-pattern table -> thread-local "sticky" cache -> larger thread-local `FxHashMap` -> global `RwLock<FxHashMap>`, all LRU-evicted when full |
| **SmallVec** | Stack allocation for depth stacks, number buffers, and 0-2 replacement patterns |
| **CompactString + Arena Keys** | Keys inline up to 24 bytes (`CompactString`); flatten's slow path (key lowercasing/replacement/collision-handling) additionally uses a `bumpalo` arena to avoid one heap allocation per dotted key path |
| **First-Byte Discriminators** | Rapid rejection of non-convertible strings |
| **Rayon Parallelism** | Persistent work-stealing thread pool for batch and nested parallelism (no per-call spawn cost) |
| **Zero-Copy (Cow)** | Avoid allocations when strings don't need modification |
| **Stack-Allocated Integer Formatting** | Custom `IntBuf` formatter for array-index keys (replaced the `itoa` crate) |
| **mimalloc** | Optional high-performance allocator (`features = ["mimalloc"]`, ~14-28% measured on allocation-heavy paths) |
| **orjson** (Python) | Bundled dependency -- replaces the stdlib `json` module for dict/DataFrame-row (de)serialization, with a per-call stdlib fallback for inputs it can't handle (e.g. integers beyond 64-bit range) |
| **Zero-Copy Arrow** (Python) | `pyo3-arrow` reads an embedded JSON-string column directly from a Polars `DataFrame`/PyArrow `Table`'s Arrow buffer instead of round-tripping through the DataFrame's native JSON writer -- ~41-48% faster `execute()` end-to-end for that case; plain pandas/PySpark unaffected |
| **Arrow-Native `normalise()`** (Python) | Reconstruction builds one real Arrow `RecordBatch` directly in Rust (no per-value `PyObject` boxing) and derives every target from it -- measured (interleaved A/B, 100-4,000 columns): a modest, honest **~1-4% end-to-end** win, since core flattening/input serialization -- unchanged by this -- dominates total wall time for typical column-heavy data, not reconstruction. The real deliverable here is correctness, not raw speed: genuinely typed `List<T>` and `Date32`/`Timestamp` columns (no longer stringified) and consistent typing across all four targets, where only PySpark got real type-checking before. `.convert_dates(True)`'s own detection cost is small but real and separately measured: **~0.1-4.5% end-to-end**, including the worst case where nothing actually is a date -- not charged at all when date conversion is off |
| **Flat-DataFrame Fast Path** (Python) | `.flatten().execute(df)` on a DataFrame with no nested columns skips the JSON-text round trip entirely (see [DataFrame & Series Support](../guide/dataframe-support.md#performance-the-flat-dataframe-fast-path)) -- measured (interleaved A/B, 2 rounds, 20K rows x 20 cols): Polars **~2.7-3.7x faster**, PyArrow **~4.2-5.6x faster**, pandas **~1.5-2.2x faster** for scenarios with real per-cell work (`auto_convert_types`, `always_array_keys`), up to **~345x faster** for pandas' pure column-rename case (no value transform, reuses the source `Series` object directly). Automatic, no flag to set; falls back to the existing pipeline for any nested/struct column, embedded-JSON-string column, `remove_nulls`/`value_exclusions`, or a rename-induced column collision |
| **Flat-DataFrame Fast Path: GIL Release** (Python) | A concurrency fix, not a speed one -- same wall-clock time per call, but the fast path above now releases the GIL during its computation like every other `execute()` path does, so it no longer stalls other Python threads in the process for the duration of a large call. Measured via a background pure-Python counting thread's throughput while `execute(df)` runs concurrently (ratio to uncontended solo throughput, 20K x 20 DataFrame): Polars **0.21 -> 1.00**, PyArrow **0.14 -> 0.97**, pandas **0.73 -> 0.76** (smaller -- pandas still builds one `PyObject` per cell with the GIL held) |

## Benchmark Results

Measured on Apple Silicon (M4) via `cargo bench --bench stress_benchmarks -- --quick` against the current source tree -- a quick/low-sample Criterion run, so treat these as indicative rather than lab-precise; re-run the suite yourself (see [Running Benchmarks](#running-benchmarks) below) for reproducible numbers on your own hardware.

### Stress Benchmarks

| Benchmark | Result | Description |
|-----------|--------|-------------|
| Deep nesting (100 levels) | **~2.1 us** | `stress_01_deep_nesting/flatten/100` -- deeply nested object, 100 levels deep |
| Wide objects (1,000 keys) | **~24 us** | `stress_02_wide_objects/flatten/1000` -- single object with 1,000 top-level keys |
| Large arrays (5,000 items) | **~420 us** | `stress_03_large_arrays/flatten/5000` -- array containing 5,000 elements |
| Many small nested objects (10,000, nested-parallel) | **~610 us** | `stress_05_many_small_objects/flatten_parallel/10000` -- single document containing 10,000 small nested objects, flattened with intra-document (Rayon) parallelism enabled |

### Throughput Targets (v0.9.0)

| Operation | Target |
|-----------|--------|
| Basic flatten | >2,000 ops/ms |
| With transformations | >1,300 ops/ms |
| Regex replacements | >1,800 ops/ms |
| Batch (10 items) | >2,500 ops/ms |
| Batch (100 items) | >3,000 ops/ms |
| Roundtrip | >1,000 cycles/ms |

## Performance Tuning

Three threshold parameters control when parallelism activates. Tuning them for your workload can significantly affect throughput.

### `parallel_threshold` (default: 100)

Controls when batch processing (multiple JSON documents) switches from sequential to parallel execution.

**When to lower (e.g., 20-50):**
- Each document is large or complex (deep nesting, many keys)
- CPU cores are available and not contended
- You are processing 50-100 items and want parallel speedup

**When to raise (e.g., 200-500):**
- Each document is small (a few keys, shallow nesting)
- Thread-spawning overhead dominates processing time
- Running inside a container with limited CPU

```python
# For large documents, parallel even at small batch sizes
tools = jt.JSONTools().flatten().parallel_threshold(20)

# For tiny documents, avoid parallelism overhead
tools = jt.JSONTools().flatten().parallel_threshold(500)
```

```rust
let tools = JSONTools::new()
    .flatten()
    .parallel_threshold(50);
```

### `nested_parallel_threshold` (default: 100)

Controls when a single JSON document's top-level keys/array items are processed in parallel (intra-document parallelism). This is independent of batch parallelism.

**When to lower (e.g., 50):**
- Individual documents have very wide objects (500+ keys) with deep sub-trees
- Processing includes expensive transformations (regex replacements, type conversion)

**When to raise (e.g., 500-1000) or effectively disable:**
- Documents are moderately sized (under 100 keys)
- Sub-trees are shallow (1-2 levels), so per-key work is minimal
- You want deterministic (sequential) output ordering

```python
# Large documents with heavy per-key work
tools = jt.JSONTools().flatten().nested_parallel_threshold(50)

# Disable nested parallelism entirely
tools = jt.JSONTools().flatten().nested_parallel_threshold(999_999)
```

### `num_threads` (default: CPU count)

Controls the number of worker threads for parallel processing.

**When to set explicitly:**
- Running alongside other CPU-intensive workloads -- limit threads to avoid contention
- In a container or VM with a CPU quota -- match thread count to available cores
- Benchmarking -- fix thread count for reproducible results

```python
tools = jt.JSONTools().flatten().num_threads(4)
```

```rust
let tools = JSONTools::new()
    .flatten()
    .num_threads(Some(4));
```

### Environment Variable Overrides

All threshold defaults can be overridden without code changes via environment variables. These are read once at process startup (via `LazyLock`).

| Variable | Default | Description |
|----------|---------|-------------|
| `JSON_TOOLS_PARALLEL_THRESHOLD` | `100` | Minimum batch size for parallel processing |
| `JSON_TOOLS_NESTED_PARALLEL_THRESHOLD` | `100` | Minimum keys/items for nested parallelism |
| `JSON_TOOLS_NUM_THREADS` | (CPU count) | Thread count for parallel processing |
| `JSON_TOOLS_MAX_ARRAY_INDEX` | `100000` | Maximum array index during unflattening |

```bash
# Example: tune for a workload of many small documents
export JSON_TOOLS_PARALLEL_THRESHOLD=200
export JSON_TOOLS_NUM_THREADS=8

python my_pipeline.py
```

Environment variable values are parsed as `usize`. Invalid values (non-numeric, negative) silently fall back to the default.

## Running Benchmarks

```bash
# All benchmarks
cargo bench

# Specific suite
cargo bench --bench isolation_benchmarks
cargo bench --bench comprehensive_benchmark
cargo bench --bench stress_benchmarks
cargo bench --bench realworld_benchmarks
cargo bench --bench combination_benchmarks
```

## Benchmark Suites

| Suite | Focus |
|-------|-------|
| `isolation_benchmarks` | Individual features in isolation (10 groups) |
| `combination_benchmarks` | 2-way and 3-way feature interactions |
| `realworld_benchmarks` | AWS CloudTrail, GitHub API, K8s, Elasticsearch, Stripe, Twitter/X |
| `stress_benchmarks` | Edge cases: deep nesting, wide objects, large arrays |
| `comprehensive_benchmark` | Full feature coverage (15 groups) |

## Profiling

On macOS, use samply for profiling:

```bash
# Build with profiling symbols
cargo bench --profile profiling --bench stress_benchmarks --no-run

# Profile with samply
samply record --save-only -o /tmp/profile.json -- \
    ./target/profiling/deps/stress_benchmarks-* --bench

# View results
samply load /tmp/profile.json
```

## Architecture

The codebase is organized into focused, single-responsibility modules (see [Architecture](../reference/architecture.md) for the full breakdown):

```
src/
├── lib.rs            Facade: mod declarations + pub use re-exports
├── json_parser.rs    Conditional SIMD parser (sonic-rs / simd-json) -- used for the
│                     root-primitive fallback, not the main tape-based paths
├── types.rs          Core types: JsonInput, JsonOutput
├── error.rs          Error types with codes E001-E008
├── config.rs         Configuration structs and operation modes
├── cache.rs          Multi-tier regex pattern cache (common-pattern table, sticky,
│                     thread-local, global RwLock)
├── fxhash.rs         Custom FxHash-style Hasher for FxHashMap/FxIndexMap
├── convert.rs        Type conversion: numbers, dates, booleans, nulls
├── transform.rs      Filtering, key/value replacements, collision handling
├── flatten.rs        Tape-based flattening engine (scan -> walk -> output)
├── unflatten.rs      Tape-based unflattening with SIMD separator detection
├── builder.rs        Public JSONTools builder API and execute()
├── python.rs         Python bindings via PyO3
├── tests.rs          Unit tests
└── main.rs           CLI examples
```

The processing pipeline:

1. **Parse** -- single-pass tape scan (`scan_and_fixup()`, shared by `flatten`/`unflatten`/`transform`); `json_parser`'s SIMD parser only handles the root-primitive edge case
2. **Flatten/Unflatten** -- tape walk with `CompactString`-inlined keys (and an arena allocator for the slow path involving key transforms) (`flatten`/`unflatten`)
3. **Transform** -- Lowercase, replacements (cached regex), collision handling (`transform`)
4. **Filter** -- Remove empty strings, nulls, empty objects/arrays (`transform`)
5. **Convert** -- Type conversion with first-byte discriminators (`convert`)
6. **Serialize** -- Output to JSON string or native Python types
