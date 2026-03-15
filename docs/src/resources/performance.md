# Performance & Benchmarks

JSON Tools RS achieves ~2,000+ ops/ms through multiple optimization layers.

## Optimization Techniques

| Technique | Impact |
|-----------|--------|
| **SIMD JSON Parsing** | sonic-rs (64-bit) / simd-json (32-bit) for hardware-accelerated parsing |
| **SIMD Byte Search** | memchr/memmem for fast string operations |
| **FxHashMap** | Fast non-cryptographic hashing via rustc-hash |
| **Tiered Caching** | phf perfect hash -> thread-local FxHashMap -> global DashMap |
| **SmallVec** | Stack allocation for depth stacks and number buffers |
| **Arc\<str\> Dedup** | Shared key storage to minimize allocations |
| **First-Byte Discriminators** | Rapid rejection of non-convertible strings |
| **Crossbeam Parallelism** | Scoped thread pools for batch and nested parallelism |
| **Zero-Copy (Cow)** | Avoid allocations when strings don't need modification |
| **itoa** | Fast integer-to-string formatting |
| **mimalloc** | Optional high-performance allocator (`features = ["mimalloc"]`, ~5-10% speedup) |

## Benchmark Results

Measured on Apple Silicon. Results from the stress benchmark suite targeting edge cases and large inputs.

### Stress Benchmarks

| Benchmark | Result | Description |
|-----------|--------|-------------|
| Deep nesting (100 levels) | **8.3 us** | Deeply nested objects, 100 levels deep |
| Wide objects (1,000 keys) | **~337 us** | Single object with 1,000 top-level keys |
| Large arrays (5,000 items) | **~2.11 ms** | Array containing 5,000 elements |
| Parallel batch (10,000 items) | **~2.61 ms** | Batch processing with Crossbeam parallelism |

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

The codebase is organized into focused, single-responsibility modules:

```
src/
├── lib.rs            Facade: mod declarations + pub use re-exports
├── json_parser.rs    Conditional SIMD parser (sonic-rs / simd-json)
├── types.rs          Core types: JsonInput, JsonOutput, FlatMap
├── error.rs          Error types with codes E001-E008
├── config.rs         Configuration structs and operation modes
├── cache.rs          Tiered caching: regex, key deduplication, phf
├── convert.rs        Type conversion: numbers, dates, booleans, nulls
├── transform.rs      Filtering, key/value replacements, collision handling
├── flatten.rs        Flattening algorithm with Crossbeam parallelism
├── unflatten.rs      Unflattening with SIMD separator detection
├── builder.rs        Public JSONTools builder API and execute()
├── python.rs         Python bindings via PyO3
└── tests.rs          99 unit tests
```

The processing pipeline:

1. **Parse** -- SIMD-accelerated JSON parsing (`json_parser`)
2. **Flatten/Unflatten** -- Recursive traversal with Arc\<str\> key dedup (`flatten`/`unflatten`)
3. **Transform** -- Lowercase, replacements (cached regex), collision handling (`transform`)
4. **Filter** -- Remove empty strings, nulls, empty objects/arrays (`transform`)
5. **Convert** -- Type conversion with first-byte discriminators (`convert`)
6. **Serialize** -- Output to JSON string or native Python types
