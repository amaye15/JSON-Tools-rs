# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

