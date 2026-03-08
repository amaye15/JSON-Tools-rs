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

The processing pipeline:

1. **Parse** -- SIMD-accelerated JSON parsing (sonic-rs / simd-json)
2. **Flatten/Unflatten** -- Recursive traversal with Arc\<str\> key dedup
3. **Transform** -- Lowercase, replacements (cached regex), collision handling
4. **Filter** -- Remove empty strings, nulls, empty objects/arrays
5. **Convert** -- Type conversion with first-byte discriminators
6. **Serialize** -- Output to JSON string or native Python types
