use json_tools_rs::{JsonFlattener, JsonOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 JSON Tools RS - JsonFlattener Basic Usage Examples");
    println!("======================================================\n");

    // Example 1: Simple flattening with default settings
    println!("1. Simple Flattening:");
    let json = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = JsonFlattener::new().flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 2: Builder pattern with filtering
    println!("2. Filtering Empty Values:");
    let json = r#"{"user": {"name": "John", "email": "", "age": null, "tags": []}}"#;
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 3: Key and value replacements
    println!("3. Key and Value Replacements:");
    let json = r#"{"user_name": "John", "user_email": "john@example.com"}"#;
    let result = JsonFlattener::new()
        .key_replacement("user_", "person_")
        .value_replacement("@example.com", "@company.org")
        .flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 4: Custom separator
    println!("4. Custom Separator:");
    let json = r#"{"user": {"profile": {"name": "John"}}}"#;
    let result = JsonFlattener::new()
        .separator("_")
        .flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 5: Lowercase keys
    println!("5. Lowercase Key Conversion:");
    let json = r#"{"User": {"Name": "John", "Email": "john@example.com"}}"#;
    let result = JsonFlattener::new()
        .lowercase_keys(true)
        .flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 6: Regex replacements
    println!("6. Regex Key Replacements:");
    let json = r#"{"user_name": "John", "admin_role": "super"}"#;
    let result = JsonFlattener::new()
        .key_replacement("regex:^(user|admin)_", "")
        .flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 7: Array flattening
    println!("7. Array Flattening:");
    let json = r#"{"users": [{"name": "John"}, {"name": "Jane"}]}"#;
    let result = JsonFlattener::new().flatten(json)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }

    // Example 8: Multiple JSON strings
    println!("8. Multiple JSON Strings:");
    let json_list = vec![
        r#"{"user": {"name": "Alice"}}"#,
        r#"{"user": {"name": "Bob"}}"#,
        r#"{"user": {"name": "Charlie"}}"#,
    ];
    let result = JsonFlattener::new()
        .separator("_")
        .flatten(&json_list[..])?;
    if let JsonOutput::Multiple(results) = result {
        println!("   Input:  {:?}", json_list);
        println!("   Output: {:?}\n", results);
    }

    // Example 9: Everything combined
    println!("9. All Features Combined:");
    let json = r#"{
        "User_Profile": {
            "User_Name": "John Doe",
            "User_Email": "john@example.com",
            "User_Settings": {
                "Theme": "dark",
                "Language": "",
                "Notifications": null
            }
        }
    }"#;

    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .key_replacement("regex:User_", "")
        .value_replacement("regex:@example\\.com", "@company.org")
        .separator("::")
        .lowercase_keys(true)
        .flatten(json)?;

    if let JsonOutput::Single(output) = result {
        println!("   Features: empty filtering + regex replacements + lowercase + custom separator");
        println!("   Output: {}\n", output);
    }

    // Example 10: Error handling
    println!("10. Error Handling:");
    let invalid_json = r#"{"invalid": json}"#;
    match JsonFlattener::new().flatten(invalid_json) {
        Ok(_) => println!("   Unexpected success"),
        Err(e) => println!("   Error: {}\n", e),
    }

    println!("✅ All basic usage examples completed successfully!");
    println!("🚀 The JsonFlattener provides a clean, intuitive API!");

    println!("\n🎯 Key Benefits:");
    println!("  • Fluent, chainable method calls");
    println!("  • Self-documenting configuration");
    println!("  • No parameter counting or ordering");
    println!("  • Type-safe and compile-time checked");
    println!("  • High-performance with SIMD-accelerated parsing");

    Ok(())
}