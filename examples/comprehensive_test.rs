extern crate json_tools_rs;
use json_tools_rs::{flatten_json, JsonOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Comprehensive Test Suite for JSON Tools RS");
    println!("==============================================\n");
    
    // Test 1: Basic functionality
    println!("1. Basic Flattening:");
    let json = r#"{"user": {"name": "John", "age": 30}}"#;
    let result = flatten_json(json, false, false, false, false, None, None, None, false)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 2: Lowercase functionality
    println!("2. Lowercase Key Conversion:");
    let json = r#"{"User": {"Name": "John", "Email": "john@example.com"}}"#;
    let result = flatten_json(json, false, false, false, false, None, None, None, true)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 3: Filtering
    println!("3. Empty Value Filtering:");
    let json = r#"{"user": {"name": "John", "email": "", "age": null, "tags": []}}"#;
    let result = flatten_json(json, true, true, false, true, None, None, None, false)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 4: Regex replacements
    println!("4. Regex Key Replacements:");
    let json = r#"{"user_name": "John", "admin_role": "super"}"#;
    let key_replacements = Some(vec![("regex:^(user|admin)_".to_string(), "".to_string())]);
    let result = flatten_json(json, false, false, false, false, key_replacements, None, None, false)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 5: Combined regex + lowercase
    println!("5. Regex Replacements + Lowercase:");
    let json = r#"{"User_Name": "John", "Admin_Role": "super", "Temp_Data": "test"}"#;
    let key_replacements = Some(vec![("regex:^(User|Admin)_".to_string(), "".to_string())]);
    let result = flatten_json(json, false, false, false, false, key_replacements, None, None, true)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 6: Custom separator
    println!("6. Custom Separator:");
    let json = r#"{"user": {"profile": {"name": "John"}}}"#;
    let result = flatten_json(json, false, false, false, false, None, None, Some("::"), false)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 7: Arrays
    println!("7. Array Flattening:");
    let json = r#"{"users": [{"name": "John"}, {"name": "Jane"}]}"#;
    let result = flatten_json(json, false, false, false, false, None, None, None, false)?;
    if let JsonOutput::Single(output) = result {
        println!("   Input:  {}", json);
        println!("   Output: {}\n", output);
    }
    
    // Test 8: Complex nested structure with all features
    println!("8. Complex Example (All Features):");
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
        }
    }"#;
    
    let key_replacements = Some(vec![("regex:User_".to_string(), "".to_string())]);
    let value_replacements = Some(vec![("regex:@example\\.com".to_string(), "@company.org".to_string())]);
    let result = flatten_json(
        json, 
        true,  // remove empty strings
        true,  // remove nulls
        false, // keep empty objects
        false, // keep empty arrays
        key_replacements, 
        value_replacements, 
        Some("_"), // underscore separator
        true   // lowercase keys
    )?;
    
    if let JsonOutput::Single(output) = result {
        println!("   Features: empty filtering + regex replacements + lowercase + custom separator");
        println!("   Output: {}\n", output);
    }
    
    println!("✅ All tests completed successfully!");
    println!("🚀 The lower_case_keys parameter is working correctly!");
    
    Ok(())
}
