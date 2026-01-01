use json_tools_rs::{JSONTools, JsonOutput};
use serde_json::Value;

fn main() {
    println!("Testing Automatic Type Conversion Feature\n");
    println!("==========================================\n");

    // Test 1: Basic number conversion
    println!("Test 1: Basic Number Conversion");
    let json = r#"{"id": "123", "price": "45.67", "count": "-10"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["id"], 123);
        assert_eq!(parsed["price"], 45.67);
        assert_eq!(parsed["count"], -10);
    }

    // Test 2: Currency and thousands separators
    println!("Test 2: Currency and Thousands Separators");
    let json = r#"{"usd": "$1,234.56", "eur": "€999.99", "large": "1,000,000"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["usd"], 1234.56);
        assert_eq!(parsed["eur"], 999.99);
        assert_eq!(parsed["large"], 1000000);
    }

    // Test 3: Boolean conversion
    println!("Test 3: Boolean Conversion");
    let json = r#"{"a": "true", "b": "FALSE", "c": "True"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["a"], true);
        assert_eq!(parsed["b"], false);
        assert_eq!(parsed["c"], true);
    }

    // Test 4: Mixed conversion with invalid strings
    println!("Test 4: Mixed Conversion (keeps invalid strings)");
    let json = r#"{"id": "123", "name": "Alice", "active": "true", "code": "ABC"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["id"], 123);
        assert_eq!(parsed["name"], "Alice");
        assert_eq!(parsed["active"], true);
        assert_eq!(parsed["code"], "ABC");
    }

    // Test 5: Nested structures
    println!("Test 5: Nested Structures");
    let json = r#"{"user": {"id": "456", "age": "25", "verified": "true"}}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["user.id"], 456);
        assert_eq!(parsed["user.age"], 25);
        assert_eq!(parsed["user.verified"], true);
    }

    // Test 6: Conversion disabled (default)
    println!("Test 6: Conversion Disabled (default behavior)");
    let json = r#"{"id": "123", "active": "true"}"#;
    let result = JSONTools::new()
        .flatten()
        .execute(json)
        .unwrap();
    
    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);
        
        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["id"], "123");  // Still a string
        assert_eq!(parsed["active"], "true");  // Still a string
    }

    // Test 7: Scientific notation
    println!("Test 7: Scientific Notation");
    let json = r#"{"small": "1.23e-4", "large": "1e5"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["small"], 0.000123);
        assert_eq!(parsed["large"], 100000.0);
    }

    // Test 8: Extended boolean variants (yes/no, y/n, on/off)
    println!("Test 8: Extended Boolean Variants");
    let json = r#"{"yes": "yes", "no": "NO", "y": "y", "n": "N", "on": "on", "off": "OFF", "one": "1", "zero": "0"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["yes"], true);
        assert_eq!(parsed["no"], false);
        assert_eq!(parsed["y"], true);
        assert_eq!(parsed["n"], false);
        assert_eq!(parsed["on"], true);
        assert_eq!(parsed["off"], false);
        // "1" and "0" are now treated as numbers, not booleans
        assert_eq!(parsed["one"], 1);
        assert_eq!(parsed["zero"], 0);
    }

    // Test 9: Percentage strings
    println!("Test 9: Percentage Strings");
    let json = r#"{"discount": "50%", "commission": "12.5%", "complete": "100%"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["discount"], 50.0);
        assert_eq!(parsed["commission"], 12.5);
        assert_eq!(parsed["complete"], 100.0);
    }

    // Test 10: Null string variants
    println!("Test 10: Null String Variants");
    let json = r#"{"a": "null", "b": "NIL", "c": "none", "d": "N/A"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["a"], Value::Null);
        assert_eq!(parsed["b"], Value::Null);
        assert_eq!(parsed["c"], Value::Null);
        assert_eq!(parsed["d"], Value::Null);
    }

    // Test 11: Comprehensive mixed values
    println!("Test 11: Comprehensive Mixed Values");
    let json = r#"{"id": "123", "price": "$1,234.56", "discount": "15%", "active": "yes", "verified": "1", "premium": "FALSE", "status": "N/A", "name": "Product", "enabled": "on", "quantity": "5000"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["id"], 123);
        assert_eq!(parsed["price"], 1234.56);
        assert_eq!(parsed["discount"], 15.0);
        assert_eq!(parsed["active"], true);
        assert_eq!(parsed["verified"], 1);  // Now treated as number, not boolean
        assert_eq!(parsed["premium"], false);
        assert_eq!(parsed["status"], Value::Null);
        assert_eq!(parsed["name"], "Product");
        assert_eq!(parsed["enabled"], true);
        assert_eq!(parsed["quantity"], 5000);
    }

    // Test 12: Conversion priority (null → boolean → number)
    println!("Test 12: Conversion Priority");
    let json = r#"{"null_str": "null", "bool_str": "yes", "num_str": "100"}"#;
    let result = JSONTools::new()
        .flatten()
        .auto_convert_types(true)
        .execute(json)
        .unwrap();

    if let JsonOutput::Single(output) = result {
        println!("Input:  {}", json);
        println!("Output: {}\n", output);

        let parsed: Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["null_str"], Value::Null);  // null takes priority
        assert_eq!(parsed["bool_str"], true);         // boolean takes priority over number
        assert_eq!(parsed["num_str"], 100);           // number conversion last
    }

    println!("==========================================");
    println!("✅ All 12 tests passed!");
    println!("\nNew Features Demonstrated:");
    println!("  • Extended boolean variants (yes/no, y/n, on/off)");
    println!("  • Percentage string support (50% → 50.0)");
    println!("  • Null string variants (null, nil, none, N/A)");
    println!("  • Conversion priority: null → boolean → number");
    println!("  • Note: '1' and '0' are treated as numbers, not booleans");
}

