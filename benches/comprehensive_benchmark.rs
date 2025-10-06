use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use json_tools_rs::{JSONTools, JsonOutput};
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Load all test files from the test_assets directory
fn load_test_files() -> Vec<(String, String)> {
    let test_dir = Path::new("test_assets");
    let mut files = Vec::new();
    
    if test_dir.exists() {
        for entry in fs::read_dir(test_dir).expect("Failed to read test_assets directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let content = fs::read_to_string(&path)
                    .expect(&format!("Failed to read file: {:?}", path));
                
                files.push((filename, content));
            }
        }
    }
    
    // Sort by filename for consistent ordering
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

/// Benchmark basic flattening operation
fn bench_flatten_basic(c: &mut Criterion) {
    let test_files = load_test_files();
    
    let mut group = c.benchmark_group("flatten_basic");
    group.measurement_time(Duration::from_secs(10));
    
    for (filename, content) in &test_files {
        group.bench_with_input(
            BenchmarkId::new("file", filename),
            content,
            |b, json_content| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .execute(black_box(json_content))
                        .expect("Flatten failed");
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark flattening with all transformations applied
fn bench_flatten_comprehensive(c: &mut Criterion) {
    let test_files = load_test_files();
    
    let mut group = c.benchmark_group("flatten_comprehensive");
    group.measurement_time(Duration::from_secs(15));
    
    for (filename, content) in &test_files {
        group.bench_with_input(
            BenchmarkId::new("file", filename),
            content,
            |b, json_content| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .separator(black_box("::"))
                        .lowercase_keys(true)
                        .remove_empty_strings(true)
                        .remove_nulls(true)
                        .remove_empty_objects(true)
                        .remove_empty_arrays(true)
                        .key_replacement("Count", "Cnt")
                        .key_replacement("Amount", "Amt")
                        .key_replacement("Address", "Addr")
                        .value_replacement("^$", "N/A")
                        .value_replacement("null", "NULL")
                        .handle_key_collision(true)
                        .execute(black_box(json_content))
                        .expect("Comprehensive flatten failed");
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark flattening with collision avoidance
fn bench_flatten_collision_avoidance(c: &mut Criterion) {
    let test_files = load_test_files();
    
    let mut group = c.benchmark_group("flatten_collision_handling");
    group.measurement_time(Duration::from_secs(10));

    for (filename, content) in &test_files {
        group.bench_with_input(
            BenchmarkId::new("file", filename),
            content,
            |b, json_content| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .separator(black_box("_"))
                        .key_replacement("(customer|transaction|billing)", "data")
                        .handle_key_collision(true)
                        .execute(black_box(json_content))
                        .expect("Collision handling flatten failed");
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark roundtrip operation (flatten then unflatten)
fn bench_roundtrip_basic(c: &mut Criterion) {
    let test_files = load_test_files();
    
    let mut group = c.benchmark_group("roundtrip_basic");
    group.measurement_time(Duration::from_secs(15));
    
    for (filename, content) in &test_files {
        group.bench_with_input(
            BenchmarkId::new("file", filename),
            content,
            |b, json_content| {
                b.iter(|| {
                    // Flatten
                    let flattened = JSONTools::new()
                        .flatten()
                        .execute(black_box(json_content))
                        .expect("Flatten failed");
                    
                    let flattened_str = match flattened {
                        JsonOutput::Single(s) => s,
                        JsonOutput::Multiple(_) => panic!("Unexpected multiple output"),
                    };
                    
                    // Unflatten
                    let result = JSONTools::new()
                        .unflatten()
                        .execute(black_box(&flattened_str))
                        .expect("Unflatten failed");
                    
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark roundtrip with comprehensive transformations
fn bench_roundtrip_comprehensive(c: &mut Criterion) {
    let test_files = load_test_files();
    
    let mut group = c.benchmark_group("roundtrip_comprehensive");
    group.measurement_time(Duration::from_secs(20));
    
    for (filename, content) in &test_files {
        group.bench_with_input(
            BenchmarkId::new("file", filename),
            content,
            |b, json_content| {
                b.iter(|| {
                    // Flatten with transformations
                    let flattened = JSONTools::new()
                        .flatten()
                        .separator(black_box("__"))
                        .lowercase_keys(true)
                        .remove_empty_strings(true)
                        .remove_nulls(true)
                        .key_replacement("aggregation", "agg")
                        .key_replacement("transaction", "txn")
                        .value_replacement("^0$", "zero")
                        .handle_key_collision(true)
                        .execute(black_box(json_content))
                        .expect("Comprehensive flatten failed");
                    
                    let flattened_str = match flattened {
                        JsonOutput::Single(s) => s,
                        JsonOutput::Multiple(_) => panic!("Unexpected multiple output"),
                    };
                    
                    // Unflatten with reverse transformations
                    let result = JSONTools::new()
                        .unflatten()
                        .separator(black_box("__"))
                        .key_replacement("agg", "aggregation")
                        .key_replacement("txn", "transaction")
                        .value_replacement("zero", "0")
                        .handle_key_collision(true)
                        .execute(black_box(&flattened_str))
                        .expect("Comprehensive unflatten failed");
                    
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark batch processing
fn bench_batch_processing(c: &mut Criterion) {
    let test_files = load_test_files();

    if test_files.is_empty() {
        println!("No test files found, skipping batch benchmark");
        return;
    }

    // Create batch of all test files as owned strings
    let batch: Vec<String> = test_files.iter().map(|(_, content)| content.clone()).collect();

    let mut group = c.benchmark_group("batch_processing");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("flatten_batch", |b| {
        b.iter(|| {
            // Convert to Vec<&str> for the API
            let batch_refs: Vec<&str> = batch.iter().map(|s| s.as_str()).collect();
            let result = JSONTools::new()
                .flatten()
                .separator(black_box("::"))
                .lowercase_keys(true)
                .remove_empty_strings(true)
                .handle_key_collision(true)
                .execute(black_box(batch_refs))
                .expect("Batch flatten failed");
            black_box(result);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_flatten_basic,
    bench_flatten_comprehensive,
    bench_flatten_collision_avoidance,
    bench_roundtrip_basic,
    bench_roundtrip_comprehensive,
    bench_batch_processing
);

criterion_main!(benches);
