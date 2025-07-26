use json_tools_rs::{JsonFlattener, JsonOutput};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== JSON Tools RS - Comprehensive JsonFlattener Examples ===\n");

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

    // Example 7: Lowercase keys
    lowercase_examples()?;

    // Example 8: Multiple JSON inputs
    multiple_json_example()?;

    // Example 9: Real-world example
    real_world_example()?;

    // Example 10: Complex combinations
    complex_combinations_example()?;

    Ok(())
}

fn basic_flattening_example() -> Result<(), Box<dyn Error>> {
    println!("1. Basic Flattening Examples");
    println!("============================");

    // Simple nested object
    let json1 = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = JsonFlattener::new().flatten(json1)?;
    println!("Input:  {}", json1);
    println!("Output: {}\n", extract_single(result));

    // Deeply nested object
    let json2 = r#"{"company": {"department": {"team": {"member": {"name": "Alice"}}}}}"#;
    let result = JsonFlattener::new().flatten(json2)?;
    println!("Input:  {}", json2);
    println!("Output: {}\n", extract_single(result));

    Ok(())
}

fn array_flattening_example() -> Result<(), Box<dyn Error>> {
    println!("2. Array and Matrix Flattening");
    println!("===============================");

    // Simple array
    let json1 = r#"{"items": [1, 2, {"nested": "value"}]}"#;
    let result = JsonFlattener::new().flatten(json1)?;
    println!("Array Input:  {}", json1);
    println!("Array Output: {}\n", extract_single(result));

    // Matrix (nested arrays)
    let json2 = r#"{"matrix": [[1, 2], [3, 4]]}"#;
    let result = JsonFlattener::new().flatten(json2)?;
    println!("Matrix Input:  {}", json2);
    println!("Matrix Output: {}\n", extract_single(result));

    // Mixed array with objects
    let json3 = r#"{"users": [{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]}"#;
    let result = JsonFlattener::new().flatten(json3)?;
    println!("Mixed Array Input:  {}", json3);
    println!("Mixed Array Output: {}\n", extract_single(result));

    Ok(())
}

fn custom_separator_example() -> Result<(), Box<dyn Error>> {
    println!("3. Custom Separator Examples");
    println!("=============================");

    let json = r#"{"user": {"profile": {"name": "John"}}}"#;

    // Default dot separator
    let result = JsonFlattener::new().flatten(json)?;
    println!("Default (.):       {}", extract_single(result));

    // Underscore separator
    let result = JsonFlattener::new()
        .separator("_")
        .flatten(json)?;
    println!("Underscore (_):    {}", extract_single(result));

    // Double colon separator
    let result = JsonFlattener::new()
        .separator("::")
        .flatten(json)?;
    println!("Double colon (::): {}", extract_single(result));

    // Pipe separator
    let result = JsonFlattener::new()
        .separator("|")
        .flatten(json)?;
    println!("Pipe (|):          {}\n", extract_single(result));

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
    let result = JsonFlattener::new().flatten(json)?;
    println!("No filtering: {}", extract_single(result));

    // Remove empty strings
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .flatten(json)?;
    println!("Remove empty strings: {}", extract_single(result));

    // Remove null values
    let result = JsonFlattener::new()
        .remove_nulls(true)
        .flatten(json)?;
    println!("Remove null values: {}", extract_single(result));

    // Remove empty objects
    let result = JsonFlattener::new()
        .remove_empty_objects(true)
        .flatten(json)?;
    println!("Remove empty objects: {}", extract_single(result));

    // Remove empty arrays
    let result = JsonFlattener::new()
        .remove_empty_arrays(true)
        .flatten(json)?;
    println!("Remove empty arrays: {}", extract_single(result));

    // All filtering enabled
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .flatten(json)?;
    println!("All filtering: {}\n", extract_single(result));

    Ok(())
}

fn replacement_examples() -> Result<(), Box<dyn Error>> {
    println!("5. Key and Value Replacement Examples");
    println!("======================================");

    // Key replacements
    let json1 = r#"{"user_name": "John", "user_email": "john@example.com", "admin_role": "super"}"#;
    let result = JsonFlattener::new()
        .key_replacement("user_", "person_")
        .flatten(json1)?;
    println!("Key replacement input:  {}", json1);
    println!("Key replacement output: {}\n", extract_single(result));

    // Value replacements
    let json2 = r#"{"email": "john@example.com", "backup_email": "admin@example.com"}"#;
    let result = JsonFlattener::new()
        .value_replacement("@example.com", "@company.org")
        .flatten(json2)?;
    println!("Value replacement input:  {}", json2);
    println!("Value replacement output: {}\n", extract_single(result));

    // Combined replacements
    let json3 = r#"{"user_email": "john@example.com", "admin_phone": "555-1234"}"#;
    let result = JsonFlattener::new()
        .key_replacement("user_", "person_")
        .value_replacement("@example.com", "@company.org")
        .flatten(json3)?;
    println!("Combined replacement input:  {}", json3);
    println!("Combined replacement output: {}\n", extract_single(result));

    Ok(())
}

fn regex_examples() -> Result<(), Box<dyn Error>> {
    println!("6. Regex Pattern Examples");
    println!("==========================");

    // Regex key replacement - remove prefixes
    let json1 = r#"{"user_name": "John", "admin_role": "super", "guest_access": "limited"}"#;
    let result = JsonFlattener::new()
        .key_replacement("regex:^(user|admin|guest)_", "")
        .flatten(json1)?;
    println!("Regex key input:  {}", json1);
    println!("Regex key output: {}\n", extract_single(result));

    // Regex value replacement - email domains
    let json2 = r#"{"email": "user@example.com", "backup": "admin@example.com", "support": "help@test.org"}"#;
    let result = JsonFlattener::new()
        .value_replacement("regex:@example\\.com", "@company.org")
        .flatten(json2)?;
    println!("Regex value input:  {}", json2);
    println!("Regex value output: {}\n", extract_single(result));

    // Complex regex with capture groups
    let json3 = r#"{"field_123_name": "John", "field_456_email": "john@example.com"}"#;
    let result = JsonFlattener::new()
        .key_replacement("regex:^field_(\\d+)_(.+)", "$2_id_$1")
        .flatten(json3)?;
    println!("Capture groups input:  {}", json3);
    println!("Capture groups output: {}\n", extract_single(result));

    // Phone number formatting
    let json4 = r#"{"phone": "+1-555-123-4567", "fax": "+1-555-987-6543"}"#;
    let result = JsonFlattener::new()
        .value_replacement("regex:\\+(\\d)-(\\d{3})-(\\d{3})-(\\d{4})", "($2) $3-$4")
        .flatten(json4)?;
    println!("Phone format input:  {}", json4);
    println!("Phone format output: {}\n", extract_single(result));

    Ok(())
}

fn lowercase_examples() -> Result<(), Box<dyn Error>> {
    println!("7. Lowercase Key Examples");
    println!("==========================");

    let json = r#"{"User": {"Name": "John", "Email": "john@example.com", "Profile": {"Age": 30, "City": "NYC"}}}"#;

    // Without lowercase conversion
    let result = JsonFlattener::new().flatten(json)?;
    println!("Without lowercase: {}", extract_single(result));

    // With lowercase conversion
    let result = JsonFlattener::new()
        .lowercase_keys(true)
        .flatten(json)?;
    println!("With lowercase: {}", extract_single(result));

    // Lowercase with regex replacement
    let json2 = r#"{"User_Name": "John", "Admin_Role": "super", "Temp_Data": "test"}"#;
    let result = JsonFlattener::new()
        .key_replacement("regex:^(User|Admin)_", "")
        .lowercase_keys(true)
        .flatten(json2)?;
    println!("Regex + lowercase input:  {}", json2);
    println!("Regex + lowercase output: {}\n", extract_single(result));

    Ok(())
}

fn multiple_json_example() -> Result<(), Box<dyn Error>> {
    println!("8. Multiple JSON Inputs");
    println!("=======================");

    let json_list = [r#"{"user1": {"name": "Alice", "age": 25}}"#,
        r#"{"user2": {"name": "Bob", "age": 30}}"#,
        r#"{"user3": {"name": "Charlie", "age": 35}}"#];

    let result = JsonFlattener::new().flatten(&json_list[..])?;

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

fn real_world_example() -> Result<(), Box<dyn Error>> {
    println!("9. Real-World Example: E-commerce Product");
    println!("===========================================");

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
    let result = JsonFlattener::new().flatten(product_json)?;
    println!("\nBasic flattening:");
    println!("{}", extract_single(result));

    // With filtering and key simplification
    let result = JsonFlattener::new()
        .remove_nulls(true)
        .remove_empty_objects(true)
        .remove_empty_arrays(true)
        .key_replacement("product.", "")
        .flatten(product_json)?;
    println!("\nWith filtering and simplified keys:");
    println!("{}", extract_single(result));

    Ok(())
}

fn complex_combinations_example() -> Result<(), Box<dyn Error>> {
    println!("10. Complex Feature Combinations");
    println!("=================================");

    let json = r#"{
        "User_Profile": {
            "User_Name": "John Doe",
            "User_Email": "john@example.com",
            "User_Settings": {
                "Theme": "dark",
                "Language": "",
                "Notifications": null
            },
            "User_History": [
                {"Action": "login", "Timestamp": "2023-01-01"},
                {"Action": "logout", "Timestamp": "2023-01-02"}
            ]
        },
        "Admin_Data": {
            "Admin_Role": "super",
            "Admin_Permissions": ["read", "write", "delete"]
        }
    }"#;

    // All features combined
    let result = JsonFlattener::new()
        .remove_empty_strings(true)
        .remove_nulls(true)
        .key_replacement("regex:(User|Admin)_", "")
        .value_replacement("regex:@example\\.com", "@company.org")
        .separator("_")
        .lowercase_keys(true)
        .flatten(json)?;

    println!("Features: empty filtering + regex replacements + lowercase + custom separator");
    println!("Output: {}\n", extract_single(result));

    println!("✅ All examples completed successfully!");
    println!("🚀 The JsonFlattener builder pattern provides a clean, fluent API!");

    Ok(())
}

// Helper function to extract single result from JsonOutput
fn extract_single(result: JsonOutput) -> String {
    match result {
        JsonOutput::Single(s) => s,
        JsonOutput::Multiple(_) => panic!("Expected single result"),
    }
}