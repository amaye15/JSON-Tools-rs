# Changelog

## v0.9.6 (2026-07-19)

### Added
- **Fine-grained, per-category control over automatic type conversion**, across Rust, Python, and JVM: `.convert_dates()`, `.convert_nulls()`, `.convert_booleans()`, `.convert_numbers()` let each category be enabled/disabled independently instead of the previous all-or-nothing `.auto_convert_types(bool)` (unchanged, still means "all four categories, default behavior"). Each category also accepts real customization via a `_config` method/kwargs/dedicated fluent methods (per language idiom) -- dates: `normalize_to_utc`/`assume_utc_for_naive`; nulls/booleans: extra recognized tokens (additive); numbers: individually disable currency, percent/permille, text basis points, K/M/B/T suffixes, fractions, or hex/binary/octal parsing. See [Type Conversion](../guide/type-conversion.md#fine-grained-control).
- New public types: `TypeConversionConfig`, `DateConversionConfig`, `NullConversionConfig`, `BooleanConversionConfig`, `NumberConversionConfig`, plus runnable examples, tests, and benchmarks for the new API across all three languages.

### Changed (BREAKING)
- `ProcessingConfig` (and `FilteringConfig`/`CollisionConfig`/`ReplacementConfig`) are now `#[non_exhaustive]`, matching `JsonToolsError`'s existing precedent. Breaks external code constructing these via a bare struct literal instead of `::new()` + the fluent builder methods.
- `ProcessingConfig.auto_convert_types: bool` removed, replaced by `ProcessingConfig.type_conversion: TypeConversionConfig`. The `JSONTools` builder's own `.auto_convert_types(bool)` method is unaffected.

### Performance
- The existing, heavily-profiled `try_convert_string_to_json_bytes` hot path is unmodified -- it remains the code path for the common (all-default) case, selected via a mode cached once per `execute()` call. `all_default_via_new_api` benchmark confirms within ~1% of the prior `auto_convert_types` cost.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list including edge-case coverage details.

## v0.9.5 (2026-07-18)

### Fixed
- **Documentation-wide accuracy sweep**: every root-level doc, the full mdBook site, and the JVM Java source's own doc comments audited against actual source code and live runtime behavior across four parallel passes, rather than trusting existing prose. Corrected fabricated/stale internals (references to a `phf` key cache, `rustc-hash`, `Arc<str>` key dedup, and function names that no longer exist), benchmark numbers stale by up to 14x, wrong error-handling semantics, several broken guide examples (a `.normal()` mode key-replacement/lowercasing ordering bug, an impossible collision-handling example, a no-op Polars example), and stale "not yet published" claims for Maven Central/PyPI (both have been live for a while). Fixed a real internal contradiction in the JVM Java source itself (`FlattenUDF`/`BatchTransform` javadoc claimed Lakeflow Pipeline support that Databricks doesn't allow). See [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

### Added
- **Runnable examples** covering every builder feature individually, plus curated multi-feature pipelines, mirrored with matching inputs/outputs across Rust, Python, and Java.
- **[JVM API reference](./jvm-api.md)** page, closing a gap where Rust and Python each had one and the JVM bindings didn't.

### Changed
- Regex pattern lookup for `key_replacement`/`value_replacement` no longer re-hashes and re-walks the cache on every key/value check -- a thread-local "sticky" cache of recently-used patterns short-circuits the common case. ~9-22% faster on regex-heavy scenarios (Criterion).
- Consolidated two near-duplicate replacement-application code paths, which also fixed a missing SIMD fast-path for literal value replacement (~15-19% faster for that case).

## v0.9.4 (2026-07-17)

### Fixed
- **`auto_convert_types` silently corrupted the trailing digits of large integer strings**: numeric-string-to-JSON-number conversion always routed every candidate through `f64` (only ~15-17 significant decimal digits of exact precision) before reformatting, so any string-encoded integer longer than that came back corrupted, e.g. `"999999999999999999"` → `1000000000000000000`. Real-world 64-bit IDs (Snowflake/Discord/database bigint primary keys) are commonly stored as JSON strings *specifically to avoid* this exact class of precision loss elsewhere, and are typically 17-19 digits, so this was a live bug. Already-canonical integer strings are now reused directly instead of being parsed to `f64` and reformatted, covering the entire range the previous float round-trip claimed to support (checked precisely against `i64`/`u64` bounds, not a rough digit-count cutoff).

### Changed
- **Python bindings: `dict`/`list[dict]`/DataFrame/Series conversion switched from `pythonize`/`depythonize` to Python's own `json` module.** Benchmarked against the actual built extension (not just reasoned about): `depythonize`'s generic serde-based Python↔Rust traversal was 5-30% *slower* than a plain `json.dumps`/`json.loads` round-trip for nested dicts (the case `.flatten()`/`.unflatten()` exist for), and ~1.6x slower end-to-end for DataFrame rows (this library's other headline feature) — the reverse of what the code's own prior comments claimed. DataFrame input now uses each library's native line-delimited JSON export (pandas `to_json`, polars `write_ndjson`) instead of `to_dict()`/`to_dicts()` + per-row conversion. Removes the `pythonize` dependency entirely. Trade-off, reported honestly: flat/shallow dicts are slower under the new approach (still microsecond-scale in absolute terms) — see [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full numbers.
- Credit/debit currency suffix stripping (`"100CR"`/`"100DR"`, part of `auto_convert_types`) no longer chains `str::trim_end_matches` calls with string patterns, which forced std to construct generic substring-search machinery for a fixed 2-byte suffix check. ~13-17% faster on currency-heavy conversion (Criterion).
- Literal (non-regex) key/value replacement now locates matches with SIMD substring search (`memchr::memmem`) instead of `str::replace`'s matcher. ~2.6-4.8% faster (Criterion).
- `unflatten`'s internal object maps (root and per-branch) now start pre-sized instead of growing from empty capacity one key at a time. ~7-9% faster combined (Criterion), found via sampling profiler.
- `auto_convert_types`'s date detection now validates via `chrono`'s direct date constructors instead of its generic format-string parser. ~25% faster on mixed real-dates/false-positive-numeric-ID workloads.

### Added
- `flatten`'s slow path (key lowercasing/replacement/collision-handling configured) now uses an arena allocator for key storage on single-document processing, instead of allocating each dotted key path individually. Up to ~14% faster end-to-end on deep-nesting workloads; neutral on shallow/mixed data.

## v0.9.3 (2026-07-16)

### Fixed
- **`flatten` produced invalid JSON for keys with escaped characters**: any key containing `\"`, `\\`, or a control-character escape produced syntactically invalid JSON output when no key transform (`lowercase_keys`/`key_replacement`/collision-handling) was configured -- the default, most common usage. The fast path unescaped such keys to build its internal path buffer but never re-escaped before writing that buffer directly as the output key.
- **Re-escaping corrupted multi-byte UTF-8 characters**: whenever a string needed escaping (an embedded quote, backslash, or control character) and also contained non-ASCII text, the slow escaping path reinterpreted each byte individually as its own Latin-1 codepoint, e.g. turning `café "quoted"` into `cafÃ© \"quoted\"`. Affected key escaping under `lowercase_keys`/`key_replacement`/collision-handling, value escaping under `value_replacement`, and `unflatten`'s key serialization.

### Changed
- JSON object keys now use `CompactString` instead of `String`, inlining keys up to 24 bytes with no heap allocation. `unflatten` is ~19-22% faster (Criterion, p < 0.05).
- `unflatten`'s tree-building pass no longer re-scans each key's separators a second time.
- The regex pattern cache now evicts the genuinely least-recently-used entry when full, instead of an arbitrary one.
- `unflatten`'s output buffer is sized from the input JSON's byte length instead of a fixed 256-byte default.

## v0.9.2 (2026-07-15)

Note: `v0.9.1` was tagged the day before but only completed publishing to Maven
Central -- a crates.io/PyPI release pipeline bug caused those two to fail before any
upload. Fixed and re-cut as v0.9.2 across all three registries; no code changes
beyond the release pipeline fix itself.

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
- `mem::replace` -> `mem::take` across 13 builder methods, eliminating a default `JSONTools::new()` construction per call
- O(N) -> O(1) DataFrame/Series reconstruction (single `into_pyobject` + `clone_ref` instead of a per-item clone)
- GIL release via `py.detach()` during compute-intensive operations

## v0.8.0 (2026-01-01)

- **Full Python Bindings Feature Parity**: all Rust features now available in Python, including `.auto_convert_types()`, `.parallel_threshold()`, `.num_threads()`, and `.nested_parallel_threshold()`
- 128 comprehensive Python tests covering all features

## v0.7.0 (2025-10-17)

- Parallel configuration methods (`parallel_threshold`, `num_threads`, `nested_parallel_threshold`)
- HashMap capacity and hashing optimizations

## v0.6.0 (2025-10-13)

- Python GIL release for parallel operations (5-13% improvement)
- Inline hints on hot functions

## v0.5.0 (2025-10-12)

- `#[inline(always)]` on hot-path functions and `#[cold]`/`#[inline(never)]` on error paths (2-5% additional improvement, 32-60% cumulative from baseline)

## v0.4.0 (2025-10-11)

- FxHashMap replacing standard `HashMap` (15-30% faster string key operations)
- SIMD JSON parsing optimizations, reduced string clones (~50% fewer), pre-allocated collections (30-55% overall improvement)

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

For the full changelog with migration guides, see [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md).
