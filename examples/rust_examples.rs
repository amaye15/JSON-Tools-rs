use json_tools_rs::{flatten_json, FlattenError, JsonOutput};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== JSON Tools RS - Comprehensive Examples ===\n");

    // Example 1: Basic flattening
    basic_flattening_example()?;

    // Example 2: Array and matrix flattening
    array_flattening_example()?;

    // Example 3: Custom separators
    custom_separator_example()?;

    // Example 4: Filtering options
    filtering_examples()?;

    // Example 5: Key and value replacements
    replacement_examples()?;

    // Example 6: Regex patterns
    regex_examples()?;

    // Example 7: Multiple JSON inputs
    multiple_json_example()?;

    // Example 8: Error handling
    error_handling_example();

    // Example 9: Real-world example
    real_world_example()?;

    Ok(())
}

fn basic_flattening_example() -> Result<(), Box<dyn Error>> {
    println!("1. Basic Flattening Examples");
    println!("============================");

    // Simple nested object
    let json1 = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = flatten_json(json1, false, false, false, false, None, None, None, false)?;
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Deeply nested object
    let json2 = r#"{"company": {"department": {"team": {"member": {"name": "Alice"}}}}}"#;
    let result = flatten_json(json2, false, false, false, false, None, None, None, false)?;
    println!("Input:  {}", json2);
    println!("Output: {}\n", extract_single(result));

    Ok(())
}

fn array_flattening_example() -> Result<(), Box<dyn Error>> {
    println!("2. Array and Matrix Flattening");
    println!("===============================");

    // Simple array
    let json1 = r#"{"items": [1, 2, {"nested": "value"}]}"#;
    let result = flatten_json(json1, false, false, false, false, None, None, None, false)?;
    println!("Array Input:  {}", json1);
    println!("Array Output: {}\n", extract_single(result));

    // Matrix (nested arrays)
    let json2 = r#"{"matrix": [[1, 2], [3, 4]]}"#;
    let result = flatten_json(json2, false, false, false, false, None, None, None, false)?;
    println!("Matrix Input:  {}", json2);
    println!("Matrix Output: {}\n", extract_single(result));

    // Mixed array with objects
    let json3 = r#"{"users": [{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]}"#;
    let result = flatten_json(json3, false, false, false, false, None, None, None, false)?;
    println!("Mixed Array Input:  {}", json3);
    println!("Mixed Array Output: {}\n", extract_single(result));

    Ok(())
}

fn custom_separator_example() -> Result<(), Box<dyn Error>> {
    println!("3. Custom Separator Examples");
    println!("=============================");

    let json = r#"{"user": {"profile": {"name": "John"}}}"#;

    // Default dot separator
    let result = flatten_json(json, false, false, false, false, None, None, None, false)?;
    println!("Default (.):     {}", extract_single(result));

    // Underscore separator
    let result = flatten_json(
        json,
        false,
        false,
        false,
        false,
        None,
        None,
        Some("_"),
        false,
    )?;
    println!("Underscore (_):  {}", extract_single(result));

    // Double colon separator
    let result = flatten_json(
        json,
        false,
        false,
        false,
        false,
        None,
        None,
        Some("::"),
        false,
    )?;
    println!("Double colon (::): {}", extract_single(result));

    // Pipe separator
    let result = flatten_json(
        json,
        false,
        false,
        false,
        false,
        None,
        None,
        Some("|"),
        false,
    )?;
    println!("Pipe (|):        {}\n", extract_single(result));

    Ok(())
}

fn filtering_examples() -> Result<(), Box<dyn Error>> {
    println!("4. Filtering Options");
    println!("====================");

    let json = r#"{
        "user": {
            "name": "John",
            "email": "",
            "age": null,
            "preferences": {},
            "tags": [],
            "active": true
        },
        "metadata": {},
        "items": []
    }"#;

    println!("Original: {}", json.replace('\n', "").replace("  ", ""));

    // No filtering
    let result = flatten_json(json, false, false, false, false, None, None, None, false)?;
    println!("No filtering: {}", extract_single(result));

    // Remove empty strings
    let result = flatten_json(json, true, false, false, false, None, None, None, false)?;
    println!("Remove empty strings: {}", extract_single(result));

    // Remove null values
    let result = flatten_json(json, false, true, false, false, None, None, None, false)?;
    println!("Remove null values: {}", extract_single(result));

    // Remove empty objects
    let result = flatten_json(json, false, false, true, false, None, None, None, false)?;
    println!("Remove empty objects: {}", extract_single(result));

    // Remove empty arrays
    let result = flatten_json(json, false, false, false, true, None, None, None, false)?;
    println!("Remove empty arrays: {}", extract_single(result));

    // All filtering enabled
    let result = flatten_json(json, true, true, true, true, None, None, None, false)?;
    println!("All filtering: {}\n", extract_single(result));

    Ok(())
}

fn replacement_examples() -> Result<(), Box<dyn Error>> {
    println!("5. Key and Value Replacement Examples");
    println!("======================================");

    // Key replacements
    let json1 = r#"{"user_name": "John", "user_email": "john@example.com", "admin_role": "super"}"#;
    let key_replacements = Some(vec![("user_".to_string(), "person_".to_string())]);
    let result = flatten_json(
        json1,
        false,
        false,
        false,
        false,
        key_replacements,
        None,
        None,
        false,
    )?;
    println!("Key replacement input:  {}", json1);
    println!("Key replacement output: {}\n", extract_single(result));

    // Value replacements
    let json2 = r#"{"email": "john@example.com", "backup_email": "admin@example.com"}"#;
    let value_replacements = Some(vec![(
        "@example.com".to_string(),
        "@company.org".to_string(),
    )]);
    let result = flatten_json(
        json2,
        false,
        false,
        false,
        false,
        None,
        value_replacements,
        None,
        false,
    )?;
    println!("Value replacement input:  {}", json2);
    println!("Value replacement output: {}\n", extract_single(result));

    // Combined replacements
    let json3 = r#"{"user_email": "john@example.com", "admin_phone": "555-1234"}"#;
    let key_replacements = Some(vec![("user_".to_string(), "person_".to_string())]);
    let value_replacements = Some(vec![(
        "@example.com".to_string(),
        "@company.org".to_string(),
    )]);
    let result = flatten_json(
        json3,
        false,
        false,
        false,
        false,
        key_replacements,
        value_replacements,
        None,
        false,
    )?;
    println!("Combined replacement input:  {}", json3);
    println!("Combined replacement output: {}\n", extract_single(result));

    Ok(())
}

fn regex_examples() -> Result<(), Box<dyn Error>> {
    println!("6. Regex Pattern Examples");
    println!("==========================");

    // Regex key replacement - remove prefixes
    let json1 = r#"{"user_name": "John", "admin_role": "super", "guest_access": "limited"}"#;
    let key_replacements = Some(vec![(
        "regex:^(user|admin|guest)_".to_string(),
        "".to_string(),
    )]);
    let result = flatten_json(
        json1,
        false,
        false,
        false,
        false,
        key_replacements,
        None,
        None,
        false,
    )?;
    println!("Regex key input:  {}", json1);
    println!("Regex key output: {}\n", extract_single(result));

    // Regex value replacement - email domains
    let json2 = r#"{"email": "user@example.com", "backup": "admin@example.com", "support": "help@test.org"}"#;
    let value_replacements = Some(vec![(
        "regex:@example\\.com".to_string(),
        "@company.org".to_string(),
    )]);
    let result = flatten_json(
        json2,
        false,
        false,
        false,
        false,
        None,
        value_replacements,
        None,
        false,
    )?;
    println!("Regex value input:  {}", json2);
    println!("Regex value output: {}\n", extract_single(result));

    // Complex regex with capture groups
    let json3 = r#"{"field_123_name": "John", "field_456_email": "john@example.com"}"#;
    let key_replacements = Some(vec![(
        "regex:^field_(\\d+)_(.+)".to_string(),
        "$2_id_$1".to_string(),
    )]);
    let result = flatten_json(
        json3,
        false,
        false,
        false,
        false,
        key_replacements,
        None,
        None,
        false,
    )?;
    println!("Capture groups input:  {}", json3);
    println!("Capture groups output: {}\n", extract_single(result));

    // Phone number formatting
    let json4 = r#"{"phone": "+1-555-123-4567", "fax": "+1-555-987-6543"}"#;
    let value_replacements = Some(vec![(
        "regex:\\+(\\d)-(\\d{3})-(\\d{3})-(\\d{4})".to_string(),
        "($2) $3-$4".to_string(),
    )]);
    let result = flatten_json(
        json4,
        false,
        false,
        false,
        false,
        None,
        value_replacements,
        None,
        false,
    )?;
    println!("Phone format input:  {}", json4);
    println!("Phone format output: {}\n", extract_single(result));

    Ok(())
}

fn multiple_json_example() -> Result<(), Box<dyn Error>> {
    println!("7. Multiple JSON Inputs");
    println!("=======================");

    let json_list = vec![
        r#"{"user1": {"name": "Alice", "age": 25}}"#,
        r#"{"user2": {"name": "Bob", "age": 30}}"#,
        r#"{"user3": {"name": "Charlie", "age": 35}}"#,
    ];

    let result = flatten_json(
        &json_list[..],
        false,
        false,
        false,
        false,
        None,
        None,
        None,
        false,
    )?;

    match result {
        JsonOutput::Multiple(results) => {
            println!("Processing {} JSON strings:", results.len());
            for (i, flattened) in results.iter().enumerate() {
                println!("  Result {}: {}", i + 1, flattened);
            }
        }
        JsonOutput::Single(_) => unreachable!(),
    }
    println!();

    Ok(())
}

fn error_handling_example() {
    println!("8. Error Handling Examples");
    println!("===========================");

    // Invalid JSON
    let invalid_json = r#"{"invalid": json}"#;
    match flatten_json(
        invalid_json,
        false,
        false,
        false,
        false,
        None,
        None,
        None,
        false,
    ) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
                match flatten_err {
                    FlattenError::JsonParseError(json_err) => {
                        println!("JSON parse error: {}", json_err);
                    }
                    _ => println!("Other flatten error: {}", flatten_err),
                }
            }
        }
    }

    // Invalid regex pattern
    let key_replacements = Some(vec![(
        "regex:[invalid".to_string(),
        "replacement".to_string(),
    )]);
    match flatten_json(
        r#"{"test": "value"}"#,
        false,
        false,
        false,
        false,
        key_replacements,
        None,
        None,
        false,
    ) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            if let Some(flatten_err) = e.downcast_ref::<FlattenError>() {
                match flatten_err {
                    FlattenError::RegexError(regex_err) => {
                        println!("Regex error: {}", regex_err);
                    }
                    _ => println!("Other flatten error: {}", flatten_err),
                }
            }
        }
    }
    println!();
}

fn real_world_example() -> Result<(), Box<dyn Error>> {
    println!("9. Real-World Example: E-commerce Product");
    println!("==========================================");

    let product_json = r#"{
        "product": {
            "id": 12345,
            "name": "Gaming Laptop",
            "details": {
                "brand": "TechCorp",
                "model": "Pro-X1",
                "specs": {
                    "cpu": "Intel i7",
                    "ram": "16GB",
                    "storage": "512GB SSD"
                }
            },
            "pricing": {
                "base_price": 999.99,
                "discount": null,
                "final_price": 999.99
            },
            "availability": {
                "in_stock": true,
                "quantity": 50,
                "warehouses": ["NYC", "LA", "CHI"]
            },
            "metadata": {},
            "tags": []
        }
    }"#;

    println!("Original product data (truncated for display):");
    println!("{{\"product\": {{\"id\": 12345, \"name\": \"Gaming Laptop\", ...}}}}");

    // Basic flattening
    let result = flatten_json(
        product_json,
        false,
        false,
        false,
        false,
        None,
        None,
        None,
        false,
    )?;
    println!("\nBasic flattening:");
    println!("{}", extract_single(result));

    // With filtering and key simplification
    let key_replacements = Some(vec![("product.".to_string(), "".to_string())]);
    let result = flatten_json(
        product_json,
        false,
        true,
        true,
        true,
        key_replacements,
        None,
        None,
        false,
    )?;
    println!("\nWith filtering and simplified keys:");
    println!("{}", extract_single(result));

    Ok(())
}

// Helper function to extract single result from JsonOutput
fn extract_single(result: JsonOutput) -> String {
    match result {
        JsonOutput::Single(s) => s,
        JsonOutput::Multiple(_) => panic!("Expected single result"),
    }
}
