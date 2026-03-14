# Architecture

JSON Tools RS is organized into focused, single-responsibility modules. This modular design improves maintainability while preserving performance -- Rust modules are compile-time organization only, with zero runtime overhead.

## Module Structure

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
├── tests.rs          99 unit tests
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
- `FlatMap` -- Internal type alias for `FxHashMap<Arc<str>, Value>`

### `error` -- Error Handling

`JsonToolsError` enum with 8 error variants (E001-E008), each with machine-readable codes, Display/Error impls, and constructors. Includes `From` impls for automatic conversion from parse and regex errors.

### `config` -- Configuration

All configuration structs used by the builder:
- `ProcessingConfig` -- Main config holding all options
- `FilteringConfig` -- Empty string/null/object/array removal
- `CollisionConfig` -- Key collision handling settings
- `ReplacementConfig` -- Key and value replacement patterns
- `OperationMode` -- Flatten, Unflatten, or Normal

### `cache` -- Caching Infrastructure

Three-tier caching system for performance:
1. **phf perfect hash** (`COMMON_JSON_KEYS`) -- Zero-cost lookup for common keys
2. **Thread-local FxHashMap** (`KeyDeduplicator`) -- Per-thread key deduplication
3. **Global DashMap** (`REGEX_CACHE`) -- Compiled regex pattern cache with LRU eviction

### `convert` -- Type Conversion

Automatic type conversion for string values (~1,000 lines, the largest leaf module):
- Number parsing: integers, decimals, currency, percentages, basis points, scientific notation, suffixed (K/M/B)
- Date parsing: ISO-8601 variants with UTC normalization
- Boolean/null detection via phf perfect hash maps
- SIMD-optimized `clean_number_string()` with `extend_skipping_3/4` helpers

### `transform` -- Transformations

Core transformation logic applied after flatten/unflatten:
- Key/value replacements (literal and regex, with SIMD fast-path)
- Filtering (empty strings, nulls, empty objects/arrays)
- Key collision handling (collect into arrays)
- Lowercase key conversion

### `flatten` -- Flattening Algorithm

Recursive JSON flattening with performance optimizations:
- `SeparatorCache` for pre-computed separator properties
- `FastStringBuilder` with thread-local caching
- `flatten_value_with_threshold()` for Crossbeam parallel flattening of large objects/arrays
- `quick_leaf_estimate()` for O(1) HashMap pre-sizing

### `unflatten` -- Unflattening Algorithm

Reconstructs nested JSON from flat key-value pairs:
- SIMD-accelerated separator detection (`find_separator*()` functions)
- Path type analysis for array vs. object reconstruction
- Recursive `set_nested_value()` and `set_nested_array_value()`

### `builder` -- Public API

The `JSONTools` struct and all 35+ builder methods. Routes `execute()` calls to the appropriate processing function based on operation mode (flatten, unflatten, normal).

### `python` -- Python Bindings

PyO3-based Python bindings with:
- Perfect type preservation (input type = output type)
- Native DataFrame/Series support (Pandas, Polars, PyArrow, PySpark)
- GIL release during compute-intensive operations

## Processing Pipeline

```
Input → Parse → Flatten/Unflatten → Transform → Filter → Convert → Serialize → Output
         │            │                  │          │         │          │
    json_parser    flatten/         transform   transform   convert   json_parser
                   unflatten
```

## Public API Surface

All public types are re-exported from `lib.rs`, preserving a flat import path:

```rust
use json_tools_rs::{JSONTools, JsonInput, JsonOutput, JsonToolsError};
use json_tools_rs::{ProcessingConfig, FilteringConfig, CollisionConfig, ReplacementConfig};
```

Internal modules use `pub(crate)` visibility for cross-module access without exposing internals.
