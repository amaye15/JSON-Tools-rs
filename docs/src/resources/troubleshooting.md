# Troubleshooting

This guide covers common errors, their causes, and how to resolve them.

## Error Code Reference

All errors embed a machine-readable code (`E001`-`E008`) in square brackets in the error message. Use these codes for programmatic error handling. In Rust, `Display` on `JsonToolsError` always starts with the bracketed code (e.g. `[E001] ...`). In Python, the bindings prepend their own context (e.g. `"Failed to process JSON string: "`) before the underlying message, so the code is embedded in `str(e)` but is not always its first characters -- match `"[E00x]"` as a substring, not a prefix.

### E001: JsonParseError

**Message:** `[E001] JSON parsing failed: ...`

**Cause:** The input string is not valid JSON.

**Common triggers:**
- Missing quotes around keys or values
- Trailing commas after the last element
- Single quotes instead of double quotes
- Unescaped special characters in strings
- Incomplete JSON (missing closing braces or brackets)
- Passing a file path instead of the file contents

**Solution:**
```python
# Wrong
result = tools.execute("hello world")          # Not JSON
result = tools.execute("{'key': 'value'}")     # Single quotes
result = tools.execute('{"a": 1,}')            # Trailing comma

# Correct
result = tools.execute('{"key": "value"}')
result = tools.execute({"key": "value"})       # Pass a dict directly
```

### E002: RegexError

**Message:** `[E002] Regex pattern error: ...`

**Cause:** Reserved for regex compilation failures. In practice this code is not
currently reachable through `.key_replacement()` / `.value_replacement()`: patterns
are **literal by default** (exact substring match, no regex engine involved at all),
and a pattern explicitly wrapped in `r'...'` that fails to compile as regex is
**silently treated as "no match"** rather than raised as an error -- so a broken
`r'...'` pattern won't crash your pipeline, it just won't replace anything. `E002` is
kept in the error enum for API completeness/forward compatibility.

**Common mistake this code used to cover, now handled differently:**
```python
# "user.name" is now a LITERAL pattern -- the dot is not a wildcard.
# It only matches the exact substring "user.name", not "username".
tools.key_replacement("user.name", "id")

# To use regex (e.g. so the dot matches any character), wrap in r'...':
tools.key_replacement("r'user.name'", "id")

# To match the literal dot as regex, escape it inside the wrapper:
tools.key_replacement(r"r'user\.name'", "id")
```

See [Key & Value Replacements](../guide/replacements.md) for the full `r'...'` convention.

### E003: InvalidReplacementPattern

**Message:** `[E003] Invalid replacement pattern: ...`

**Cause:** Reserved for a malformed replacement pattern configuration. Like `E002`, this
code is not currently constructed anywhere in the codebase -- `.key_replacement()` /
`.value_replacement()` always take exactly two arguments (`find`, `replace`), so
there's no "wrong number of arguments in a pattern list" case to detect. It's kept in
the error enum for API completeness/forward compatibility.

```python
# Both key_replacement and value_replacement always take exactly (find, replace)
tools.key_replacement("find_pattern", "replacement")
tools.value_replacement("old_value", "new_value")
```

### E004: InvalidJsonStructure

**Message:** `[E004] Invalid JSON structure: ...`

**Cause:** The set of flat keys given to `.unflatten()` describes a structurally
inconsistent tree -- most commonly, one key is a strict prefix of another but already
holds a scalar value, so the longer key can't navigate "into" it.

Two inputs that look like they *should* trigger this but don't: a root-level JSON
array passed to `.unflatten()` is a dedicated early-exit case that returns `"{}"`
instead of erroring, and a value that's itself a nested object (e.g. `{"a": {"b":
1}}`) is accepted as-is -- `.unflatten()` only requires *keys* to be splittable on the
separator; it doesn't require *values* to be scalar.

**Solution:**
```python
# Wrong -- "a" is set to a scalar (1), then "a.b" tries to navigate into it as an object
result = jt.JSONTools().unflatten().execute('{"a": 1, "a.b": 2}')
# Raises: [E004] Invalid JSON structure: Cannot navigate into non-object/non-array
# value at key: a

# Correct -- keys don't collide on structure
result = jt.JSONTools().unflatten().execute('{"a.b": 1, "a.c": 2}')
```

### E005: ConfigurationError

**Message:** `[E005] Operation mode not configured: ...`

**Cause:** `.execute()` was called without first setting an operation mode.

**Solution:** Always call `.flatten()`, `.unflatten()`, or `.normal()` before `.execute()`:

```python
# Wrong
result = jt.JSONTools().execute(data)

# Correct
result = jt.JSONTools().flatten().execute(data)
result = jt.JSONTools().unflatten().execute(data)
result = jt.JSONTools().normal().execute(data)
```

This error also occurs if `num_threads` is set to `0` -- note the check happens at `.execute()` time, not when `.num_threads(0)` is called (the message text is the shared `ConfigurationError` template, "Operation mode not configured: ...", even though the actual problem is the thread count, not a missing mode):

```python
# Wrong -- raises E005 when .execute() runs, even though a mode was set
tools = jt.JSONTools().flatten().num_threads(0)
tools.execute(data)

# Correct
tools = jt.JSONTools().flatten().num_threads(1)    # At least 1
tools = jt.JSONTools().flatten()                    # Use default (CPU count)
```

### E006: BatchProcessingError

**Message:** `[E006] Batch processing failed at index {N}: Failed to process item at index {N}`

**Cause:** One or more items in a batch failed to process. The Rust core wraps the failing item's original error as the `source` of a `BatchProcessingError` -- but the *printed* message only ever says "Failed to process item at index N", not the specific underlying reason (e.g. the E001 parse error text). In Rust, you can recover the specific cause by pattern-matching the `source` field (see the [Rust API](../reference/rust-api.md#jsontoolserror) error-handling example); the Python bindings don't expose `source`, so `str(e)` alone won't tell you *why* that item failed.

**Solution:** Use the reported index to isolate and re-run just that item, so its own error (not the generic batch wrapper) surfaces:

```python
try:
    results = tools.execute(batch_of_json)
except jt.JsonToolsError as e:
    msg = str(e)
    if "[E006]" in msg:
        # The message only gives you the index, e.g. "...at index 1: Failed to
        # process item at index 1" -- re-run that single item to see its real cause.
        for i, item in enumerate(batch_of_json):
            try:
                tools.execute(item)
            except jt.JsonToolsError as item_err:
                print(f"Item {i} failed: {item_err}")
```

### E007: InputValidationError

**Message:** `[E007] Input validation failed: ...`

**Cause:** Raised by the Rust core itself (not the Python/JVM binding layer) for a handful of specific conditions:
- Empty JSON input (an empty or all-whitespace string passed to `.flatten()` -- `.unflatten()` treats this as `{}` instead, not an error)
- Input exceeding the 4 GiB size limit
- An array index during `.unflatten()` that exceeds `max_array_index()` (e.g. a flattened key like `"items.999999999"`)

**Solution:**
```python
# Wrong -- empty input to flatten
result = jt.JSONTools().flatten().execute("")  # Raises E007

# Wrong -- array index beyond max_array_index (default 100,000)
result = jt.JSONTools().unflatten().execute('{"items.999999999": 1}')  # Raises E007

# Correct -- either supply valid data or raise the limit
result = jt.JSONTools().unflatten().max_array_index(10_000_000).execute('{"items.999999999": 1}')
```

**Not the same as an unsupported Python input type.** Passing a type the Python bindings don't recognize at all (an `int`/`float`/`bool` directly, a list containing something other than strings/dicts, or a DataFrame/Series to `execute_to_output()`) raises a plain Python **`ValueError`** at the binding layer, not `jt.JsonToolsError` -- `jt.JsonToolsError` does not subclass `ValueError`, so `except jt.JsonToolsError` will not catch it:

```python
# Wrong -- these all raise plain ValueError, not jt.JsonToolsError
result = tools.execute(42)
result = tools.execute([1, 2, 3])
output = tools.execute_to_output(some_dataframe)

# Correct
result = tools.execute('{"value": 42}')
result = tools.execute({"value": 42})
result = tools.execute(['{"a": 1}', '{"b": 2}'])
result = tools.execute(some_dataframe)  # use execute(), not execute_to_output(), for DataFrames
```

### E008: SerializationError

**Message:** `[E008] JSON serialization failed: ...`

**Cause:** The processed result could not be serialized back to JSON. This is typically an internal error.

**Solution:** If you encounter this error, please report it as a bug. As a workaround, check that your input does not contain unusual Unicode sequences or extremely large numbers that may not round-trip through JSON.

## Common Issues

### Empty Separator

The separator must be a non-empty string. Using an empty separator is always a logic error -- it would make keys ambiguous.

```python
# This raises an error
tools = jt.JSONTools().flatten().separator("")

# Use any non-empty string
tools = jt.JSONTools().flatten().separator(".")
tools = jt.JSONTools().flatten().separator("::")
tools = jt.JSONTools().flatten().separator("/")
```

In Rust, `.separator("")` itself doesn't fail -- the check happens at `.execute()` time, which returns `Err(JsonToolsError::ConfigurationError)` (`E005`), not a panic. In Python, `.separator("")` raises a `ValueError` immediately, at builder-call time.

### Missing Operation Mode

The most common mistake is forgetting to set a mode:

```python
# This always raises E005
tools = jt.JSONTools()
tools.execute(data)  # Error!

# Set a mode first
tools = jt.JSONTools().flatten()
tools.execute(data)  # OK
```

### Dict vs String Input

Both `str` and `dict` inputs are accepted, but the output type mirrors the input type:

```python
# String in -> string out
result = tools.execute('{"a": {"b": 1}}')
assert isinstance(result, str)
# result == '{"a.b":1}'

# Dict in -> dict out
result = tools.execute({"a": {"b": 1}})
assert isinstance(result, dict)
# result == {"a.b": 1}
```

If you need the raw JSON string output from a dict input, use `.execute_to_output()`:

```python
output = tools.execute_to_output({"a": {"b": 1}})
json_str = output.get_single()  # Returns a JSON string
```

### Literal vs. Regex Patterns in Replacements

Replacement patterns are **literal (exact substring match) by default**. Wrap a
pattern in `r'...'` to use standard regex syntax instead:

```python
# Literal: matches the exact substring "user_" anywhere in the key
tools.key_replacement("user_", "")

# Regex: anchors, character classes, etc. only work inside r'...'
tools.key_replacement("r'^user_'", "")       # Only at start of key
tools.key_replacement("r'_suffix$'", "")     # Only at end of key
tools.key_replacement("r'user.name'", "id")  # Dot matches any character

# A bare pattern with regex metacharacters is still literal --
# this looks for the exact substring "user.name", not "username"
tools.key_replacement("user.name", "id")
```

A malformed pattern inside `r'...'` is silently ignored (no match, no error) rather
than raising `E002` -- see above.

## Performance Tuning

### When Parallelism Helps

Parallel processing adds overhead for thread spawning and synchronization. It helps when:

- **Batch size is large** (100+ items by default) -- amortizes spawning cost
- **Individual documents are complex** -- deep nesting, many keys, expensive transformations
- **CPU cores are available** -- parallelism on a single-core machine adds only overhead

### When Parallelism Hurts

Reduce or disable parallelism when:

- **Documents are tiny** (a few flat keys) -- thread overhead dominates
- **Batch sizes are small** (<50 items) -- raise `parallel_threshold`
- **Memory is constrained** -- each thread needs its own stack and working set
- **Running inside a GIL-heavy Python workload** -- the GIL is released during Rust processing, but other Python threads may contend

```python
# Disable parallelism for small workloads
tools = jt.JSONTools().flatten().parallel_threshold(999_999)

# Or limit threads
tools = jt.JSONTools().flatten().num_threads(1)
```

### Profiling Tips

Use the built-in benchmark suites to profile your specific workload pattern:

```bash
# Profile stress scenarios
cargo bench --profile profiling --bench stress_benchmarks --no-run
samply record --save-only -o /tmp/profile.json -- \
    ./target/profiling/deps/stress_benchmarks-* --bench
```

For Python profiling, measure wall-clock time since CPU profilers may not capture time spent in Rust:

```python
import time
start = time.perf_counter()
result = tools.execute(data)
elapsed = time.perf_counter() - start
print(f"Processing took {elapsed:.3f}s")
```

## Platform Notes

### mimalloc (Rust-only)

The `mimalloc` global allocator is an optional feature that provides a 5-10% performance improvement. Enable it with `features = ["mimalloc"]` in your `Cargo.toml`. It is **not included in Python builds** because PyO3 manages memory through Python's allocator.

### sonic-rs (64-bit only)

The main flatten/unflatten/normal-mode paths use an in-tree, tape-based scanner (`scan_and_fixup()`, shared across those three operations) rather than a general-purpose `serde_json`-style parser. `sonic-rs` (SIMD, 64-bit platforms) / `simd-json` (32-bit fallback) is used for a narrower case: a root-level JSON primitive (e.g. a bare `"hello"` or `42`, not an object/array). This is transparent either way -- the public API is identical regardless of which parser handles a given input.

### macOS Profiling

On macOS, `flamegraph` requires full Xcode (not just Command Line Tools). Use `samply` instead:

```bash
cargo install samply
samply record --save-only -o profile.json -- ./target/profiling/deps/BENCH_BINARY --bench
samply load profile.json  # Opens Firefox Profiler
```

Valgrind does not work on modern macOS. Use Instruments (if Xcode is installed) or samply for profiling.
