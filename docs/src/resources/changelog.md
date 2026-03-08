# Changelog

## v0.9.0 (2026-03-09)

### Added
- **DataFrame & Series Support** (Python): Native support for Pandas, Polars, PyArrow, and PySpark DataFrames and Series with perfect type preservation.
- **Crossbeam Parallelism**: Migrated from Rayon to Crossbeam for finer-grained parallel control with scoped threads.

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
