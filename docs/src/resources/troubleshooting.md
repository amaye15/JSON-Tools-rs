# Troubleshooting

This guide covers common errors, their causes, and how to resolve them.

## Error Code Reference

All errors include a machine-readable code (`E001`-`E008`) at the start of the error message. Use these codes for programmatic error handling.

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

**Cause:** A key or value replacement pattern failed to compile as a regex.

**Common triggers:**
- Unescaped special regex characters (`.`, `*`, `+`, `?`, `(`, `)`, `[`, `]`)
- Unclosed groups or character classes
- Invalid backreferences

**Solution:**
```python
# Wrong -- unescaped dot matches any character
tools.key_replacement("user.name", "username")

# Correct -- escape the dot for literal matching
tools.key_replacement(r"user\.name", "username")

# Or use a simpler pattern that won't be misinterpreted
tools.key_replacement("user_name", "username")
```

Note: If regex compilation fails, the library automatically falls back to literal string matching. This error only surfaces when the pattern is syntactically broken (e.g., unclosed groups).

### E003: InvalidReplacementPattern

**Message:** `[E003] Invalid replacement pattern: ...`

**Cause:** The replacement pattern configuration is malformed.

**Solution:** Ensure replacement patterns are provided as `(find, replace)` pairs:

```python
# Correct usage
tools.key_replacement("find_pattern", "replacement")
tools.value_replacement("old_value", "new_value")
```

### E004: InvalidJsonStructure

**Message:** `[E004] Invalid JSON structure: ...`

**Cause:** The JSON is valid but not compatible with the requested operation.

**Common triggers:**
- Unflattening a JSON array (unflatten requires a flat object)
- Unflattening a non-flat object (nested values where flat keys are expected)

**Solution:**
```python
# Wrong -- unflatten expects a flat object, not an array
result = jt.JSONTools().unflatten().execute('[1, 2, 3]')

# Wrong -- unflatten expects flat keys
result = jt.JSONTools().unflatten().execute('{"a": {"b": 1}}')

# Correct -- flat object with dot-separated keys
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

This error also occurs if `num_threads` is set to `0`:

```python
# Wrong
tools = jt.JSONTools().flatten().num_threads(0)

# Correct
tools = jt.JSONTools().flatten().num_threads(1)    # At least 1
tools = jt.JSONTools().flatten()                    # Use default (CPU count)
```

### E006: BatchProcessingError

**Message:** `[E006] Batch processing failed at index {N}: ...`

**Cause:** One or more items in a batch failed to process. The error includes the index of the failing item and the underlying error.

**Solution:** Check the item at the reported index. The inner error (usually E001 or E004) describes what went wrong:

```python
try:
    results = tools.execute(batch_of_json)
except jt.JsonToolsError as e:
    msg = str(e)
    if "[E006]" in msg:
        # Extract the index from the message to find the bad item
        print(f"Batch error: {e}")
        # Fix or filter the problematic items and retry
```

### E007: InputValidationError

**Message:** `[E007] Input validation failed: ...`

**Cause:** The input type is not supported.

**Common triggers:**
- Passing an integer, float, or boolean directly
- Passing a non-JSON-string, non-dict type in a list
- Using `execute_to_output()` with a DataFrame or Series (use `execute()` instead)

**Solution:**
```python
# Wrong
result = tools.execute(42)
result = tools.execute([1, 2, 3])

# Correct
result = tools.execute('{"value": 42}')
result = tools.execute({"value": 42})
result = tools.execute(['{"a": 1}', '{"b": 2}'])
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

In Rust, an empty separator causes a **panic** (via `assert!`). In Python, it raises a `ValueError`.

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

### Regex Patterns in Replacements

Replacement patterns use standard regex syntax. Common pitfalls:

```python
# The dot matches ANY character -- "user.name" matches "username" too
tools.key_replacement("user.name", "id")

# Escape dots for literal matching
tools.key_replacement(r"user\.name", "id")

# Use anchors for precise matching
tools.key_replacement("^user_", "")       # Only at start of key
tools.key_replacement("_suffix$", "")      # Only at end of key
```

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

The `mimalloc` global allocator is enabled for the Rust library and provides a 5-10% performance improvement. It is **disabled for Python builds** because PyO3 manages memory through Python's allocator. This is handled automatically by Cargo feature flags -- no user action is required.

### sonic-rs (64-bit only)

The default JSON parser is `sonic-rs`, which uses SIMD instructions available on 64-bit platforms (x86_64, aarch64). On 32-bit platforms, the library automatically falls back to `simd-json`. This is transparent -- the API is identical regardless of which parser is active.

### macOS Profiling

On macOS, `flamegraph` requires full Xcode (not just Command Line Tools). Use `samply` instead:

```bash
cargo install samply
samply record --save-only -o profile.json -- ./target/profiling/deps/BENCH_BINARY --bench
samply load profile.json  # Opens Firefox Profiler
```

Valgrind does not work on modern macOS. Use Instruments (if Xcode is installed) or samply for profiling.
