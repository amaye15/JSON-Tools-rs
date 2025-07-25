#!/usr/bin/env python3
"""
Basic usage examples for json_tools_rs Python bindings

This script demonstrates the main features of the json_tools_rs library
including basic flattening, filtering, replacements, and batch processing.
"""

import json
from typing import List, Dict, Any

try:
    from json_tools_rs import flatten_json, JsonOutput, JsonFlattenError
except ImportError:
    print("Error: json_tools_rs module not found. Please install the package first.")
    print("Run: maturin develop --features python")
    exit(1)


def print_example(title: str, json_input: str, result: JsonOutput, description: str = ""):
    """Helper function to print examples in a formatted way."""
    print(f"\n{'='*60}")
    print(f"Example: {title}")
    print(f"{'='*60}")
    if description:
        print(f"Description: {description}")
        print()
    
    print(f"Input JSON:")
    print(json.dumps(json.loads(json_input), indent=2))
    print()
    
    if result.is_single:
        flattened = result.get_single()
        print(f"Flattened Result:")
        print(json.dumps(json.loads(flattened), indent=2))
    else:
        results = result.get_multiple()
        print(f"Flattened Results ({len(results)} items):")
        for i, flattened in enumerate(results, 1):
            print(f"  {i}. {json.dumps(json.loads(flattened), indent=2)}")


def main():
    """Run all examples."""
    print("JSON Tools RS - Python Bindings Examples")
    print("========================================")
    
    # Example 1: Basic flattening
    json1 = '{"user": {"profile": {"name": "John", "age": 30}, "settings": {"theme": "dark"}}}'
    result1 = flatten_json(json1)
    print_example(
        "Basic Flattening",
        json1,
        result1,
        "Flatten nested JSON objects using dot notation"
    )
    
    # Example 2: Array flattening
    json2 = '{"items": [1, 2, {"nested": "value"}], "matrix": [[1, 2], [3, 4]]}'
    result2 = flatten_json(json2)
    print_example(
        "Array Flattening",
        json2,
        result2,
        "Flatten arrays using numeric indices"
    )
    
    # Example 3: Filtering empty values
    json3 = '{"user": {"name": "John", "email": "", "age": null, "metadata": {}, "tags": []}}'
    result3 = flatten_json(
        json3,
        remove_empty_string_values=True,
        remove_null_values=True,
        remove_empty_dict=True,
        remove_empty_list=True
    )
    print_example(
        "Filtering Empty Values",
        json3,
        result3,
        "Remove empty strings, null values, empty objects, and empty arrays"
    )
    
    # Example 4: Key and value replacements
    json4 = '{"user_name": "john@example.com", "user_email": "john@example.com"}'
    key_replacements = [("user_", "person_")]
    value_replacements = [("@example.com", "@company.org")]
    result4 = flatten_json(
        json4,
        key_replacements=key_replacements,
        value_replacements=value_replacements
    )
    print_example(
        "Key and Value Replacements",
        json4,
        result4,
        "Replace key prefixes and email domains"
    )
    
    # Example 5: Custom separator
    json5 = '{"user": {"profile": {"name": "John", "location": {"city": "NYC", "country": "USA"}}}}'
    result5 = flatten_json(json5, separator="_")
    print_example(
        "Custom Separator",
        json5,
        result5,
        "Use underscore instead of dot as separator"
    )
    
    # Example 6: Multiple JSON strings
    json_list = [
        '{"user1": {"name": "Alice", "age": 25}}',
        '{"user2": {"name": "Bob", "age": 30}}',
        '{"user3": {"name": "Charlie", "age": 35}}'
    ]
    result6 = flatten_json(json_list)
    print(f"\n{'='*60}")
    print(f"Example: Multiple JSON Strings")
    print(f"{'='*60}")
    print("Description: Process multiple JSON strings in a single call")
    print()
    print(f"Input JSON List ({len(json_list)} items):")
    for i, json_str in enumerate(json_list, 1):
        print(f"  {i}. {json.dumps(json.loads(json_str), indent=2)}")
    print()
    
    results = result6.get_multiple()
    print(f"Flattened Results ({len(results)} items):")
    for i, flattened in enumerate(results, 1):
        print(f"  {i}. {json.dumps(json.loads(flattened), indent=2)}")
    
    # Example 7: Complex real-world example
    complex_json = '''
    {
        "api_response": {
            "status": "success",
            "data": {
                "users": [
                    {
                        "id": 1,
                        "profile": {
                            "name": "Alice Johnson",
                            "email": "alice@example.com",
                            "preferences": {
                                "theme": "dark",
                                "notifications": {
                                    "email": true,
                                    "push": false
                                }
                            }
                        },
                        "metadata": {
                            "created_at": "2023-01-15",
                            "last_login": null,
                            "tags": ["premium", "beta-tester"]
                        }
                    }
                ],
                "pagination": {
                    "page": 1,
                    "total": 1,
                    "has_more": false
                }
            }
        }
    }
    '''
    
    result7 = flatten_json(
        complex_json,
        remove_null_values=True,
        key_replacements=[("api_response.", ""), ("data.", "")],
        separator="."
    )
    print_example(
        "Complex Real-World Example",
        complex_json,
        result7,
        "Process a complex API response with nested structures, arrays, and filtering"
    )
    
    # Example 8: Error handling
    print(f"\n{'='*60}")
    print(f"Example: Error Handling")
    print(f"{'='*60}")
    print("Description: Demonstrate error handling for invalid JSON")
    print()
    
    try:
        invalid_json = '{"invalid": json}'
        print(f"Attempting to flatten invalid JSON: {invalid_json}")
        flatten_json(invalid_json)
    except (JsonFlattenError, ValueError) as e:
        print(f"Caught exception: {e}")

    try:
        print("Attempting to call get_single() on multiple results...")
        multiple_result = flatten_json(['{"a": 1}', '{"b": 2}'])
        multiple_result.get_single()
    except ValueError as e:
        print(f"Caught ValueError: {e}")
    
    print(f"\n{'='*60}")
    print("All examples completed successfully!")
    print("For more information, see the documentation and test files.")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
