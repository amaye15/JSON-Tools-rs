#!/usr/bin/env python3
"""
JSON Tools RS Python Example

This example demonstrates the unified JsonFlattener API in Python.
"""

import json_tools_rs

def main():
    print("JSON Tools RS - Python Example")
    print("Perfect Type Matching: Input Type = Output Type!")
    print("=" * 50)
    
    # Example 1: Basic flattening
    print("\n1. Basic Flattening")
    print("-" * 20)
    
    # Now you can pass Python dicts directly - no need to serialize to JSON!
    json_data = {
        "user": {
            "profile": {
                "name": "John Doe",
                "age": 30,
                "email": "john@example.com"
            },
            "settings": {
                "theme": "dark",
                "notifications": True
            }
        },
        "metadata": {
            "created": "2024-01-01",
            "version": "1.0"
        }
    }

    print(f"Input (Python dict): {json_data}")

    flattener = json_tools_rs.JsonFlattener()
    result = flattener.flatten(json_data)  # Pass Python dict directly!

    print(f"Output: {result}")
    
    # Example 2: Advanced configuration
    print("\n2. Advanced Configuration")
    print("-" * 30)
    
    # Use Python dict directly (much more convenient!)
    python_data = {
        "User": {
            "Name": "Alice",
            "Email": "",
            "Details": None,  # Python None becomes JSON null
            "Preferences": {},
            "Tags": [],
            "Active": True
        }
    }

    print(f"Input (Python dict): {python_data}")

    # Configure with builder pattern
    advanced_flattener = (json_tools_rs.JsonFlattener()
                         .remove_empty_strings(True)
                         .remove_nulls(True)
                         .remove_empty_objects(True)
                         .remove_empty_arrays(True)
                         .separator("_")
                         .lowercase_keys(True))

    result = advanced_flattener.flatten(python_data)  # Pass Python dict directly!
    print(f"Output: {result}")
    
    # Example 3: Key and value replacements
    print("\n3. Key and Value Replacements")
    print("-" * 35)
    
    # Use Python dict directly
    python_patterns = {
        "user_name": "bob@example.com",
        "admin_role": "super",
        "user_status": "active@example.com"
    }

    print(f"Input (Python dict): {python_patterns}")

    replacement_flattener = (json_tools_rs.JsonFlattener()
                           .key_replacement("regex:^(user|admin)_", "")
                           .value_replacement("@example.com", "@company.org"))

    result = replacement_flattener.flatten(python_patterns)  # Pass Python dict directly!
    print(f"Output: {result}")
    
    # Example 4: Batch processing
    print("\n4. Batch Processing (Mixed Types)")
    print("-" * 35)

    # Mix of JSON strings and Python dicts
    mixed_batch = [
        '{"order1": {"item": "laptop", "price": 999}}',  # JSON string
        {"order2": {"item": "mouse", "price": 25}},      # Python dict
        {"order3": {"item": "keyboard", "price": 75}}    # Python dict
    ]

    print(f"Input (mixed types): {mixed_batch}")

    batch_flattener = json_tools_rs.JsonFlattener()
    results = batch_flattener.flatten(mixed_batch)  # Handles mixed types automatically!

    print(f"Output: {results}")
    
    print("\n" + "=" * 40)
    print("✅ All examples completed successfully!")
    print("🚀 JsonFlattener provides a unified, powerful API for JSON manipulation.")

if __name__ == "__main__":
    main()
