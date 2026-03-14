# Parallel Processing

JSON Tools RS uses Crossbeam-based parallelism to automatically speed up batch operations and large nested structures.

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

Large individual JSON objects/arrays can also be parallelized:

```rust
let result = JSONTools::new()
    .flatten()
    .nested_parallel_threshold(200)  // Parallelize objects with 200+ entries
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

- **Batch parallelism**: Input is split into chunks, each processed by a separate thread via `crossbeam::thread::scope`. Results are written to pre-allocated slots preserving input order.
- **Nested parallelism**: Large JSON objects (many keys) or arrays (many elements) are split across threads for parallel flattening, then merged.
- **Thread safety**: All parallelism uses scoped threads -- no `'static` bounds required, no data races possible.

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

Environment variables take effect when `JSONTools::new()` is called. Builder method calls (e.g., `.parallel_threshold(n)`) override them.
