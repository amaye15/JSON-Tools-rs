use json_tools_rs::{flatten_json, JsonOutput};

fn main() {
    println!("JSON Tools RS - Flatten JSON Example\n");

    // Example 1: Basic flattening
    println!("Example 1: Basic flattening");
    let json1 = r#"{"user": {"profile": {"name": "John", "age": 30}, "settings": {"theme": "dark"}}}"#;
    match flatten_json(json1, false, false, false, false, None, None, None, false) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json1, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 2: Removing empty values
    println!("Example 2: Removing empty values");
    let json2 = r#"{"user": {"name": "John", "details": {"age": null, "city": "", "tags": []}}}"#;
    match flatten_json(json2, true, true, false, true, None, None, None, false) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json2, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 3: Array flattening
    println!("Example 3: Array flattening");
    let json3 = r#"{"items": [{"id": 1, "name": "Item 1"}, {"id": 2, "name": "Item 2"}]}"#;
    match flatten_json(json3, false, false, false, false, None, None, None, false) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json3, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 4: Key replacement with regex (new tuple format)
    println!("Example 4: Key replacement with regex (new tuple format)");
    let json4 = r#"{"user_name": "John", "user_email": "john@example.com", "admin_role": "super"}"#;
    let key_replacements = Some(vec![("regex:^(user|admin)_".to_string(), "".to_string())]);
    match flatten_json(json4, false, false, false, false, key_replacements, None, None, false) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json4, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 5: Value replacement with regex (new tuple format)
    println!("Example 5: Value replacement with regex (new tuple format)");
    let json5 = r#"{"email": "user@example.com", "backup_email": "admin@example.com"}"#;
    let value_replacements = Some(vec![("regex:@example\\.com".to_string(), "@company.org".to_string())]);
    match flatten_json(json5, false, false, false, false, None, value_replacements, None, false) {
        Ok(JsonOutput::Single(result)) => println!("Input:  {}\nOutput: {}\n", json5, result),
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 6: Complex nested structure with all features (new tuple format)
    println!("Example 6: Complex example with all features (new tuple format)");
    let json6 = r#"{"api_response": {"user_data": {"user_name": "John", "user_email": "john@example.com", "metadata": {"last_login": null, "preferences": {"theme": "", "notifications": []}}}, "status_code": 200}}"#;
    let key_replacements = Some(vec![
        ("user_".to_string(), "".to_string()),
        ("api_".to_string(), "".to_string()),
    ]);
    let value_replacements = Some(vec![("@example.com".to_string(), "@company.org".to_string())]);
    match flatten_json(json6, true, true, true, true, key_replacements, value_replacements, None, false) {
        Ok(JsonOutput::Single(result)) => {
            println!("Input:  {}", json6);
            println!("Output: {}", result);
            println!("Features used: remove empty strings, remove nulls, remove empty objects, remove empty arrays, key replacement, value replacement\n");
        },
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 7: Unified function with single JSON (backward compatible)
    println!("Example 7: Unified function with single JSON");
    let json7 = r#"{"api": {"version": "1.0", "endpoints": {"users": "/api/users", "orders": "/api/orders"}}}"#;
    match flatten_json(json7, false, false, false, false, None, None, None, false) {
        Ok(JsonOutput::Single(result)) => {
            println!("Input:  {}", json7);
            println!("Output: {}\n", result);
        },
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 8: Batch processing multiple JSON strings
    println!("Example 8: Batch processing multiple JSON strings");
    let json_batch = vec![
        r#"{"user1": {"name": "Alice", "age": 25}}"#,
        r#"{"user2": {"name": "Bob", "age": 30}}"#,
        r#"{"user3": {"name": "Charlie", "age": 35}}"#,
    ];

    match flatten_json(&json_batch[..], false, false, false, false, None, None, None, false) {
        Ok(JsonOutput::Multiple(results)) => {
            println!("Batch processing {} JSON strings:", json_batch.len());
            for (i, result) in results.iter().enumerate() {
                println!("  Result {}: {}", i + 1, result);
            }
            println!();
        },
        Ok(JsonOutput::Single(_)) => println!("Unexpected single result\n"),
        Err(e) => eprintln!("Batch error: {}\n", e),
    }

    // Example 9: Unified function with multiple inputs and filtering
    println!("Example 9: Unified function with multiple inputs and filtering");
    let json_multi = vec![
        r#"{"product1": {"name": "Widget", "price": null, "category": ""}}"#,
        r#"{"product2": {"name": "Gadget", "price": 29.99, "category": "electronics"}}"#,
    ];

    match flatten_json(&json_multi[..], true, true, false, false, None, None, None, false) {
        Ok(JsonOutput::Multiple(results)) => {
            println!("Filtered batch processing:");
            for (i, result) in results.iter().enumerate() {
                println!("  Product {}: {}", i + 1, result);
            }
            println!();
        },
        Ok(JsonOutput::Single(_)) => println!("Unexpected single result\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 10: Custom separators
    println!("Example 10: Custom separators");
    let json10 = r#"{"user": {"profile": {"name": "John", "contacts": {"emails": ["john@work.com", "john@personal.com"]}}}}"#;

    // Using underscore separator
    match flatten_json(json10, false, false, false, false, None, None, Some("_"), false) {
        Ok(JsonOutput::Single(result)) => {
            println!("Input:  {}", json10);
            println!("Output (underscore): {}\n", result);
        },
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Using double colon separator
    match flatten_json(json10, false, false, false, false, None, None, Some("::"), false) {
        Ok(JsonOutput::Single(result)) => {
            println!("Output (double colon): {}\n", result);
        },
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 11: Advanced tuple-based replacement patterns
    println!("Example 11: Advanced tuple-based replacement patterns");
    let json11 = r#"{"session.pageTimesInMs.homepage": 1500, "session.pageTimesInMs.checkout": 2000, "user_profile": {"user_name": "John", "user_email": "john@example.com", "user_status": "inactive"}}"#;

    // Demonstrate the new tuple format with complex patterns
    let key_replacements = Some(vec![
        ("regex:session\\.pageTimesInMs\\.".to_string(), "session__pagetimesinms__".to_string()),
        ("regex:user_".to_string(), "".to_string()),
    ]);
    let value_replacements = Some(vec![
        ("regex:@example\\.com".to_string(), "@company.org".to_string()),
        ("inactive".to_string(), "disabled".to_string()),
    ]);

    match flatten_json(json11, false, false, false, false, key_replacements, value_replacements, None, false) {
        Ok(JsonOutput::Single(result)) => {
            println!("Input:  {}", json11);
            println!("Output: {}\n", result);
        },
        Ok(JsonOutput::Multiple(_)) => println!("Unexpected multiple results\n"),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    println!("All examples completed (including new tuple-based replacements)!");
}
