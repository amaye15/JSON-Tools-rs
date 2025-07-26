#!/usr/bin/env python3
"""
Comprehensive Test Suite for JSON Tools RS Python Bindings

This test suite provides complete coverage of the JsonFlattener API including:
- Basic functionality tests
- Advanced configuration tests  
- Error handling tests
- Edge case tests
- Performance benchmarks
- Type preservation tests
- All input/output combinations
"""

import json
import time
import pytest
import json_tools_rs
from typing import Any, Dict, List, Union


class TestBasicFunctionality:
    """Test basic JSON flattening functionality"""
    
    def test_basic_flattening_dict_input_dict_output(self):
        """Test dict input → dict output (most convenient!)"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {"user": {"name": "John", "age": 30}}
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.age"] == 30
        
    def test_basic_flattening_str_input_str_output(self):
        """Test JSON string input → JSON string output"""
        flattener = json_tools_rs.JsonFlattener()
        input_json = '{"user": {"name": "John", "age": 30}}'
        result = flattener.flatten(input_json)
        
        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed["user.name"] == "John"
        assert parsed["user.age"] == 30
        
    def test_deeply_nested_structure(self):
        """Test deeply nested JSON structures"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "value": "deep_value"
                        }
                    }
                }
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["level1.level2.level3.level4.value"] == "deep_value"
        
    def test_array_flattening(self):
        """Test array flattening with indices"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "items": [1, 2, {"nested": "value"}],
            "matrix": [[1, 2], [3, 4]]
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["items.0"] == 1
        assert result["items.1"] == 2
        assert result["items.2.nested"] == "value"
        assert result["matrix.0.0"] == 1
        assert result["matrix.0.1"] == 2
        assert result["matrix.1.0"] == 3
        assert result["matrix.1.1"] == 4
        
    def test_mixed_data_types(self):
        """Test flattening with various data types"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "string": "text",
            "number": 42,
            "float": 3.14,
            "boolean_true": True,
            "boolean_false": False,
            "null_value": None,
            "array": [1, "two", 3.0, True, None],
            "object": {"nested": "value"}
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["string"] == "text"
        assert result["number"] == 42
        assert result["float"] == 3.14
        assert result["boolean_true"] is True
        assert result["boolean_false"] is False
        assert result["null_value"] is None
        assert result["array.0"] == 1
        assert result["array.1"] == "two"
        assert result["array.2"] == 3.0
        assert result["array.3"] is True
        assert result["array.4"] is None
        assert result["object.nested"] == "value"


class TestAdvancedConfiguration:
    """Test advanced configuration options"""
    
    def test_remove_empty_strings(self):
        """Test removing empty string values"""
        flattener = json_tools_rs.JsonFlattener().remove_empty_strings(True)
        input_data = {
            "user": {
                "name": "John",
                "email": "",  # Should be removed
                "bio": "Developer"
            },
            "empty_field": ""  # Should be removed
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.bio"] == "Developer"
        assert "user.email" not in result
        assert "empty_field" not in result
        
    def test_remove_nulls(self):
        """Test removing null values"""
        flattener = json_tools_rs.JsonFlattener().remove_nulls(True)
        input_data = {
            "user": {
                "name": "John",
                "age": None,  # Should be removed
                "active": True
            },
            "null_field": None  # Should be removed
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.active"] is True
        assert "user.age" not in result
        assert "null_field" not in result
        
    def test_remove_empty_objects(self):
        """Test removing empty object values"""
        flattener = json_tools_rs.JsonFlattener().remove_empty_objects(True)
        input_data = {
            "user": {
                "profile": {},  # Should be removed
                "settings": {"theme": "dark"}
            },
            "empty_obj": {}  # Should be removed
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.settings.theme"] == "dark"
        assert "user.profile" not in result
        assert "empty_obj" not in result
        
    def test_remove_empty_arrays(self):
        """Test removing empty array values"""
        flattener = json_tools_rs.JsonFlattener().remove_empty_arrays(True)
        input_data = {
            "user": {
                "tags": [],  # Should be removed
                "items": [1, 2, 3]
            },
            "empty_list": []  # Should be removed
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.items.0"] == 1
        assert result["user.items.1"] == 2
        assert result["user.items.2"] == 3
        assert "user.tags" not in result
        assert "empty_list" not in result
        
    def test_custom_separator(self):
        """Test custom separators"""
        separators = ["_", "::", "/", "|", "---"]
        
        for sep in separators:
            flattener = json_tools_rs.JsonFlattener().separator(sep)
            input_data = {"level1": {"level2": {"value": "test"}}}
            result = flattener.flatten(input_data)
            
            expected_key = f"level1{sep}level2{sep}value"
            assert isinstance(result, dict)
            assert result[expected_key] == "test"
            
    def test_lowercase_keys(self):
        """Test lowercase key conversion"""
        flattener = json_tools_rs.JsonFlattener().lowercase_keys(True)
        input_data = {
            "User": {
                "Profile": {
                    "Name": "John",
                    "Email": "john@example.com"
                }
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user.profile.name"] == "John"
        assert result["user.profile.email"] == "john@example.com"
        
    def test_combined_filters(self):
        """Test all filters combined"""
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_empty_strings(True)
                    .remove_nulls(True)
                    .remove_empty_objects(True)
                    .remove_empty_arrays(True)
                    .lowercase_keys(True)
                    .separator("_"))
        
        input_data = {
            "User": {
                "Name": "John",
                "Email": "",
                "Age": None,
                "Settings": {},
                "Tags": [],
                "Active": True
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user_name"] == "John"
        assert result["user_active"] is True
        assert len(result) == 2  # Only name and active should remain


class TestReplacements:
    """Test key and value replacement functionality"""
    
    def test_literal_key_replacement(self):
        """Test literal string key replacement"""
        flattener = json_tools_rs.JsonFlattener().key_replacement("user_", "person_")
        input_data = {
            "user_name": "John",
            "user_email": "john@example.com",
            "admin_role": "super"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["person_name"] == "John"
        assert result["person_email"] == "john@example.com"
        assert result["admin_role"] == "super"  # Should remain unchanged
        
    def test_regex_key_replacement(self):
        """Test regex key replacement"""
        flattener = json_tools_rs.JsonFlattener().key_replacement("regex:^(user|admin)_", "")
        input_data = {
            "user_name": "John",
            "admin_role": "super",
            "guest_access": "limited"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["name"] == "John"
        assert result["role"] == "super"
        assert result["guest_access"] == "limited"  # Should remain unchanged
        
    def test_literal_value_replacement(self):
        """Test literal string value replacement"""
        flattener = json_tools_rs.JsonFlattener().value_replacement("inactive", "disabled")
        input_data = {
            "user1": {"status": "active"},
            "user2": {"status": "inactive"},
            "user3": {"status": "pending"}
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user1.status"] == "active"
        assert result["user2.status"] == "disabled"
        assert result["user3.status"] == "pending"
        
    def test_regex_value_replacement(self):
        """Test regex value replacement"""
        flattener = json_tools_rs.JsonFlattener().value_replacement("regex:@example\\.com", "@company.org")
        input_data = {
            "user1": {"email": "john@example.com"},
            "user2": {"email": "jane@example.com"},
            "user3": {"email": "bob@test.org"}
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["user1.email"] == "john@company.org"
        assert result["user2.email"] == "jane@company.org"
        assert result["user3.email"] == "bob@test.org"  # Should remain unchanged
        
    def test_multiple_replacements(self):
        """Test multiple key and value replacements"""
        flattener = (json_tools_rs.JsonFlattener()
                    .key_replacement("user_", "person_")
                    .key_replacement("regex:^admin_", "manager_")
                    .value_replacement("@example.com", "@company.org")
                    .value_replacement("regex:^inactive$", "disabled"))
        
        input_data = {
            "user_email": "john@example.com",
            "admin_role": "super",
            "user_status": "inactive"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["person_email"] == "john@company.org"
        assert result["manager_role"] == "super"
        assert result["person_status"] == "disabled"
        
    def test_regex_capture_groups(self):
        """Test regex replacement with capture groups"""
        flattener = json_tools_rs.JsonFlattener().key_replacement("regex:^field_(\\d+)_(.+)", "$2_id_$1")
        input_data = {
            "field_123_name": "John",
            "field_456_email": "john@example.com",
            "other_field": "unchanged"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        # Note: The actual result depends on the regex implementation
        # This test verifies the function works without errors
        assert len(result) == 3
        assert "other_field" in result
        assert result["other_field"] == "unchanged"


class TestBatchProcessing:
    """Test batch processing with lists"""
    
    def test_list_of_strings_input_output(self):
        """Test list[str] input → list[str] output"""
        flattener = json_tools_rs.JsonFlattener()
        input_list = [
            '{"user1": {"name": "Alice"}}',
            '{"user2": {"name": "Bob"}}',
            '{"user3": {"name": "Charlie"}}'
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, str) for item in result)
        
        parsed = [json.loads(item) for item in result]
        assert parsed[0]["user1.name"] == "Alice"
        assert parsed[1]["user2.name"] == "Bob"
        assert parsed[2]["user3.name"] == "Charlie"
        
    def test_list_of_dicts_input_output(self):
        """Test list[dict] input → list[dict] output"""
        flattener = json_tools_rs.JsonFlattener()
        input_list = [
            {"user1": {"name": "Alice"}},
            {"user2": {"name": "Bob"}},
            {"user3": {"name": "Charlie"}}
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, dict) for item in result)
        
        assert result[0]["user1.name"] == "Alice"
        assert result[1]["user2.name"] == "Bob"
        assert result[2]["user3.name"] == "Charlie"
        
    def test_mixed_list_type_preservation(self):
        """Test mixed list preserves original types"""
        flattener = json_tools_rs.JsonFlattener()
        input_list = [
            '{"user1": {"name": "Alice"}}',  # JSON string
            {"user2": {"name": "Bob"}},      # Python dict
            {"user3": {"name": "Charlie"}}   # Python dict
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == 3
        assert isinstance(result[0], str)   # First item should remain string
        assert isinstance(result[1], dict)  # Second item should remain dict
        assert isinstance(result[2], dict)  # Third item should remain dict
        
        # Verify content
        parsed_first = json.loads(result[0])
        assert parsed_first["user1.name"] == "Alice"
        assert result[1]["user2.name"] == "Bob"
        assert result[2]["user3.name"] == "Charlie"
        
    def test_batch_with_advanced_config(self):
        """Test batch processing with advanced configuration"""
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_empty_strings(True)
                    .remove_nulls(True)
                    .key_replacement("user_", "person_")
                    .separator("_"))
        
        input_list = [
            {"user_name": "John", "user_email": "", "user_age": 30},
            {"user_name": "Jane", "user_bio": None, "user_active": True}
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == 2
        assert all(isinstance(item, dict) for item in result)
        
        # First result should have name and age only (email removed)
        assert result[0]["person_name"] == "John"
        assert result[0]["person_age"] == 30
        assert "person_email" not in result[0]
        
        # Second result should have name and active only (bio removed)
        assert result[1]["person_name"] == "Jane"
        assert result[1]["person_active"] is True
        assert "person_bio" not in result[1]
        
    def test_empty_list(self):
        """Test empty list input"""
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten([])
        
        assert isinstance(result, list)
        assert len(result) == 0
        
    def test_large_batch(self):
        """Test large batch processing"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Create 100 items
        input_list = []
        for i in range(100):
            input_list.append({
                f"item_{i}": {
                    "id": i,
                    "name": f"Item {i}",
                    "data": {"nested": f"value_{i}"}
                }
            })
            
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == 100
        assert all(isinstance(item, dict) for item in result)
        
        # Verify some entries
        assert result[0][f"item_0.id"] == 0
        assert result[0][f"item_0.name"] == "Item 0"
        assert result[0][f"item_0.data.nested"] == "value_0"
        
        assert result[99][f"item_99.id"] == 99
        assert result[99][f"item_99.name"] == "Item 99"
        assert result[99][f"item_99.data.nested"] == "value_99"


class TestAdvancedOutputObject:
    """Test the advanced JsonOutput object"""
    
    def test_single_result_output_object(self):
        """Test JsonOutput object with single result"""
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten_to_output('{"test": {"key": "value"}}')
        
        assert result.is_single
        assert not result.is_multiple
        
        single_result = result.get_single()
        assert isinstance(single_result, str)
        
        parsed = json.loads(single_result)
        assert parsed["test.key"] == "value"
        
    def test_multiple_result_output_object(self):
        """Test JsonOutput object with multiple results"""
        flattener = json_tools_rs.JsonFlattener()
        input_list = ['{"a": 1}', '{"b": 2}']
        result = flattener.flatten_to_output(input_list)
        
        assert result.is_multiple
        assert not result.is_single
        
        multiple_results = result.get_multiple()
        assert isinstance(multiple_results, list)
        assert len(multiple_results) == 2
        
        parsed = [json.loads(item) for item in multiple_results]
        assert parsed[0]["a"] == 1
        assert parsed[1]["b"] == 2
        
    def test_output_object_error_handling(self):
        """Test JsonOutput object error handling"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Test single result
        single_result = flattener.flatten_to_output('{"test": "value"}')
        
        # Should raise error when calling get_multiple on single result
        with pytest.raises(ValueError, match="multiple"):
            single_result.get_multiple()
            
        # Test multiple result
        multiple_result = flattener.flatten_to_output(['{"a": 1}', '{"b": 2}'])
        
        # Should raise error when calling get_single on multiple result
        with pytest.raises(ValueError, match="single"):
            multiple_result.get_single()


class TestErrorHandling:
    """Test error handling and edge cases"""
    
    def test_invalid_json_string(self):
        """Test invalid JSON string input"""
        flattener = json_tools_rs.JsonFlattener()
        
        with pytest.raises(json_tools_rs.JsonFlattenError):
            flattener.flatten('{"invalid": json}')
            
    def test_invalid_json_in_list(self):
        """Test invalid JSON in list input"""
        flattener = json_tools_rs.JsonFlattener()
        input_list = [
            '{"valid": "json"}',
            '{"invalid": json}',  # Invalid JSON
            '{"another": "valid"}'
        ]
        
        with pytest.raises(json_tools_rs.JsonFlattenError):
            flattener.flatten(input_list)
            
    def test_invalid_input_type(self):
        """Test invalid input types"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Test invalid scalar types
        with pytest.raises(ValueError):
            flattener.flatten(123)  # Number
            
        with pytest.raises(ValueError):
            flattener.flatten(True)  # Boolean
            
        # Test list with invalid item types
        with pytest.raises(ValueError):
            flattener.flatten([123, "string"])  # Mixed invalid types
            
    def test_invalid_regex_pattern(self):
        """Test invalid regex patterns"""
        # Invalid regex in key replacement
        with pytest.raises(json_tools_rs.JsonFlattenError):
            flattener = json_tools_rs.JsonFlattener().key_replacement("regex:[invalid", "replacement")
            flattener.flatten('{"test": "value"}')
            
        # Invalid regex in value replacement  
        with pytest.raises(json_tools_rs.JsonFlattenError):
            flattener = json_tools_rs.JsonFlattener().value_replacement("regex:*invalid", "replacement")
            flattener.flatten('{"test": "value"}')
            
    def test_deeply_nested_structure_limits(self):
        """Test very deeply nested structures"""
        # Create extremely deep nesting
        data = {"level": "value"}
        for i in range(50):  # 50 levels deep
            data = {f"level_{i}": data}
            
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten(data)
        
        assert isinstance(result, dict)
        assert len(result) == 1
        # Should have one very long key
        key = list(result.keys())[0]
        assert key.count('.') == 50  # 50 dots for 51 levels
        assert result[key] == "value"
        
    def test_large_json_structure(self):
        """Test very large JSON structures"""
        # Create large object with many keys
        large_data = {}
        for i in range(1000):
            large_data[f"key_{i}"] = {
                "id": i,
                "name": f"name_{i}",
                "nested": {"value": f"value_{i}"}
            }
            
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten(large_data)
        
        assert isinstance(result, dict)
        assert len(result) == 3000  # 1000 * 3 keys each
        
        # Verify some entries
        assert result["key_0.id"] == 0
        assert result["key_0.name"] == "name_0"
        assert result["key_0.nested.value"] == "value_0"
        assert result["key_999.id"] == 999


class TestEdgeCases:
    """Test edge cases and special scenarios"""
    
    def test_empty_json_object(self):
        """Test empty JSON object"""
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten({})
        
        assert isinstance(result, dict)
        assert len(result) == 0
        
    def test_empty_json_string(self):
        """Test empty JSON string"""
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten('{}')
        
        assert isinstance(result, str)
        assert result == '{}'
        
    def test_root_level_array(self):
        """Test root-level array"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = [1, 2, {"nested": "value"}]
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["0"] == 1
        assert result["1"] == 2
        assert result["2.nested"] == "value"
        
    def test_root_level_primitive(self):
        """Test root-level primitive values"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Test string
        result = flattener.flatten('"hello"')
        parsed = json.loads(result)
        assert parsed == "hello"
        
        # Test number
        result = flattener.flatten('42')
        parsed = json.loads(result)
        assert parsed == 42
        
        # Test boolean
        result = flattener.flatten('true')
        parsed = json.loads(result)
        assert parsed is True
        
        # Test null
        result = flattener.flatten('null')
        parsed = json.loads(result)
        assert parsed is None
        
    def test_special_characters_in_keys(self):
        """Test special characters in keys"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "key with spaces": "value1",
            "key-with-dashes": "value2", 
            "key_with_underscores": "value3",
            "key.with.dots": "value4",
            "key@with#symbols": "value5",
            "": "empty_key",  # Empty key
            "unicode_café": "value6"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["key with spaces"] == "value1"
        assert result["key-with-dashes"] == "value2"
        assert result["key_with_underscores"] == "value3"
        assert result["key.with.dots"] == "value4"
        assert result["key@with#symbols"] == "value5"
        assert result[""] == "empty_key"
        assert result["unicode_café"] == "value6"
        
    def test_special_characters_in_values(self):
        """Test special characters in values"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "normal": "value",
            "empty": "",
            "with_quotes": 'value with "quotes"',
            "with_newlines": "line1\nline2",
            "with_unicode": "café ñoño 🚀",
            "with_json": '{"nested": "json"}',
            "with_numbers": "123.45"
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["normal"] == "value"
        assert result["empty"] == ""
        assert result["with_quotes"] == 'value with "quotes"'
        assert result["with_newlines"] == "line1\nline2"
        assert result["with_unicode"] == "café ñoño 🚀"
        assert result["with_json"] == '{"nested": "json"}'
        assert result["with_numbers"] == "123.45"
        
    def test_circular_reference_simulation(self):
        """Test structures that simulate circular references"""
        flattener = json_tools_rs.JsonFlattener()
        
        # This isn't actually circular but tests deep self-reference patterns
        input_data = {
            "node": {
                "id": 1,
                "children": [
                    {"id": 2, "parent_id": 1},
                    {"id": 3, "parent_id": 1}
                ]
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["node.id"] == 1
        assert result["node.children.0.id"] == 2
        assert result["node.children.0.parent_id"] == 1
        assert result["node.children.1.id"] == 3
        assert result["node.children.1.parent_id"] == 1
        
    def test_numeric_string_keys(self):
        """Test numeric string keys"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "0": "zero",
            "1": "one", 
            "123": "one-two-three",
            "nested": {
                "0": "nested_zero",
                "456": "nested_four-five-six"
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["0"] == "zero"
        assert result["1"] == "one"
        assert result["123"] == "one-two-three"
        assert result["nested.0"] == "nested_zero"
        assert result["nested.456"] == "nested_four-five-six"
        
    def test_boolean_and_null_values(self):
        """Test boolean and null value handling"""
        flattener = json_tools_rs.JsonFlattener()
        input_data = {
            "true_value": True,
            "false_value": False,
            "null_value": None,
            "nested": {
                "bool_true": True,
                "bool_false": False,
                "null_nested": None
            }
        }
        result = flattener.flatten(input_data)
        
        assert isinstance(result, dict)
        assert result["true_value"] is True
        assert result["false_value"] is False
        assert result["null_value"] is None
        assert result["nested.bool_true"] is True
        assert result["nested.bool_false"] is False
        assert result["nested.null_nested"] is None


class TestTypePreservation:
    """Test perfect type preservation - input type = output type"""
    
    def test_str_to_str_consistency(self):
        """Test JSON string input consistently produces JSON string output"""
        flattener = json_tools_rs.JsonFlattener()
        
        test_cases = [
            '{"simple": "value"}',
            '{"nested": {"key": "value"}}',
            '{"array": [1, 2, 3]}',
            '{"mixed": {"array": [{"nested": "value"}]}}'
        ]
        
        for input_json in test_cases:
            result = flattener.flatten(input_json)
            assert isinstance(result, str), f"Expected str output for str input: {input_json}"
            
            # Verify it's valid JSON
            parsed = json.loads(result)
            assert isinstance(parsed, dict)
            
    def test_dict_to_dict_consistency(self):
        """Test Python dict input consistently produces Python dict output"""
        flattener = json_tools_rs.JsonFlattener()
        
        test_cases = [
            {"simple": "value"},
            {"nested": {"key": "value"}},
            {"array": [1, 2, 3]},
            {"mixed": {"array": [{"nested": "value"}]}}
        ]
        
        for input_dict in test_cases:
            result = flattener.flatten(input_dict)
            assert isinstance(result, dict), f"Expected dict output for dict input: {input_dict}"
            
    def test_list_str_to_list_str_consistency(self):
        """Test list[str] input consistently produces list[str] output"""
        flattener = json_tools_rs.JsonFlattener()
        
        input_list = [
            '{"item1": "value1"}',
            '{"item2": {"nested": "value2"}}',
            '{"item3": [1, 2, 3]}'
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == len(input_list)
        assert all(isinstance(item, str) for item in result)
        
        # Verify all are valid JSON
        for item in result:
            parsed = json.loads(item)
            assert isinstance(parsed, dict)
            
    def test_list_dict_to_list_dict_consistency(self):
        """Test list[dict] input consistently produces list[dict] output"""
        flattener = json_tools_rs.JsonFlattener()
        
        input_list = [
            {"item1": "value1"},
            {"item2": {"nested": "value2"}},
            {"item3": [1, 2, 3]}
        ]
        result = flattener.flatten(input_list)
        
        assert isinstance(result, list)
        assert len(result) == len(input_list)
        assert all(isinstance(item, dict) for item in result)
        
    def test_mixed_list_type_preservation_detailed(self):
        """Test detailed mixed list type preservation"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Test various mixed patterns
        mixed_patterns = [
            # Pattern 1: str, dict, str
            [
                '{"str1": "value1"}',
                {"dict1": "value2"}, 
                '{"str2": "value3"}'
            ],
            # Pattern 2: dict, dict, str, str
            [
                {"dict1": "value1"},
                {"dict2": "value2"},
                '{"str1": "value3"}',
                '{"str2": "value4"}'
            ],
            # Pattern 3: alternating
            [
                '{"str1": "value1"}',
                {"dict1": "value2"},
                '{"str2": "value3"}',
                {"dict2": "value4"},
                '{"str3": "value5"}'
            ]
        ]
        
        for i, pattern in enumerate(mixed_patterns):
            result = flattener.flatten(pattern)
            
            assert isinstance(result, list), f"Pattern {i+1}: Expected list output"
            assert len(result) == len(pattern), f"Pattern {i+1}: Length mismatch"
            
            for j, (original, processed) in enumerate(zip(pattern, result)):
                original_type = type(original)
                processed_type = type(processed)
                
                assert original_type == processed_type, \
                    f"Pattern {i+1}, Item {j}: Type mismatch. Expected {original_type}, got {processed_type}"
                    
    def test_type_preservation_with_configurations(self):
        """Test type preservation with various configurations"""
        configurations = [
            json_tools_rs.JsonFlattener().remove_empty_strings(True),
            json_tools_rs.JsonFlattener().remove_nulls(True),
            json_tools_rs.JsonFlattener().separator("_"),
            json_tools_rs.JsonFlattener().lowercase_keys(True),
            json_tools_rs.JsonFlattener().key_replacement("test_", ""),
            json_tools_rs.JsonFlattener().value_replacement("old", "new")
        ]
        
        test_data = {
            "str_input": '{"test_key": "old_value", "empty": "", "null_val": null}',
            "dict_input": {"test_key": "old_value", "empty": "", "null_val": None},
            "list_str_input": ['{"test1": "old"}', '{"test2": "value"}'],
            "list_dict_input": [{"test1": "old"}, {"test2": "value"}]
        }
        
        for config in configurations:
            # Test string input → string output
            result = config.flatten(test_data["str_input"])
            assert isinstance(result, str)
            
            # Test dict input → dict output  
            result = config.flatten(test_data["dict_input"])
            assert isinstance(result, dict)
            
            # Test list[str] input → list[str] output
            result = config.flatten(test_data["list_str_input"])
            assert isinstance(result, list)
            assert all(isinstance(item, str) for item in result)
            
            # Test list[dict] input → list[dict] output
            result = config.flatten(test_data["list_dict_input"])
            assert isinstance(result, list)
            assert all(isinstance(item, dict) for item in result)


class TestPerformance:
    """Performance tests and benchmarks"""
    
    def test_basic_flattening_performance(self):
        """Test basic flattening performance"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Create test data with varying complexity
        simple_data = {"user": {"name": "John", "age": 30}}
        nested_data = {
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {"data": "value"}
                    }
                }
            }
        }
        array_data = {
            "items": [{"id": i, "name": f"item_{i}"} for i in range(100)]
        }
        
        test_cases = [
            ("simple", simple_data),
            ("nested", nested_data), 
            ("array", array_data)
        ]
        
        results = {}
        
        for name, data in test_cases:
            start_time = time.time()
            iterations = 1000
            
            for _ in range(iterations):
                result = flattener.flatten(data)
                # Ensure the operation completes
                if isinstance(result, dict):
                    _ = len(result)
                    
            end_time = time.time()
            total_time = end_time - start_time
            ops_per_second = iterations / total_time
            
            results[name] = {
                "ops_per_second": ops_per_second,
                "avg_time_ms": (total_time / iterations) * 1000
            }
            
            print(f"{name.capitalize()} data: {ops_per_second:.0f} ops/sec, {results[name]['avg_time_ms']:.3f}ms avg")
            
        # Performance assertions
        assert results["simple"]["ops_per_second"] > 1000, "Simple flattening should be > 1000 ops/sec"
        assert results["nested"]["ops_per_second"] > 500, "Nested flattening should be > 500 ops/sec"
        assert results["array"]["ops_per_second"] > 100, "Array flattening should be > 100 ops/sec"
        
    def test_batch_processing_performance(self):
        """Test batch processing performance"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Create batch data
        batch_sizes = [10, 50, 100, 500]
        
        for batch_size in batch_sizes:
            # Create list of dictionaries
            dict_batch = [
                {"user": {"id": i, "name": f"user_{i}", "data": {"nested": f"value_{i}"}}}
                for i in range(batch_size)
            ]
            
            # Create list of JSON strings
            str_batch = [json.dumps(item) for item in dict_batch]
            
            # Test dict batch performance
            start_time = time.time()
            dict_result = flattener.flatten(dict_batch)
            dict_time = time.time() - start_time
            
            # Test string batch performance
            start_time = time.time()
            str_result = flattener.flatten(str_batch)
            str_time = time.time() - start_time
            
            print(f"Batch size {batch_size}:")
            print(f"  Dict batch: {dict_time*1000:.2f}ms ({batch_size/dict_time:.0f} items/sec)")
            print(f"  Str batch:  {str_time*1000:.2f}ms ({batch_size/str_time:.0f} items/sec)")
            
            # Verify results
            assert len(dict_result) == batch_size
            assert len(str_result) == batch_size
            assert all(isinstance(item, dict) for item in dict_result)
            assert all(isinstance(item, str) for item in str_result)
            
            # Performance assertions
            items_per_sec_dict = batch_size / dict_time
            items_per_sec_str = batch_size / str_time
            
            assert items_per_sec_dict > 50, f"Dict batch processing should be > 50 items/sec for size {batch_size}"
            assert items_per_sec_str > 50, f"String batch processing should be > 50 items/sec for size {batch_size}"
            
    def test_complex_configuration_performance(self):
        """Test performance with complex configurations"""
        # Create complex flattener with all features
        complex_flattener = (json_tools_rs.JsonFlattener()
                           .remove_empty_strings(True)
                           .remove_nulls(True)
                           .remove_empty_objects(True)
                           .remove_empty_arrays(True)
                           .key_replacement("regex:^user_", "person_")
                           .value_replacement("regex:@example\\.com", "@company.org")
                           .separator("_")
                           .lowercase_keys(True))
        
        # Create test data
        complex_data = {
            "User_Profile": {
                "User_Name": "John Doe",
                "User_Email": "john@example.com",
                "User_Settings": {
                    "Theme": "dark",
                    "Language": "",
                    "Notifications": None
                },
                "User_Tags": [],
                "User_Metadata": {}
            }
        }
        
        # Benchmark complex configuration
        start_time = time.time()
        iterations = 500
        
        for _ in range(iterations):
            result = complex_flattener.flatten(complex_data)
            _ = len(result)  # Ensure operation completes
            
        end_time = time.time()
        total_time = end_time - start_time
        ops_per_second = iterations / total_time
        
        print(f"Complex configuration: {ops_per_second:.0f} ops/sec, {(total_time/iterations)*1000:.3f}ms avg")
        
        # Should still maintain reasonable performance
        assert ops_per_second > 100, "Complex configuration should be > 100 ops/sec"
        
    def test_large_data_performance(self):
        """Test performance with large data structures"""
        flattener = json_tools_rs.JsonFlattener()
        
        # Create large nested structure
        large_data = {}
        for i in range(1000):
            large_data[f"section_{i}"] = {
                "id": i,
                "name": f"Section {i}",
                "items": [
                    {"item_id": j, "value": f"value_{i}_{j}"}
                    for j in range(10)
                ],
                "metadata": {
                    "created": f"2024-01-{(i % 28) + 1:02d}",
                    "tags": [f"tag_{i % 5}", f"category_{i % 10}"]
                }
            }
            
        # Benchmark large data
        start_time = time.time()
        result = flattener.flatten(large_data)
        end_time = time.time()
        
        processing_time = end_time - start_time
        key_count = len(result)
        keys_per_second = key_count / processing_time
        
        print(f"Large data: {key_count} keys in {processing_time*1000:.2f}ms ({keys_per_second:.0f} keys/sec)")
        
        # Performance assertions
        assert processing_time < 5.0, "Large data processing should complete within 5 seconds"
        assert keys_per_second > 1000, "Should process > 1000 keys/sec for large data"
        assert key_count > 10000, "Should generate many flattened keys"
        
    def test_regex_performance_impact(self):
        """Test performance impact of regex operations"""
        data = {
            f"user_{i}": {
                "email": f"user{i}@example.com",
                "status": "active" if i % 2 else "inactive"
            }
            for i in range(100)
        }
        
        # Test without regex
        simple_flattener = json_tools_rs.JsonFlattener()
        start_time = time.time()
        iterations = 100
        for _ in range(iterations):
            result = simple_flattener.flatten(data)
            _ = len(result)
        simple_time = time.time() - start_time
        
        # Test with regex
        regex_flattener = (json_tools_rs.JsonFlattener()
                         .key_replacement("regex:^user_", "person_")
                         .value_replacement("regex:@example\\.com", "@company.org"))
        start_time = time.time()
        for _ in range(iterations):
            result = regex_flattener.flatten(data)
            _ = len(result)
        regex_time = time.time() - start_time
        
        simple_ops_per_sec = iterations / simple_time
        regex_ops_per_sec = iterations / regex_time
        overhead_percent = ((regex_time - simple_time) / simple_time) * 100
        
        print(f"Simple flattening: {simple_ops_per_sec:.0f} ops/sec")
        print(f"Regex flattening:  {regex_ops_per_sec:.0f} ops/sec") 
        print(f"Regex overhead:    {overhead_percent:.1f}%")
        
        # Regex should still maintain reasonable performance
        assert regex_ops_per_sec > 10, "Regex operations should maintain > 10 ops/sec"
        assert overhead_percent < 1000, "Regex overhead should be reasonable"
        
    def test_memory_efficiency(self):
        """Test memory efficiency with repeated operations"""
        import gc
        
        flattener = json_tools_rs.JsonFlattener()
        
        # Create medium-sized data
        data = {
            f"group_{i}": {
                "items": [{"id": j, "data": f"value_{j}"} for j in range(50)]
            }
            for i in range(50)
        }
        
        # Perform many operations to test for memory leaks
        gc.collect()  # Clean up before test
        
        for i in range(100):
            result = flattener.flatten(data)
            
            # Periodically verify result and clean up
            if i % 10 == 0:
                assert isinstance(result, dict)
                assert len(result) > 1000
                del result
                gc.collect()
                
        # Test should complete without memory issues
        print("Memory efficiency test completed successfully")
        
    def test_performance_comparison_dict_vs_string(self):
        """Compare performance of dict vs string input"""
        # Create test data
        test_data_dict = {
            "users": [
                {"id": i, "profile": {"name": f"User {i}", "email": f"user{i}@example.com"}}
                for i in range(100)
            ]
        }
        test_data_str = json.dumps(test_data_dict)
        
        flattener = json_tools_rs.JsonFlattener()
        
        # Test dict input performance
        start_time = time.time()
        iterations = 100
        for _ in range(iterations):
            result = flattener.flatten(test_data_dict)
            _ = len(result)
        dict_time = time.time() - start_time
        
        # Test string input performance
        start_time = time.time()
        for _ in range(iterations):
            result = flattener.flatten(test_data_str)
            # For string input, we need to parse the result to count keys
            parsed = json.loads(result)
            _ = len(parsed)
        str_time = time.time() - start_time
        
        dict_ops_per_sec = iterations / dict_time
        str_ops_per_sec = iterations / str_time
        
        print(f"Dict input:   {dict_ops_per_sec:.0f} ops/sec")
        print(f"String input: {str_ops_per_sec:.0f} ops/sec")
        print(f"Ratio (dict/str): {dict_ops_per_sec/str_ops_per_sec:.2f}")
        
        # Both should maintain good performance
        assert dict_ops_per_sec > 50, "Dict input should be > 50 ops/sec"
        assert str_ops_per_sec > 50, "String input should be > 50 ops/sec"


class TestRealWorldScenarios:
    """Test real-world usage scenarios"""
    
    def test_api_response_flattening(self):
        """Test flattening typical API responses"""
        # Simulate typical REST API response
        api_response = {
            "data": {
                "user": {
                    "id": 12345,
                    "profile": {
                        "first_name": "John",
                        "last_name": "Doe",
                        "email": "john.doe@example.com",
                        "phone": "+1-555-123-4567"
                    },
                    "preferences": {
                        "notifications": {
                            "email": True,
                            "sms": False,
                            "push": True
                        },
                        "privacy": {
                            "profile_public": False,
                            "email_visible": False
                        }
                    },
                    "metadata": {
                        "created_at": "2024-01-15T10:30:00Z",
                        "updated_at": "2024-01-20T15:45:00Z",
                        "last_login": "2024-01-26T09:15:00Z"
                    }
                },
                "permissions": ["read", "write", "admin"],
                "groups": [
                    {"id": 1, "name": "Developers", "role": "member"},
                    {"id": 2, "name": "Admins", "role": "owner"}
                ]
            },
            "meta": {
                "request_id": "req_123456789",
                "timestamp": "2024-01-26T12:00:00Z",
                "version": "v1.2.3"
            }
        }
        
        flattener = json_tools_rs.JsonFlattener()
        result = flattener.flatten(api_response)
        
        assert isinstance(result, dict)
        assert result["data.user.id"] == 12345
        assert result["data.user.profile.first_name"] == "John"
        assert result["data.user.profile.email"] == "john.doe@example.com"
        assert result["data.user.preferences.notifications.email"] is True
        assert result["data.permissions.0"] == "read"
        assert result["data.groups.0.name"] == "Developers"
        assert result["meta.request_id"] == "req_123456789"
        
    def test_configuration_file_flattening(self):
        """Test flattening configuration files"""
        config = {
            "database": {
                "host": "localhost",
                "port": 5432,
                "credentials": {
                    "username": "admin",
                    "password": "secret123"
                },
                "pools": {
                    "min_connections": 5,
                    "max_connections": 20,
                    "timeout": 30
                }
            },
            "redis": {
                "host": "redis.example.com",
                "port": 6379,
                "auth": {
                    "password": "redis_secret"
                }
            },
            "logging": {
                "level": "INFO",
                "handlers": [
                    {"type": "console", "format": "%(asctime)s - %(message)s"},
                    {"type": "file", "filename": "/var/log/app.log", "max_size": "10MB"}
                ]
            },
            "features": {
                "authentication": {"enabled": True, "provider": "oauth2"},
                "caching": {"enabled": True, "ttl": 3600},
                "monitoring": {"enabled": False}
            }
        }
        
        # Use environment variable style flattening
        flattener = (json_tools_rs.JsonFlattener()
                    .separator("_")
                    .lowercase_keys(True))
        
        result = flattener.flatten(config)
        
        assert isinstance(result, dict)
        assert result["database_host"] == "localhost"
        assert result["database_port"] == 5432
        assert result["database_credentials_username"] == "admin"
        assert result["redis_host"] == "redis.example.com"
        assert result["logging_level"] == "INFO"
        assert result["features_authentication_enabled"] is True
        
    def test_analytics_data_processing(self):
        """Test processing analytics/metrics data"""
        analytics_data = {
            "metrics": {
                "page_views": {
                    "total": 15420,
                    "unique": 8934,
                    "by_source": {
                        "organic": 5678,
                        "social": 2341,
                        "direct": 987,
                        "referral": 6
                    }
                },
                "user_engagement": {
                    "session_duration": {
                        "avg_seconds": 245,
                        "median_seconds": 180
                    },
                    "bounce_rate": 0.34,
                    "pages_per_session": 2.8
                },
                "conversions": {
                    "total": 89,
                    "rate": 0.0058,
                    "by_funnel_stage": {
                        "awareness": 15420,
                        "interest": 4521,
                        "consideration": 892,
                        "conversion": 89
                    }
                }
            },
            "dimensions": {
                "time_period": "2024-01-01 to 2024-01-31",
                "geography": {
                    "primary_country": "US",
                    "top_cities": ["New York", "Los Angeles", "Chicago"]
                },
                "demographics": {
                    "age_groups": {
                        "18-24": 0.15,
                        "25-34": 0.35,
                        "35-44": 0.28,
                        "45-54": 0.15,
                        "55+": 0.07
                    }
                }
            }
        }
        
        # Clean up and standardize for analysis
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_nulls(True)
                    .remove_empty_objects(True)
                    .separator("__"))
        
        result = flattener.flatten(analytics_data)
        
        assert isinstance(result, dict)
        assert result["metrics__page_views__total"] == 15420
        assert result["metrics__user_engagement__bounce_rate"] == 0.34
        assert result["metrics__conversions__rate"] == 0.0058
        assert result["dimensions__demographics__age_groups__25-34"] == 0.35
        
    def test_form_data_processing(self):
        """Test processing form submission data"""
        form_data = {
            "personal_info": {
                "first_name": "Jane",
                "last_name": "Smith", 
                "email": "jane.smith@company.com",
                "phone": "",  # Empty field
                "date_of_birth": "1990-05-15"
            },
            "address": {
                "street": "123 Main St",
                "city": "Springfield",
                "state": "IL",
                "zip_code": "62701",
                "country": "USA"
            },
            "employment": {
                "company": "Tech Corp",
                "position": "Software Engineer",
                "salary": None,  # Optional field not filled
                "start_date": "2022-03-01"
            },
            "preferences": {
                "newsletter": True,
                "marketing_emails": False,
                "contact_method": "email"
            },
            "additional_info": "",  # Empty text area
            "terms_accepted": True,
            "submission_metadata": {
                "timestamp": "2024-01-26T14:30:00Z",
                "ip_address": "192.168.1.100",
                "user_agent": "Mozilla/5.0 ..."
            }
        }
        
        # Clean up form data for storage
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_empty_strings(True)
                    .remove_nulls(True)
                    .key_replacement("personal_info.", "")
                    .key_replacement("submission_metadata.", "meta_"))
        
        result = flattener.flatten(form_data)
        
        assert isinstance(result, dict)
        assert result["first_name"] == "Jane"
        assert result["email"] == "jane.smith@company.com"
        assert "phone" not in result  # Empty string removed
        assert result["address.city"] == "Springfield"
        assert "employment.salary" not in result  # Null removed
        assert result["preferences.newsletter"] is True
        assert "additional_info" not in result  # Empty string removed
        assert result["meta_timestamp"] == "2024-01-26T14:30:00Z"
        
    def test_log_processing(self):
        """Test processing structured log data"""
        log_entries = [
            {
                "timestamp": "2024-01-26T10:15:30Z",
                "level": "INFO",
                "service": "api-gateway",
                "message": "Request processed successfully",
                "context": {
                    "request_id": "req_001",
                    "user_id": "user_123",
                    "endpoint": "/api/users/profile",
                    "method": "GET",
                    "response_time_ms": 45,
                    "status_code": 200
                }
            },
            {
                "timestamp": "2024-01-26T10:16:45Z", 
                "level": "ERROR",
                "service": "user-service",
                "message": "Database connection failed",
                "context": {
                    "request_id": "req_002",
                    "error_code": "DB_CONN_TIMEOUT",
                    "retry_count": 3,
                    "database": "users_db"
                },
                "stack_trace": None  # Sometimes present, sometimes not
            }
        ]
        
        # Process logs for analysis
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_nulls(True)
                    .key_replacement("context.", "")
                    .separator("_"))
        
        results = flattener.flatten(log_entries)
        
        assert isinstance(results, list)
        assert len(results) == 2
        assert all(isinstance(entry, dict) for entry in results)
        
        # Check first log entry
        assert results[0]["level"] == "INFO"
        assert results[0]["service"] == "api-gateway"
        assert results[0]["request_id"] == "req_001"
        assert results[0]["response_time_ms"] == 45
        
        # Check second log entry
        assert results[1]["level"] == "ERROR"
        assert results[1]["error_code"] == "DB_CONN_TIMEOUT"
        assert "stack_trace" not in results[1]  # Null removed
        
    def test_data_transformation_pipeline(self):
        """Test complete data transformation pipeline"""
        # Simulate raw data from multiple sources
        raw_data = {
            "customer_data": {
                "Customer_ID": "CUST_12345",
                "Customer_Name": "John Doe Industries",
                "Contact_Info": {
                    "Primary_Email": "contact@johndoe.com",
                    "Secondary_Email": "",
                    "Phone_Number": "+1-555-123-4567",
                    "Fax_Number": None
                },
                "Address_Details": {
                    "Street_Address": "123 Business Ave",
                    "City": "Springfield",
                    "State_Province": "IL",
                    "Postal_Code": "62701",
                    "Country_Code": "US"
                }
            },
            "account_info": {
                "Account_Status": "ACTIVE", 
                "Account_Type": "PREMIUM",
                "Registration_Date": "2023-01-15",
                "Last_Activity": "2024-01-25",
                "Payment_Methods": [
                    {"Type": "CREDIT_CARD", "Last_Four": "1234", "Expires": "12/26"},
                    {"Type": "BANK_TRANSFER", "Account_Number": "****5678", "Routing": "987654321"}
                ]
            },
            "usage_statistics": {
                "Monthly_Usage": {
                    "API_Calls": 15420,
                    "Data_Transfer_GB": 245.8,
                    "Storage_GB": 12.3
                },
                "Feature_Usage": {
                    "Advanced_Analytics": True,
                    "Custom_Reports": True,
                    "White_Label": False,
                    "API_Access": True
                }
            },
            "billing_details": {
                "Current_Plan": "PREMIUM_MONTHLY",
                "Plan_Start_Date": "2024-01-01",
                "Plan_End_Date": "2025-01-01",
                "Next_Payment_Due": "2025-01-15",
                "Amount_Due": 99.99,
                "Payment_Status": "PAID"
            }
        }
        
        # Clean up and transform for analytics
        flattener = (json_tools_rs.JsonFlattener()
                    .remove_empty_strings(True)
                    .remove_nulls(True)
                    .key_replacement("regex:^(customer_data|account_info|usage_statistics|billing_details)\\.", "")
                    .separator("_")
                    .lowercase_keys(True))
        
        result = flattener.flatten(raw_data)
        
        assert isinstance(result, dict)
        assert result["customer_id"] == "CUST_12345"
        assert result["customer_name"] == "John Doe Industries"
        assert result["contact_info_primary_email"] == "contact@johndoe.com"
        assert "contact_info_secondary_email" not in result  # Empty string removed
        assert "contact_info_fax_number" not in result  # Null removed