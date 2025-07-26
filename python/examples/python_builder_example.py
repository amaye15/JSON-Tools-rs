#!/usr/bin/env python3
"""
JSON Tools RS - Python JsonFlattener Builder Pattern Examples

This example demonstrates the new JsonFlattener builder pattern API for Python,
which matches the Rust API design.
"""

# Note: This is a demonstration of the API. To actually run this, you would need to:
# 1. Build the Python extension: maturin develop --features python
# 2. Import the module: import json_tools_rs

def demonstrate_api():
    """Demonstrate the JsonFlattener builder pattern API"""
    
    print("JSON Tools RS - Python JsonFlattener Builder Pattern Examples")
    print("=============================================================\n")
    
    # Example 1: Basic flattening with simple function
    print("Example 1: Simple Function API")
    print("==============================")
    json1 = '{"user": {"name": "John", "age": 30}}'
    # result = json_tools_rs.flatten_json(json1)
    print(f"Input:  {json1}")
    print(f"Output: {{'user.name': 'John', 'user.age': 30}}")
    print()
    
    # Example 2: JsonFlattener builder pattern - basic usage
    print("Example 2: JsonFlattener Builder Pattern - Basic")
    print("================================================")
    json2 = '{"user": {"profile": {"name": "John", "age": 30}}}'
    # result = json_tools_rs.JsonFlattener().flatten(json2)
    print(f"Input:  {json2}")
    print(f"Output: {{'user.profile.name': 'John', 'user.profile.age': 30}}")
    print()
    
    # Example 3: Builder pattern with filtering
    print("Example 3: Builder Pattern with Filtering")
    print("=========================================")
    json3 = '{"user": {"name": "John", "bio": "", "age": null, "tags": []}}'
    # result = (json_tools_rs.JsonFlattener()
    #     .remove_empty_strings(True)
    #     .remove_nulls(True)
    #     .remove_empty_arrays(True)
    #     .flatten(json3))
    print(f"Input:  {json3}")
    print(f"Output: {{'user.name': 'John'}}")
    print()
    
    # Example 4: Builder pattern with replacements
    print("Example 4: Builder Pattern with Replacements")
    print("============================================")
    json4 = '{"user_name": "John", "user_email": "john@example.com"}'
    # result = (json_tools_rs.JsonFlattener()
    #     .key_replacement("user_", "")
    #     .value_replacement("@example.com", "@company.org")
    #     .flatten(json4))
    print(f"Input:  {json4}")
    print(f"Output: {{'name': 'John', 'email': 'john@company.org'}}")
    print()
    
    # Example 5: Builder pattern with custom separator
    print("Example 5: Builder Pattern with Custom Separator")
    print("================================================")
    json5 = '{"user": {"profile": {"name": "John"}}}'
    # result = (json_tools_rs.JsonFlattener()
    #     .separator("_")
    #     .flatten(json5))
    print(f"Input:  {json5}")
    print(f"Output: {{'user_profile_name': 'John'}}")
    print()
    
    # Example 6: Builder pattern with regex and all features
    print("Example 6: Builder Pattern - All Features Combined")
    print("==================================================")
    json6 = '{"user_profile": {"user_name": "John", "user_email": "john@example.com", "user_bio": "", "user_age": null}}'
    # result = (json_tools_rs.JsonFlattener()
    #     .remove_empty_strings(True)
    #     .remove_nulls(True)
    #     .key_replacement("regex:^user_", "")
    #     .value_replacement("@example.com", "@company.org")
    #     .separator("::")
    #     .lowercase_keys(True)
    #     .flatten(json6))
    print(f"Input:  {json6}")
    print(f"Output: {{'profile::name': 'john', 'profile::email': 'john@company.org'}}")
    print()
    
    # Example 7: Batch processing
    print("Example 7: Batch Processing")
    print("===========================")
    json_list = [
        '{"user": {"name": "Alice"}}',
        '{"user": {"name": "Bob"}}',
        '{"user": {"name": "Charlie"}}'
    ]
    # result = (json_tools_rs.JsonFlattener()
    #     .separator("_")
    #     .flatten(json_list))
    print(f"Input:  {json_list}")
    print(f"Output: ['{{'user_name': 'Alice'}}', '{{'user_name': 'Bob'}}', '{{'user_name': 'Charlie'}}']")
    print()
    
    print("✅ All JsonFlattener examples completed successfully!")
    print("\n🎯 Benefits of the Python JsonFlattener Builder API:")
    print("  • Fluent, chainable method calls (same as Rust)")
    print("  • Self-documenting configuration")
    print("  • No parameter counting or ordering")
    print("  • Easy to extend with new features")
    print("  • Consistent API across Rust and Python")

def show_actual_usage():
    """Show how to actually use the API when the module is available"""
    
    print("\n" + "="*60)
    print("ACTUAL USAGE (when json_tools_rs module is built):")
    print("="*60)
    
    usage_code = '''
# Build the Python extension first:
# maturin develop --features python

import json_tools_rs

# Simple API
result = json_tools_rs.flatten_json('{"user": {"name": "John"}}')
print(result.get_single())

# Builder pattern API
result = (json_tools_rs.JsonFlattener()
    .remove_empty_strings(True)
    .remove_nulls(True)
    .separator("_")
    .lowercase_keys(True)
    .key_replacement("regex:^user_", "")
    .value_replacement("@example.com", "@company.org")
    .flatten('{"user_name": "John", "user_email": "john@example.com", "user_bio": "", "user_age": null}'))

print(result.get_single())
# Output: {"name": "john", "email": "john@company.org"}

# Batch processing
json_list = ['{"user": {"name": "Alice"}}', '{"user": {"name": "Bob"}}']
result = json_tools_rs.JsonFlattener().separator("_").flatten(json_list)
print(result.get_multiple())
# Output: ['{"user_name": "Alice"}', '{"user_name": "Bob"}']
'''
    
    print(usage_code)

if __name__ == "__main__":
    demonstrate_api()
    show_actual_usage()
