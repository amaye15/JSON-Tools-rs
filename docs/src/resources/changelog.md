# Changelog

## Unreleased

### Added
- **JVM (Java) bindings**: Apache Spark UDFs (row and batched `mapPartitions` tiers) via a JNI shim over the same Rust core, full feature parity with the Python bindings. See [Setting Up on Databricks](../guide/databricks-setup.md).
- **crates.io and Maven Central publishing** on tagged releases.

### Changed (BREAKING)
- **`key_replacement`/`value_replacement` pattern syntax**: patterns are now literal (exact substring match) by default; wrap a pattern in `r'...'` (e.g. `r'^admin_'`) to use it as a regex. Previously every pattern was always compiled as regex regardless of content. See [Key & Value Replacements](../guide/replacements.md).

### Fixed
- **`has_escape` scanner bug**: escape sequences not adjacent to a quote (a lone `\n`, `\t`, `\r`, `\uXXXX`) were invisible to the tape scanner, so `auto_convert_types`, replacements, `lowercase_keys`, and collision handling could silently operate on still-escaped text for affected strings.
- **Parallelism reverted from Crossbeam back to Rayon**: batch processing now uses Rayon's persistent work-stealing pool instead of spawning fresh `std::thread::scope` OS threads on every `.execute()` call -- measurably faster for small-to-medium batches.
- `unflatten`'s object tree switched from a hash map + full key sort to an order-preserving map (`IndexMap`), removing an O(n) lookup that degraded to O(n^2) for JSON objects used as wide keyed maps.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for full details.

## v0.9.0 (2026-03-09)

### Added
- **DataFrame & Series Support** (Python): Native support for Pandas, Polars, PyArrow, and PySpark DataFrames and Series with perfect type preservation.
- **Crossbeam Parallelism**: Migrated from Rayon to Crossbeam for finer-grained parallel control with scoped threads.
- **Modular Architecture**: Refactored monolithic `lib.rs` into 10 focused modules (`json_parser`, `types`, `error`, `config`, `cache`, `convert`, `transform`, `flatten`, `unflatten`, `builder`) with zero public API changes.

### Performance Improvements

**Rust Core (6 optimizations):**
- Eliminated per-entry HashMap allocation in parallel flatten -- single partial map per chunk
- Added early-exit first-byte discriminators for type conversion fast-path
- SIMD literal fallback for regex patterns (memchr before regex compilation)
- Thread-local regex cache half-eviction (LRU-style, capacity 64)
- Expanded SmallVec buffers (32 -> 64 bytes) and separator cache
- Vectorized `clean_number_string()` with SIMD skip helpers

**Python Bindings (3 optimizations):**
- `mem::take` for zero-cost builder field extraction
- Batch type detection via first-element sampling
- O(1) DataFrame/Series reconstruction

## v0.8.0 (2026-01-01)

- **Python Feature Parity**: `auto_convert_types`, `parallel_threshold`, `num_threads`, `nested_parallel_threshold` in Python
- **Enhanced Type Conversion**: ISO-8601 dates, currency codes, basis points, suffixed numbers
- **Date Normalization**: Automatic UTC normalization

## v0.7.0 (2025-10-17)

- Parallel configuration methods (`parallel_threshold`, `num_threads`, `nested_parallel_threshold`)
- HashMap capacity and hashing optimizations

## v0.6.0 (2025-10-13)

- Python GIL release for parallel operations (5-13% improvement)
- Inline hints on hot functions

## v0.5.0 (2025-10-12)

- Rust inline optimizations (2-5% improvement)
- Iterator adapter chains

## v0.4.0 (2025-10-11)

- FxHashMap migration (30-55% improvement)
- SIMD JSON parsing (sonic-rs / simd-json)
- SmallVec stack allocation
- Arc\<str\> key deduplication

## v0.3.0 (2025-10-10)

- Automatic type conversion
- Python bindings via PyO3

## v0.2.0 (2025-10-09)

- Key collision handling
- Comprehensive filtering (empty strings, nulls, objects, arrays)
- Regex-based replacements

## v0.1.0 (2025-10-08)

- Initial release
- JSON flattening and unflattening
- Custom separators
- Batch processing

For the full changelog with migration guides, see [CHANGELOG.md](https://github.com/amaye15/JSON-Tools-rs/blob/master/CHANGELOG.md).
