extern crate json_tools_rs;
use json_tools_rs::{flatten_json, JsonOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing lowercase key functionality:");
    
    let json = r#"{"User": {"Name": "John", "Email": "john@example.com", "Profile": {"Age": 30, "City": "NYC"}}}"#;
    
    // Test without lowercase conversion
    println!("\n1. Without lowercase conversion:");
    let result = flatten_json(json, false, false, false, false, None, None, None, false)?;
    match result {
        JsonOutput::Single(flattened) => {
            println!("Input:  {}", json);
            println!("Output: {}", flattened);
        }
        _ => unreachable!(),
    }
    
    // Test with lowercase conversion
    println!("\n2. With lowercase conversion:");
    let result = flatten_json(json, false, false, false, false, None, None, None, true)?;
    match result {
        JsonOutput::Single(flattened) => {
            println!("Input:  {}", json);
            println!("Output: {}", flattened);
        }
        _ => unreachable!(),
    }
    
    // Test with regex replacement AND lowercase conversion
    println!("\n3. With regex replacement AND lowercase conversion:");
    let json2 = r#"{"User_Name": "John", "Admin_Role": "super", "Temp_Data": "test"}"#;
    let key_replacements = Some(vec![("regex:^(User|Admin)_".to_string(), "".to_string())]);
    let result = flatten_json(json2, false, false, false, false, key_replacements, None, None, true)?;
    match result {
        JsonOutput::Single(flattened) => {
            println!("Input:  {}", json2);
            println!("Output: {}", flattened);
            println!("Note: 'User_' and 'Admin_' prefixes removed by regex, then keys lowercased");
        }
        _ => unreachable!(),
    }
    
    Ok(())
}
