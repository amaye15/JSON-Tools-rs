# Parallel Processing

JSON Tools RS uses Rayon-based parallelism to automatically speed up batch operations and large nested structures.

## Automatic Parallelism

Batch processing (100+ items by default) automatically uses parallel execution:

```rust
let batch: Vec<&str> = large_json_collection;
let result = JSONTools::new()
    .flatten()
    .execute(batch.as_slice())?;
// Automatically parallelized
```

```python
batch = [{"data": i} for i in range(2000)]
results = jt.JSONTools().flatten().execute(batch)
# Automatically parallelized
```

## Configuration

### Batch Threshold

Control the minimum batch size before parallelism kicks in:

```rust
let result = JSONTools::new()
    .flatten()
    .parallel_threshold(50)  // Only parallelize batches of 50+ items
    .execute(batch.as_slice())?;
```

### Thread Count

Limit the number of threads used:

```rust
let result = JSONTools::new()
    .flatten()
    .num_threads(Some(4))  // Use 4 threads (default: CPU count)
    .execute(batch.as_slice())?;
```

### Nested Parallelism

A single large JSON document being flattened can also be parallelized, based on how
many direct children the *root* object/array has (nested containers deeper inside
the document don't independently trigger this -- only the root's own fan-out is
counted). This currently applies only to `.flatten()` -- `.unflatten()` and
`.normal()` have no nested-parallel path and are unaffected by
`nested_parallel_threshold`, though all three modes share the batch-level
parallelism above.

```rust
let result = JSONTools::new()
    .flatten()
    .nested_parallel_threshold(200)  // Parallelize when the root has MORE than 200 direct children
    .execute(large_json)?;
```

## Python Configuration

```python
tools = (jt.JSONTools()
    .flatten()
    .parallel_threshold(50)
    .num_threads(4)
    .nested_parallel_threshold(200)
)

results = tools.execute(large_batch)
```

## How It Works

- **Batch parallelism**: Input is split into chunks processed via Rayon's `par_chunks`. By default this runs on Rayon's persistent, process-wide work-stealing pool (no per-call thread spawn cost); setting `.num_threads(Some(n))` instead builds and installs a dedicated pool sized to `n` for that call. Results preserve input order. Applies to `.flatten()`, `.unflatten()`, and `.normal()` alike.
- **Nested parallelism**: A `.flatten()` call on a single document whose root object/array has more than `nested_parallel_threshold` direct children splits those children across threads for parallel flattening, then merges the results.
- **Thread safety**: Rayon's work-stealing model requires no `'static` bounds and guarantees no data races.

## Environment Variables

All parallelism settings can be overridden via environment variables (applied at construction time):

| Variable | Default | Description |
|----------|---------|-------------|
| `JSON_TOOLS_PARALLEL_THRESHOLD` | `100` | Minimum batch size to trigger parallel processing |
| `JSON_TOOLS_NESTED_PARALLEL_THRESHOLD` | `100` | Minimum object/array size for nested parallelism |
| `JSON_TOOLS_NUM_THREADS` | CPU count | Number of threads for parallel processing |
| `JSON_TOOLS_MAX_ARRAY_INDEX` | `100000` | Maximum array index during unflattening (DoS protection) |

```bash
export JSON_TOOLS_PARALLEL_THRESHOLD=50
export JSON_TOOLS_NESTED_PARALLEL_THRESHOLD=200
export JSON_TOOLS_NUM_THREADS=4
export JSON_TOOLS_MAX_ARRAY_INDEX=500000
```

Environment variables are read once per process -- at the first `JSONTools::new()`
call anywhere in the program, not on every call -- and the resulting defaults are
cached for the rest of the process's lifetime. Set them before your program starts;
changing them at runtime (e.g. via `std::env::set_var`) after the first `JSONTools`
has already been constructed has no effect. Builder method calls (e.g.,
`.parallel_threshold(n)`) always override the compiled-in default on a
per-instance basis, regardless of when the environment variable was read.

## Examples

### Easy: let the default threshold decide

```python
import json_tools_rs as jt

batch = [{"user": {"id": i}} for i in range(250)]
results = jt.JSONTools().flatten().execute(batch)
# 250 items, well over the default threshold of 100 -- parallelized automatically,
# no configuration needed. Order matches the input order.
```

### Medium: tune the threshold and thread count together

```python
batch = [{"user": {"id": i, "score": str(i * 1.5)}} for i in range(500)]

tools = (jt.JSONTools()
    .flatten()
    .parallel_threshold(50)   # parallelize batches of 50+ (lower than the default 100)
    .num_threads(4)           # cap at 4 threads for this call
    .auto_convert_types(True)
)
results = tools.execute(batch)
```

`.num_threads(Some(n))` builds a dedicated Rayon pool sized to `n` just for this call;
leave it unset to reuse the process-wide work-stealing pool (no per-call spawn cost).

### Hard: batch + nested parallelism + env-configured limits together

A single very wide document (many direct children at the root) and a large batch of
documents can both be parallelized at once -- they're independent mechanisms that
stack:

```rust
use json_tools_rs::JSONTools;

// Each document in the batch has 300 top-level keys; the batch itself has 1000 items.
let batch: Vec<String> = generate_wide_documents(1000, 300);
let batch_refs: Vec<&str> = batch.iter().map(String::as_str).collect();

let result = JSONTools::new()
    .flatten()
    .parallel_threshold(100)          // batch-level: split 1000 items across threads
    .nested_parallel_threshold(200)   // per-document: each doc's 300 root keys (> 200) also fan out
    .num_threads(Some(8))
    .execute(batch_refs.as_slice())?;
```

Set `JSON_TOOLS_MAX_ARRAY_INDEX` lower than its 100,000 default in an environment that
processes untrusted input, to bound how large an array `.unflatten()` will build from a
numeric key like `"items.99999.name"` before treating it as suspicious rather than
allocating a 100k-element array per document.
