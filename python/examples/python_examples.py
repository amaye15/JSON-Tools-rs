#!/usr/bin/env python3
"""
Comprehensive Python examples for json_tools_rs flatten_json function
"""

import json
from typing import List, Dict, Any

try:
    from json_tools_rs import flatten_json, JsonOutput, JsonFlattenError
except ImportError:
    print("Error: json_tools_rs module not found. Please install the package first.")
    print("Run: maturin develop --features python")
    exit(1)

def main():
    print("=== JSON Tools RS Python - Comprehensive Examples ===\n")
    
    # Example 1: Basic flattening
    basic_flattening_example()
    
    # Example 2: Array and matrix flattening
    array_flattening_example()
    
    # Example 3: Custom separators
    custom_separator_example()
    
    # Example 4: Filtering options
    filtering_examples()
    
    # Example 5: Key and value replacements
    replacement_examples()
    
    # Example 6: Regex patterns
    regex_examples()
    
    # Example 7: Multiple JSON inputs
    multiple_json_example()
    
    # Example 8: Error handling
    error_handling_example()
    
    # Example 9: JsonOutput methods
    json_output_methods_example()
    
    # Example 10: Real-world example
    real_world_example()

def basic_flattening_example():
    print("1. Basic Flattening Examples")
    print("============================")
    
    # Simple nested object
    json_str1 = '{"user": {"name": "John", "age": 30}}'
    result = flatten_json(json_str1)
    print(f"Input:  {json_str1}")
    print(f"Output: {result.get_single()}\n")
    
    # Deeply nested object
    json_str2 = '{"company": {"department": {"team": {"member": {"name": "Alice"}}}}}'
    result = flatten_json(json_str2)
    print(f"Input:  {json_str2}")
    print(f"Output: {result.get_single()}\n")

def array_flattening_example():
    print("2. Array and Matrix Flattening")
    print("===============================")
    
    # Simple array
    json_str1 = '{"items": [1, 2, {"nested": "value"}]}'
    result = flatten_json(json_str1)
    print(f"Array Input:  {json_str1}")
    print(f"Array Output: {result.get_single()}\n")
    
    # Matrix (nested arrays)
    json_str2 = '{"matrix": [[1, 2], [3, 4]]}'
    result = flatten_json(json_str2)
    print(f"Matrix Input:  {json_str2}")
    print(f"Matrix Output: {result.get_single()}\n")
    
    # Mixed array with objects
    json_str3 = '{"users": [{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]}'
    result = flatten_json(json_str3)
    print(f"Mixed Array Input:  {json_str3}")
    print(f"Mixed Array Output: {result.get_single()}\n")

def custom_separator_example():
    print("3. Custom Separator Examples")
    print("=============================")
    
    json_str = '{"user": {"profile": {"name": "John"}}}'
    
    # Default dot separator
    result = flatten_json(json_str)
    print(f"Default (.):       {result.get_single()}")
    
    # Underscore separator
    result = flatten_json(json_str, separator="_")
    print(f"Underscore (_):    {result.get_single()}")
    
    # Double colon separator
    result = flatten_json(json_str, separator="::")
    print(f"Double colon (::): {result.get_single()}")
    
    # Pipe separator
    result = flatten_json(json_str, separator="|")
    print(f"Pipe (|):          {result.get_single()}\n")

def filtering_examples():
    print("4. Filtering Options")
    print("====================")
    
    json_str = '''{
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
    }'''
    
    # Compact the JSON for display
    compact_json = json.dumps(json.loads(json_str), separators=(',', ':'))
    print(f"Original: {compact_json}")
    
    # No filtering
    result = flatten_json(json_str)
    print(f"No filtering: {result.get_single()}")
    
    # Remove empty strings
    result = flatten_json(json_str, remove_empty_string_values=True)
    print(f"Remove empty strings: {result.get_single()}")
    
    # Remove null values
    result = flatten_json(json_str, remove_null_values=True)
    print(f"Remove null values: {result.get_single()}")
    
    # Remove empty objects
    result = flatten_json(json_str, remove_empty_dict=True)
    print(f"Remove empty objects: {result.get_single()}")
    
    # Remove empty arrays
    result = flatten_json(json_str, remove_empty_list=True)
    print(f"Remove empty arrays: {result.get_single()}")
    
    # All filtering enabled
    result = flatten_json(
        json_str,
        remove_empty_string_values=True,
        remove_null_values=True,
        remove_empty_dict=True,
        remove_empty_list=True
    )
    print(f"All filtering: {result.get_single()}\n")

def replacement_examples():
    print("5. Key and Value Replacement Examples")
    print("======================================")
    
    # Key replacements
    json_str1 = '{"user_name": "John", "user_email": "john@example.com", "admin_role": "super"}'
    key_replacements = [("user_", "person_")]
    result = flatten_json(json_str1, key_replacements=key_replacements)
    print(f"Key replacement input:  {json_str1}")
    print(f"Key replacement output: {result.get_single()}\n")
    
    # Value replacements
    json_str2 = '{"email": "john@example.com", "backup_email": "admin@example.com"}'
    value_replacements = [("@example.com", "@company.org")]
    result = flatten_json(json_str2, value_replacements=value_replacements)
    print(f"Value replacement input:  {json_str2}")
    print(f"Value replacement output: {result.get_single()}\n")
    
    # Combined replacements
    json_str3 = '{"user_email": "john@example.com", "admin_phone": "555-1234"}'
    key_replacements = [("user_", "person_")]
    value_replacements = [("@example.com", "@company.org")]
    result = flatten_json(
        json_str3,
        key_replacements=key_replacements,
        value_replacements=value_replacements
    )
    print(f"Combined replacement input:  {json_str3}")
    print(f"Combined replacement output: {result.get_single()}\n")

def regex_examples():
    print("6. Regex Pattern Examples")
    print("==========================")
    
    # Regex key replacement - remove prefixes
    json_str1 = '{"user_name": "John", "admin_role": "super", "guest_access": "limited"}'
    key_replacements = [("regex:^(user|admin|guest)_", "")]
    result = flatten_json(json_str1, key_replacements=key_replacements)
    print(f"Regex key input:  {json_str1}")
    print(f"Regex key output: {result.get_single()}\n")
    
    # Regex value replacement - email domains
    json_str2 = '{"email": "user@example.com", "backup": "admin@example.com", "support": "help@test.org"}'
    value_replacements = [("regex:@example\\.com", "@company.org")]
    result = flatten_json(json_str2, value_replacements=value_replacements)
    print(f"Regex value input:  {json_str2}")
    print(f"Regex value output: {result.get_single()}\n")
    
    # Complex regex with capture groups
    json_str3 = '{"field_123_name": "John", "field_456_email": "john@example.com"}'
    key_replacements = [("regex:^field_(\\d+)_(.+)", "$2_id_$1")]
    result = flatten_json(json_str3, key_replacements=key_replacements)
    print(f"Capture groups input:  {json_str3}")
    print(f"Capture groups output: {result.get_single()}\n")
    
    # Case-insensitive regex
    json_str4 = '{"User_Name": "John", "user_email": "john@example.com"}'
    key_replacements = [("regex:(?i)^user_", "person_")]
    result = flatten_json(json_str4, key_replacements=key_replacements)
    print(f"Case-insensitive input:  {json_str4}")
    print(f"Case-insensitive output: {result.get_single()}\n")

def multiple_json_example():
    print("7. Multiple JSON Inputs")
    print("=======================")
    
    json_list = [
        '{"user1": {"name": "Alice", "age": 25}}',
        '{"user2": {"name": "Bob", "age": 30}}',
        '{"user3": {"name": "Charlie", "age": 35}}'
    ]
    
    result = flatten_json(json_list)
    
    print(f"Processing {len(json_list)} JSON strings:")
    if result.is_multiple:
        results = result.get_multiple()
        for i, flattened in enumerate(results):
            print(f"  Result {i + 1}: {flattened}")
    print()

def error_handling_example():
    print("8. Error Handling Examples")
    print("===========================")
    
    # Invalid JSON
    try:
        result = flatten_json('{"invalid": json}')
    except JsonFlattenError as e:
        print(f"JSON parse error: {e}")
    except ValueError as e:
        print(f"Value error: {e}")
    
    # Invalid input type
    try:
        result = flatten_json(123)  # type: ignore
    except ValueError as e:
        print(f"Invalid input type: {e}")
    
    # Method call errors
    json_list = ['{"a": 1}', '{"b": 2}']
    result = flatten_json(json_list)
    
    try:
        single_result = result.get_single()  # Error: multiple results
    except ValueError as e:
        print(f"Method call error: {e}")
    
    # Correct usage
    multiple_results = result.get_multiple()
    print(f"Correct multiple access: {len(multiple_results)} results")
    print()

def json_output_methods_example():
    print("9. JsonOutput Methods")
    print("=====================")
    
    # Single result
    result_single = flatten_json('{"a": {"b": 1}}')
    print(f"Is single: {result_single.is_single}")
    print(f"Is multiple: {result_single.is_multiple}")
    print(f"Single result: {result_single.get_single()}")
    print(f"To Python: {result_single.to_python()}")
    print()
    
    # Multiple results
    result_multiple = flatten_json(['{"a": 1}', '{"b": 2}'])
    print(f"Is single: {result_multiple.is_single}")
    print(f"Is multiple: {result_multiple.is_multiple}")
    print(f"Multiple results: {result_multiple.get_multiple()}")
    print(f"To Python: {result_multiple.to_python()}")
    print()

def real_world_example():
    print("10. Real-World Example: E-commerce Product")
    print("===========================================")
    
    product_json = '''{
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
    }'''
    
    print("Original product data (truncated for display):")
    print('{"product": {"id": 12345, "name": "Gaming Laptop", ...}}')
    
    # Basic flattening
    result = flatten_json(product_json)
    print(f"\nBasic flattening:")
    print(result.get_single())
    
    # With filtering and key simplification
    key_replacements = [("product.", "")]
    result = flatten_json(
        product_json,
        remove_null_values=True,
        remove_empty_dict=True,
        remove_empty_list=True,
        key_replacements=key_replacements
    )
    print(f"\nWith filtering and simplified keys:")
    print(result.get_single())
    
    # Parse the result to show it as a Python dict
    flattened_str = result.get_single()
    parsed_dict = json.loads(flattened_str)
    print(f"\nAs Python dictionary ({len(parsed_dict)} keys):")
    for key, value in list(parsed_dict.items())[:5]:  # Show first 5 keys
        print(f"  '{key}': {repr(value)}")
    if len(parsed_dict) > 5:
        print(f"  ... and {len(parsed_dict) - 5} more keys")

if __name__ == "__main__":
    main()
