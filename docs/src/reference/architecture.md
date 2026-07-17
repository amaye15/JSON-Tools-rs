# Architecture

JSON Tools RS is organized into focused, single-responsibility modules. This modular design improves maintainability while preserving performance -- Rust modules are compile-time organization only, with zero runtime overhead.

## Module Structure

```
src/
├── lib.rs            Facade: mod declarations + pub use re-exports
├── json_parser.rs    Conditional SIMD parser (sonic-rs / simd-json)
├── types.rs          Core types: JsonInput, JsonOutput
├── error.rs          Error types with codes E001-E008
├── config.rs         Configuration structs and operation modes
├── cache.rs          Multi-tier regex pattern cache (compile-time table, sticky,
│                     thread-local, global)
├── fxhash.rs         Custom FxHash-style Hasher/BuildHasher for FxHashMap/FxIndexMap
├── convert.rs        Type conversion: numbers, dates, booleans, nulls
├── transform.rs      Filtering, key/value replacements, collision handling
├── flatten.rs        Tape-based flattening engine (scan -> walk -> output)
├── unflatten.rs      Tape-based unflattening with SIMD separator detection
├── builder.rs        Public JSONTools builder API and execute()
├── python.rs         Python bindings via PyO3
├── jvm.rs            JVM bindings via JNI (Java/Spark UDFs, see jvm/)
├── tests.rs          Unit tests
└── main.rs           CLI examples
```

## Module Descriptions

### `json_parser` -- JSON Parsing Abstraction

Conditional compilation wrapper that selects the fastest available JSON parser:
- **64-bit platforms**: sonic-rs (AVX2/SSE4.2 SIMD, 30-50% faster)
- **32-bit platforms**: simd-json (fallback)

Exposes `from_str()`, `to_string()`, and `parse_json()` with a unified `JsonError` type.

### `types` -- Core Types

Defines the public-facing input/output types:
- `JsonInput<'a>` -- Enum accepting `&str`, `&[&str]`, `Vec<String>`, etc.
- `JsonOutput` -- Enum returning `Single(String)` or `Multiple(Vec<String>)`

### `error` -- Error Handling

`JsonToolsError` enum with 8 error variants (E001-E008), each with machine-readable codes, Display/Error impls, and constructors. Includes `From` impls for automatic conversion from parse and regex errors.

### `config` -- Configuration

All configuration structs used by the builder:
- `ProcessingConfig` -- Main config holding all options
- `FilteringConfig` -- Empty string/null/object/array removal
- `CollisionConfig` -- Key collision handling settings
- `ReplacementConfig` -- Key and value replacement patterns
- `OperationMode` -- Flatten, Unflatten, or Normal

### `cache` -- Regex Pattern Caching

Multi-tier cache for compiled regex patterns used by `key_replacement()`/`value_replacement()` (not a general key-deduplication cache -- there is no `phf`-based key cache or `KeyDeduplicator` in the current codebase; `phf` is not a dependency of this crate):
1. **`COMMON_REGEX_PATTERNS`** -- a `LazyLock<FxHashMap<...>>` of ~60 pre-compiled common patterns (whitespace, UUIDs, dates, `user_`/`admin_` prefixes, etc.), checked first
2. **`STICKY_REGEX_CACHE`** -- a tiny thread-local linear-scan cache (capacity 4) of the most recently used patterns, added to short-circuit the hashing/locking below for the common case of the same 1-2 patterns reused across an entire batch
3. **`THREAD_LOCAL_REGEX_CACHE`** -- a larger thread-local `FxHashMap` (capacity 128)
4. **`REGEX_CACHE`** -- a global `RwLock<FxHashMap<Arc<str>, (Arc<Regex>, AtomicU64)>>` (capacity 512)

Tiers 2-4 track per-entry "last used" ticks and evict the genuinely least-recently-used entry when full (not an arbitrary one).

### `fxhash` -- Fast Hashing

Hand-rolled `FxHasher`/`FxBuildHasher` (the same algorithm popularized by `rustc-hash`, reimplemented in-tree rather than taken as a dependency) backing the `FxHashMap`/`FxIndexMap` type aliases used throughout `cache`, `flatten`, and `unflatten`.

### `convert` -- Type Conversion

Automatic type conversion for string values (~1,200 lines, the largest leaf module):
- Number parsing: integers, decimals, currency, percentages, basis points, scientific notation, suffixed (K/M/B)
- Date parsing: ISO-8601 variants with UTC normalization
- Boolean/null detection via direct string matching (`try_parse_bool()`, `is_null_string()`) -- not a `phf` perfect hash map; `phf` is not a dependency of this crate
- SIMD-optimized `clean_number_string()` with `extend_skipping_3/4` helpers

### `transform` -- Transformations

Core transformation logic applied after flatten/unflatten:
- Key/value replacements (literal and regex, with SIMD fast-path)
- Filtering (empty strings, nulls, empty objects/arrays)
- Key collision handling (collect into arrays)
- Lowercase key conversion

### `flatten` -- Flattening Algorithm

Tape-based engine (`scan -> walk -> output`), not a naive recursive `serde_json::Value` walk:
- `scan_and_fixup()` -- single-pass structural scanner producing a `TapeEntry` tape (merges structural scan, validation, container pairing, and string-length computation), with a byte-classification lookup table instead of multiple `memchr` calls
- `SeparatorCache` for pre-computed separator properties (single-byte fast path vs. multi-byte)
- Zero-copy `ValueRef::Raw` byte ranges into the original input, avoiding `serde_json::Value` tree allocation, with a direct-to-output fast path when no key transforms/collision handling are configured
- `flatten_collecting_parallel()` for Rayon-parallel flattening of large objects/arrays once a document crosses `nested_parallel_threshold`
- Arena-allocated (`bumpalo::Bump`) key storage on the slow path (key lowercasing/replacement/collision-handling), avoiding one heap allocation per dotted key path

### `unflatten` -- Unflattening Algorithm

Reconstructs nested JSON from flat key-value pairs using the same tape scanner as `flatten`:
- SIMD-accelerated separator detection (`find_separator()`/`find_separator_offsets()`)
- Path type analysis for array vs. object reconstruction
- Recursive `set_nested_value()`/`set_nested_value_recursive()` and `set_nested_array_value()`
- `FxIndexMap`-backed object tree (insertion-ordered, O(1) lookup) instead of a hash map + full key sort

### `builder` -- Public API

The `JSONTools` struct and its ~19 public methods (3 mode setters, 14 configuration methods, plus `new()` and `execute()`). Routes `execute()` calls to the appropriate processing function based on operation mode (flatten, unflatten, normal).

### `python` -- Python Bindings

PyO3-based Python bindings with:
- Perfect type preservation (input type = output type)
- Native DataFrame/Series support (Pandas, Polars, PyArrow, PySpark)
- GIL release during compute-intensive operations

### `jvm` -- JVM Bindings

JNI-based Java bindings (see [`jvm/`](https://github.com/amaye15/json-tools-rs/tree/master/jvm)
for the Maven project, and the [JVM API reference](./jvm-api.md) for the Java-side
API), primarily for use as Apache Spark UDFs:
- A single-JSON-config-blob handoff from a pure-Java fluent builder (`JsonTools`) to
  an immutable, `Send + Sync` boxed `JSONTools` handle -- no mutex needed, unlike the
  Python bindings' `Mutex<JSONTools>` (which exists there only to support Python's
  step-by-step builder mutation)
- Two Spark usage tiers: a row UDF (`FlattenUDF`/`UnflattenUDF`) and a batched
  `mapPartitions` transform (`BatchTransform`) that reuses the existing rayon-parallel
  batch `execute(Vec<String>)` path

## Processing Pipeline

```
Input → Parse → Flatten/Unflatten → Transform → Filter → Convert → Serialize → Output
         │            │                  │          │         │          │
    json_parser    flatten/         transform   transform   convert   json_parser
                   unflatten
```

For a single JSON document, `flatten`/`unflatten`/`normal` all share one tape scanner (`scan_and_fixup()`, defined in `flatten.rs`, imported by `unflatten.rs` and `transform.rs`) and walk it directly to output. `json_parser`'s conditional SIMD parser (sonic-rs/simd-json) is reserved for a narrower case: a root-level JSON primitive (e.g. a bare `"hello"` or `42`, not an object/array), which falls back to a `serde_json::Value` round-trip in both `flatten` and `unflatten`.

## Public API Surface

All public types are re-exported from `lib.rs`, preserving a flat import path:

```rust
use json_tools_rs::{JSONTools, JsonInput, JsonOutput, JsonToolsError};
use json_tools_rs::{ProcessingConfig, FilteringConfig, CollisionConfig, ReplacementConfig};
```

Internal modules use `pub(crate)` visibility for cross-module access without exposing internals.
