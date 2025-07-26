extern crate json_tools_rs;
use json_tools_rs::{flatten_json_with_params, JsonOutput};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Performance test for lowercase key functionality:");

    // Create a large JSON with mixed case keys
    let json = r#"{
        "User": {
            "Name": "John",
            "Email": "john@example.com",
            "Profile": {
                "Age": 30,
                "City": "NYC",
                "Preferences": {
                    "Theme": "dark",
                    "Language": "en",
                    "Notifications": {
                        "Email": true,
                        "SMS": false,
                        "Push": true
                    }
                }
            },
            "History": [
                {"Action": "login", "Timestamp": "2023-01-01"},
                {"Action": "purchase", "Timestamp": "2023-01-02"},
                {"Action": "logout", "Timestamp": "2023-01-03"}
            ]
        },
        "Admin": {
            "Role": "super",
            "Permissions": ["read", "write", "delete"],
            "LastLogin": "2023-01-01"
        }
    }"#;

    let iterations = 10000;

    // Test without lowercase conversion
    println!("\nTesting WITHOUT lowercase conversion:");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = flatten_json_with_params(json, false, false, false, false, None, None, None, false)?;
    }
    let duration_without = start.elapsed();
    println!(
        "Time: {:?} ({:.2} ops/sec)",
        duration_without,
        iterations as f64 / duration_without.as_secs_f64()
    );

    // Test with lowercase conversion
    println!("\nTesting WITH lowercase conversion:");
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = flatten_json_with_params(json, false, false, false, false, None, None, None, true)?;
    }
    let duration_with = start.elapsed();
    println!(
        "Time: {:?} ({:.2} ops/sec)",
        duration_with,
        iterations as f64 / duration_with.as_secs_f64()
    );

    // Calculate overhead
    let overhead = duration_with.as_nanos() as f64 / duration_without.as_nanos() as f64;
    println!(
        "\nPerformance overhead: {:.2}x ({:.1}% increase)",
        overhead,
        (overhead - 1.0) * 100.0
    );

    // Show sample output
    println!("\nSample outputs:");

    let result_without = flatten_json_with_params(json, false, false, false, false, None, None, None, false)?;
    if let JsonOutput::Single(output) = result_without {
        println!("\nWithout lowercase (first 3 keys):");
        let parsed: serde_json::Value = serde_json::from_str(&output)?;
        for (i, (key, value)) in parsed.as_object().unwrap().iter().enumerate() {
            if i < 3 {
                println!("  {}: {}", key, value);
            }
        }
    }

    let result_with = flatten_json_with_params(json, false, false, false, false, None, None, None, true)?;
    if let JsonOutput::Single(output) = result_with {
        println!("\nWith lowercase (first 3 keys):");
        let parsed: serde_json::Value = serde_json::from_str(&output)?;
        for (i, (key, value)) in parsed.as_object().unwrap().iter().enumerate() {
            if i < 3 {
                println!("  {}: {}", key, value);
            }
        }
    }

    Ok(())
}
