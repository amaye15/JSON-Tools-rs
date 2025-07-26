#!/usr/bin/env python3
"""
JSON Tools RS Python Example

This example demonstrates the unified JsonFlattener and JsonUnflattener APIs in Python.
"""

import json_tools_rs


def main():
    print("JSON Tools RS - Python Example")
    print("Perfect Type Matching: Input Type = Output Type!")
    print("Complete Flatten/Unflatten Solution")
    print("=" * 50)

    # Example 1: Basic flattening
    print("\n1. Basic Flattening")
    print("-" * 20)

    # Now you can pass Python dicts directly - no need to serialize to JSON!
    json_data = {
        "user": {
            "profile": {"name": "John Doe", "age": 30, "email": "john@example.com"},
            "settings": {"theme": "dark", "notifications": True},
        },
        "metadata": {"created": "2024-01-01", "version": "1.0"},
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
            "Active": True,
        }
    }

    print(f"Input (Python dict): {python_data}")

    # Configure with builder pattern
    advanced_flattener = (
        json_tools_rs.JsonFlattener()
        .remove_empty_strings(True)
        .remove_nulls(True)
        .remove_empty_objects(True)
        .remove_empty_arrays(True)
        .separator("_")
        .lowercase_keys(True)
    )

    result = advanced_flattener.flatten(python_data)  # Pass Python dict directly!
    print(f"Output: {result}")

    # Example 3: Key and value replacements
    print("\n3. Key and Value Replacements")
    print("-" * 35)

    # Use Python dict directly
    python_patterns = {
        "user_name": "bob@example.com",
        "admin_role": "super",
        "user_status": "active@example.com",
    }

    print(f"Input (Python dict): {python_patterns}")

    replacement_flattener = (
        json_tools_rs.JsonFlattener()
        .key_replacement("regex:^(user|admin)_", "")
        .value_replacement("@example.com", "@company.org")
    )

    result = replacement_flattener.flatten(
        python_patterns
    )  # Pass Python dict directly!
    print(f"Output: {result}")

    # Example 4: Batch processing
    print("\n4. Batch Processing (Mixed Types)")
    print("-" * 35)

    # Mix of JSON strings and Python dicts
    mixed_batch = [
        '{"order1": {"item": "laptop", "price": 999}}',  # JSON string
        {"order2": {"item": "mouse", "price": 25}},  # Python dict
        {"order3": {"item": "keyboard", "price": 75}},  # Python dict
    ]

    print(f"Input (mixed types): {mixed_batch}")

    batch_flattener = json_tools_rs.JsonFlattener()
    results = batch_flattener.flatten(mixed_batch)  # Handles mixed types automatically!

    print(f"Output: {results}")

    # Example 5: Basic unflattening
    print("\n5. Basic Unflattening")
    print("-" * 20)

    # Use the flattened result from Example 1
    flattened_data = result  # This is a Python dict from Example 1
    print(f"Input (flattened dict): {flattened_data}")

    unflattener = json_tools_rs.JsonUnflattener()
    restored = unflattener.unflatten(flattened_data)  # Pass Python dict directly!

    print(f"Output (restored): {restored}")
    print(f"Type preserved: {type(restored)}")

    # Example 6: Advanced JsonUnflattener configuration
    print("\n6. Advanced Unflattener Configuration")
    print("-" * 40)

    # Create some flattened data with prefixes
    flattened_with_prefixes = {
        "PREFIX_name": "john@company.org",
        "PREFIX_age": 30,
        "PREFIX_profile_city": "NYC",
    }

    print(f"Input (flattened dict): {flattened_with_prefixes}")

    # Configure unflattener with transformations
    advanced_unflattener = (
        json_tools_rs.JsonUnflattener()
        .separator("_")
        .lowercase_keys(True)
        .key_replacement("prefix_", "user_")
        .value_replacement("@company.org", "@example.com")
    )

    restored_advanced = advanced_unflattener.unflatten(flattened_with_prefixes)
    print(f"Output (transformed): {restored_advanced}")

    # Example 7: Roundtrip demonstration
    print("\n7. Complete Roundtrip (Flatten → Unflatten)")
    print("-" * 45)

    # Start with complex nested data
    original_complex = {
        "user": {
            "profile": {"name": "Alice", "age": 28},
            "emails": ["alice@work.com", "alice@personal.com"],
            "settings": {
                "theme": "light",
                "notifications": {"email": True, "sms": False},
            },
        },
        "metadata": {"created": "2024-01-01", "version": 2.1},
    }

    print(f"Original: {original_complex}")

    # Flatten
    roundtrip_flattener = json_tools_rs.JsonFlattener()
    flattened_complex = roundtrip_flattener.flatten(original_complex)
    print(f"Flattened: {flattened_complex}")

    # Unflatten
    roundtrip_unflattener = json_tools_rs.JsonUnflattener()
    restored_complex = roundtrip_unflattener.unflatten(flattened_complex)
    print(f"Restored: {restored_complex}")

    # Verify they're identical
    print(f"Roundtrip successful: {original_complex == restored_complex}")

    # Example 8: Batch unflattening with type preservation
    print("\n8. Batch Unflattening (Type Preservation)")
    print("-" * 45)

    # Create batch of flattened data (mix of strings and dicts)
    flattened_batch_strings = [
        '{"order.id": 1, "order.item": "laptop"}',
        '{"order.id": 2, "order.item": "mouse"}',
        '{"order.id": 3, "order.item": "keyboard"}',
    ]

    flattened_batch_dicts = [
        {"product.name": "laptop", "product.price": 999},
        {"product.name": "mouse", "product.price": 25},
        {"product.name": "keyboard", "product.price": 75},
    ]

    print(f"String batch input: {flattened_batch_strings}")

    batch_unflattener = json_tools_rs.JsonUnflattener()

    # Process string batch → returns list of strings
    string_results = batch_unflattener.unflatten(flattened_batch_strings)
    print(f"String batch output: {string_results}")
    print(f"Output types: {[type(item) for item in string_results]}")

    print(f"\nDict batch input: {flattened_batch_dicts}")

    # Process dict batch → returns list of dicts
    dict_results = batch_unflattener.unflatten(flattened_batch_dicts)
    print(f"Dict batch output: {dict_results}")
    print(f"Output types: {[type(item) for item in dict_results]}")

    print("\n" + "=" * 60)
    print("✅ All examples completed successfully!")
    print("🚀 JsonFlattener and JsonUnflattener provide a complete,")
    print("   unified solution for JSON manipulation with perfect type preservation!")


if __name__ == "__main__":
    main()
