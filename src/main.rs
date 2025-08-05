use json_tools_rs::{JsonOutput, JSONTools};

fn main() {
    println!("🚀 JSON Tools RS - Unified JSONTools API Examples");
    println!("==================================================\n");

    // Example 1: Basic flattening
    println!("1. Basic Flattening:");
    let json1 = r#"{"user": {"profile": {"name": "John", "age": 30}, "settings": {"theme": "dark"}}}"#;
    match JSONTools::new().flatten().execute(json1) {
        Ok(JsonOutput::Single(result)) => {
            println!("   Input:  {}", json1);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 2: Advanced flattening with all features
    println!("2. Advanced Flattening (All Features):");
    let json2 = r#"{"user_profile": {"user_name": "John", "user_email": "john@example.com", "user_age": null, "user_bio": "", "user_tags": []}}"#;
    match JSONTools::new()
        .flatten()
        .separator("::")
        .lowercase_keys(true)
        .key_replacement("user_", "")
        .value_replacement("@example.com", "@company.org")
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .execute(json2)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: custom separator, lowercase keys, replacements, filtering");
            println!("   Input:  {}", json2);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 3: Basic unflattening
    println!("3. Basic Unflattening:");
    let flattened1 = r#"{"user::profile::name": "John", "user::profile::age": 30, "user::settings::theme": "dark"}"#;
    match JSONTools::new()
        .unflatten()
        .separator("::")
        .execute(flattened1)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Input:  {}", flattened1);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 4: Advanced unflattening with transformations and filtering
    println!("4. Advanced Unflattening (With Transformations & Filtering):");
    let flattened2 = r#"{"PREFIX::USER::NAME": "John", "PREFIX::USER::EMAIL": "john@company.org", "PREFIX::USER::BIO": "", "PREFIX::USER::AGE": null, "PREFIX::USER::TAGS": []}"#;
    match JSONTools::new()
        .unflatten()
        .separator("::")
        .lowercase_keys(true)
        .key_replacement("prefix::", "")
        .value_replacement("@company.org", "@example.com")
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .execute(flattened2)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: custom separator, lowercase keys, key/value replacements, filtering");
            println!("   Input:  {}", flattened2);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 5: Regex patterns
    println!("5. Regex Pattern Replacements:");
    let json3 = r#"{"user_name": "John", "admin_role": "super", "temp_data": "test"}"#;
    match JSONTools::new()
        .flatten()
        .key_replacement("regex:^(user|admin)_", "")
        .value_replacement("regex:^super$", "administrator")
        .execute(json3)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: regex key and value replacements");
            println!("   Input:  {}", json3);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 6: Roundtrip demonstration
    println!("6. Roundtrip Demonstration:");
    let original = r#"{"user": {"name": "John", "age": 30}, "items": [1, 2, {"nested": "value"}]}"#;

    // First flatten
    match JSONTools::new().flatten().execute(original) {
        Ok(JsonOutput::Single(flattened)) => {
            println!("   Original:   {}", original);
            println!("   Flattened:  {}", flattened);

            // Then unflatten back
            match JSONTools::new().unflatten().execute(&flattened) {
                Ok(JsonOutput::Single(unflattened)) => {
                    println!("   Unflattened: {}", unflattened);

                    // Verify they're equivalent
                    let original_parsed: serde_json::Value = serde_json::from_str(original).unwrap();
                    let result_parsed: serde_json::Value = serde_json::from_str(&unflattened).unwrap();
                    if original_parsed == result_parsed {
                        println!("   ✅ Roundtrip successful - data preserved!\n");
                    } else {
                        println!("   ❌ Roundtrip failed - data not preserved\n");
                    }
                }
                Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
                Err(e) => eprintln!("   Unflatten Error: {}\n", e),
            }
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Flatten Error: {}\n", e),
    }

    // Example 5: Key Collision Handling - Avoid Strategy
    println!("5. Key Collision Handling - Avoid Strategy:");
    let collision_json = r#"{"User_name": "John", "Admin_name": "Jane", "Guest_name": "Bob"}"#;
    match JSONTools::new()
        .flatten()
        .separator("::")
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .avoid_key_collision(true)
        .execute(collision_json)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: key collision avoidance with index suffixes");
            println!("   Input:  {}", collision_json);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 6: Key Collision Handling - Collect Strategy
    println!("6. Key Collision Handling - Collect Strategy:");
    match JSONTools::new()
        .flatten()
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .handle_key_collision(true)
        .execute(collision_json)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: key collision handling by collecting values into arrays");
            println!("   Input:  {}", collision_json);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 7: Key Collision with Filtering
    println!("7. Key Collision with Filtering:");
    let collision_with_empty = r#"{"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}"#;
    match JSONTools::new()
        .flatten()
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .remove_empty_strings(true)
        .handle_key_collision(true)
        .execute(collision_with_empty)
    {
        Ok(JsonOutput::Single(result)) => {
            println!("   Features: collision handling with filtering (empty strings removed)");
            println!("   Input:  {}", collision_with_empty);
            println!("   Output: {}\n", result);
        }
        Ok(JsonOutput::Multiple(_)) => println!("   Unexpected multiple results\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 8: Batch processing
    println!("8. Batch Processing:");
    let json_list = vec![
        r#"{"user": {"name": "John"}}"#,
        r#"{"user": {"name": "Jane"}}"#,
        r#"{"user": {"name": "Bob"}}"#,
    ];

    match JSONTools::new()
        .flatten()
        .separator("_")
        .execute(json_list.as_slice())
    {
        Ok(JsonOutput::Multiple(results)) => {
            println!("   Processed {} JSON objects:", results.len());
            for (i, output) in results.iter().enumerate() {
                println!("   [{}]: {}", i, output);
            }
            println!();
        }
        Ok(JsonOutput::Single(_)) => println!("   Unexpected single result\n"),
        Err(e) => eprintln!("   Error: {}\n", e),
    }

    // Example 8: Error handling
    println!("8. Error Handling:");
    println!("   Attempting to execute without setting operation mode...");
    match JSONTools::new().execute(r#"{"test": "data"}"#) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   ✅ Correctly caught error: {}\n", e),
    }

    println!("🎯 Benefits of the Unified JSONTools API:");
    println!("  • Single entry point for both flatten and unflatten operations");
    println!("  • Consistent builder pattern across all operations");
    println!("  • All configuration options available for both modes");
    println!("  • Type-safe and compile-time checked");
    println!("  • Perfect roundtrip compatibility");
    println!("  • Comprehensive error handling");
    println!("  • Advanced key collision handling strategies");
    println!("  • Intelligent filtering during collision resolution");
    println!("  • Batch processing support");
    println!("  • Clean, readable, and maintainable code");
}
