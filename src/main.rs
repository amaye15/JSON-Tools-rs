use json_tools_rs::{JsonFlattener, JsonUnflattener, JsonOutput};

fn main() {
    println!("JSON Tools RS - JsonFlattener & JsonUnflattener Builder API Examples\n");

    // Example 1: Basic flattening with JsonFlattener
    println!("Example 1: Basic flattening (JsonFlattener)");
    let json1 =
        r#"{"user": {"profile": {"name": "John", "age": 30}, "settings": {"theme": "dark"}}}"#;
    match JsonFlattener::new().flatten(json1) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json1, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 2: Using JsonFlattener builder pattern with filtering
    println!("Example 2: JsonFlattener Builder Pattern with Filtering");
    let json2 = r#"{"user": {"name": "John", "details": {"age": null, "city": "", "tags": []}}}"#;
    match JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .flatten(json2)
    {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json2, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 3: Advanced configuration with all features
    println!("Example 3: Advanced Configuration (All Features)");
    let json3 = r#"{"user_profile": {"user_name": "John", "user_email": "john@example.com", "user_age": null, "user_bio": "", "user_tags": []}}"#;
    match JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .key_replacement("user_", "")
        .value_replacement("@example.com", "@company.org")
        .separator("::")
        .lowercase_keys(true)
        .flatten(json3)
    {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json3, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 4: Regex patterns
    println!("Example 4: Regex Key Replacement");
    let json4 = r#"{"user_name": "John", "admin_role": "super", "temp_data": "test"}"#;
    match JsonFlattener::new()
        .key_replacement("regex:^(user|admin)_", "")
        .separator("_")
        .flatten(json4)
    {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json4, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 5: Basic unflattening with JsonUnflattener
    println!("Example 5: Basic unflattening (JsonUnflattener)");
    let flattened1 = r#"{"user.profile.name": "John", "user.profile.age": 30, "user.settings.theme": "dark"}"#;
    match JsonUnflattener::new().unflatten(flattened1) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", flattened1, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 6: Unflattening arrays
    println!("Example 6: Unflattening Arrays");
    let flattened2 = r#"{"users.0.name": "John", "users.0.age": 30, "users.1.name": "Jane", "users.1.age": 25, "items.0": "first", "items.1": "second"}"#;
    match JsonUnflattener::new().unflatten(flattened2) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", flattened2, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 7: Roundtrip - Flatten then Unflatten
    println!("Example 7: Roundtrip (Flatten → Unflatten)");
    let original = r#"{"user": {"name": "John", "age": 30}, "items": [1, 2, {"nested": "value"}]}"#;

    // First flatten
    let flattened_result = JsonFlattener::new().flatten(original);
    match flattened_result {
        Ok(JsonOutput::Single(flattened)) => {
            println!("Original: {}", original);
            println!("Flattened: {}", flattened);

            // Then unflatten
            match JsonUnflattener::new().unflatten(&flattened) {
                Ok(JsonOutput::Single(unflattened)) => {
                    println!("Unflattened: {}\n", unflattened);

                    // Verify they're equivalent
                    let original_parsed: serde_json::Value = serde_json::from_str(original).unwrap();
                    let result_parsed: serde_json::Value = serde_json::from_str(&unflattened).unwrap();
                    if original_parsed == result_parsed {
                        println!("✅ Roundtrip successful - original and result are identical!\n");
                    } else {
                        println!("❌ Roundtrip failed - original and result differ\n");
                    }
                }
                Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
                Err(e) => eprintln!("Unflatten error: {}\n", e),
            }
        }
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Flatten error: {}\n", e),
    }

    // Example 8: Advanced JsonUnflattener configuration
    println!("Example 8: Advanced JsonUnflattener Configuration");
    let flattened3 = r#"{"prefix_name": "John", "prefix_email": "john@company.org"}"#;
    match JsonUnflattener::new()
        .key_replacement("prefix_", "user.")
        .value_replacement("@company.org", "@example.com")
        .unflatten(flattened3)
    {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", flattened3, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    println!("✅ All examples completed successfully!");
    println!("\n🎯 Benefits of the JsonFlattener & JsonUnflattener Builder APIs:");
    println!("  • Fluent, chainable method calls");
    println!("  • Self-documenting configuration");
    println!("  • No parameter counting or ordering");
    println!("  • Easy to extend with new features");
    println!("  • Type-safe and compile-time checked");
    println!("  • Perfect roundtrip compatibility");
    println!("  • Unified API design for both operations");
}
