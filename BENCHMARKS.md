# JSON-Tools-rs Comprehensive Benchmark Suite

## Overview

The JSON-Tools-rs benchmark suite is designed to comprehensively test all features both in **isolation** and in **combination** to identify performance characteristics and interaction effects. The suite includes real-world scenarios and stress tests to ensure robust performance across diverse use cases.

## Benchmark Categories

### 1. **Isolation Benchmarks** (`isolation_benchmarks.rs`)

Tests each feature individually to measure its specific performance impact without interference from other features.

#### Benchmark Groups:

| Group | Description | Purpose |
|-------|-------------|---------|
| `iso_01_baseline` | Basic flatten/unflatten with no transformations | Establish performance baseline |
| `iso_02_separator_only` | Different separator configurations | Measure separator overhead |
| `iso_03_lowercase_only` | Lowercase key transformation | Measure lowercase conversion cost |
| `iso_04_key_replacement_only` | Key replacement (literal vs regex) | Compare literal vs regex performance |
| `iso_05_value_replacement_only` | Value replacement (literal vs regex) | Measure value transformation cost |
| `iso_06_filters_individual` | Each filter tested separately | Identify most/least expensive filters |
| `iso_07_auto_type_conversion_only` | Type conversion feature alone | Measure type parsing overhead |
| `iso_08_key_collision_only` | Collision handling feature | Measure collision resolution cost |
| `iso_09_normal_mode` | Normal mode (no flatten/unflatten) | Test transformations without flattening |
| `iso_10_unflatten_only` | Unflatten operations | Measure unflatten performance |

**Key Insights:**
- Identify which individual features have the highest overhead
- Understand the cost of literal vs regex replacements
- Determine filter ordering for optimal performance
- Measure baseline performance for comparison

**Run Command:**
```bash
cargo bench --bench isolation_benchmarks
```

---

### 2. **Combination Benchmarks** (`combination_benchmarks.rs`)

Tests systematic combinations of 2-3 features to identify interaction effects and compound costs.

#### 2-Feature Combinations:

| Group | Features | Purpose |
|-------|----------|---------|
| `combo_2f_01_separator_lowercase` | Separator + Lowercase | Test key transformation interactions |
| `combo_2f_02_lowercase_key_replacement` | Lowercase + Key Replacement | Measure combined key transformations |
| `combo_2f_03_key_value_replacement` | Key + Value Replacement | Test dual replacement overhead |
| `combo_2f_04_filters_auto_convert` | Filters + Auto Type Conversion | Measure filter+conversion cost |
| `combo_2f_05_all_filters` | All 4 filters together | Identify filter interaction effects |

#### 3-Feature Combinations:

| Group | Features | Purpose |
|-------|----------|---------|
| `combo_3f_01_sep_lower_keyrep` | Separator + Lowercase + Key Replacement | Complete key transformation pipeline |
| `combo_3f_02_filters_convert_collision` | Filters + Auto Convert + Collision Handling | Data cleanup + collision handling |
| `combo_3f_03_key_value_filters` | Key Transform + Value Transform + Filters | Full transformation pipeline |

#### Maximum Combination:

| Group | Features | Purpose |
|-------|----------|---------|
| `combo_max_all_features` | All features enabled | Worst-case performance scenario |

**Key Insights:**
- Identify non-linear performance effects (where 2 features cost more than the sum)
- Find optimal feature combinations
- Understand interaction between key and value transformations
- Measure compound overhead

**Run Command:**
```bash
cargo bench --bench combination_benchmarks
```

---

### 3. **Real-World Benchmarks** (`realworld_benchmarks.rs`)

Tests with actual API response formats and log structures from popular services.

#### Datasets:

| Benchmark | Source | Characteristics | Typical Use Case |
|-----------|--------|-----------------|------------------|
| `realworld_01_aws_cloudtrail` | AWS CloudTrail logs | Deep nesting, many nulls | Log processing, security analysis |
| `realworld_02_github_api` | GitHub Repository API | snake_case keys, nested objects | API integration, data sync |
| `realworld_03_kubernetes` | K8s Pod manifest | Labels, annotations, resources | Infrastructure monitoring |
| `realworld_04_elasticsearch` | Elasticsearch document | _source prefix, metadata | Log aggregation, search |
| `realworld_05_stripe_api` | Stripe Payment Intent | Nested charges, billing info | Payment processing, analytics |
| `realworld_06_twitter_api` | Twitter/X Tweet | Public metrics, entities | Social media analytics |

**Each benchmark includes:**
- Basic flatten
- Typical processing for that data source
- Common transformations and extractions

**Key Insights:**
- Validate performance with real-world data structures
- Identify common transformation patterns
- Optimize for actual use cases
- Test with varying data complexity

**Run Command:**
```bash
cargo bench --bench realworld_benchmarks
```

---

### 4. **Stress Benchmarks** (`stress_benchmarks.rs`)

Tests edge cases, extreme scenarios, and pathological inputs.

#### Stress Test Categories:

| Benchmark | Scenario | Parameters | Purpose |
|-----------|----------|------------|---------|
| `stress_01_deep_nesting` | Deeply nested objects | 10, 25, 50, 100 levels | Test recursion limits, stack usage |
| `stress_02_wide_objects` | Objects with many keys | 100, 500, 1K, 5K keys | Test HashMap performance |
| `stress_03_large_arrays` | Large arrays | 100, 500, 1K, 5K items | Test array flattening performance |
| `stress_04_unicode_heavy` | Unicode characters | Emoji, CJK, special chars | Test string handling, UTF-8 |
| `stress_05_many_small_objects` | Many small objects | 1K, 5K, 10K objects | Test parallelization efficiency |
| `stress_06_mixed_types` | All JSON types mixed | Numbers, bools, nulls, arrays | Test type handling robustness |
| `stress_07_long_strings` | Very long string values | 5K+ characters | Test string allocation, copying |
| `stress_08_nulls_and_empties` | Many null/empty values | High ratio of empty data | Test filter performance |
| `stress_09_regex_heavy` | Many regex matches | 100+ matches per pattern | Test regex cache effectiveness |
| `stress_10_parallel_thresholds` | Parallel processing tuning | Various thresholds | Optimize parallel settings |

**Key Insights:**
- Identify performance cliffs
- Validate error handling and edge cases
- Optimize for extreme scenarios
- Test parallel processing effectiveness

**Run Command:**
```bash
cargo bench --bench stress_benchmarks
```

---

### 5. **Comprehensive Benchmarks** (`comprehensive_benchmark.rs`)

The original comprehensive benchmark suite with 15 groups covering all features.

**Run Command:**
```bash
cargo bench --bench comprehensive_benchmark
```

---

## Running Benchmarks

### Quick Start

```bash
# Run all benchmarks
./scripts/run_benchmarks.sh --all

# Run specific suite
cargo bench --bench isolation_benchmarks
cargo bench --bench combination_benchmarks
cargo bench --bench realworld_benchmarks
cargo bench --bench stress_benchmarks

# Run specific benchmark group
cargo bench --bench isolation_benchmarks -- iso_03_lowercase
```

### Using the Benchmark Runner Script

The `scripts/run_benchmarks.sh` script provides convenient access to all benchmark suites:

```bash
# Run all benchmarks
./scripts/run_benchmarks.sh --all

# Run specific suites
./scripts/run_benchmarks.sh --isolation
./scripts/run_benchmarks.sh --combination
./scripts/run_benchmarks.sh --realworld
./scripts/run_benchmarks.sh --stress

# Quick mode (shorter measurement time)
./scripts/run_benchmarks.sh --all --quick

# Save results as baseline
./scripts/run_benchmarks.sh --all --baseline

# Compare with another branch
./scripts/run_benchmarks.sh --all --compare main
```

---

## Benchmark Results Analysis

### Understanding Output

Criterion outputs results in this format:

```
benchmark_name          time:   [XX.XXX µs XX.XXX µs XX.XXX µs]
                        change: [-X.XX% +X.XX% +X.XX%] (p = X.XX < 0.05)
```

- **time**: [lower bound, estimate, upper bound] with 95% confidence
- **change**: Performance change vs previous run (if available)
- **p-value**: Statistical significance (< 0.05 indicates significant change)

### Performance Metrics to Track

1. **Throughput**: Operations per millisecond
2. **Latency**: Time per operation
3. **Scalability**: How performance scales with input size
4. **Regression**: Changes compared to baseline/previous runs

### What to Look For

#### 🔴 Performance Regressions
- Time increase > 5%
- Significant p-value (< 0.05)
- Non-linear scaling with input size

#### 🟢 Optimization Opportunities
- Features with disproportionate overhead
- Non-linear combination effects
- Inefficient parallel thresholds

#### 🟡 Interaction Effects
- 2+ features combined cost more than sum of individual costs
- Filters that interact poorly with transformations
- Regex cache misses

---

## Performance Optimization Workflow

### 1. Establish Baseline
```bash
./scripts/run_benchmarks.sh --all --baseline
```

### 2. Make Changes
Edit source code, apply optimizations, etc.

### 3. Run Targeted Benchmarks
```bash
# If optimizing key transformations
cargo bench --bench isolation_benchmarks -- iso_04_key_replacement

# If optimizing filters
cargo bench --bench combination_benchmarks -- combo_2f_04_filters
```

### 4. Compare Results
```bash
# Compare with baseline
./scripts/run_benchmarks.sh --isolation

# Compare with another branch
./scripts/run_benchmarks.sh --isolation --compare main
```

### 5. Validate with Real-World Data
```bash
cargo bench --bench realworld_benchmarks
```

### 6. Stress Test
```bash
cargo bench --bench stress_benchmarks
```

---

## Benchmark Design Principles

### Isolation Testing
- Each feature tested independently
- No interference from other features
- Establishes individual cost baseline
- Enables targeted optimization

### Combination Testing
- Systematic 2-way and 3-way combinations
- Identifies interaction effects
- Tests real-world usage patterns
- Validates compound performance

### Real-World Validation
- Uses actual API response structures
- Tests common transformation patterns
- Validates practical performance
- Ensures production readiness

### Stress Testing
- Tests edge cases and limits
- Validates error handling
- Identifies performance cliffs
- Ensures robustness

---

## Benchmark Maintenance

### Adding New Benchmarks

1. **For new features**: Add to isolation benchmarks first
2. **For feature interactions**: Add to combination benchmarks
3. **For new use cases**: Add to real-world benchmarks
4. **For edge cases**: Add to stress benchmarks

### Benchmark Hygiene

- Keep benchmarks focused and minimal
- Use `black_box()` to prevent compiler optimizations
- Use consistent test data across related benchmarks
- Document expected performance characteristics

---

## Performance Targets

Based on v0.7.0 baseline:

| Operation | Target | Notes |
|-----------|--------|-------|
| Basic flatten (medium) | > 2,000 ops/ms | Without transformations |
| With transformations | > 1,300 ops/ms | All features enabled |
| Regex replacements | > 1,800 ops/ms | With caching |
| Batch processing (10) | > 2,500 ops/ms | Parallel speedup |
| Batch processing (100) | > 3,000 ops/ms | Better parallelization |
| Roundtrip | > 1,000 cycles/ms | Flatten + unflatten |

---

## CI/CD Integration

### Regression Detection

Add to CI pipeline:

```bash
# Run quick benchmarks
./scripts/run_benchmarks.sh --all --quick

# Compare with main branch
./scripts/run_benchmarks.sh --all --compare main

# Fail if regression > 10%
# (Implementation depends on CI system)
```

### Performance Tracking

- Save benchmark results as artifacts
- Track performance over time
- Alert on significant regressions
- Generate performance trends

---

## Tips and Best Practices

### 🎯 Focus Areas

1. **Hot Paths**: Benchmark frequently-used code paths
2. **Bottlenecks**: Identify and optimize slowest operations
3. **Scalability**: Test with varying input sizes
4. **Real-World**: Validate with actual use cases

### ⚡ Optimization Guidelines

1. **Profile First**: Use benchmarks to identify bottlenecks
2. **Measure Impact**: Benchmark before and after changes
3. **Validate Gains**: Ensure improvements are statistically significant
4. **Test Edge Cases**: Don't optimize only for the common case

### 📊 Interpreting Results

1. **Statistical Significance**: p < 0.05 indicates real change
2. **Confidence Intervals**: Narrow intervals indicate stable results
3. **Variance**: High variance suggests unstable benchmark
4. **Outliers**: Investigate unusual results

---

## Advanced Usage

### Comparing Specific Configurations

```bash
# Compare different parallel thresholds
cargo bench --bench stress_benchmarks -- stress_10_parallel_thresholds
```

### Generating Flame Graphs

```bash
# Requires cargo-flamegraph
cargo flamegraph --bench isolation_benchmarks -- --bench
```

### Memory Profiling

```bash
# Requires valgrind and cargo-valgrind
cargo valgrind --bench isolation_benchmarks
```

---

## Troubleshooting

### Benchmark Fails to Run

1. Check that Cargo.toml includes the benchmark
2. Ensure criterion is in dev-dependencies
3. Verify benchmark harness = false

### Unstable Results

1. Close other applications
2. Disable CPU frequency scaling
3. Use longer measurement times
4. Run multiple iterations

### Performance Regression

1. Compare with baseline
2. Run specific benchmark group
3. Profile to identify bottleneck
4. Validate with real-world benchmarks

---

## Future Enhancements

- [ ] Memory allocation profiling
- [ ] CPU cache performance analysis
- [ ] Automated regression detection
- [ ] Performance trend visualization
- [ ] Comparison with alternative libraries
- [ ] SIMD effectiveness benchmarks
- [ ] Regex cache hit rate tracking

---

## Questions?

For benchmark-related questions or suggestions:
- Open an issue: https://github.com/amaye15/json-tools-rs/issues
- Review existing benchmarks in `benches/` directory
- Check CI/CD benchmark results
