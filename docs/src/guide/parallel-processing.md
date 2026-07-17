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
