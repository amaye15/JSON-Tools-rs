use json_tools_rs::{flatten_json, JsonFlattener, JsonOutput};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("JSON Tools RS - JsonFlattener Builder Pattern Examples");
    println!("======================================================\n");

    basic_examples()?;
    filtering_examples()?;
    replacement_examples()?;
    advanced_examples()?;

    Ok(())
}

fn basic_examples() -> Result<(), Box<dyn Error>> {
    println!("1. Basic Flattening Examples");
    println!("=============================");

    // Simple function for basic flattening
    let json1 = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = flatten_json(json1)?;
    println!("Simple API:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // JsonFlattener for the same result
    let result = JsonFlattener::new().flatten(json1)?;
    println!("JsonFlattener API:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Custom separator
    let result = JsonFlattener::new()
        .separator("_")
        .flatten(json1)?;
    println!("Custom separator:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    Ok(())
}

fn filtering_examples() -> Result<(), Box<dyn Error>> {
    println!("2. Filtering Examples");
    println!("=====================");

    // Remove empty strings
    let json1 = r#"{"user": {"name": "John", "bio": "", "email": "john@example.com"}}"#;
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .flatten(json1)?;
    println!("Remove empty strings:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Remove null values
    let json2 = r#"{"user": {"name": "John", "age": null, "email": "john@example.com"}}"#;
    let result = JsonFlattener::new()
        .remove_nulls(true)
        .flatten(json2)?;
    println!("Remove nulls:");
    println!("Input:  {}", json2);
    println!("Output: {}\n", extract_single(result));

    // Remove empty arrays and objects
    let json3 = r#"{"user": {"name": "John", "tags": [], "metadata": {}}}"#;
    let result = JsonFlattener::new()
        .remove_empty_arrays(true)
        .remove_empty_objects(true)
        .flatten(json3)?;
    println!("Remove empty arrays/objects:");
    println!("Input:  {}", json3);
    println!("Output: {}\n", extract_single(result));

    // Combined filtering
    let json4 = r#"{"user": {"name": "John", "bio": "", "age": null, "tags": [], "active": true}}"#;
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .flatten(json4)?;
    println!("Combined filtering:");
    println!("Input:  {}", json4);
    println!("Output: {}\n", extract_single(result));

    Ok(())
}

fn replacement_examples() -> Result<(), Box<dyn Error>> {
    println!("3. Replacement Examples");
    println!("========================");

    // Key replacement
    let json1 = r#"{"user_name": "John", "user_email": "john@example.com", "user_age": 30}"#;
    let result = JsonFlattener::new()
        .key_replacement("user_", "")
        .flatten(json1)?;
    println!("Key replacement:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Value replacement
    let json2 = r#"{"user": {"email": "john@example.com", "backup": "john.doe@example.com"}}"#;
    let result = JsonFlattener::new()
        .value_replacement("@example.com", "@company.org")
        .flatten(json2)?;
    println!("Value replacement:");
    println!("Input:  {}", json2);
    println!("Output: {}\n", extract_single(result));

    // Regex key replacement
    let json3 = r#"{"user_name": "John", "admin_role": "super", "temp_data": "test"}"#;
    let result = JsonFlattener::new()
        .key_replacement("regex:^(user|admin)_", "")
        .flatten(json3)?;
    println!("Regex key replacement:");
    println!("Input:  {}", json3);
    println!("Output: {}\n", extract_single(result));

    // Multiple replacements
    let json4 = r#"{"user_name": "John", "user_email": "john@example.com"}"#;
    let result = JsonFlattener::new()
        .key_replacement("user_", "")
        .value_replacement("@example.com", "@company.org")
        .flatten(json4)?;
    println!("Multiple replacements:");
    println!("Input:  {}", json4);
    println!("Output: {}\n", extract_single(result));

    Ok(())
}

fn advanced_examples() -> Result<(), Box<dyn Error>> {
    println!("4. Advanced Examples");
    println!("====================");

    // Everything combined
    let json1 = r#"{"user_profile": {"user_name": "John", "user_email": "john@example.com", "user_bio": "", "user_age": null, "user_tags": []}}"#;
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_arrays(true)
        .key_replacement("user_", "")
        .value_replacement("@example.com", "@company.org")
        .separator("::")
        .lowercase_keys(true)
        .flatten(json1)?;
    println!("All features combined:");
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Batch processing
    let json_list = vec![
        r#"{"user": {"name": "Alice"}}"#,
        r#"{"user": {"name": "Bob"}}"#,
        r#"{"user": {"name": "Charlie"}}"#,
    ];
    let result = JsonFlattener::new()
        .separator("_")
        .flatten(&json_list[..])?;
    println!("Batch processing:");
    println!("Input:  {:?}", json_list);
    if let JsonOutput::Multiple(results) = result {
        println!("Output: {:?}\n", results);
    }

    println!("✅ All JsonFlattener examples completed successfully!");
    println!("\n🎯 Benefits of JsonFlattener:");
    println!("  • Fluent, chainable method calls");
    println!("  • Self-documenting configuration");
    println!("  • No parameter counting or ordering");
    println!("  • Easy to extend with new features");
    println!("  • Type-safe and compile-time checked");

    Ok(())
}

fn extract_single(output: JsonOutput) -> String {
    match output {
        JsonOutput::Single(s) => s,
        JsonOutput::Multiple(_) => "Multiple results".to_string(),
    }
}
