use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use json_tools_rs::JSONTools;
use std::hint::black_box;
use std::time::Duration;

// ============================================================================
// PART 1: COMPLETE ISOLATION BENCHMARKS
// Tests each feature individually to measure its specific performance impact
// ============================================================================

mod test_data {
    pub fn small_json() -> &'static str {
        r#"{
            "user": {
                "id": 12345,
                "name": "John Doe",
                "email": "john.doe@example.com",
                "profile": {
                    "age": 30,
                    "city": "New York",
                    "country": "USA",
                    "bio": "",
                    "website": null,
                    "preferences": {
                        "theme": "dark",
                        "language": "en",
                        "notifications": true
                    }
                },
                "tags": ["developer", "rust", "json"],
                "metadata": {}
            }
        }"#
    }

    pub fn medium_json() -> &'static str {
        r#"{
            "order": {
                "id": "ORD-2024-001234",
                "customer": {
                    "id": "CUST-98765",
                    "firstName": "Jane",
                    "lastName": "Smith",
                    "email": "jane.smith@example.com",
                    "phone": "+1-555-0123",
                    "address": {
                        "street": "123 Main St",
                        "city": "San Francisco",
                        "state": "CA",
                        "zipCode": "94102",
                        "country": "USA"
                    }
                },
                "items": [
                    {
                        "productId": "PROD-001",
                        "name": "Wireless Keyboard",
                        "price": "$59.99",
                        "quantity": "2",
                        "inStock": "true"
                    },
                    {
                        "productId": "PROD-002",
                        "name": "USB Mouse",
                        "price": "$29.99",
                        "quantity": "1",
                        "inStock": "true"
                    }
                ],
                "totals": {
                    "subtotal": "$171.96",
                    "tax": "$15.00",
                    "total": "$186.96"
                }
            }
        }"#
    }
}

// ============================================================================
// ISOLATION 1: Baseline Operations (no transformations)
// ============================================================================

fn iso_01_baseline_flatten(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_01_baseline");
    group.measurement_time(Duration::from_secs(3));

    let sizes = vec![
        ("small", test_data::small_json()),
        ("medium", test_data::medium_json()),
    ];

    for (size, json) in sizes {
        group.bench_with_input(
            BenchmarkId::new("flatten", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .execute(black_box(*json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// ISOLATION 2: Separator (only separator change, nothing else)
// ============================================================================

fn iso_02_separator_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_02_separator_only");
    group.measurement_time(Duration::from_secs(3));

    let separators = vec![
        ("dot", "."),
        ("double_colon", "::"),
        ("underscore", "_"),
        ("double_underscore", "__"),
        ("slash", "/"),
        ("arrow", "->"),
    ];

    let json = test_data::medium_json();

    for (name, sep) in separators {
        group.bench_with_input(
            BenchmarkId::new("separator", name),
            &sep,
            |b, &sep| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .separator(sep)
                        .execute(black_box(json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// ISOLATION 3: Lowercase Keys (only lowercase, no other transformations)
// ============================================================================

fn iso_03_lowercase_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_03_lowercase_only");
    group.measurement_time(Duration::from_secs(3));

    let sizes = vec![
        ("small", test_data::small_json()),
        ("medium", test_data::medium_json()),
    ];

    for (size, json) in sizes {
        // Baseline (no lowercase)
        group.bench_with_input(
            BenchmarkId::new("baseline", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .execute(black_box(*json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );

        // With lowercase
        group.bench_with_input(
            BenchmarkId::new("lowercase", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .lowercase_keys(true)
                        .execute(black_box(*json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// ISOLATION 4: Key Replacement (literal vs regex, isolation)
// ============================================================================

fn iso_04_key_replacement_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_04_key_replacement_only");
    group.measurement_time(Duration::from_secs(3));

    let json = test_data::medium_json();

    // Baseline
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Single literal replacement
    group.bench_function("literal_single", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("name", "fullName")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Multiple literal replacements
    group.bench_function("literal_multiple", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("name", "fullName")
                .key_replacement("email", "emailAddress")
                .key_replacement("phone", "phoneNumber")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Single regex replacement
    group.bench_function("regex_single", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("regex:(first|last)Name", "name")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Multiple regex replacements
    group.bench_function("regex_multiple", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("regex:(first|last)Name", "name")
                .key_replacement("regex:_id$", "Id")
                .key_replacement("regex:^product", "prod")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

// ============================================================================
// ISOLATION 5: Value Replacement (literal vs regex, isolation)
// ============================================================================

fn iso_05_value_replacement_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_05_value_replacement_only");
    group.measurement_time(Duration::from_secs(3));

    let json = test_data::medium_json();

    // Baseline
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Single literal replacement
    group.bench_function("literal_single", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .value_replacement("@example.com", "@company.org")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Multiple literal replacements
    group.bench_function("literal_multiple", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .value_replacement("@example.com", "@company.org")
                .value_replacement("USA", "United States")
                .value_replacement("CA", "California")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Single regex replacement
    group.bench_function("regex_single", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .value_replacement("regex:@example\\.com$", "@company.org")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Multiple regex replacements
    group.bench_function("regex_multiple", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .value_replacement("regex:@example\\.com$", "@company.org")
                .value_replacement("regex:^\\+1-555-", "+1-800-")
                .value_replacement("regex:^\\$", "USD ")
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

// ============================================================================
// ISOLATION 6: Individual Filters (each filter tested alone)
// ============================================================================

fn iso_06_filters_individual(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_06_filters_individual");
    group.measurement_time(Duration::from_secs(3));

    let json = test_data::small_json();

    // Baseline (no filters)
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Only remove_empty_strings
    group.bench_function("remove_empty_strings", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .remove_empty_strings(true)
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Only remove_nulls
    group.bench_function("remove_nulls", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .remove_nulls(true)
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Only remove_empty_objects
    group.bench_function("remove_empty_objects", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .remove_empty_objects(true)
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Only remove_empty_arrays
    group.bench_function("remove_empty_arrays", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .remove_empty_arrays(true)
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

// ============================================================================
// ISOLATION 7: Auto Type Conversion (only this feature)
// ============================================================================

fn iso_07_auto_type_conversion_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_07_auto_type_conversion_only");
    group.measurement_time(Duration::from_secs(3));

    let sizes = vec![
        ("small", test_data::small_json()),
        ("medium", test_data::medium_json()),
    ];

    for (size, json) in sizes {
        // Baseline (no conversion)
        group.bench_with_input(
            BenchmarkId::new("baseline", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .execute(black_box(*json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );

        // With auto conversion
        group.bench_with_input(
            BenchmarkId::new("auto_convert", size),
            &json,
            |b, json| {
                b.iter(|| {
                    let result = JSONTools::new()
                        .flatten()
                        .auto_convert_types(true)
                        .execute(black_box(*json))
                        .expect("Failed");
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// ISOLATION 8: Key Collision Handling (only this feature)
// ============================================================================

fn iso_08_key_collision_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_08_key_collision_only");
    group.measurement_time(Duration::from_secs(3));

    // JSON designed to create collisions
    let collision_json = r#"{
        "user_name": "John",
        "admin_name": "Jane",
        "guest_name": "Bob"
    }"#;

    // Baseline (no collision handling, will overwrite)
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("regex:(user|admin|guest)_", "")
                .execute(black_box(collision_json))
                .expect("Failed");
            black_box(result);
        });
    });

    // With collision handling (creates arrays)
    group.bench_function("collision_handling", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .flatten()
                .key_replacement("regex:(user|admin|guest)_", "")
                .handle_key_collision(true)
                .execute(black_box(collision_json))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

// ============================================================================
// ISOLATION 9: Normal Mode (no flatten/unflatten)
// ============================================================================

fn iso_09_normal_mode(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_09_normal_mode");
    group.measurement_time(Duration::from_secs(3));

    let json = test_data::medium_json();

    // Normal mode with no transformations
    group.bench_function("baseline", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .normal()
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    // Normal mode with transformations
    group.bench_function("with_transformations", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .normal()
                .value_replacement("@example.com", "@company.org")
                .remove_empty_strings(true)
                .remove_nulls(true)
                .execute(black_box(json))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

// ============================================================================
// ISOLATION 10: Unflatten (isolated from flatten)
// ============================================================================

fn iso_10_unflatten_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("iso_10_unflatten_only");
    group.measurement_time(Duration::from_secs(3));

    let flattened = r#"{
        "user.id": 12345,
        "user.name": "John Doe",
        "user.email": "john.doe@example.com",
        "user.profile.age": 30,
        "user.profile.city": "New York",
        "user.tags.0": "developer",
        "user.tags.1": "rust"
    }"#;

    // Basic unflatten
    group.bench_function("basic", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .unflatten()
                .execute(black_box(flattened))
                .expect("Failed");
            black_box(result);
        });
    });

    // Unflatten with custom separator
    let flattened_custom = r#"{
        "user::id": 12345,
        "user::name": "John Doe",
        "user::profile::age": 30
    }"#;

    group.bench_function("custom_separator", |b| {
        b.iter(|| {
            let result = JSONTools::new()
                .unflatten()
                .separator("::")
                .execute(black_box(flattened_custom))
                .expect("Failed");
            black_box(result);
        });
    });

    group.finish();
}

criterion_group!(
    isolation_benches,
    iso_01_baseline_flatten,
    iso_02_separator_only,
    iso_03_lowercase_only,
    iso_04_key_replacement_only,
    iso_05_value_replacement_only,
    iso_06_filters_individual,
    iso_07_auto_type_conversion_only,
    iso_08_key_collision_only,
    iso_09_normal_mode,
    iso_10_unflatten_only,
);

criterion_main!(isolation_benches);
