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

## Performance Targets (v0.9.0)

| Operation | Target |
|-----------|--------|
| Basic flatten | >2,000 ops/ms |
| With transformations | >1,300 ops/ms |
| Regex replacements | >1,800 ops/ms |
| Batch (10 items) | >2,500 ops/ms |
| Batch (100 items) | >3,000 ops/ms |
| Roundtrip | >1,000 cycles/ms |

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
└── tests.rs          89 unit tests + 21 doc tests
```

The processing pipeline:

1. **Parse** -- SIMD-accelerated JSON parsing (`json_parser`)
2. **Flatten/Unflatten** -- Recursive traversal with Arc\<str\> key dedup (`flatten`/`unflatten`)
3. **Transform** -- Lowercase, replacements (cached regex), collision handling (`transform`)
4. **Filter** -- Remove empty strings, nulls, empty objects/arrays (`transform`)
5. **Convert** -- Type conversion with first-byte discriminators (`convert`)
6. **Serialize** -- Output to JSON string or native Python types
