# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.22] - 2026-08-02

### Added
- **`.always_array_keys([...])`** -- flattened key names that must always
  render as a JSON array, even when only one value is present in a given
  document. `.handle_key_collision(true)` alone makes a key's shape depend
  on whether a collision actually happened *in that specific document*: a
  key that collides in some rows of a batch (e.g. because of
  `.key_replacement()`) but not others ends up a plain value in some
  results and an array in others -- an inconsistent shape that's awkward
  for anything expecting a stable schema, including
  [`normalise()`](https://amaye15.github.io/JSON-Tools-rs/guide/dataframe-support.html)'s
  own column typing, which only resolves a column to `List<T>` once at
  least one row in a given batch actually collides. Naming a key in
  `always_array_keys` guarantees every document with that key emits it as
  an array (`[value]` for one value, the full collected array for more),
  independent of `.handle_key_collision()`, and -- since it acts at the
  `.flatten()` level -- flows through to `normalise()` automatically with
  no changes needed there: a column can now be guaranteed `List<T>` even
  when a particular batch happens to have zero collisions for it. Matched
  against the *final* flattened key name; works for `.flatten()`,
  `.unflatten()`, `.normal()`, and `normalise()`/DataFrame input. See the
  [Key Collision Handling
  guide](https://amaye15.github.io/JSON-Tools-rs/guide/collision-handling.html#consistent-shape-across-documents-always_array_keys).

### Performance
Audited every `.clone()`/copy site across the codebase (the core engine --
`flatten.rs`/`unflatten.rs`/`convert.rs`/`transform.rs` -- already had zero
`.clone()` calls; activity was concentrated in `python.rs`/`builder.rs`).
Three fixes, each measured via interleaved A/B (alternating rebuilds,
multiple rounds) rather than assumed from reasoning alone:
- **`PyJsonOutput.get_single()`/`get_multiple()`/`__str__()`** (`src/python.rs`,
  the `execute_to_output()` API) cloned the Rust `String`/`Vec<String>`
  result before PyO3's own unavoidable Rust->Python copy at the FFI
  boundary -- two full copies where one suffices, building the Python
  object directly from the borrowed value instead (matching the sibling
  `to_python()` method's existing pattern). **Confirmed ~25-27% faster**,
  consistent across 3 interleaved rounds. Zero observable behavior change
  (same Python types/values/error paths, verified directly).
- Two further changes made on the same reasoning -- `unflatten.rs`'s
  array-to-object fallback path avoiding a heap `String` for array indices
  (via the crate's existing stack-buffer `IntBuf` formatter), and a missing
  capacity hint on `flatten.rs`'s `collect_child_ranges` -- were measured
  the same rigorous way and **did not hold up**: the `IntBuf` change showed
  no measurable difference (within noise), and the capacity hint showed
  only a weak, inconsistent signal (~1-2%, favorable in 6/8 rounds).
  Reported honestly rather than claimed as wins; kept since they're
  correct and harmless, not because they're proven faster.

## [0.9.21] - 2026-08-01

### Performance
Follow-up round after v0.9.20, driven by profiling (`samply`/macOS `sample`
against the Criterion stress suite) plus a targeted code audit of the core
engine and the new Arrow-native `normalise()` reconstruction path. Three
fixes, each verified via interleaved A/B (fresh subprocess per run, git
worktree at the prior commit, multiple rounds):
- **Date/datetime normalization output no longer re-parses a `chrono` format
  string per value.** `.convert_dates(True)`/`.auto_convert_types(True)`'s
  normalized-date/datetime output (`src/convert.rs`) used
  `DateTime::format("%Y-%m-%dT%H:%M:%SZ").to_string()`, which re-interprets
  the format string via `chrono`'s `StrftimeItems` engine on every call --
  the same avoidable cost this file's own date *parsing* already worked
  around with hand-rolled formatting. Replaced with a direct integer
  `format!` of the same fixed shape. **~10.6% faster** for date-heavy
  `convert_dates(True)` workloads.
- **`unflatten()` no longer allocates a fresh path buffer per flattened
  key.** `set_nested_value` (`src/unflatten.rs`) allocated a new `String`
  on every call (once per top-level flattened entry); the buffer is
  guaranteed empty again by the time each call returns (truncated back on
  unwind), so it's hoisted to `build_unflatten_tree` and reused across all
  entries in a document instead. **~3.8% faster** for wide flattened
  documents.
- **`normalise=True`'s Arrow-native reconstruction no longer parses a
  list-valued column's JSON array text twice.** `build_normalise_table`
  (`src/python.rs`) parsed each list cell's array once to classify its
  element kind and a second time to build the `List<T>` array. The
  classification pass now caches the parsed elements (`ListCell`) and the
  build pass reuses them. This is exactly `handle_key_collision(True)`'s
  own headline scenario (issue #35's real `List<T>` support). **~5-6%
  faster** for a collision-heavy scenario (300 columns, 8-element lists,
  600 rows) across all three non-PySpark targets; new
  `collision_medium` scenario added to `python/benchmarks/bench_normalise.py`
  to keep this covered going forward.

Second round, focused specifically on batch processing and DataFrame
conversion per user request. Profiled the core parallel batch dispatch
(`process_batch` in `src/builder.rs`) directly and found it already using
established best practice (rayon's persistent thread pool, single-pass
allocation, no redundant work) -- no viable fix found there. Separately
verified, via direct isolated timing, that batching Python-side JSON parse
calls (`orjson.loads` on a joined array vs. one call per row) makes no
measurable difference -- object-construction cost dominates over per-call
overhead, so that avenue (which looked "obviously correct" by reasoning)
was correctly abandoned before touching any code. `cProfile`-driven
investigation of a full `execute(df)` call found the real remaining cost:
- **DataFrame extraction no longer allocates an owned `String` per JSON
  object key when un-nesting or splicing embedded object/string columns.**
  `unnest_object_valued_columns` and `splice_row` (`src/python.rs`) both ran
  on every row of a flatten-mode DataFrame conversion, each parsing that
  row's JSON object into an `IndexMap<String, &RawValue>` -- one heap
  allocation per key just to check shape or look up a handful of target
  keys, never needing ownership. Both now try a zero-copy
  `IndexMap<&str, &RawValue>` parse first (correct whenever no key needs
  JSON-unescaping, the overwhelmingly common case for DataFrame column
  names), falling back to owned keys only when that fails -- never a
  behavior change, only an allocation avoided in the common case.
  `unnest_object_valued_columns` alone measured ~1.4us/call, roughly a
  quarter of a realistic `execute(df)` call's total time (confirmed via a
  direct isolated timing harness, not just profiler inference); the fix
  measured **~6-9% faster per call** (6/6 interleaved rounds in the same
  direction). `splice_row` (the embedded-JSON-string-column path, issues
  #30/#31) measured **~8.9% faster end to end** for a JSON-string-column
  DataFrame (4/4 interleaved rounds).

Third round, continuing the same batch-processing/DataFrame-conversion
focus. Found the same "owned key per row" pattern one level up, with a
second, compounding cost on top of it:
- **`normalise=True`'s row-union pass no longer clones every key of every
  row into `key_order`, even when that key was already seen.**
  `build_normalise_table` (`src/python.rs`) parsed each row into an
  `IndexMap<String, &RawValue>` (the same per-key allocation fixed
  elsewhere this round) *and* unconditionally called `key_order.insert(key
  .clone())` for every key of every row to build the cross-row column
  union -- for the realistic case of many rows sharing (almost) the same
  column set, every row after the first was cloning (and immediately
  discarding) keys already present in `key_order`. At issue #31's own
  reported scale (754 rows x 4,042 columns) that's on the order of 3
  million wasted clones. Now: a zero-copy borrowed-key parse (falling back
  to owned keys, with the original detailed parse error, only when a key
  needs unescaping), and `key_order` only allocates a key the first time
  it's actually new (a cheap `contains` check first). **~13.0% faster**
  for the issue #31-scale scenario (754 rows x 4,042 columns), consistent
  across 4/4 interleaved rounds -- the largest single win of these three
  rounds.
- **`splice_zerocopy_columns`** (the polars/pyarrow zero-copy embedded-
  JSON-string-column path, `src/python.rs`) updated to the same
  zero-copy-key parse for consistency and code reuse; measured as noise-
  level (~0.2%, not a reportable win) for the scenario tested, since this
  path already minimizes the JSON re-parsed per row by design (the
  JSON-string target column is dropped before the native writer runs, so
  what's re-parsed here is typically small regardless of the DataFrame's
  real width) -- included for consistency, not claimed as a performance
  win.

## [0.9.20] - 2026-07-31

### Changed (BREAKING)
- **DataFrame column expansion no longer prefixes with the source column's
  name.** A DataFrame column holding an embedded object -- whether a native
  dict/struct-typed column, or a JSON-*string* column auto-detected per
  [issue #30](https://github.com/amaye15/JSON-Tools-rs/issues/30) -- used to
  expand into columns prefixed by the source column's own name (e.g. a column
  named `payload` holding `{"user": {"name": "Alice"}}` produced
  `payload.user.name`). It now expands using only the embedded content's own
  keys (`user.name`) -- every top-level key in a DataFrame row is a column
  name by construction, so the column boundary itself carries no meaning
  worth preserving in the output, only what's genuinely nested *within* a
  column's content does. Applies to both plain `execute(df)` and
  `execute(df, normalise=True)`, uniformly across pandas/polars/pyarrow/
  PySpark. A JSON-string-encoded **array**-valued column is unaffected --
  still expands into indexed sub-columns prefixed by its own column name
  (`tags.0`, `tags.1`, ...), since a bare `0`/`1`/... column name wouldn't be
  meaningful. Two columns whose content shares a key name (or a column's
  content colliding with another top-level column) is resolved by whatever
  `.handle_key_collision()` is already set to -- collected into an array when
  `True`, last value wins when `False` (the default) -- the same policy this
  engine already applies to any other duplicate key, not a new one invented
  for this. See the [DataFrame & Series Support
  guide](https://amaye15.github.io/JSON-Tools-rs/guide/dataframe-support.html#auto-expanding-json-string-columns)
  for the full behavior and examples.
- **`normalise=True`/`target=...` reconstruction is now Arrow-native**
  ([issue #35](https://github.com/amaye15/JSON-Tools-rs/issues/35)). Replaces
  the old per-value Python-object-boxing reconstruction with one real Apache
  Arrow `RecordBatch` built directly in Rust, from which every target is
  derived. No new methods -- `normalise=True, target=...`'s existing surface
  is unchanged, only what happens internally. Three real, user-visible
  consequences:
  - **List-valued columns (from `handle_key_collision(True)`) now build as
    real, correctly-typed Arrow `List<T>` columns** instead of being
    stringified -- verified directly that pyarrow/polars/pandas all consume
    `List<T>` correctly; the old stringify-on-any-list behavior was
    unnecessarily conservative.
  - **Recognized date/datetime columns now build as real `Date32`/
    `Timestamp` Arrow columns**, gated on `.convert_dates(True)`/
    `.auto_convert_types(True)` -- this engine never independently
    pattern-matches an ordinary string into a date, only promotes what the
    core engine's own opt-in date recognition already normalized. A bare
    date becomes `Date32`; a datetime becomes a UTC `Timestamp` (any input
    timezone offset is converted, not just relabeled); mixing a date with a
    datetime in the same column promotes to `Timestamp`, mixing either with
    a non-date value stringifies the whole column, matching the existing
    mixed-kind fallback rule.
  - **`target="pandas"` output uses Arrow-backed dtypes**
    (`int64[pyarrow]`, `string[pyarrow]`, ...) instead of classic numpy
    dtypes -- genuinely zero-copy (measured: flat ~0.3ms regardless of row
    count from 100K to 5M rows, vs. ~700ms+ at 5M rows for a real copy to
    numpy dtypes), but a real, intentional breaking dtype change.
  - **`target="pandas"` and `target="pyspark"` now require `pyarrow`
    installed** (verified directly: there is no pyarrow-free route to get
    genuinely Arrow-built data into a pandas DataFrame, even via a polars
    intermediary). `target="polars"` is unaffected -- confirmed it stays
    fully usable without pyarrow.

### Performance
- **`normalise=True`/`target=...` reconstruction: honest, modest end-to-end
  win, ~1-4%** measured via interleaved A/B (100-4,000 columns, all four
  targets) -- core flattening and input serialization, unchanged by this
  round, dominate total wall time for typical column-heavy data, so
  eliminating reconstruction's `PyObject` boxing moves the needle less than
  the architecture change might suggest. Reported plainly rather than
  oversold; the real deliverable of this round is column-typing correctness
  (see Changed above), not raw speed.
- **`.convert_dates(True)`'s new date-detection cost, measured directly**
  (interleaved A/B, fresh subprocess per run): a real but small overhead,
  **~0.1-4.5%** end-to-end across pandas/polars/pyarrow at 500-4,000
  columns, including the adversarial worst case (date conversion enabled
  but no column actually contains a date, so every string cell pays a
  wasted `chrono` parse attempt for no typing benefit). Not charged at all
  when `.convert_dates()`/`.auto_convert_types()` is off, the default.

### Removed (BREAKING)
- **The JVM/Java/Scala binding has been removed entirely.** `jvm/` (the Maven
  project: Java source, tests, examples, `pom.xml`), `src/jvm.rs` (the JNI
  shim over the Rust core), the `jni` dependency and `jvm` Cargo feature, and
  `.github/workflows/jvm-ci.yml` (the native-lib build matrix, Maven test
  job, fat-jar packaging, and Maven Central release job) are all gone. The
  published `io.github.amaye15:json-tools-rs-spark` Maven Central artifact
  will no longer receive new versions -- `0.9.19` remains available there
  but will not be updated; existing consumers should pin to it if they still
  need the jar. Nothing else changes: the Rust core (`cargo add
  json-tools-rs`) and Python bindings (`pip install json-tools-rs`) are
  unaffected and continue to be published as before. Databricks/Spark users
  of the JVM bindings should switch to the Python bindings wrapped in a
  `pandas_udf`, per the existing [Setting Up on
  Databricks](https://amaye15.github.io/JSON-Tools-rs/guide/databricks-setup.html)
  guide, which already documents that path.

## [0.9.19] - 2026-07-30

### Changed
- **MSRV raised from 1.80 to 1.85.** Required to use current `pyo3-arrow`
  releases (see Performance below); the only MSRV-1.80-compatible
  `pyo3-arrow` version was 0.8.0, ~11 releases behind and not worth pinning
  to indefinitely for a fast-moving crate. Affects every consumer building
  from source (`cargo add`), not just the `python` feature.

### Performance
- **`execute()` on a Polars `DataFrame`/PyArrow `Table` with an embedded
  JSON-string column is ~41-48% faster end-to-end.** Detection/extraction of
  such columns now goes through `pyo3-arrow`'s zero-copy Arrow buffer access
  instead of round-tripping through the DataFrame's native JSON writer,
  which previously had to *escape* the embedded content into a quoted
  string (only for the existing splice step to immediately *unescape* it
  back out). The target column is dropped from the DataFrame before the
  native writer runs (confirmed empirically that escaping the embedded
  content, not fixed per-row overhead, dominates that writer's cost --
  dropping the column cut its own time ~44x in isolation) and the
  zero-copy-extracted values are spliced back into their *original* schema
  position afterward, so column ordering matches the existing text-based
  path's "not alphabetized" guarantee. Verified via interleaved A/B (git
  worktree, 6 rounds): isolated column extraction ~8.7x (polars)/~28x
  (pyarrow) faster; real end-to-end `execute()` ~41.3%/~47.5% faster for a
  754-row-scale table with a large embedded payload. Scoped to Polars/
  PyArrow specifically -- plain pandas isn't Arrow-backed by default, and
  PySpark already bridges through pandas, so both keep using the existing
  text-based detection/splice path unchanged. New `TestJsonStringColumn
  ZeroCopyArrow` test class covers column-order preservation, multiple
  target columns, multi-chunk Arrow arrays, nulls, and the fallback/warning
  path for a value that samples as JSON-like but isn't for one row.

## [0.9.18] - 2026-07-30

### Performance
- **`execute()` on a PyArrow `Table`/`RecordBatch` is ~2x faster.** Extraction
  now bridges through pandas's native `to_json()` writer (`dataframe_to_json_
  strings`, `src/python.rs`) instead of `to_pylist()` + per-item conversion,
  mirroring the same bridge already used for PySpark extraction and the
  documented ~2-3x win pandas/polars already get from their own native
  writers. **Correctness note, not just an optimization detail:** the first
  version of this fix used plain `to_pandas()`, which silently corrupts an
  integer column containing any null into `float64` (pandas' legacy dtype
  has no integer-null sentinel) -- caught via direct before/after comparison,
  not assumed. Fixed with `types_mapper=pd.ArrowDtype` (pandas >=1.5),
  which keeps the bridge Arrow-native and measured no slower; falls back to
  plain `to_pandas()` then to the original `to_pylist()` path on older
  pandas or if pandas isn't installed. Verified via interleaved A/B (git
  worktree, 6 rounds): ~53% faster for a realistic 5,000-row/20-column
  table. New regression test pins the integer-nullability fix.
- `splice_row`'s per-key JSON escaping (`src/python.rs`) now reuses the
  existing zero-allocation `write_json_escaped_key` (`src/flatten.rs`,
  already proven in two other call sites) instead of allocating a fresh
  `String` per key via `serde_json::to_string`. A real, always-correct
  cleanup with no downside -- but measured honestly: at realistic scale
  (754 rows x 4,042 keys) the end-to-end difference was within noise
  (~0.2%), since the per-key allocation this removes is small relative to
  the row's own parsing/copying cost at that scale. Kept for code quality
  and the cases where it does matter more (many small rows), not claimed as
  a measurable win.

## [0.9.17] - 2026-07-30

### Performance
- **Core `flatten()` collision-handling path (key replacement/lowercase/
  collision detection) is ~5-8% faster** for documents that trigger it.
  `resolve_and_write` (`src/flatten.rs`) built a key→indices map, then
  re-looked-up each key a second time in the output loop right after having
  just inserted it -- restructured to carry each key's indices alongside its
  first-seen position directly, eliminating the redundant second hashmap
  lookup. Verified via interleaved A/B (git worktree at the prior commit, 5
  rounds) against `bench_quick`'s `all_transforms`/`regex_replacements`
  scenarios.
- The equivalent double lookup in `unflatten.rs`'s `handle_entry_collisions`
  was fixed the same way. A first attempt also removed the separate
  `has_duplicate_keys` pre-check entirely (reasoning that `handle_entry_
  collisions`'s own no-collisions fast path made it redundant) -- measurement
  caught that this was actually a ~2-4% *regression* for the dominant
  real-world case (no explicit collision handling, no actual duplicate
  keys): the pre-check's plain `FxHashMap<&str, ()>` is cheaper to populate
  than `handle_entry_collisions`'s richer `FxHashMap<&str, SmallVec<[usize;
  1]>>`, so paying for the richer map unconditionally lost more than it
  saved. Reverted that part and kept only the double-lookup fix, which has
  no such trade-off.
- `execute(pandas_df)`/`execute(polars_df)`/`execute(pyarrow_df)` and their
  `Series` equivalents (`src/python.rs`) now use the same `PyOnceLock`
  module-import caching added for the `normalise`/PySpark path in 0.9.16 --
  these older, non-`normalise` reconstruction functions had the identical
  uncached-`py.import()`-per-call gap.
- `mimalloc`'s doc comment updated from a generic "~5-10%" estimate to
  measured numbers (~14-28% faster on allocation-heavy paths, macOS
  aarch64) after actually benchmarking the previously-untested opt-in
  feature; a new CI step runs the test suite with it enabled so the feature
  doesn't silently bit-rot. Still deliberately not used for published Python
  wheels or JVM jars -- an earlier attempt to bundle it there (alongside a
  since-reverted PGO effort) hit real cross-compilation breakage on
  aarch64/ppc64le manylinux and musllinux; this is only the pure-Rust
  opt-in Cargo feature for `cargo add` consumers who enable it themselves.

## [0.9.16] - 2026-07-30

### Performance
- **`execute(..., normalise=True, target=...)` and PySpark `execute(spark_df)` are
  13-18% faster for large/wide results**, e.g. a 754-row, 4,042-column case (matching
  issue #31's own reported scale). Found via a dedicated codebase audit after issues
  #30-33 added a large amount of DataFrame-reconstruction machinery
  (`union_and_columnarize`/`reconstruct_*_normalise` in `src/python.rs`) without a
  performance pass, which also had zero benchmark coverage (the existing Criterion
  suite only exercises the pure-Rust core). Two fixes, both benefiting all four
  DataFrame targets since they share this code path:
  - `union_and_columnarize` rewritten from an O(rows &times; columns) `PyDict.get_item`
    hash lookup (re-scanning every row for every union'd key -- ~3 million individual
    PyO3 dict lookups for the 754x4,042 case) to a single forward pass that scatters
    each row's own `dict.items()` into pre-sized column slots, using a Rust-side
    `IndexSet` index lookup instead of a Python-level hash lookup per cell.
  - `pandas`/`polars`/`pyarrow`/`pyspark.sql`/`pyspark.sql.types` module imports
    across this reconstruction code (previously re-imported on every call, including
    up to 4 redundant imports of the *same* module within one `execute(spark_df)`
    call) are now cached via `PyOnceLock`, mirroring the existing `JSON_CALLABLES`
    caching idiom already used for `orjson`/stdlib `json`.
  - Verified via an interleaved A/B (git worktree at the prior commit, alternating
    fresh-process runs, medians over 6 rounds) against the real Python API, not an
    isolated probe -- consistent with this project's own established measurement
    practice. New `python/benchmarks/bench_normalise.py` harness added for future
    before/after comparisons on this code path.

## [0.9.15] - 2026-07-29

### Fixed
- **`execute(spark_df)` no longer crashes when a `.key_replacement()` /
  `.handle_key_collision(True)` list column holds genuinely mixed element
  types.** ([#33](https://github.com/amaye15/JSON-Tools-rs/issues/33)) The
  list-flavored twin of the 0.9.14 fix: a `handle_key_collision` list is
  built from each colliding source key's own independently
  `auto_convert_types`-converted value, so a single row's collision could
  already mix kinds (e.g. `[100, "abc"]`), and `infer_spark_type` hardcoded
  `ArrayType(StringType)` for every list-valued column regardless of the
  actual element types -- Arrow rejected the mismatch with `PySparkTypeError:
  ... to Arrow Array (list<element: string>)`, confirmed via direct
  reproduction against the published 0.9.14 wheel. `union_and_columnarize`
  now checks every element of every list in a column for the same kind of
  consistency the 0.9.14 fix applied to scalar columns, falling back to
  string elements when they're genuinely mixed; `infer_spark_type` derives
  the array's element type from every element across the column instead of
  hardcoding `StringType`, so a uniformly-typed list column (e.g. all `int`)
  now correctly gets `ArrayType(LongType)` instead of unnecessarily
  stringifying. Also hardens `normalise(target=...)` for all four DataFrame
  backends, since the fix lives in the shared column-unioning step.

## [0.9.14] - 2026-07-29

### Fixed
- **`execute(spark_df)` with `auto_convert_types(True)` no longer crashes on
  columns with genuinely mixed types.**
  ([#32](https://github.com/amaye15/JSON-Tools-rs/issues/32))
  `auto_convert_types` converts each value independently based on its own
  content, so the same flattened key can hold a clean numeric string in one
  row (`"123"` -> `int 123`) and ordinary text in another (`"Smith"` -> stays
  `str`). The PySpark reconstruction path (shared with
  `normalise(target="pyspark")`, added for
  [#31](https://github.com/amaye15/JSON-Tools-rs/issues/31)) inferred a
  column's Spark schema type from only its *first* non-null value, so a
  later, differently-typed value in the same column broke Spark's Arrow
  bridge with `PySparkTypeError: Exception thrown when converting
  pandas.Series (object) ... to Arrow Array` -- confirmed via direct
  reproduction of the reported error, under the same Arrow-fallback-disabled
  configuration Databricks Serverless uses. Columns with genuinely
  incompatible mixed types (string mixed with number and/or boolean) now
  fall back to a uniform string column instead of crashing; columns mixing
  only `int`/`float` still promote correctly to a numeric `double` column
  (matching pandas's own automatic promotion), and uniform bool/numeric/
  string columns are unaffected. This also hardens
  `normalise(target=...)` for all four DataFrame backends, since the fix
  lives in the column-unioning step they all share.
- The issue's other reported symptom -- `execute(spark_df)` returning
  `list[dict]` instead of a real PySpark DataFrame -- was independently
  reproduced against the published 0.9.13 wheel and found to already be
  fixed by [#31](https://github.com/amaye15/JSON-Tools-rs/issues/31); no
  further change was needed for that part.

## [0.9.13] - 2026-07-29

### Fixed
- **`execute(df)` on a PySpark DataFrame now returns a real, distributed
  `pyspark.sql.DataFrame`, not a plain `list[dict]`.**
  ([#31](https://github.com/amaye15/JSON-Tools-rs/issues/31)) Reuses the same
  schema-driven reconstruction `normalise(target="pyspark")` already used
  (active `SparkSession` auto-discovered via `SparkSession.getActiveSession()`,
  raising `JsonToolsError` if none is found). **Behavior change:** any code
  that relied on `.execute(spark_df)` returning a plain list needs updating --
  it now returns a DataFrame. Polars input was independently confirmed to
  already work correctly (`execute(polars_df)` -> `polars.DataFrame`); the
  issue's claim it was untested/unsupported did not hold up under direct
  reproduction.

### Performance
- **JSON-string-column auto-expansion (added in 0.9.12 for issue #30) is now
  meaningfully faster for large embedded payloads**, root-caused via direct
  profiling rather than guessing: the per-row splice step was building a full
  `serde_json::Value` tree for the embedded content and reserializing the
  whole row, both `O(field count)` in that content -- expensive and, for a
  payload expanding into thousands of columns, done twice over (the core
  flatten engine parses the same content again moments later regardless).
  Rewritten around `serde_json::value::RawValue` (validated via a dedicated
  design-review pass against indexmap's and serde_json's actual source, not
  assumed) to avoid ever building that tree, while keeping the exact same
  graceful per-row fallback behavior (a malformed cell keeps its original
  string value and triggers a warning, rather than failing the whole call --
  confirmed necessary regardless of performance goals, since the core batch
  engine fails the entire batch on any single bad item). Benchmarked
  ~40-43% faster on synthetic 200-row/500-key and 100-row/4,000-key cases
  (the latter matching the issue's own reported scale); also strictly safer
  for non-target fields, which are now copied verbatim instead of being
  reformatted through a parse/reserialize round-trip.

## [0.9.12] - 2026-07-29

### Fixed
- **`execute(df)` in `.flatten()` mode now auto-expands DataFrame columns
  holding JSON *strings*, not just columns already typed as dicts/structs.**
  ([#30](https://github.com/amaye15/JSON-Tools-rs/issues/30)) Previously, a
  column already holding a Python dict/struct serialized as genuine nested JSON
  within each row and flattened correctly; a column holding a JSON *string*
  serialized as an escaped string value instead, which `.flatten()` correctly
  never re-parses as JSON (not its contract) -- so it stayed an opaque,
  unexpanded string. Detection (bounded to the first 20 rows) requires a
  column's sampled string values to *all* parse as a JSON object or array
  (never any bare scalar); a detected column's row that fails to re-parse later
  keeps its original string value and triggers an aggregated `warnings.warn(...)`
  naming the column and failure count, rather than silently producing a
  structurally inconsistent row. Implemented as a backend-agnostic Rust-side
  splice over the row-JSON text already produced for all four DataFrame
  backends (pandas/polars/pyarrow/PySpark), avoiding the Python-level
  `orjson.loads()`-per-row + dict-object-graph-construction overhead the
  issue's own workaround was paying. Required enabling `serde_json`'s
  `preserve_order` feature so spliced rows keep their original key order
  instead of being silently alphabetized.
  **Behavior change:** any DataFrame with a column that happens to hold
  JSON-object/array-shaped strings will now produce more (differently-shaped)
  output columns from `execute(df)` in `.flatten()` mode than before -- scoped
  specifically to flatten mode; `.unflatten()`/`.normal()` DataFrame processing
  is unaffected.

## [0.9.11] - 2026-07-28

### Fixed
- **`execute(..., normalise=True, target="pyspark")` could silently corrupt an
  all-`None` column.** Schema was previously left for Spark to infer from the
  intermediate pandas DataFrame; confirmed empirically that inference is
  unreliable specifically on the non-Arrow fallback path Spark silently takes
  when pyarrow isn't installed (a real, reachable configuration -- pyspark
  does not depend on pyarrow). A missing value could serialize as the literal
  string `"<NA>"` instead of a real null (from pandas's nullable `"string"`
  dtype's `pd.NA` sentinel), and separately, an all-`None` column could infer
  as `StructType([])` (empty struct) instead of a null string column. Fixed
  by computing an explicit `pyspark.sql.types.StructType` schema from the
  data and using plain Python `None` instead of the nullable extension type,
  verified correct on both the Arrow and non-Arrow paths, with and without
  pyarrow installed. This also allowed removing the all-`None`-column special
  casing previously present in the pandas/polars/pyarrow reconstructors:
  none of those three actually need it (each has its own harmless default
  for an all-`None` column), so it's now scoped only to where it's actually
  needed.

## [0.9.10] - 2026-07-28

### Fixed
- **Critical: `auto_convert_types` panicked on multi-byte UTF-8 content in
  specific positions** (e.g. `"5€ García"`, `"0xÁ1F"`, a timezone offset like
  `"+1Á2"`) -- two fixed-byte-offset string slices (the 3-letter
  currency-code check in `strip_currency_indicators`, and the `+HHMM` ->
  `+HH:MM` offset formatter in the compact-datetime parser) assumed the
  offset was always a UTF-8 character boundary without checking. Root-caused
  by direct reproduction (not just static analysis) with `RUST_BACKTRACE=1`,
  then swept the rest of `convert.rs`'s fixed-offset slices by construction
  to confirm no others were reachable. Fixed with a cheap `is_char_boundary`
  guard at each site -- zero behavior change for the (always-ASCII) valid
  currency-code/offset formats, and a string that merely resembles one but
  contains non-ASCII content in the checked position is now correctly left
  as a string instead of crashing. ([#29](https://github.com/amaye15/JSON-Tools-rs/issues/29), point 1)

### Added
- **Python: `JSONTools` is now picklable** (`pickle.dumps`/`pickle.loads`),
  which also means it can be captured in a cloudpickle-serialized closure --
  e.g. inside a PySpark UDF or `mapInPandas` function -- without the
  workarounds the linked issue describes needing before this existed.
  Implemented via `__reduce__` plus a new `to_config_json()`/
  `from_config_json()` method pair (also directly useful on their own for
  reconstructing a fresh, independent instance from a captured config
  string in a distributed worker process). Verified across a real process
  boundary (`multiprocessing`, spawn context), not just in-process.
  ([#29](https://github.com/amaye15/JSON-Tools-rs/issues/29), point 2)
- **Python: `execute(input, normalise=True, target=None)`** -- always returns a
  wide DataFrame (one column per flattened key) regardless of input shape (a
  bare `str`/`dict` becomes a 1-row DataFrame), working natively across pandas,
  polars, pyarrow, and now genuinely **PySpark**: `target="pyspark"` closes the
  long-standing gap where DataFrame reconstruction fell back to a plain list of
  dicts (see the "Supported Libraries" caveat above and in
  `docs/src/guide/dataframe-support.md`), by reusing the pandas reconstruction
  path and handing it to Spark's own Arrow-optimized
  `SparkSession.createDataFrame(pandas.DataFrame, schema)` bridge. The schema
  is passed explicitly rather than left for Spark to infer: confirmed
  empirically that inference is unreliable on the non-Arrow fallback path
  Spark silently takes when pyarrow isn't installed (a real, reachable
  configuration), including a case where a missing value serialized as the
  literal string `"<NA>"` instead of a real null. Target resolution: an
  explicit `target` wins; otherwise a live DataFrame/Series input keeps its
  own backend; otherwise pandas → polars → pyarrow is tried in order (pyspark
  is never auto-selected for bare JSON input). Column union/null-fill order is
  handled uniformly across all four targets, and a key-collision column
  that's list-valued in only some rows (via `handle_key_collision(True)`) is
  now made uniformly list-valued rather than crashing pyarrow/polars on the
  mixed types.

## [0.9.8] - 2026-07-26

### Changed
- **Python: [orjson](https://github.com/ijl/orjson) is now a required
  dependency**, used automatically for the dict/DataFrame-row JSON
  (de)serialization step (`pip install json-tools-rs` pulls it in -- no separate
  opt-in needed). Has published wheels for every platform/arch this project
  ships wheels for and for Python 3.9+ (pip resolves the right release per
  interpreter automatically), so this doesn't affect installability anywhere.
  A per-call fallback to the standard library still covers the inputs orjson
  can't handle (integers beyond 64-bit range, which orjson silently parses as
  lossy floats -- documents that could contain one are routed to the stdlib
  path automatically to preserve this library's exact-integer guarantee).

### Performance Improvements
- **Python binding marshaling**: `execute()`/`execute_to_output()` skip the
  DataFrame/Series duck-typing detection entirely for exactly-typed
  `str`/`dict`/`list` inputs (subclasses still take the full detection path), and
  the `json.dumps`/`json.loads` callables (now always orjson, per the `Changed`
  entry above) are resolved once via a `PyOnceLock` instead of re-importing per
  call. dict-input calls ~37% faster, str-input calls ~39% faster.
- **JVM binding marshaling**: the native `execute`/`executeBatch` methods now pass
  UTF-8 `byte[]` instead of `String` across the JNI boundary -- Java's
  `String.getBytes(UTF_8)`/`new String(bytes, UTF_8)` are JIT-intrinsified, turning
  the crossing into two plain array copies instead of JNI's UTF-16 <-> modified-UTF-8
  conversions. ~22-38% faster per call, ~33% faster for batches. The public
  `JsonToolsHandle` API is unchanged; only the internal native method signatures moved.
- **Unflatten tree-building**: nested objects/arrays created during tree assembly
  now start at a corpus-tuned capacity hint (was a flat, undersized guess of 4;
  real-world nested objects average 4.1 fields, with a p75 of 5) and use a single
  `IndexMap::entry()` lookup instead of a `contains_key` + `insert` + `get_mut`
  sequence. ~5-6% faster unflatten, ~4-5% faster roundtrip.
- **Core scanner**: fixed a double-scan in the tape scanner's scalar handling --
  the scalar/whitespace bytes following a colon, comma, or array-start were fully
  walked once to compute the scalar's length, then re-walked a second time by the
  main loop one byte at a time. The scan position now advances past what was
  already consumed. ~13-16% faster flatten across payload sizes.

## [0.9.7] - 2026-07-20

### Added
- **`.exclude_key(pattern)`** (Rust/Python/JVM): drop any key -- and its entire
  value/subtree -- whose name contains `pattern` (literal substring match by
  default, `r'...'` for regex, matching `.key_replacement()`'s convention).
  Additive -- call once per keyword to exclude multiple. Works identically in
  `.flatten()`, `.unflatten()`, and `.normal()` mode: checked against the full
  dot-path in flatten/unflatten, and per key at each nesting level in normal mode.
  Matching a container key drops its entire subtree in O(1), without walking it --
  the tape format's precomputed container end-index is reused to skip straight past
  an excluded subtree instead of visiting and discarding every nested leaf. Array
  elements are never matched, since they have no key name to check.
- **`.exclude_value(pattern)`** (Rust/Python/JVM): drop a key-value pair whose
  value contains `pattern`. Same literal/`r'...'` convention as `.exclude_key()`.
  Additive. Unlike `.exclude_key()`, this only ever applies to scalar leaf values
  (strings/numbers/booleans/null) -- containers have no single value to check, so
  a per-leaf check runs after any configured `.value_replacement()`/
  `.auto_convert_types()` have already transformed the value, matching
  `.remove_nulls()`'s existing "runs last" ordering guarantee (a value that only
  matches after being replaced or converted is still caught). A no-op at the
  document root, since there's no parent key to drop the value from. In
  `.unflatten()` mode specifically, string values are matched against their
  JSON-serialized form (quotes included), not the unescaped logical text --
  literal patterns are unaffected, but a regex with anchors needs to account for
  the quotes (e.g. `r'^"admin"$'`, not `r'^admin$'`).

### Fixed
- **`.remove_nulls()` now reliably runs last, consistently across
  `.flatten()`/`.unflatten()`/`.normal()` mode and both single-document and batch
  input.** Previously, `.value_replacement()` and `.auto_convert_types()` composed
  in three different orders across the three engines: flatten mode returned
  immediately on a replacement match without ever trying conversion on the
  replaced value; unflatten mode tried conversion *before* replacement (the
  opposite order); only normal mode already composed them correctly (replacement,
  then conversion applied to the replacement's result). This meant the same
  document and config could produce different results depending on which mode
  processed it, and a value that only became null after a replacement ran could
  slip past `.remove_nulls()` in flatten/unflatten mode. All three engines now
  compose replacement-then-conversion identically, including each engine's
  root-primitive path (a whole document that's a bare scalar, not an object/array)
  -- flatten's root-primitive handler previously skipped type conversion entirely
  for this case.

## [0.9.6] - 2026-07-19

### Added
- **Fine-grained, per-category control over automatic type conversion**, across
  Rust, Python, and JVM: `.convert_dates()`, `.convert_nulls()`,
  `.convert_booleans()`, `.convert_numbers()` let each category be enabled/disabled
  independently instead of the previous all-or-nothing `.auto_convert_types(bool)`,
  which is unchanged and continues to mean "all four categories, default behavior"
  -- it now only flips each category's own `enabled` bit, preserving any
  per-category customization already configured (regardless of call order).
  Each category also accepts real customization via a `_config` method (Rust:
  `.convert_dates_config(DateConversionConfig::new()...)`; Python: kwargs, e.g.
  `.convert_dates(True, assume_utc_for_naive=False)`; JVM: dedicated fluent methods,
  e.g. `.dateAssumeUtcForNaive(false)`):
  - **Dates**: `normalize_to_utc` (disable UTC normalization, leaving a recognized
    date unchanged but still protected from number-parsing fallthrough) and
    `assume_utc_for_naive` (disable the `Z`-append-on-naive-datetime behavior).
  - **Nulls** / **Booleans**: recognize additional tokens beyond the built-in lists
    (additive only -- the built-in list stays active regardless).
  - **Numbers**: individually disable currency, percent/permille, text basis
    points, K/M/B/T suffixes, fractions, or hex/binary/octal parsing. Plain
    integers/decimals, scientific notation, and thousands-separator cleanup remain
    always-on core behavior whenever the category is enabled.
  New public types: `TypeConversionConfig`, `DateConversionConfig`,
  `NullConversionConfig`, `BooleanConversionConfig`, `NumberConversionConfig`
  (mirroring `FilteringConfig`'s existing shape: plain `pub` fields, `::new()`,
  fluent `#[must_use]` setters).
- Runnable examples and tests for the new API added across all three languages
  (`examples/feature_by_feature.rs`, `python/examples/feature_by_feature.py`,
  `jvm/examples/.../FeatureByFeature.java`; `src/tests.rs`,
  `python/tests/tests.py`, `jvm/src/test/java/.../JsonToolsTest.java`); the
  `iso_07_auto_type_conversion_only` Criterion benchmark gained `all_default_via_new_api`
  (confirmed within ~1% of `auto_convert_types`, i.e. no hot-path regression for
  existing usage -- see below), `partial_enable`, and `custom_config` cases. A new
  `iso_07b_type_conversion_per_category` benchmark group isolates each category's
  own per-string cost and specifically quantifies the extra-tokens fallback path's
  overhead (~50% slower than the no-customization case for a mixed document, since
  it adds a linear-scan check for every string that didn't match via the main
  byte-dispatch -- a real, now-documented cost of using extra tokens, not a
  regression in existing behavior).
- Edge case coverage for the new per-category/customization surface, cross-checked
  across all three languages: every number sub-format disabled at once (core
  parsing still works), a negative radix-looking string with radix disabled, an
  extra token appearing in both true/false lists (locks in true-wins precedence),
  an extra token duplicating a built-in one (harmless), a disabled category with
  leftover/inert customization (no effect), category-priority-order interactions
  between dates and null-extra-tokens (dates win when enabled; the null extra
  token wins when dates are off), fine-grained config across a full batch
  (exercises the parallel dispatch path), and extra tokens matching against the
  *trimmed* value rather than the raw string (consistent with every other
  category -- e.g. `"si "` still matches an extra token `"si"`, discovered while
  porting the Rust test to Python and locked in on both sides). Python additionally
  covers its `extra_tokens` kwarg's bulk-replace semantics (a second call's list
  replaces, not merges with, an earlier one -- unlike Rust/Java's additive
  `add_extra_token()`/`.nullExtraToken()`), unicode extra tokens round-tripping
  correctly across the Python↔Rust FFI boundary, and the distinction between
  `extra_tokens=[]` (explicit empty list -- clears previously-set customization)
  vs. omitting the kwarg entirely (`None` -- preserves it).
- Two further rounds of edge cases, again cross-checked across all three
  languages: a numeric-looking extra boolean/null token (e.g. `"1"`) is only
  reachable through the extra-tokens fallback pass, so it loses to plain number
  parsing when `convert_numbers` is also enabled (the digit-dispatch arm claims
  the string first) but wins when numbers is disabled -- a genuinely non-obvious
  interaction, now locked in explicitly rather than left as an accident of
  implementation order. Also: extra-token case-sensitivity, a malformed
  superficially-date-shaped string (`"2024-13-45T99:99:99"`) failing safely
  without corruption, an invalid leap-year date (`"2023-02-29"`, 2023 isn't a
  leap year) correctly falling through rather than being misrecognized, a valid
  leap-year date recognized but left unchanged (already canonical), the compact
  datetime format (`"20240115T103000"`, no separators) exercised through the
  configured code path for the first time, keys never being type-converted
  (only values are candidates), and each number sub-format tested individually
  enabled (complementing the existing "all-but-one-disabled" coverage). One
  finding worth calling out specifically: **replacement-then-conversion
  chaining is a pre-existing (not introduced by this feature) inconsistency
  across processing modes** -- `.flatten()`/`.unflatten()` return as soon as a
  `value_replacement` matches, without ever trying type conversion on the
  replaced value, while `.normal()` chains the two. Confirmed identical through
  the new fine-grained API in all three languages and locked in with tests, so a
  future refactor of any of the three walkers doesn't silently change it.

### Changed (BREAKING)
- **`ProcessingConfig` (and `FilteringConfig`/`CollisionConfig`/`ReplacementConfig`)
  are now `#[non_exhaustive]`**, matching `JsonToolsError`'s existing precedent.
  Breaks external code constructing these via a bare struct literal instead of
  `::new()` + the fluent builder methods; does not affect the `JSONTools` builder
  (the recommended interface) or any code already using `::new()`. Added now,
  deliberately, so that future config additions (to any of these four structs) don't
  need this same breaking-change deliberation again.
- **`ProcessingConfig.auto_convert_types: bool` removed**, replaced by
  `ProcessingConfig.type_conversion: TypeConversionConfig`. Breaks external code
  that directly *read* `config.auto_convert_types` (constructing via struct literal
  was already blocked by `#[non_exhaustive]` above); avoids a dual-source-of-truth
  bug class between a flat bool and the new per-category structure. The `JSONTools`
  builder's own `.auto_convert_types(bool)` method is unaffected.

### Performance
- The existing, heavily-profiled `try_convert_string_to_json_bytes` hot-path
  function is **not modified at all** by this change -- it remains the code path
  for the common case (all four categories enabled with untouched default
  sub-settings, which is what `.auto_convert_types(true)` alone produces),
  selected via a `TypeConversionMode` (`Disabled`/`AllDefault`/`Custom`) computed
  once per `execute()` call and cached on `ProcessingConfig`, not recomputed per
  string. A new, separate `_configured` code path (real, not thin-wrapper,
  duplication of the per-category-gated logic) only runs for genuinely new usage
  (partial-enable or customized knobs). Verified via the extended
  `iso_07_auto_type_conversion_only` benchmark: `all_default_via_new_api` lands
  within ~1% of `auto_convert_types` across repeated runs, well inside this
  project's own established ~3-5% binary-layout noise floor.

## [0.9.5] - 2026-07-18

### Fixed
- **Documentation-wide accuracy sweep**, covering every root-level doc, the full
  mdBook site, and the JVM Java source's own doc comments -- four parallel audit
  passes (root docs; Getting Started; User Guide; Reference/Resources), each
  verifying every claim against actual source code or live runtime behavior
  (`cargo run`/`cargo bench`, `python3` against the built extension, `mvn compile`)
  rather than trusting existing prose, since several rounds of incremental doc edits
  had let real drift accumulate. Highlights:
  - **Fabricated/stale internals.** `architecture.md`, `performance.md`, `README.md`,
    and `BENCHMARKS.md` described caching/hashing machinery that doesn't exist in this
    codebase at all -- a `phf` perfect-hash key cache, a `KeyDeduplicator`, `rustc-hash`
    (actually a hand-rolled `FxHasher` in `fxhash.rs`), `Arc<str>` key deduplication
    (actual key storage is `CompactString` plus a `bumpalo` arena on flatten's slow
    path), and flatten/unflatten function names (`FastStringBuilder`,
    `flatten_value_with_threshold`, `quick_leaf_estimate`) that no longer exist post-tape-scanner-rewrite.
    Rewritten to describe the real tape-based `scan_and_fixup()` engine, the actual
    4-tier regex cache (compile-time table / sticky / thread-local / global LRU), and
    `builder.rs`'s real ~19 public methods (previously claimed "35+").
  - **Stale benchmark numbers**, some off by 3-14x (e.g. deep-nesting-100 claimed as
    8.3µs, measured at ~2.1µs on current `master`). Replaced with numbers from a live
    `cargo bench --bench stress_benchmarks` run against current code, with the exact
    command noted so they're reproducible rather than another point-in-time snapshot
    that silently rots.
  - **Wrong error-handling semantics.** `.separator("")` was documented as panicking in
    Rust; it actually returns `Err(ConfigurationError)` (`E005`) at `.execute()` time.
    Python's error messages were documented as always starting with `[E00x]`; the
    bindings actually prepend their own context (e.g. `"Failed to process JSON
    string: "`) first, so the code is a substring, not a prefix. `E003`/`E004`/`E007`'s
    documented trigger conditions were largely invented (e.g. E007 was said to cover
    passing a raw `int`/DataFrame-to-`execute_to_output()`, which actually raise a
    plain Python `ValueError` that `except JsonToolsError` does *not* catch -- real
    `E007` triggers are empty input, >4GiB input, and an unflatten array index beyond
    `max_array_index`). `E006`'s message was documented as including the underlying
    per-item error; it only ever says "Failed to process item at index N".
  - **Broken guide examples.** `normal-mode.md`'s example relied on
    `key_replacement()` running after `lowercase_keys()`, matching `.flatten()`'s
    order -- `.normal()` mode actually applies them in the opposite order, so the
    documented output was never what the code produces (verified both ways with real
    runs). `collision-handling.md`'s "collision with filtering" example was missing
    the `key_replacement()` call needed to make the keys collide at all, and showed an
    impossible output key alongside it. `dataframe-support.md`'s Polars example used a
    JSON-*string* column, which flattening is a documented no-op on (there's nothing
    nested inside a string); replaced with a struct-column example that actually
    flattens. Also documented that PySpark DataFrame/Series inputs fall back to a
    plain `list`/`list[dict]` on output rather than a reconstructed PySpark object,
    correcting an implicit "perfect type preservation applies everywhere" claim.
  - **`docs/src/resources/changelog.md`** (the mdBook mirror of this file) had several
    bullet points, especially in the v0.4.0-v0.9.0 range, that don't correspond to
    anything in this file's real history for those versions -- replaced with entries
    traceable to the actual historical changes.
  - **Stale distribution status.** Maven Central publishing has been live since v0.9.2
    (`io.github.amaye15:json-tools-rs-spark`, confirmed live via a real `mvn
    dependency:get` against the real coordinate), but `README.md`, `jvm/README.md`,
    `docs/src/getting-started/installation.md`, and `quickstart-jvm.md` still said "not
    yet published" / "once published", one with a fictional `0.10.0` placeholder
    version. Corrected all four to the real, currently-published dependency
    coordinates. Python's `pip install json-tools-rs` was similarly still flagged
    "once published" in `databricks-setup.md` despite being live on PyPI throughout.
  - **A genuine internal contradiction in the JVM Java source itself** (not just the
    docs): `FlattenUDF.java`/`BatchTransform.java`'s javadoc and `jvm/pom.xml`'s
    `<description>` described using the JVM UDFs from inside a Databricks Lakeflow
    Declarative Pipeline via `spark._jvm`/`df._jdf` -- directly contradicting the
    already-correct, already-verified restriction stated in `jvm/README.md` and
    `CHANGELOG.md` (Databricks does not permit JVM libraries on pipeline compute at
    all; confirmed against Databricks' own docs in an earlier round). The Java doc
    comments had never been updated when that correction was made. Fixed to match:
    Jobs/notebooks on classic compute only, pointing at the `pandas_udf` path for
    genuine in-pipeline use.
  - Python's documented minimum version (3.8+) didn't match `pyproject.toml`'s actual
    `requires-python = ">=3.9"` -- pip would refuse 3.8 as documented. Fixed.
  - A Markdown rendering bug in `replacements.md`: a backslash inside a code span
    (`` `r'\d+'` ``) was silently dropped by the renderer, confirmed by inspecting the
    built `docs/book/` HTML output before and after the fix.
  - Added `docs/src/reference/jvm-api.md` (method tables, error handling, a complete
    example), closing a real gap where Rust and Python each had a Reference API page
    and the JVM bindings -- an equally complete, equally shipped public API -- had
    none.

### Added
- **Runnable examples covering every `JSONTools` builder feature, individually and in
  curated combination, mirrored across all three language bindings.** Two new files
  per language: `feature_by_feature` (one isolated example per builder method --
  mode, separator, lowercase_keys, key/value replacement in both literal and regex
  form, all four empty-value filters, collision handling, type conversion, the
  parallel-tuning knobs, max_array_index, and batch execution) and
  `feature_combinations` (curated multi-feature pipelines -- not an exhaustive
  combinatorial sweep, since the builder has ~10 independent toggles and a literal
  power-set would be 1000+ cases, but realistic groupings commonly used together,
  plus one "kitchen sink" pipeline). Rust: `examples/feature_by_feature.rs` /
  `examples/feature_combinations.rs`. Python: `python/examples/feature_by_feature.py`
  / `python/examples/feature_combinations.py`. Java:
  `jvm/examples/.../FeatureByFeature.java` / `FeatureCombinations.java` (new, kept
  out of the packaged jar via a dedicated `examples` Maven profile using
  `build-helper-maven-plugin` + `exec-maven-plugin`). All three language versions
  use matching inputs and were verified to produce byte-identical output.

### Changed
- **Regex pattern lookup for key/value replacements no longer re-hashes and re-walks
  the cache on every single key/value check.** Found via sampling profiler on a
  batch/parallel workload with a regex `key_replacement` configured:
  `get_cached_regex` was the single largest hotspot by a wide margin (roughly a
  fifth of all samples), because the same pattern -- shared by the same
  `ProcessingConfig` across an entire `execute()` call, potentially a large batch
  -- was being re-parsed and re-fetched from the (thread-local `FxHashMap` +
  global `RwLock`) cache from scratch on every key. Added a tiny thread-local
  "sticky" cache (`STICKY_REGEX_CACHE`, capacity 4) checked before the existing
  tiers: a linear scan over a handful of recently-used `(pattern, regex)` pairs,
  comparing full pattern strings directly instead of hashing them, which almost
  always hits after the first call. Deliberately *not* a bigger architectural
  change (pre-resolving patterns once into `ProcessingConfig`/`ReplacementConfig`
  themselves, which an isolated benchmark showed would be faster still): those
  types are re-exported as public API with all-`pub` fields and their own fluent
  builder, so changing `key_replacements`/`value_replacements`' field type would
  be a breaking change for anyone constructing them directly rather than through
  `JSONTools`. The sticky cache gets the bulk of the win with zero public API
  impact. Verified against the real `iso_04_key_replacement_only`/
  `iso_05_value_replacement_only` Criterion benchmarks: regex scenarios 9-22%
  faster, consistent across repeated runs. (Validation note: comparing the two
  builds also surfaced a ~3-5% binary-layout noise floor from this project's
  `lto = "fat"` + `codegen-units = 1` release profile -- confirmed by finding the
  *same* magnitude of drift in `iso_01_baseline_flatten`, a benchmark that
  touches none of this code at all -- so deltas below that floor elsewhere in
  this round shouldn't be over-interpreted, but all the reported numbers here
  are well clear of it.) Re-profiling the same batch workload after the fix
  confirmed `get_cached_regex`'s sample count dropped by roughly a third.
- Consolidated two separate, near-duplicate implementations of replacement-pattern
  application (`apply_key_replacement_patterns` in `transform.rs`, used for keys,
  and `apply_value_replacement_cow` in `flatten.rs`, used for values) into one
  shared `apply_replacement_patterns` -- the logic never actually depended on
  whether the string being processed was a key or a value. Found while
  investigating the regex cache hotspot above: the value-replacement copy had
  never received the SIMD literal-replace fix (`memchr::memmem`-based, faster
  than `str::replace`) applied to the key-replacement path in an earlier round,
  so `value_replacement` with literal (non-regex) patterns was needlessly still
  on the slower std-matcher path. Fixed as a side effect of the consolidation;
  picked up in the `iso_05_value_replacement_only` benchmark numbers above
  (`literal_multiple` alone: ~15-19% faster).

## [0.9.4] - 2026-07-17

### Fixed
- **`auto_convert_types` silently corrupted the trailing digits of large integer
  strings.** Numeric-string-to-JSON-number conversion always routed every candidate
  through `f64` (`str::parse::<f64>` then reformat), but `f64` only has ~15-17
  significant decimal digits of exact precision -- so any string-encoded integer
  longer than that came back with corrupted digits, e.g. `"999999999999999999"` ->
  `1000000000000000000`, `"1234567890123456789"` -> `1234567890123456768`. Real-world
  64-bit IDs (Snowflake/Discord/database bigint primary keys) are commonly stored as
  JSON strings specifically *to avoid* this exact class of precision loss in other
  JSON parsers/languages, and are typically 17-19 digits, so this was a live bug, not
  a hypothetical one. Fixed by adding `canonical_json_integer` in `convert.rs`: when a
  string is already the exact canonical JSON integer we'd otherwise reconstruct
  (optional leading `-`, no leading zeros, and exactly representable as an `i64` or
  `u64` -- checked precisely, not via a blanket digit-count cutoff, so the fix covers
  the *entire* range the existing float round-trip claims to support), it's reused
  directly instead of being parsed to `f64` and reformatted. Deliberately excludes
  `"-0"` (the existing round-trip collapses it to `"0"`, matched to avoid an unrelated
  behavior change) and anything exceeding `u64::MAX` (falls through to the existing,
  unaffected float-fallback path). Verified via byte-for-byte A/B comparison against
  the pre-fix implementation across the full boundary set (`i64::MIN`/`MAX`,
  `u64::MAX`, and their neighbors) -- every previously-corrupted case now round-trips
  exactly. Also a real performance win for the common case (skips a full float parse
  and a heap-allocating reformat): ~8x faster in isolation for a realistic mix of
  mostly-clean integer strings; ~2.6% faster end-to-end on this project's own
  "medium" realistic-document benchmark, which doesn't happen to contain many long
  numeric-ID-style fields (most of its win is on the many short quantities/zip codes
  it does have, which are nanosecond-scale to begin with). Added 5 regression tests
  covering the fast path, leading zeros, negative zero, and exact i64/u64 boundaries.

### Changed
- `clean_number_string`'s credit/debit suffix stripping (the "100CR"/"100DR" accounting
  notation, part of `auto_convert_types`'s currency parsing) no longer chains
  `.trim_end_matches("CR")`/`"DR"`/`"cr"`/`"dr"`. Found via sampling profiler (same
  session as the `unflatten` root-`ObjectMap` fix): a stress-test call-stack sample
  showed `try_parse_number` spending real time inside `core::str::pattern::StrSearcher::new`
  -- std constructs its generic substring-search machinery for a `&str` pattern even
  though these are fixed 2-byte ASCII suffixes with no need for a general search
  algorithm. Replaced with a small hand-rolled `strip_trailing_ascii_pair` helper that
  directly compares the last two bytes in a loop. Isolated benchmark: ~21-28x faster
  for the specific chain; real end-to-end win on the `iso_07_auto_type_conversion_only`
  Criterion benchmark's `auto_convert/medium` case (the one that actually exercises
  currency values): ~13-17% faster, verified via the git-show A/B technique. Kept as 4
  sequential exhaustive-strip passes (matching `trim_end_matches`'s exact chained
  semantics, not a single combined loop) after confirming with a worked example that
  the two aren't equivalent: `"100CRDR"` and `"100DRCR"` resolve differently depending
  on strip order (only `"100DRCR"` ends up as a valid number) -- a regression test
  covers this asymmetry directly.
- `unflatten`'s root `ObjectMap` now starts pre-sized to `entries.len()` (an upper
  bound -- every entry lands either directly in root or in a nested object under it)
  instead of `ObjectMap::default()`'s zero capacity. Found via sampling profiler
  (`sample`/`samply` against a release build with debug symbols, not code reading):
  a stress-test call-stack sample showed `IndexMap::insert_full` repeatedly
  triggering `realloc`+copy as `root` grew one key at a time, accounting for a
  meaningful share of total allocator activity (`tiny_malloc`/`free_tiny`/
  `_platform_memmove` all showed up prominently in the leaf-function sample
  breakdown). Verified via the existing `iso_10_unflatten_only` Criterion
  benchmark: ~5% faster, consistent across repeated runs. Nested objects (created
  per-branch during tree building) also now start at a small fixed capacity (4)
  instead of zero -- a follow-up profiling run after the root fix still showed
  `_platform_memcmp`/`_platform_memmove` under `set_nested_value_recursive`, traced
  to the same `IndexMap` growth pattern one level down. Their eventual size isn't
  known without a separate counting pass (so, unlike root, this is a fixed guess,
  not an exact bound), but a small constant is enough to skip the first couple of
  regrow cycles for the common case of a handful of children. Re-verified via the
  same benchmark: a further ~2-4% faster on top of the root fix, consistent across
  repeated runs.
- **Python bindings: replaced `pythonize`/`depythonize` (generic serde-based Python<->Rust
  conversion) with direct calls to Python's own `json` module** for `dict`/`list[dict]`
  input/output and DataFrame/Series row conversion. The existing code's own comments
  claimed this was a "TIER 6→3 OPTIMIZATION... saves 50K-500K cycles per dict" versus a
  plain `json.dumps`/`json.loads` round-trip -- benchmarked with the actual built
  extension (`timeit`-style measurement, not just reasoning about it) and found that
  claim false for the realistic case: nested dicts (the case `.flatten()`/`.unflatten()`
  exist for -- a flat dict doesn't need flattening) were 5-30% *slower* through
  `depythonize` than a plain `json.dumps`+`execute(str)`+`json.loads`, and DataFrame rows
  (this library's other headline feature) were ~1.6x slower end-to-end. Root cause:
  `depythonize`/`pythonize` cross the PyO3 FFI boundary once per field/nesting level via
  serde's generic Serializer/Deserializer visitor pattern, while CPython's own `json`
  module is hand-tuned C that never leaves native CPython object representations until
  the final result. For DataFrames specifically, `to_dict()`/`to_dicts()` were *also*
  already expensive before any Rust-side conversion even started (materializing a full
  Python dict-object graph, one per row) compared to pandas' `to_json(orient='records',
  lines=True)` / polars' `write_ndjson()`, which write directly from columnar storage.
  Rewrote the DataFrame input path to use each library's native line-delimited JSON
  export where available (PyArrow has no equivalent, kept on `to_pylist()` +
  `json.dumps` per row -- still faster than the old `depythonize` path). Net effect,
  measured end-to-end against the actual built extension: single nested dict ~18%
  faster (now matches a hand-written `json` round-trip almost exactly, since it's doing
  the same work); 200-row pandas DataFrame ~1.6x faster. **Trade-off, reported
  honestly**: flat/shallow dicts, where `depythonize`'s per-field approach had less
  overhead to begin with, are slower under the new approach (single flat 5-field dict:
  ~2.5us -> ~3.7us; a batch of 100 flat dicts: ~238us -> ~326us) -- both still
  microsecond/sub-millisecond in absolute terms, and the crossover consistently tracks
  nesting depth, not batch size (a batch of 100 *nested* dicts was a small win, not a
  regression). Removes the `pythonize` crate dependency entirely (only user was this
  code). Verified equivalent output between old and new implementations via the actual
  built wheel across dict/list[dict]/DataFrame(pandas+polars+pyarrow)/Series inputs,
  including the empty-DataFrame edge case; full existing Python test suite (188 tests)
  passes unchanged. Exception message text for unsupported Python types changes slightly
  (Python's own `TypeError` message instead of `depythonize`'s) -- no existing test
  asserted on the old exact wording.
- Literal (non-regex) key/value replacement now locates matches with
  `memchr::memmem::find` (SIMD substring search) in a loop, building the result via
  bulk copy between matches, instead of `str::replace`'s std (non-SIMD) matcher.
  Getting this right took two iterations: a first version built one `memmem::Finder`
  per call and regressed the real `iso_04_key_replacement_only` Criterion benchmark's
  `literal_multiple` case by ~9-13% despite winning ~1.6-2x in an isolated
  microbenchmark -- `memchr::memmem::find`'s own source picks a lightweight
  Rabin-Karp search for haystacks under 64 bytes and only reaches for the heavier
  SIMD `Finder` above that threshold, and JSON keys (dotted flatten paths) are almost
  always under 64 bytes, so a `Finder` built fresh per call was paying SIMD setup
  cost it never earned back (`memmem::find_iter` has the identical problem -- it
  builds a `Finder` internally too, regardless of haystack size). Switched to
  calling the free `memmem::find` function per match instead, so each call re-picks
  the cheaper algorithm as the remaining haystack shrinks; re-validated against the
  same Criterion benchmark, now a consistent ~2.6-4.8% improvement with no
  regression. Caught only because this project validates against its own real
  benchmark suite, not just an isolated microbenchmark that happened to only
  exercise the SIMD path. Verified byte-for-byte identical output against the
  pre-change implementation across overlapping/adjacent matches, multi-byte UTF-8,
  empty patterns, and replacement strings that contain the search pattern. Added 2
  regression tests.
- `auto_convert_types`'s date detection (`try_parse_compact_date`, `try_parse_ordinal_date`,
  and standard-date's date-only branch in `convert.rs`) now validates via
  `NaiveDate::from_ymd_opt`/`from_yo_opt` directly instead of chrono's generic
  format-string parser, which re-interprets the format string on every call. The
  `could_be_date` prefilter can't distinguish a real compact date from an 8-digit
  numeric ID/zip+4/order number starting with 1 or 2 (common once `auto_convert_types`
  is enabled), so this path is reached for plenty of non-dates. ~25% faster on a
  realistic mixed workload (real dates + false-positive numeric IDs); verified
  byte-for-byte identical output against the old implementation across 31 cases
  (leap years, invalid months/days, all four date formats, timezones, non-dates).
  Datetime/timezone-offset parsing is untouched (left on chrono's parser -- more
  complex to hand-roll correctly, and less prone to false-positive collisions).

### Added
- `flatten`'s slow path (`CollectingWalker`, used when key lowercasing/replacement/
  collision-handling is configured) now stores keys in a `bumpalo` arena instead of
  `CompactString` for single-document (non-nested-parallel) processing. Flatten's
  slow-path keys are full dotted paths (e.g. `"response.data.attributes.firstName"`),
  which commonly exceed `CompactString`'s 24-byte inline cap for 3+ level nesting and
  would otherwise still heap-allocate one at a time. An isolated benchmark measured
  ~5.9x faster for realistic deep-nested-path collection; end-to-end (real
  flatten+lowercase+key_replacement call) measured ~14% faster on a deep-nesting
  workload. On mixed/shallow-nesting data (this project's own "medium" realistic
  benchmark payload) the effect is neutral -- measured within ±1-2% noise across
  repeated runs, not a consistent win or loss, since short paths were already
  allocation-free via `CompactString`'s inlining and gain nothing from the arena.
  The nested-parallel path (`flatten_collecting_parallel`, used for very large
  documents) intentionally still uses `CompactString`: `bumpalo::Bump` isn't
  `Send`/`Sync`, and safely bundling per-thread arenas with the entries that borrow
  from them across the parallel merge would require unsafe self-referential-struct
  code or a specialized crate -- not worth it for what's already a narrower case
  (a document large enough to cross `nested_parallel_threshold` *and* key transforms
  configured at once). `CollectedEntry`/`CollectingWalker` are now generic over a
  `KeyBuilder` trait so both strategies share all the tape-walking logic. Added 5
  regression tests covering deep nesting combined with lowercase, key_replacement,
  collision handling, and multi-byte UTF-8, plus confirmation the nested-parallel
  path is unaffected.

## [0.9.3] - 2026-07-16

### Fixed
- **`flatten` produced invalid JSON for any key containing an escaped character**
  (`\"`, `\\`, or a control-character escape) **when no key transform was configured**
  (the common case: plain `.flatten()` with no `lowercase_keys`/`key_replacement`/
  collision-handling). The fast path unescaped such a key to build the internal path
  buffer but never re-escaped it before writing that buffer directly as the output
  key, so e.g. a source key `"say \"hi\""` produced the syntactically invalid output
  key `"say "hi""`. Fixed by never unescaping in this path at all -- it doesn't need
  the logical key value (no transform is applied to it), so the original,
  already-correctly-escaped source bytes can be used directly. Also removes an
  unnecessary allocation.
- **`flatten`'s and `unflatten`'s JSON string re-escaping corrupted multi-byte UTF-8
  characters** whenever a string needed *any* escaping (an embedded quote, backslash,
  or control character) and also contained non-ASCII text -- e.g. `café "quoted"`
  became `cafÃ© \"quoted\"`. The escaping slow path reinterpreted each byte
  individually as its own Latin-1 codepoint (`output.push(b as char)`) instead of
  bulk-copying multi-byte sequences intact. Same bug class as the `unescape_json_string`
  fix from the 2026-07 audit, but present in the opposite (re-escaping) direction and
  missed at the time. Affected: key escaping when `lowercase_keys`/`key_replacement`/
  collision-handling is configured (`CollectingWalker`'s slow path), value escaping
  via `value_replacement`, and `unflatten`'s key serialization (which reuses the same
  function). Fixed by bulk-copying plain byte runs between escape sequences, mirroring
  `unescape_json_string`'s existing correct pattern -- this also fixes the performance
  issue the rewrite was originally intended for (Criterion: ~17-22% faster on
  key-replacement scenarios, p < 0.05).

### Added
- Object keys throughout `unflatten`'s tree (`ObjectMap`) now use `CompactString`
  instead of `String`, inlining keys up to 24 bytes with no heap allocation. Real-world
  JSON keys are short (this project's own benchmark corpus averages ~8.6 chars, max
  22), so nearly every key insertion avoids allocating entirely. Validated with an
  isolated micro-benchmark before adopting (~3.4x faster than `String` for realistic
  insert+lookup key-map workloads) and confirmed on the real unflatten path via
  Criterion: ~19-22% faster (p < 0.05, reproduced across multiple runs).
  `flatten`'s slow path (`CollectedEntry`, used when key lowercasing/replacement/
  collision-handling is configured) got the same change for consistency -- its keys
  are full flattened paths rather than single segments, so more of them exceed the
  24-byte inline threshold and the win is smaller and noisier to measure (the machine
  used for benchmarking this session was under heavy thermal load by this point: the
  *same* binary showed 60% run-to-run variance with zero code changes). Directionally
  positive across repeated controlled A/B comparisons, kept for architectural
  consistency and because it carries no measured downside, but not claiming a precise
  number for this specific path.

### Changed
- `unflatten`'s tree-building pass no longer re-scans each key's separators a second
  time: the path-type analysis pass and the tree-building pass now share one set of
  separator offsets per key (previously the analysis pass located separators via
  `find_separator`, then tree-building independently re-split the same key via
  `str::split`). Pure algorithmic change, portable to every platform and to plain
  `cargo add` library consumers, not just published wheels.
- The regex pattern cache (both the thread-local and global tiers) now evicts the
  genuinely least-recently-used entry when full, via a shared monotonic tick bumped
  on every cache hit, instead of an arbitrary entry (global tier: `cache.keys().next()`;
  thread-local tier: alternating half-retain with no recency awareness). Protects hot
  patterns from eviction under high pattern-cardinality workloads. No new dependency.
- Bumped patch/minor dependencies within already-declared `Cargo.toml` ranges:
  mimalloc 0.1.48->0.1.52, smallvec 1.15.1->1.15.2, regex, serde_json, and chrono
  to latest patch. sonic-rs is pinned to exactly `=0.5.7`: 0.5.8 uses
  `exposed_provenance`/`strict_provenance`, which need a newer rustc than this
  crate's MSRV of 1.80.
- `unflatten`'s output buffer is now sized from the input JSON's byte length instead
  of a fixed 256-byte default, avoiding repeated capacity-doubling reallocation on
  larger payloads (`flatten`'s equivalent buffers already did this correctly).
- `lowercase_if_needed` (key lowercasing's no-op fast-path check) now uses a
  byte-level ASCII scan instead of decoding full Unicode codepoints, when the
  string is pure ASCII -- the common case for real-world JSON keys. Falls back to
  the original Unicode-aware scan for non-ASCII input, so correctness on e.g. 'Ñ'
  is unchanged.

## [0.9.2] - 2026-07-15

Note: `v0.9.1` was tagged on 2026-07-14 but only completed publishing to Maven
Central -- a bug in the crates.io/PyPI release job (untracked downloaded wheel
artifacts tripping `cargo publish`'s dirty-checkout guard) caused it to fail before
any crates.io or PyPI upload, so those two never got a 0.9.1 release. Fixed and
re-cut as 0.9.2 across all three registries the same day; no code changes from what
0.9.1 would have shipped, only the release pipeline fix itself.

### Added
- **JVM (Java) bindings**, for use as Apache Spark UDFs -- see [`jvm/README.md`](jvm/README.md).
  Full feature parity with the Python bindings (regex/literal key & value replacement,
  empty-value filtering, key casing, type conversion), via a new opt-in `jvm` Cargo
  feature (`src/jvm.rs`, JNI shim over the same core `JSONTools` builder). Ships two
  usage tiers: a simple row UDF (`FlattenUDF`/`UnflattenUDF`, SQL-callable via
  `spark.udf.registerJavaFunction`) and a higher-throughput batched `Dataset.mapPartitions`
  transform (`BatchTransform`) that amortizes JNI-crossing overhead across many rows
  per native call. Packaged as a multi-platform (`linux-x86_64`, `linux-aarch64`) fat
  jar built by a new `jvm-ci.yml` CI workflow, intended for Databricks Jobs/notebooks
  on classic compute and other Spark workloads -- **not** usable inside a Lakeflow
  Declarative Pipeline (formerly Delta Live Tables): Databricks does not permit JVM
  libraries on pipeline compute at all. For running inside a pipeline, wrap the
  Python bindings in a `pandas_udf` instead -- see [Setting Up on
  Databricks](docs/src/guide/databricks-setup.md).
  Tagged releases (`git tag vX.Y.Z`) now also publish `io.github.amaye15:json-tools-rs-spark`
  to Maven Central automatically (GPG-signed, via Sonatype's Central Portal).
- **crates.io publishing**: `publish = false` removed from `Cargo.toml` and the
  (previously dormant, commented-out) `cargo publish` step in `maturin-ci.yml`'s tag-gated
  release job is now active. Also trimmed the published package to Rust-relevant files
  only (`exclude`d `jvm/`, `python/`, `docs/`, and tooling config -- those aren't useful
  to a `cargo add json-tools-rs` consumer and don't belong in the crate archive).

### Changed (BREAKING)
- **`key_replacement`/`value_replacement` pattern syntax**: patterns are now literal
  (exact substring match) by default; wrap a pattern in `r'...'` (e.g. `r'^admin_'`) to
  use it as a regex. Previously *every* pattern was always compiled as regex regardless
  of content, with silent fallback to literal matching only on a regex syntax error --
  meaning a pattern with regex metacharacters (`.`, `$`, `(`, etc.) could never be matched
  literally, and the documented `regex:` prefix (see Fixed, below) never actually worked.
  **Action needed**: any pattern relying on regex syntax (anchors, character classes,
  alternation, capture groups, etc.) must now be wrapped in `r'...'`.

### Fixed
- **`has_escape` scanner bug**: the tape scanner's detection of "does this key/value
  contain a JSON escape sequence" only recognized escaped quotes (`\"`) and backslash
  runs immediately before a matched quote character. Any escape not adjacent to a quote
  -- a lone `\n`, `\t`, `\r`, `\b`, `\f`, `\/`, or `\uXXXX` -- was invisible to it, so
  `auto_convert_types`, `value_replacement`, `key_replacement`, `lowercase_keys`, and
  collision handling would silently operate on the still-escaped text for such strings
  (e.g. `.auto_convert_types(true)` failing to convert `"123\t"` to a number).
- The documented `regex:` prefix for replacement patterns was never implemented -- no
  code anywhere recognized it, so patterns written the documented way silently never
  matched anything. Replaced by the `r'...'` syntax described above.
- `value_replacement` + `auto_convert_types` together unescaped the same string twice
  when the replacement pattern didn't match.
- `batch_flatten`'s parallel dispatch (`std::thread::scope`) spawned fresh OS threads on
  every `.execute()` call; at the default `parallel_threshold` this was measurably
  *slower* than sequential processing on Windows and Linux for small-to-medium batches.
  Replaced with `rayon`'s persistent work-stealing pool.
- `unflatten`'s object tree used a hash map plus a full key sort at every serialized
  object node purely to get deterministic output; switched to an order-preserving map
  (no sort needed, and no more O(n) lookup degrading to O(n^2) for JSON objects used as
  wide keyed maps, e.g. many `"user_<id>.field"` entries).
- `maturin-ci.yml`'s `cargo publish` step failed on every run: the job downloads all
  wheel/sdist artifacts before the publish step, and the resulting untracked files
  tripped `cargo publish`'s dirty-checkout guard. Added `--allow-dirty` (safe here --
  the job always starts from a fresh tag checkout, so the only "dirty" files are ever
  those artifact downloads, never an actual uncommitted change).

## [0.9.0] - 2026-03-09

### Added
- **DataFrame & Series Support** (Python)
  - Native support for Pandas, Polars, PyArrow, and PySpark DataFrames
  - Native support for Pandas, Polars, PyArrow, and PySpark Series
  - Automatic type detection via duck typing (no explicit imports required)
  - Input type preservation: DataFrame in → DataFrame out, Series in → Series out
  - Graceful fallback to list of dicts when library reconstruction fails
- **Crossbeam Parallelism**
  - Migrated all parallel paths from Rayon to Crossbeam `thread::scope`
  - Finer-grained control over thread spawning and chunk distribution
  - Ordered parallel output via `chunks().zip(slots.chunks_mut())` pattern

### Performance Improvements
- **Rust Core Optimizations**
  - Eliminated per-entry HashMap in parallel flatten — each thread now flattens directly into a single pre-sized `partial` map with `quick_leaf_estimate()` sizing
  - Early-exit byte discriminators in `try_parse_number()` — gates 4 specialized parsers behind cheap byte checks (basis points, suffixed, fractions, radix)
  - SIMD literal fallback in key/value replacements — `str::contains()` replaced with `memmem::find()` for SIMD-accelerated substring search
  - Thread-local regex cache half-eviction — retains ~50% of entries instead of clearing all 64 on overflow
  - SmallVec buffer expanded from 32 to 64 bytes in `clean_number_string()` to reduce heap spillover
  - Separator cache expanded from 6 to 12 static entries (`->`, `__`, `#`, `~`, `@`, `%`)
- **Python Binding Optimizations**
  - `mem::replace` → `mem::take` across 13 builder methods (eliminates `JSONTools::new()` default construction)
  - O(N) → O(1) DataFrame/Series reconstruction (single `into_pyobject` + `clone_ref` instead of per-item clone)
  - GIL release via `py.detach()` during all compute-intensive operations

### Changed
- **Modular Architecture**: Refactored monolithic `src/lib.rs` (5,447 lines) into 10 focused modules for maintainability
  - `json_parser.rs` -- Conditional SIMD parser (sonic-rs / simd-json)
  - `types.rs` -- Core types (`JsonInput`, `JsonOutput`, `FlatMap`)
  - `error.rs` -- Error types with machine-readable codes (E001-E008)
  - `config.rs` -- Configuration structs and operation modes
  - `cache.rs` -- Tiered caching (regex, key deduplication, phf)
  - `convert.rs` -- Type conversion (numbers, dates, booleans, nulls)
  - `transform.rs` -- Filtering, replacements, collision handling
  - `flatten.rs` -- Flattening algorithm with Crossbeam parallelism
  - `unflatten.rs` -- Unflattening with SIMD separator detection
  - `builder.rs` -- Public `JSONTools` builder API
  - `lib.rs` now serves as a thin facade with `mod` declarations and `pub use` re-exports
  - Zero public API changes -- all existing import paths preserved
  - Performance-neutral -- Rust modules are compile-time organization only
- Updated all documentation to reflect Crossbeam migration, modular architecture, and new features
- Fixed stale Rayon references in Python binding docstrings
- Bumped version to 0.9.0

## [0.8.0] - 2026-01-01

### Added
- **Full Python Bindings Feature Parity**
  - All Rust features now available in Python bindings
  - `.auto_convert_types(bool)` - Convert strings to numbers/booleans
  - `.parallel_threshold(n)` - Configure batch parallelism threshold
  - `.num_threads(n)` - Configure thread count
  - `.nested_parallel_threshold(n)` - Configure nested parallelism
  - 128 comprehensive Python tests covering all features
- **Enhanced Testing**
  - 89 Rust unit tests + 21 doc tests
  - 128 Python binding tests
  - Improved test coverage for all features

### Changed
- Updated Python `__init__.py` with auto_convert_types documentation
- Bumped version to 0.8.0

## [0.7.0] - 2025-10-17

### Added
- **Parallel Processing Configuration**
  - `.parallel_threshold(usize)` - Configure minimum batch size for parallel processing (default: 1000)
  - `.num_threads(Option<usize>)` - Configure number of threads for parallel processing (default: system CPU count)
  - `.nested_parallel_threshold(usize)` - Configure threshold for nested parallel processing within individual JSON documents (default: 100)
  - Environment variable support: `JSON_TOOLS_PARALLEL_THRESHOLD` and `JSON_TOOLS_NESTED_PARALLEL_THRESHOLD`
- **Enhanced Testing**
  - Added 671 new lines of comprehensive tests
  - Improved test coverage for parallel processing scenarios
  - Additional edge case testing for type conversion and filtering

### Performance Improvements
- **Optimized HashMap Initialization**
  - Pre-allocated FxHashMap with known capacity for better performance
  - Reduced memory allocations during regex caching
  - Improved thread-local regex cache initialization
  - Enhanced key deduplication cache performance

### Changed
- Improved parallel processing defaults for better out-of-the-box performance
- Enhanced documentation for parallel processing configuration
- Updated benchmarks to include parallel processing scenarios

## [0.6.0] - 2025-10-13

### Added
- **Python Bindings Performance Optimizations**
  - GIL (Global Interpreter Lock) release during compute-intensive operations
  - Enables true multi-threading in Python applications
  - `#[inline]` attributes on all builder methods for better optimization

### Performance Improvements
- **Python Bindings**: 5-13% performance improvement across most operations
  - Roundtrip operations: +13.2% (75K → 85K ops/sec)
  - Array flattening: +9.6% (8.3K → 9.1K ops/sec)
  - Batch string processing: +8.5% (54.7K → 59.3K ops/sec)
  - Large data processing: +7.7% (666 → 717 ops/sec)
  - Batch operations: +4.8% to +5.6% across all sizes
  - Complex configurations: +5.0% (90K → 95K ops/sec)
- **Multi-threading**: Python applications can now run other threads while Rust code executes
- **Rust Core**: Cumulative 32-60% improvement from previous optimizations (v0.4.0-0.5.0)
  - FxHashMap for 15-30% faster string key operations
  - SIMD JSON parsing optimizations
  - Reduced memory allocations (~50% fewer string clones)
  - Pre-allocated collections
  - Optimized hash lookups with entry() API
  - #[inline(always)] on hot path functions
  - #[cold] on error paths

### Changed
- Python bindings now release GIL during all execute operations
- All Python builder methods now have inline optimization hints

### Technical Details
- Added `py.allow_threads()` around compute operations in:
  - `execute()` method (3 locations: string, dict, list)
  - `execute_to_output()` method (3 locations: string, dict, list)
- Added `#[inline]` to 13 builder methods in Python bindings

## [0.5.0] - 2025-10-12

### Added
- **Rust Core Performance Optimizations (Phase 3)**
  - #[inline(always)] on 6 critical hot path functions
  - #[cold] + #[inline(never)] on 4 error path functions
  - Optimized compiler hints for better code generation

### Performance Improvements
- **Rust Core**: Additional 2-5% improvement on top of Phase 1-2 optimizations
  - Batch processing: ~2% faster
  - Roundtrip operations: ~2-5% faster
  - Total cumulative improvement: 32-60% from baseline

## [0.4.0] - 2025-10-11

### Added
- **Rust Core Performance Optimizations (Phase 1-2)**
  - Enhanced Cargo.toml with LTO "fat" for better cross-crate inlining
  - CPU-specific optimizations with target-cpu=native
  - FxHashMap replacing standard HashMap for 15-30% faster string operations
  - Reduced string clones in key transformations (~50% reduction)
  - Optimized SIMD JSON parsing for reduced memory allocations
  - Pre-allocated Vec and Map capacity
  - Entry API for faster hash lookups
  - Optimized struct field ordering for better memory alignment

### Performance Improvements
- **Rust Core**: 30-55% performance improvement across all operations
  - Basic flattening: 2,000+ ops/ms
  - Advanced configuration: 1,300+ ops/ms
  - Regex replacements: 1,800+ ops/ms
  - Batch processing: 1,900+ ops/ms
  - Roundtrip operations: 1,000+ cycles/ms

## [0.3.0] - 2025-10-10

### Added
- **Automatic Type Conversion** feature
  - Convert strings to numbers and booleans with `.auto_convert_types(true)`
  - Handles currency symbols ($, €, £, ¥)
  - Supports thousands separators (1,234.56 and 1.234,56)
  - Scientific notation support (1.23e10)
  - Boolean conversion (true/false, TRUE/FALSE, True/False)
  - Opportunistic conversion - keeps original value if conversion fails
- **Python Bindings** with full feature parity
  - Type preservation: str→str, dict→dict, list[str]→list[str], list[dict]→list[dict]
  - Batch processing support
  - All Rust features available in Python
  - Comprehensive test suite (107 tests)

### Changed
- Unified API with `JSONTools` as single entry point
- Builder pattern for all operations
- Consistent API across Rust and Python

## [0.2.0] - 2025-10-09

### Added
- **Collision Handling** with `.handle_key_collision(true)`
  - Collects duplicate keys into arrays
  - Filtering applied during collision resolution
- **Comprehensive Filtering** for both flatten and unflatten
  - `.remove_empty_strings(true)`
  - `.remove_nulls(true)`
  - `.remove_empty_objects(true)`
  - `.remove_empty_arrays(true)`
- **Advanced Replacements**
  - Literal and regex-based key/value replacements
  - Standard Rust regex syntax
  - Automatic fallback to literal matching for invalid regex
- **Batch Processing**
  - Process single JSON or Vec<String>
  - Efficient batch operations

### Changed
- Improved error handling with `JsonToolsError` enum
- Better error messages with suggestions

## [0.1.0] - 2025-10-08

### Added
- Initial release
- **Basic Flattening** - Convert nested JSON to flat structure
- **Basic Unflattening** - Reconstruct nested JSON from flat structure
- **Roundtrip Support** - Perfect fidelity for flatten→unflatten
- **Custom Separators** - Configure key separator (default: ".")
- **Lowercase Keys** - Convert all keys to lowercase
- **SIMD JSON Parsing** - Hardware-accelerated parsing via simd-json
- **Comprehensive Error Handling** - Detailed error messages
- **Extensive Test Coverage** - 48 unit tests + 17 doc tests

### Technical Details
- Rust 2021 edition
- SIMD-accelerated JSON parsing
- Zero-copy optimizations where possible
- Comprehensive documentation

---

## Version History Summary

| Version | Release Date | Key Features | Performance |
|---------|--------------|--------------|-------------|
| **0.9.0** | 2026-03-09 | Crossbeam parallelism, DataFrame/Series, modular architecture | +3-5% Rust, O(1) Python reconstruction |
| **0.8.0** | 2026-01-01 | Full Python bindings feature parity | Feature release |
| **0.7.0** | 2025-10-17 | Parallel processing config, optimizations | HashMap improvements |
| **0.6.0** | 2025-10-13 | Python GIL release, inline hints | +5-13% Python |
| **0.5.0** | 2025-10-12 | Rust inline optimizations | +2-5% Rust |
| **0.4.0** | 2025-10-11 | FxHashMap, SIMD, allocations | +30-55% Rust |
| **0.3.0** | 2025-10-10 | Type conversion, Python bindings | Feature release |
| **0.2.0** | 2025-10-09 | Collision handling, filtering | Feature release |
| **0.1.0** | 2025-10-08 | Initial release | Baseline |

---

## Migration Guide

### Upgrading from 0.8.0 to 0.9.0

**No breaking changes!** This is a performance and feature enhancement release.

**What's New**:
- Crossbeam-based parallelism (replaces Rayon) for finer-grained thread control
- Native DataFrame/Series support in Python (Pandas, Polars, PyArrow, PySpark)
- Modular architecture: `lib.rs` refactored into 10 focused modules (zero API changes)
- 6 Rust core performance optimizations (parallel flatten, type conversion, regex, caching)
- 3 Python binding optimizations (mem::take, O(1) reconstruction, GIL release)

**Action Required**: None - just update your dependency version. If you were using the library with DataFrames, you can now pass them directly to `.execute()` instead of converting to dicts first.

### Upgrading from 0.7.0 to 0.8.0

**No breaking changes!** This is a feature enhancement release.

**What's New**:
- Full Python bindings feature parity - all Rust features now available in Python
- `.auto_convert_types()` now available in Python for type conversion
- `.parallel_threshold()`, `.num_threads()`, `.nested_parallel_threshold()` in Python
- Enhanced test coverage (128 Python tests, 109 Rust tests)

**Action Required**: None - just update your dependency version.

### Upgrading from 0.6.0 to 0.7.0

**No breaking changes!** This is a feature enhancement and performance improvement release.

**What's New**:
- New parallel processing configuration methods
- Better control over thread usage and parallelism thresholds
- Optimized HashMap initialization for better performance

**Action Required**: None - just update your dependency version. Optionally, you can configure parallel processing settings for your specific workload.

### Upgrading from 0.5.0 to 0.6.0

**No breaking changes!** This is a pure performance improvement release.

**What's New**:
- Python applications automatically benefit from GIL release
- Better multi-threading support in Python
- 5-13% faster Python operations

**Action Required**: None - just update your dependency version

### Upgrading from 0.4.0 to 0.5.0

**No breaking changes!** Pure performance improvements.

### Upgrading from 0.3.0 to 0.4.0

**No breaking changes!** Pure performance improvements.

### Upgrading from 0.2.0 to 0.3.0

**API Changes**:
- Removed separate `JsonFlattener` and `JsonUnflattener` APIs
- Use unified `JSONTools` API instead
- All functionality preserved, just cleaner API

**Migration Example**:
```rust
// Old (0.2.0)
use json_tools_rs::JsonFlattener;
let result = JsonFlattener::new()
    .flatten()
    .execute(json)?;

// New (0.3.0+)
use json_tools_rs::JSONTools;
let result = JSONTools::new()
    .flatten()
    .execute(json)?;
```

---

## Links

- [Repository](https://github.com/amaye15/JSON-Tools-rs)
- [Crates.io](https://crates.io/crates/json-tools-rs)
- [Documentation](https://docs.rs/json-tools-rs)
- [Issues](https://github.com/amaye15/JSON-Tools-rs/issues)

