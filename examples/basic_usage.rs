use json_tools_rs::{flatten_json_with_params, JsonOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example from the requirements
    let json_input = r#"{"user": {"user_name": "John", "user_email": "john@example.com", "details": {"age": null, "city": ""}}}"#;

    println!("Original JSON:");
    println!("{}", json_input);

    // Example with new tuple-based replacement format
    let key_replacements = Some(vec![
        ("user_".to_string(), "person_".to_string()), // Replace user_ prefix with person_
    ]);
    let value_replacements = Some(vec![
        ("@example.com".to_string(), "@company.org".to_string()), // Replace email domain
    ]);

    let result = flatten_json_with_params(
        json_input,
        true,               // remove_empty_string_values
        true,               // remove_null_values
        false,              // remove_empty_dict
        false,              // remove_empty_list
        key_replacements,   // key_replacements (new intuitive tuple format)
        value_replacements, // value_replacements (new intuitive tuple format)
        None,               // separator (default ".")
        false,              // lower_case_keys
    )?;

    println!("\nFlattened JSON (with empty strings and null values removed):");
    match result {
        JsonOutput::Single(flattened) => println!("{}", flattened),
        JsonOutput::Multiple(_) => println!("Unexpected multiple results"),
    }

    // Expected output: {"user.person_name":"John","user.person_email":"john@company.org"}

    Ok(())
}
