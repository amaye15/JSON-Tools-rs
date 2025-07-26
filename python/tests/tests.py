#!/usr/bin/env python3
"""
Test script for JSON Tools RS Python bindings

This script tests the updated Python bindings that use only the JsonFlattener API.
"""

def test_basic_flattening():
    """Test basic JSON flattening functionality with matching input/output types"""
    import json_tools_rs

    flattener = json_tools_rs.JsonFlattener()

    # Test 1: JSON string input → JSON string output
    json_string = '{"user": {"name": "John", "age": 30, "address": {"city": "NYC", "zip": "10001"}}}'
    flattened_str = flattener.flatten(json_string)

    assert isinstance(flattened_str, str)
    print(f"JSON string result: {flattened_str} (type: {type(flattened_str).__name__})")

    # Test 2: Python dict input → Python dict output (much more convenient!)
    python_dict = {"user": {"name": "John", "age": 30, "address": {"city": "NYC", "zip": "10001"}}}
    flattened_dict = flattener.flatten(python_dict)

    assert isinstance(flattened_dict, dict)
    print(f"Python dict result: {flattened_dict} (type: {type(flattened_dict).__name__})")

    # Verify both contain expected keys
    import json
    parsed_str = json.loads(flattened_str)  # Parse JSON string to dict for comparison

    for parsed in [parsed_str, flattened_dict]:
        assert "user.name" in parsed
        assert "user.age" in parsed
        assert "user.address.city" in parsed
        assert "user.address.zip" in parsed

    print("✅ Basic flattening test passed!")


def test_advanced_configuration():
    """Test advanced configuration with builder pattern and Python dict input/output"""
    import json_tools_rs

    # Use Python dict directly (more convenient than JSON string!)
    python_input = {"User": {"Name": "John", "Email": "", "Details": None, "Active": True}}

    # Configure flattener with builder pattern
    flattener = (json_tools_rs.JsonFlattener()
                .remove_empty_strings(True)
                .remove_nulls(True)
                .separator("_")
                .lowercase_keys(True))

    flattened = flattener.flatten(python_input)

    # Result should be a dict directly (matching input type)
    assert isinstance(flattened, dict)
    print(f"Advanced configuration result: {flattened} (type: {type(flattened).__name__})")

    # Verify filtering and transformations
    assert "user_name" in flattened  # lowercase transformation
    assert "user_active" in flattened
    assert "user_email" not in flattened  # empty string removed
    assert "user_details" not in flattened  # null removed

    print("✅ Advanced configuration test passed!")


def test_key_value_replacements():
    """Test key and value replacement functionality with type preservation"""
    import json_tools_rs

    # Test with JSON string input → JSON string output
    json_input = '{"user_name": "john@example.com", "admin_role": "super"}'

    flattener = (json_tools_rs.JsonFlattener()
                .key_replacement("regex:^(user|admin)_", "")
                .value_replacement("@example.com", "@company.org"))

    flattened_str = flattener.flatten(json_input)

    # Result should be a string (matching input type)
    assert isinstance(flattened_str, str)
    print(f"Replacement result (str): {flattened_str} (type: {type(flattened_str).__name__})")

    import json
    parsed_str = json.loads(flattened_str)
    assert "name" in parsed_str  # user_ prefix removed
    assert "role" in parsed_str  # admin_ prefix removed
    assert parsed_str["name"] == "john@company.org"  # email replaced

    # Test with Python dict input → Python dict output
    dict_input = {"user_name": "john@example.com", "admin_role": "super"}
    flattened_dict = flattener.flatten(dict_input)

    # Result should be a dict (matching input type)
    assert isinstance(flattened_dict, dict)
    print(f"Replacement result (dict): {flattened_dict} (type: {type(flattened_dict).__name__})")

    assert "name" in flattened_dict  # user_ prefix removed
    assert "role" in flattened_dict  # admin_ prefix removed
    assert flattened_dict["name"] == "john@company.org"  # email replaced

    print("✅ Key/value replacement test passed!")


def test_batch_processing():
    """Test batch processing with type preservation"""
    import json_tools_rs

    flattener = json_tools_rs.JsonFlattener()

    # Test 1: List of JSON strings → List of JSON strings
    json_string_list = [
        '{"user1": {"name": "Alice"}}',
        '{"user2": {"name": "Bob"}}',
        '{"user3": {"name": "Charlie"}}'
    ]

    results_str = flattener.flatten(json_string_list)
    assert isinstance(results_str, list)
    assert len(results_str) == 3
    assert all(isinstance(item, str) for item in results_str)
    print(f"JSON string list results: {results_str} (types: {[type(item).__name__ for item in results_str]})")

    # Test 2: List of Python dicts → List of Python dicts
    python_dict_list = [
        {"user1": {"name": "Alice"}},
        {"user2": {"name": "Bob"}},
        {"user3": {"name": "Charlie"}}
    ]

    results_dict = flattener.flatten(python_dict_list)
    assert isinstance(results_dict, list)
    assert len(results_dict) == 3
    assert all(isinstance(item, dict) for item in results_dict)
    print(f"Python dict list results: {results_dict} (types: {[type(item).__name__ for item in results_dict]})")

    # Test 3: Mixed list (preserves original types)
    mixed_list = [
        '{"user1": {"name": "Alice"}}',  # JSON string
        {"user2": {"name": "Bob"}},      # Python dict
        {"user3": {"name": "Charlie"}}   # Python dict
    ]

    results_mixed = flattener.flatten(mixed_list)
    assert isinstance(results_mixed, list)
    assert len(results_mixed) == 3
    assert isinstance(results_mixed[0], str)   # First item should be string
    assert isinstance(results_mixed[1], dict)  # Second item should be dict
    assert isinstance(results_mixed[2], dict)  # Third item should be dict
    print(f"Mixed list results: {results_mixed} (types: {[type(item).__name__ for item in results_mixed]})")

    # Verify all results contain expected keys
    import json
    for i, flattened in enumerate(results_str):
        parsed = json.loads(flattened)
        assert f"user{i+1}.name" in parsed

    for i, flattened in enumerate(results_dict):
        assert f"user{i+1}.name" in flattened  # Already a dict

    for i, flattened in enumerate(results_mixed):
        if isinstance(flattened, str):
            parsed = json.loads(flattened)
            assert f"user{i+1}.name" in parsed
        else:
            assert f"user{i+1}.name" in flattened  # Already a dict

    print("✅ Batch processing test passed!")


def test_error_handling():
    """Test error handling for invalid JSON"""
    import json_tools_rs

    try:
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten('{"invalid": json}')  # Invalid JSON
        assert False, "Should have raised an exception"
    except json_tools_rs.JsonFlattenError as e:
        print(f"Expected error caught: {e}")
        print("✅ Error handling test passed!")


def test_input_type_flexibility():
    """Test all supported input types with proper output type matching"""
    import json_tools_rs

    flattener = json_tools_rs.JsonFlattener()
    test_data = {"user": {"name": "John", "age": 30}}
    expected_keys = ["user.name", "user.age"]

    # Test 1: str (JSON string) → str output
    json_str = '{"user": {"name": "John", "age": 30}}'
    result_str = flattener.flatten(json_str)
    assert isinstance(result_str, str)

    # Test 2: dict (Python dictionary) → dict output
    result_dict = flattener.flatten(test_data)
    assert isinstance(result_dict, dict)

    # Test 3: list[str] (list of JSON strings) → list[str] output
    json_list = ['{"user": {"name": "John", "age": 30}}', '{"admin": {"role": "super"}}']
    result_list_str = flattener.flatten(json_list)
    assert isinstance(result_list_str, list)
    assert len(result_list_str) == 2
    assert all(isinstance(item, str) for item in result_list_str)

    # Test 4: list[dict] (list of Python dictionaries) → list[dict] output
    dict_list = [{"user": {"name": "John", "age": 30}}, {"admin": {"role": "super"}}]
    result_list_dict = flattener.flatten(dict_list)
    assert isinstance(result_list_dict, list)
    assert len(result_list_dict) == 2
    assert all(isinstance(item, dict) for item in result_list_dict)

    # Verify all results are equivalent for the same data
    import json
    parsed_str = json.loads(result_str)
    # result_dict is already a dict, no need to parse

    for expected_key in expected_keys:
        assert expected_key in parsed_str
        assert expected_key in result_dict
        assert parsed_str[expected_key] == result_dict[expected_key]

    print(f"String input result: {result_str} (type: {type(result_str).__name__})")
    print(f"Dict input result: {result_dict} (type: {type(result_dict).__name__})")
    print(f"List[str] input results: {result_list_str} (types: {[type(item).__name__ for item in result_list_str]})")
    print(f"List[dict] input results: {result_list_dict} (types: {[type(item).__name__ for item in result_list_dict]})")

    print("✅ Input type flexibility test passed!")


def test_advanced_output_object():
    """Test the advanced JsonOutput object with different input types"""
    import json_tools_rs

    flattener = json_tools_rs.JsonFlattener()

    # Test single JSON string with output object
    result = flattener.flatten_to_output('{"test": {"key": "value"}}')
    assert result.is_single
    assert not result.is_multiple
    single_result = result.get_single()
    assert isinstance(single_result, str)
    print(f"Advanced single result (str): {single_result}")

    # Test single Python dict with output object
    result = flattener.flatten_to_output({"test": {"key": "value"}})
    assert result.is_single
    assert not result.is_multiple
    single_result = result.get_single()
    assert isinstance(single_result, str)
    print(f"Advanced single result (dict): {single_result}")

    # Test multiple with mixed types
    mixed_list = ['{"a": 1}', {"b": 2}]
    result = flattener.flatten_to_output(mixed_list)
    assert result.is_multiple
    assert not result.is_single
    multiple_results = result.get_multiple()
    assert isinstance(multiple_results, list)
    assert len(multiple_results) == 2
    print(f"Advanced multiple results (mixed): {multiple_results}")

    print("✅ Advanced output object test passed!")


if __name__ == "__main__":
    print("Testing JSON Tools RS Python bindings...")
    print("=" * 50)
    
    try:
        test_basic_flattening()
        test_advanced_configuration()
        test_key_value_replacements()
        test_batch_processing()
        test_error_handling()
        test_input_type_flexibility()
        test_advanced_output_object()
        
        print("=" * 50)
        print("🚀 All tests passed! Python bindings are working correctly.")
        print("✅ JsonFlattener is the unified API entry point.")
        
    except ImportError as e:
        print(f"❌ Import error: {e}")
        print("Make sure to build the Python package first with:")
        print("  maturin develop --features python")
    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
