use json_tools_rs::{JsonFlattener, JsonOutput};

fn main() {
    println!("JSON Tools RS - JsonFlattener Builder API Examples\n");

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

    println!("✅ All examples completed successfully!");
    println!("\n🎯 Benefits of the JsonFlattener Builder API:");
    println!("  • Fluent, chainable method calls");
    println!("  • Self-documenting configuration");
    println!("  • No parameter counting or ordering");
    println!("  • Easy to extend with new features");
    println!("  • Type-safe and compile-time checked");
}
