# Changelog

## Unreleased

### Added
- **`.always_array_keys([...])`** -- flattened key names that must always render as a JSON array, even with only one value present, keeping a key's shape consistent across every document/row of a batch regardless of `.handle_key_collision()`. Also guarantees `normalise()` resolves that column to `List<T>` even when a particular batch has zero collisions for it. See the [Key Collision Handling guide](../guide/collision-handling.md#consistent-shape-across-documents-always_array_keys).

### Performance
Audited every clone/copy site in the codebase (the core engine had zero `.clone()` calls already). `PyJsonOutput.get_single()`/`get_multiple()`/`__str__()` (the `execute_to_output()` API) cloned a result before PyO3's own unavoidable copy at the FFI boundary -- fixed to build the Python object directly from the borrowed value. **Measured ~25-27% faster**, confirmed via interleaved A/B, zero behavior change. Two further changes made on the same reasoning (an `unflatten.rs` allocation avoidance, a `flatten.rs` capacity hint) were measured the same way and honestly did not hold up as real wins -- reported plainly rather than claimed.

## v0.9.21 (2026-08-01)

### Performance
Profiling-driven follow-up round after v0.9.20 (`samply`/macOS `sample` against the Criterion stress suite, plus a targeted audit of the Arrow-native `normalise()` path). Three fixes, each verified via interleaved A/B:
- **Date/datetime normalization output no longer re-parses a `chrono` format string per value** (`src/convert.rs`) -- hand-rolled formatting replaces `DateTime::format()`. **~10.6% faster** for date-heavy `convert_dates(True)` workloads.
- **`unflatten()` no longer allocates a fresh path buffer per flattened key** (`src/unflatten.rs`) -- one buffer reused across a document instead of one per entry. **~3.8% faster** for wide flattened documents.
- **`normalise=True`'s Arrow-native reconstruction no longer parses a list-valued column's JSON array text twice** (`src/python.rs`) -- exactly `handle_key_collision(True)`'s own headline scenario. **~5-6% faster** for a collision-heavy scenario.

Second round, focused on batch processing and DataFrame conversion. Batch processing's core parallel dispatch was profiled directly and found already optimal (no fix needed); batching Python-side JSON parse calls was tested and found to make no difference (correctly abandoned before shipping). The real cost, found via `cProfile`: DataFrame extraction re-allocated an owned `String` per JSON object key on every row. `unnest_object_valued_columns` and `splice_row` now try a zero-copy key parse first, falling back to owned keys only when a key needs unescaping. **~6-9% faster per call** for un-nesting (a quarter of a realistic `execute(df)` call), **~8.9% faster end to end** for embedded-JSON-string-column DataFrames.

Third round, same focus: found the same "owned key per row" pattern one level up in `build_normalise_table`, with a second cost stacked on top -- `key_order.insert(key.clone())` ran for every key of every row, even ones already seen (at issue #31's scale, 754 rows x 4,042 columns, ~3 million wasted clones). Now a `contains` check first means a key only allocates once, the first time it's actually new. **~13.0% faster** for the issue #31-scale scenario, the largest single win of these three rounds. The same key-parse fix applied to `splice_zerocopy_columns` for consistency, measured as noise-level (not claimed as a win) since that path already minimizes what it re-parses per row by design.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.20 (2026-07-31)

### Changed (BREAKING)
- **DataFrame column expansion no longer prefixes with the source column's name** -- a column named `payload` holding `{"user": {"name": "Alice"}}` now expands to `user.name`, not `payload.user.name` (dict/struct-typed columns and JSON-string columns alike, in both `execute(df)` and `execute(df, normalise=True)`). Genuine nesting *within* a column's content still prefixes normally; array-valued columns are unaffected (`tags.0`, `tags.1`, ...). A key colliding across two columns resolves via the existing `.handle_key_collision()` setting. See [DataFrame & Series Support](../guide/dataframe-support.md#auto-expanding-json-string-columns).
- **`normalise=True`/`target=...` reconstruction is now Arrow-native** ([issue #35](https://github.com/amaye15/JSON-Tools-rs/issues/35)) -- one real Arrow `RecordBatch` built directly in Rust, no new methods. `handle_key_collision(True)` list columns now build as real, correctly-typed `List<T>` instead of being stringified. Recognized date/datetime columns build as real `Date32`/`Timestamp` columns, gated on `.convert_dates(True)`/`.auto_convert_types(True)` (never independently guessed). `target="pandas"` output uses Arrow-backed dtypes (`int64[pyarrow]`, ...) -- genuinely zero-copy, but a breaking dtype change. `target="pandas"`/`"pyspark"` now require `pyarrow` installed (`target="polars"` does not). See [DataFrame & Series Support](../guide/dataframe-support.md#normalise-always-get-a-wide-dataframe).

### Performance
- **`normalise=True`/`target=...` reconstruction: honest, modest ~1-4% end-to-end win** -- core flattening/input serialization dominate total time for typical data, not reconstruction; the real win this round is column-typing correctness, not speed.
- **`.convert_dates(True)`'s own detection cost: ~0.1-4.5% end-to-end**, measured directly including the worst case (conversion on, no actual dates present) -- not charged when date conversion is off.

### Removed (BREAKING)
- **The JVM/Java/Scala binding has been removed entirely** -- `jvm/`, `src/jvm.rs`, the `jni` dependency/`jvm` Cargo feature, and the JVM CI workflow are all gone, and the `io.github.amaye15:json-tools-rs-spark` Maven Central artifact will not receive new versions (`0.9.19` remains available there). The Rust core and Python bindings are unaffected. Databricks/Spark users should switch to the Python bindings wrapped in a `pandas_udf` -- see [Setting Up on Databricks](../guide/databricks-setup.md).

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.19 (2026-07-30)

### Changed
- **MSRV raised from 1.80 to 1.85** -- required for current `pyo3-arrow` releases (see Performance below). Affects every source (`cargo add`) consumer.

### Performance
- **`execute()` on a Polars `DataFrame`/PyArrow `Table` with an embedded JSON-string column is ~41-48% faster end-to-end.** Detection/extraction of such columns now uses `pyo3-arrow`'s zero-copy Arrow buffer access instead of round-tripping through the DataFrame's native JSON writer (escape, then immediately unescape). Column ordering is preserved exactly as before. Scoped to Polars/PyArrow -- plain pandas and PySpark are unaffected, still using the existing path.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.18 (2026-07-30)

### Performance
- **`execute()` on a PyArrow `Table`/`RecordBatch` is ~2x faster** -- extraction now bridges through pandas's native JSON writer (using `types_mapper=pd.ArrowDtype` to avoid a real integer-with-nulls-to-float corruption bug caught while building this fix) instead of `to_pylist()` + per-item conversion. `splice_row`'s per-key escaping now reuses the crate's existing zero-allocation key writer instead of `serde_json::to_string`; a real cleanup, though measured end-to-end impact was within noise at realistic scale.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.17 (2026-07-30)

### Performance
- **Core `flatten()`/`unflatten()` collision-handling paths ~5-8% faster** for documents that trigger key transforms/collision detection -- eliminated a redundant second hashmap lookup per unique key in both `flatten.rs` and `unflatten.rs`. The remaining non-`normalise` DataFrame/Series reconstruction functions now use the same `PyOnceLock` import caching added in 0.9.16. `mimalloc`'s doc comment updated to real measured numbers (~14-28%, previously an unverified "~5-10%") plus new CI coverage; still not used for published wheels/jars.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.16 (2026-07-30)

### Performance
- **`execute(..., normalise=True, target=...)` and PySpark `execute(spark_df)` are 13-18% faster for large/wide results** (e.g. 754 rows x 4,042 columns). `union_and_columnarize` rewritten from an O(rows x columns) `PyDict` hash-lookup pattern to a single forward pass, plus `PyOnceLock`-cached pandas/polars/pyarrow/pyspark module imports across the reconstruction path. Verified via interleaved A/B against the real Python API.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.15 (2026-07-29)

### Fixed
- **`execute(spark_df)` no longer crashes when a `.key_replacement()`/`.handle_key_collision(True)` list column holds genuinely mixed element types** ([#33](https://github.com/amaye15/JSON-Tools-rs/issues/33)). The list-flavored twin of the 0.9.14 fix: a collision list is built from each colliding key's own independently-converted value, so a single row's collision could already mix kinds (e.g. `[100, "abc"]`); such columns now fall back to string elements, while uniformly-typed list columns (e.g. all `int`) correctly get a typed array instead of unnecessary stringification.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.14 (2026-07-29)

### Fixed
- **`execute(spark_df)` with `auto_convert_types(True)` no longer crashes on columns with genuinely mixed types** ([#32](https://github.com/amaye15/JSON-Tools-rs/issues/32)). A column that ends up holding e.g. both `str` and `int` values across rows (a natural consequence of per-value auto-conversion) previously broke Spark's Arrow bridge with `PySparkTypeError`; such columns now fall back to a uniform string column instead, while `int`/`float`-only mixes still promote correctly to `double`. Also hardens `normalise(target=...)` for all four DataFrame backends, which share the same column-unioning step.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.13 (2026-07-29)

### Fixed
- **`execute(df)` on a PySpark DataFrame now returns a real, distributed `pyspark.sql.DataFrame`, not a plain `list[dict]`** ([#31](https://github.com/amaye15/JSON-Tools-rs/issues/31)). **Behavior change:** code relying on the old list fallback needs updating.

### Performance
- **JSON-string-column auto-expansion (0.9.12) is ~40-43% faster for large embedded payloads**, rewritten around `serde_json::value::RawValue` to avoid a redundant full-tree parse/reserialize per row, while keeping the same graceful per-row fallback behavior. See [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full root-cause writeup.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.12 (2026-07-29)

### Fixed
- **`execute(df)` in `.flatten()` mode now auto-expands DataFrame columns holding JSON *strings*, not just columns already typed as dicts/structs** ([#30](https://github.com/amaye15/JSON-Tools-rs/issues/30)) -- see [Auto-Expanding JSON-String Columns](../guide/dataframe-support.md#auto-expanding-json-string-columns). **Behavior change:** a DataFrame with a JSON-string column now produces more/differently-shaped output columns in flatten mode than before; `.unflatten()`/`.normal()` mode are unaffected.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.11 (2026-07-28)

### Fixed
- **`normalise(target="pyspark")` could silently corrupt an all-`None` column** on Spark's non-Arrow fallback path (taken when pyarrow isn't installed) -- a missing value could serialize as the literal string `"<NA>"` instead of a real null. Fixed by passing an explicit schema to `createDataFrame` instead of relying on Spark to infer it.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.10 (2026-07-28)

### Fixed
- **Critical: `auto_convert_types` panicked on multi-byte UTF-8 content in specific positions** (e.g. `"5€ García"`, a `"+1Á2"` timezone offset) -- two fixed-byte-offset string slices assumed the offset was always a UTF-8 character boundary. Fixed with an `is_char_boundary` guard at each site; no behavior change for valid inputs. ([#29](https://github.com/amaye15/JSON-Tools-rs/issues/29))

### Added
- **Python: `JSONTools` is now picklable**, including across a real process boundary (e.g. captured in a PySpark UDF/`mapInPandas` closure via cloudpickle) -- via `__reduce__` plus a new `to_config_json()`/`from_config_json()` method pair. ([#29](https://github.com/amaye15/JSON-Tools-rs/issues/29))
- **Python: `execute(input, normalise=True, target=None)`** -- always returns a wide DataFrame (one column per flattened key) regardless of input shape, working natively across pandas, polars, pyarrow, and now genuinely PySpark (a real `pyspark.sql.DataFrame`, closing the previous list-of-dicts fallback for this path). See [DataFrame & Series Support](../guide/dataframe-support.md#normalise-always-get-a-wide-dataframe).

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.8 (2026-07-26)

### Changed
- **Python: [orjson](https://github.com/ijl/orjson) is now a required dependency**, used automatically as the dict/DataFrame-row JSON (de)serialization backend -- `pip install json-tools-rs` is all that's needed, no separate opt-in. A per-call fallback to the standard library still covers inputs orjson can't handle -- see [Installation](../getting-started/installation.md).

### Performance
- Python binding: exact-typed `str`/`dict`/`list` inputs skip DataFrame/Series detection entirely, and the JSON callables are resolved once instead of per call. Dict-input calls ~37% faster, str-input calls ~39% faster.
- JVM binding: native `execute`/`executeBatch` now cross the JNI boundary as UTF-8 `byte[]` instead of `String` (JIT-intrinsified on the Java side, avoiding JNI's UTF-16 conversion). ~22-38% faster per call, ~33% faster for batches. Public API unchanged.
- Unflatten: nested-container capacity hints tuned against this project's own benchmark corpus (was an undersized flat guess), plus a single-lookup `entry()` replacing a double hash probe. ~5-6% faster unflatten, ~4-5% faster roundtrip.
- Core scanner: removed a double-scan of scalar/whitespace bytes in the tape scanner. ~13-16% faster flatten across payload sizes.

See the repository's [CHANGELOG.md](https://github.com/amaye15/json-tools-rs/blob/master/CHANGELOG.md) for the full, itemized list.

## v0.9.7 (2026-07-20)

### Added
- **`.exclude_key(pattern)`** (Rust/Python/JVM): drop any key -- and its entire value/subtree -- whose name contains `pattern` (literal by default, `r'...'` for regex). Additive. Works identically in `.flatten()`, `.unflatten()`, and `.normal()` mode; matching a container key drops its entire subtree in O(1) without walking it. Array elements are never matched. See [Key Exclusion](../guide/replacements.md#key-exclusion).
- **`.exclude_value(pattern)`** (Rust/Python/JVM): drop a key-value pair whose value contains `pattern`. Same convention as `.exclude_key()`. Only applies to scalar leaf values; checked after `.value_replacement()`/`.auto_convert_types()` have run, matching `.remove_nulls()`'s ordering guarantee. A no-op at the document root. In `.unflatten()` mode, string values are matched against their JSON-serialized (quoted) form -- see [Value Exclusion](../guide/replacements.md#value-exclusion) for the regex-anchor caveat this implies.

### Fixed
- **`.remove_nulls()` now runs consistently last across `.flatten()`/`.unflatten()`/`.normal()` mode.** Previously `.value_replacement()` and `.auto_convert_types()` composed in three different orders across the three engines (flatten and unflatten each had a real ordering bug; only normal mode was already correct), so the same document/config could produce different results depending on mode, and a value that only became null after a replacement could slip past `.remove_nulls()`. All three engines -- including each one's root-primitive (bare-scalar-document) path -- now compose replacement-then-conversion identically.

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
