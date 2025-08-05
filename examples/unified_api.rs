use json_tools_rs::{JSONTools, JsonOutput};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 JSON Tools RS - Unified JSONTools API Examples");
    println!("==================================================\n");

    // Example 1: Basic flattening
    println!("1. Basic Flattening:");
    let json = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = JSONTools::new()
        .flatten()
        .execute(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 2: Basic unflattening
    println!("2. Basic Unflattening:");
    let flattened = r#"{"user.name": "John", "user.age": 30}"#;
    let result = JSONTools::new()
        .unflatten()
        .execute(flattened)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", flattened);
        println!("   Output: {}\n", output);
    }

    // Example 3: Advanced flattening with all features
    println!("3. Advanced Flattening (All Features):");
    let json = r#"{"user_profile": {"user_name": "John", "user_email": "john@example.com", "user_age": null, "user_bio": "", "user_tags": []}}"#;
    let result = JSONTools::new()
        .flatten()
        .separator("::")
        .lowercase_keys(true)
        .key_replacement("user_", "")
        .value_replacement("@example.com", "@company.org")
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .execute(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: custom separator, lowercase keys, replacements, filtering");
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 4: Advanced unflattening with transformations and filtering
    println!("4. Advanced Unflattening (With Transformations & Filtering):");
    let flattened = r#"{"PREFIX::USER::NAME": "John", "PREFIX::USER::EMAIL": "john@company.org", "PREFIX::USER::BIO": "", "PREFIX::USER::AGE": null, "PREFIX::USER::TAGS": []}"#;
    let result = JSONTools::new()
        .unflatten()
        .separator("::")
        .lowercase_keys(true)
        .key_replacement("prefix::", "")
        .value_replacement("@company.org", "@example.com")
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .execute(flattened)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: custom separator, lowercase keys, key/value replacements, filtering");
        println!("   Input:  {}", flattened);
        println!("   Output: {}\n", output);
    }

    // Example 5: Regex patterns
    println!("5. Regex Pattern Replacements:");
    let json = r#"{"user_name": "John", "admin_role": "super", "temp_data": "test"}"#;
    let result = JSONTools::new()
        .flatten()
        .key_replacement("regex:^(user|admin)_", "")
        .value_replacement("regex:^super$", "administrator")
        .execute(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: regex key and value replacements");
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 6: Unflattening with Filtering Only
    println!("6. Unflattening with Filtering Only:");
    let flattened_with_empties = r#"{"user.name": "John", "user.bio": "", "user.age": null, "user.profile": {}, "user.tags": [], "user.settings.theme": "dark"}"#;
    let result = JSONTools::new()
        .unflatten()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .execute(flattened_with_empties)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: filtering empty values from unflattened JSON");
        println!("   Input:  {}", flattened_with_empties);
        println!("   Output: {}\n", output);
    }

    // Example 7: Roundtrip demonstration
    println!("7. Roundtrip Demonstration:");
    let original = r#"{"user": {"name": "John", "age": 30}, "items": [1, 2, {"nested": "value"}]}"#;
    
    // Flatten
    let flattened_result = JSONTools::new()
        .flatten()
        .execute(original)?;
    let flattened = match flattened_result {
        JsonOutput::Single(f) => f,
        JsonOutput::Multiple(_) => return Err("Unexpected multiple results".into()),
    };
    
    // Unflatten
    let unflattened_result = JSONTools::new()
        .unflatten()
        .execute(&flattened)?;
    let unflattened = match unflattened_result {
        JsonOutput::Single(u) => u,
        JsonOutput::Multiple(_) => return Err("Unexpected multiple results".into()),
    };
    
    println!("   Original:   {}", original);
    println!("   Flattened:  {}", flattened);
    println!("   Unflattened: {}", unflattened);
    
    // Verify roundtrip
    let original_parsed: serde_json::Value = serde_json::from_str(original)?;
    let result_parsed: serde_json::Value = serde_json::from_str(&unflattened)?;
    if original_parsed == result_parsed {
        println!("   ✅ Roundtrip successful - data preserved!\n");
    } else {
        println!("   ❌ Roundtrip failed - data not preserved\n");
    }

    // Example 8: Key Collision Handling - Avoid Strategy
    println!("8. Key Collision Handling - Avoid Strategy:");
    let collision_json = r#"{"User_name": "John", "Admin_name": "Jane", "Guest_name": "Bob"}"#;
    let result = JSONTools::new()
        .flatten()
        .separator("::")
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .avoid_key_collision(true)
        .execute(collision_json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: key collision avoidance with index suffixes");
        println!("   Input:  {}", collision_json);
        println!("   Output: {}\n", output);
    }

    // Example 9: Key Collision Handling - Collect Strategy
    println!("9. Key Collision Handling - Collect Strategy:");
    let result = JSONTools::new()
        .flatten()
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .handle_key_collision(true)
        .execute(collision_json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: key collision handling by collecting values into arrays");
        println!("   Input:  {}", collision_json);
        println!("   Output: {}\n", output);
    }

    // Example 10: Key Collision with Filtering
    println!("10. Key Collision with Filtering:");
    let collision_with_empty = r#"{"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}"#;
    let result = JSONTools::new()
        .flatten()
        .key_replacement("regex:(User|Admin|Guest)_", "")
        .remove_empty_strings(true)
        .handle_key_collision(true)
        .execute(collision_with_empty)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: collision handling with filtering (empty strings removed)");
        println!("   Input:  {}", collision_with_empty);
        println!("   Output: {}\n", output);
    }

    // Example 11: Unflattening with Collision Handling
    println!("11. Unflattening with Collision Handling:");
    let flattened_collision = r#"{"name::0": "John", "name::1": "Jane", "name::2": "Bob"}"#;
    let result = JSONTools::new()
        .unflatten()
        .separator("::")
        .key_replacement("regex:name::\\d+", "user_name")
        .handle_key_collision(true)
        .execute(flattened_collision)?;
    if let JsonOutput::Single(output) = result {
        println!("   Features: unflattening with collision handling");
        println!("   Input:  {}", flattened_collision);
        println!("   Output: {}\n", output);
    }

    // Example 12: Batch processing
    println!("12. Batch Processing:");
    let json_list = vec![
        r#"{"user": {"name": "John"}}"#,
        r#"{"user": {"name": "Jane"}}"#,
        r#"{"user": {"name": "Bob"}}"#,
    ];
    
    let result = JSONTools::new()
        .flatten()
        .separator("_")
        .execute(json_list.as_slice())?;
    
    match result {
        JsonOutput::Multiple(results) => {
            println!("   Processed {} JSON objects:", results.len());
            for (i, output) in results.iter().enumerate() {
                println!("   [{}]: {}", i, output);
            }
            println!();
        }
        JsonOutput::Single(_) => return Err("Expected multiple results".into()),
    }

    // Example 13: Error handling
    println!("13. Error Handling:");
    println!("   Attempting to execute without setting operation mode...");
    let result = JSONTools::new().execute(r#"{"test": "data"}"#);
    match result {
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
    println!("  • Batch processing support");
    println!("  • Clean, readable, and maintainable code");

    Ok(())
}
