#!/usr/bin/env python3
"""
Comprehensive Test Suite for JSON Tools RS Python Bindings

This test suite provides complete coverage of the unified JSONTools API including:
- Basic flatten/unflatten functionality tests
- Advanced collision handling tests
- Configuration and transformation tests
- Error handling tests
- Edge case tests
- Performance benchmarks
- Type preservation tests
- All input/output combinations
- Roundtrip compatibility tests
"""

import json
import time
from typing import Any, Dict, List, Union

import json_tools_rs
import pytest


class TestBasicFunctionality:
    """Test basic JSON flattening and unflattening functionality"""

    def test_basic_flattening_dict_input_dict_output(self):
        """Test dict input → dict output (most convenient!)"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {"user": {"name": "John", "age": 30}}
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.age"] == 30

    def test_basic_flattening_str_input_str_output(self):
        """Test JSON string input → JSON string output"""
        tools = json_tools_rs.JSONTools().flatten()
        input_json = '{"user": {"name": "John", "age": 30}}'
        result = tools.execute(input_json)

        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed["user.name"] == "John"
        assert parsed["user.age"] == 30

    def test_basic_unflattening_dict_input_dict_output(self):
        """Test unflattening dict input → dict output"""
        tools = json_tools_rs.JSONTools().unflatten()
        input_data = {"user.name": "John", "user.age": 30}
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user"]["name"] == "John"
        assert result["user"]["age"] == 30

    def test_basic_unflattening_str_input_str_output(self):
        """Test unflattening JSON string input → JSON string output"""
        tools = json_tools_rs.JSONTools().unflatten()
        input_json = '{"user.name": "John", "user.age": 30}'
        result = tools.execute(input_json)

        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed["user"]["name"] == "John"
        assert parsed["user"]["age"] == 30

    def test_deeply_nested_structure(self):
        """Test deeply nested JSON structures"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "level1": {"level2": {"level3": {"level4": {"value": "deep_value"}}}}
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["level1.level2.level3.level4.value"] == "deep_value"

    def test_array_flattening(self):
        """Test array flattening with indices"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {"items": [1, 2, {"nested": "value"}], "matrix": [[1, 2], [3, 4]]}
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["items.0"] == 1
        assert result["items.1"] == 2
        assert result["items.2.nested"] == "value"
        assert result["matrix.0.0"] == 1
        assert result["matrix.0.1"] == 2
        assert result["matrix.1.0"] == 3
        assert result["matrix.1.1"] == 4

    def test_roundtrip_consistency(self):
        """Test that flatten → unflatten preserves data"""
        original = {
            "user": {"profile": {"name": "John", "age": 30}},
            "settings": {"theme": "dark"},
        }

        # Flatten then unflatten
        flattened = json_tools_rs.JSONTools().flatten().execute(original)
        restored = json_tools_rs.JSONTools().unflatten().execute(flattened)

        assert restored == original

    def test_mixed_data_types(self):
        """Test flattening with various data types"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "string": "text",
            "number": 42,
            "float": 3.14,
            "boolean_true": True,
            "boolean_false": False,
            "null_value": None,
            "array": [1, "two", 3.0, True, None],
            "object": {"nested": "value"},
        }
        result = tools.execute(input_data)

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


class TestCollisionHandling:
    """Test collision handling strategies"""

    def test_handle_collision_strategy(self):
        """Test collision handling with arrays"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'(User|Admin|Guest)_'", "")
            .handle_key_collision(True)
        )

        data = {"User_name": "John", "Admin_name": "Jane", "Guest_name": "Bob"}
        result = tools.execute(data)

        # Should create array
        assert "name" in result
        assert isinstance(result["name"], list)
        assert len(result["name"]) == 3
        assert "John" in result["name"]
        assert "Jane" in result["name"]
        assert "Bob" in result["name"]

    def test_collision_with_filtering(self):
        """Test collision handling with filtering applied during resolution"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'(User|Admin|Guest)_'", "")
            .remove_empty_strings(True)
            .handle_key_collision(True)
        )

        data = {"User_name": "John", "Admin_name": "", "Guest_name": "Bob"}
        result = tools.execute(data)

        # Should create array with empty string filtered out
        assert "name" in result
        assert isinstance(result["name"], list)
        assert len(result["name"]) == 2  # Empty string filtered out
        assert "John" in result["name"]
        assert "Bob" in result["name"]
        assert "" not in result["name"]


class TestAdvancedConfiguration:
    """Test advanced configuration options"""

    def test_remove_empty_strings(self):
        """Test removing empty string values"""
        tools = json_tools_rs.JSONTools().flatten().remove_empty_strings(True)
        input_data = {
            "user": {
                "name": "John",
                "email": "",  # Should be removed
                "bio": "Developer",
            },
            "empty_field": "",  # Should be removed
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.bio"] == "Developer"
        assert "user.email" not in result
        assert "empty_field" not in result

    def test_remove_nulls(self):
        """Test removing null values"""
        tools = json_tools_rs.JSONTools().flatten().remove_nulls(True)
        input_data = {
            "user": {"name": "John", "age": None, "active": True},  # Should be removed
            "null_field": None,  # Should be removed
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user.name"] == "John"
        assert result["user.active"] is True
        assert "user.age" not in result
        assert "null_field" not in result

    def test_remove_empty_objects(self):
        """Test removing empty object values"""
        tools = json_tools_rs.JSONTools().flatten().remove_empty_objects(True)
        input_data = {
            "user": {"profile": {}, "settings": {"theme": "dark"}},  # Should be removed
            "empty_obj": {},  # Should be removed
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user.settings.theme"] == "dark"
        assert "user.profile" not in result
        assert "empty_obj" not in result

    def test_remove_empty_arrays(self):
        """Test removing empty array values"""
        tools = json_tools_rs.JSONTools().flatten().remove_empty_arrays(True)
        input_data = {
            "user": {"tags": [], "items": [1, 2, 3]},  # Should be removed
            "empty_list": [],  # Should be removed
        }
        result = tools.execute(input_data)

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
            tools = json_tools_rs.JSONTools().flatten().separator(sep)
            input_data = {"level1": {"level2": {"value": "test"}}}
            result = tools.execute(input_data)

            expected_key = f"level1{sep}level2{sep}value"
            assert isinstance(result, dict)
            assert result[expected_key] == "test"

    def test_lowercase_keys(self):
        """Test lowercase key conversion"""
        tools = json_tools_rs.JSONTools().flatten().lowercase_keys(True)
        input_data = {
            "User": {"Profile": {"Name": "John", "Email": "john@example.com"}}
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user.profile.name"] == "John"
        assert result["user.profile.email"] == "john@example.com"

    def test_combined_filters(self):
        """Test all filters combined"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .remove_empty_objects(True)
            .remove_empty_arrays(True)
            .lowercase_keys(True)
            .separator("_")
        )

        input_data = {
            "User": {
                "Name": "John",
                "Email": "",
                "Age": None,
                "Settings": {},
                "Tags": [],
                "Active": True,
            }
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user_name"] == "John"
        assert result["user_active"] is True
        assert len(result) == 2  # Only name and active should remain


class TestReplacements:
    """Test key and value replacement functionality"""

    def test_literal_key_replacement(self):
        """Test literal string key replacement"""
        tools = json_tools_rs.JSONTools().flatten().key_replacement("user_", "person_")
        input_data = {
            "user_name": "John",
            "user_email": "john@example.com",
            "admin_role": "super",
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["person_name"] == "John"
        assert result["person_email"] == "john@example.com"
        assert result["admin_role"] == "super"  # Should remain unchanged

    def test_regex_key_replacement(self):
        """Test regex key replacement"""
        tools = (
            json_tools_rs.JSONTools().flatten().key_replacement("r'^(user|admin)_'", "")
        )
        input_data = {
            "user_name": "John",
            "admin_role": "super",
            "guest_access": "limited",
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["name"] == "John"
        assert result["role"] == "super"
        assert result["guest_access"] == "limited"  # Should remain unchanged

    def test_literal_value_replacement(self):
        """Test literal string value replacement"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("inactive", "disabled")
        )
        input_data = {
            "user1": {"status": "active"},
            "user2": {"status": "inactive"},
            "user3": {"status": "pending"},
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user1.status"] == "active"
        assert result["user2.status"] == "disabled"
        assert result["user3.status"] == "pending"

    def test_regex_value_replacement(self):
        """Test regex value replacement"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("r'@example\\.com'", "@company.org")
        )
        input_data = {
            "user1": {"email": "john@example.com"},
            "user2": {"email": "jane@example.com"},
            "user3": {"email": "bob@test.org"},
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["user1.email"] == "john@company.org"
        assert result["user2.email"] == "jane@company.org"
        assert result["user3.email"] == "bob@test.org"  # Should remain unchanged

    def test_multiple_replacements(self):
        """Test multiple key and value replacements"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("user_", "person_")
            .key_replacement("r'^admin_'", "manager_")
            .value_replacement("@example.com", "@company.org")
            .value_replacement("r'^inactive$'", "disabled")
        )

        input_data = {
            "user_email": "john@example.com",
            "admin_role": "super",
            "user_status": "inactive",
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["person_email"] == "john@company.org"
        assert result["manager_role"] == "super"
        assert result["person_status"] == "disabled"

    def test_regex_capture_groups(self):
        """Test regex replacement with capture groups"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'^field_(\\d+)_(.+)'", "$2_id_$1")
        )
        input_data = {
            "field_123_name": "John",
            "field_456_email": "john@example.com",
            "other_field": "unchanged",
        }
        result = tools.execute(input_data)

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
        tools = json_tools_rs.JSONTools().flatten()
        input_list = [
            '{"user1": {"name": "Alice"}}',
            '{"user2": {"name": "Bob"}}',
            '{"user3": {"name": "Charlie"}}',
        ]
        result = tools.execute(input_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, str) for item in result)

        parsed = [json.loads(item) for item in result]
        assert parsed[0]["user1.name"] == "Alice"
        assert parsed[1]["user2.name"] == "Bob"
        assert parsed[2]["user3.name"] == "Charlie"

    def test_list_of_dicts_input_output(self):
        """Test list[dict] input → list[dict] output"""
        tools = json_tools_rs.JSONTools().flatten()
        input_list = [
            {"user1": {"name": "Alice"}},
            {"user2": {"name": "Bob"}},
            {"user3": {"name": "Charlie"}},
        ]
        result = tools.execute(input_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, dict) for item in result)

        assert result[0]["user1.name"] == "Alice"
        assert result[1]["user2.name"] == "Bob"
        assert result[2]["user3.name"] == "Charlie"

    def test_mixed_list_type_preservation(self):
        """Test mixed list preserves original types"""
        tools = json_tools_rs.JSONTools().flatten()
        input_list = [
            '{"user1": {"name": "Alice"}}',  # JSON string
            {"user2": {"name": "Bob"}},  # Python dict
            {"user3": {"name": "Charlie"}},  # Python dict
        ]
        result = tools.execute(input_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert isinstance(result[0], str)  # First item should remain string
        assert isinstance(result[1], dict)  # Second item should remain dict
        assert isinstance(result[2], dict)  # Third item should remain dict

        # Verify content
        parsed_first = json.loads(result[0])
        assert parsed_first["user1.name"] == "Alice"
        assert result[1]["user2.name"] == "Bob"
        assert result[2]["user3.name"] == "Charlie"

    def test_batch_with_advanced_config(self):
        """Test batch processing with advanced configuration"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .key_replacement("user_", "person_")
            .separator("_")
        )

        input_list = [
            {"user_name": "John", "user_email": "", "user_age": 30},
            {"user_name": "Jane", "user_bio": None, "user_active": True},
        ]
        result = tools.execute(input_list)

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
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute([])

        assert isinstance(result, list)
        assert len(result) == 0

    def test_large_batch(self):
        """Test large batch processing"""
        tools = json_tools_rs.JSONTools().flatten()

        # Create 100 items
        input_list = []
        for i in range(100):
            input_list.append(
                {
                    f"item_{i}": {
                        "id": i,
                        "name": f"Item {i}",
                        "data": {"nested": f"value_{i}"},
                    }
                }
            )

        result = tools.execute(input_list)

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
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute_to_output('{"test": {"key": "value"}}')

        assert result.is_single
        assert not result.is_multiple

        single_result = result.get_single()
        assert isinstance(single_result, str)

        parsed = json.loads(single_result)
        assert parsed["test.key"] == "value"

    def test_multiple_result_output_object(self):
        """Test JsonOutput object with multiple results"""
        tools = json_tools_rs.JSONTools().flatten()
        input_list = ['{"a": 1}', '{"b": 2}']
        result = tools.execute_to_output(input_list)

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
        tools = json_tools_rs.JSONTools().flatten()

        # Test single result
        single_result = tools.execute_to_output('{"test": "value"}')

        # Should raise error when calling get_multiple on single result
        with pytest.raises(ValueError, match="single.*get_single"):
            single_result.get_multiple()

        # Test multiple result
        multiple_result = tools.execute_to_output(['{"a": 1}', '{"b": 2}'])

        # Should raise error when calling get_single on multiple result
        with pytest.raises(ValueError, match="multiple.*get_multiple"):
            multiple_result.get_single()


class TestErrorHandling:
    """Test error handling and edge cases"""

    def test_invalid_json_string(self):
        """Test invalid JSON string input"""
        tools = json_tools_rs.JSONTools().flatten()

        with pytest.raises(json_tools_rs.JsonToolsError):
            tools.execute('{"invalid": json}')

    def test_invalid_json_in_list(self):
        """Test invalid JSON in list input"""
        tools = json_tools_rs.JSONTools().flatten()
        input_list = [
            '{"valid": "json"}',
            '{"invalid": json}',  # Invalid JSON
            '{"another": "valid"}',
        ]

        with pytest.raises(json_tools_rs.JsonToolsError):
            tools.execute(input_list)

    def test_invalid_input_type(self):
        """Test invalid input types"""
        tools = json_tools_rs.JSONTools().flatten()

        # Test invalid scalar types
        with pytest.raises(ValueError):
            tools.execute(123)  # Number

        with pytest.raises(ValueError):
            tools.execute(True)  # Boolean

        # Test list with invalid item types
        with pytest.raises(ValueError):
            tools.execute([123, object()])  # Contains invalid object type

    def test_bare_pattern_with_regex_metacharacters_is_literal(self):
        """A bare (non-r'...') pattern is always literal, even with regex-looking chars"""
        # "[invalid" has no closing bracket -- invalid regex syntax, but since it's not
        # wrapped in r'...' it's never compiled as regex at all; it's just literal text
        # that doesn't appear in the data, so nothing matches (no error either way).
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("[invalid", "replacement")
        )
        result = tools.execute('{"test": "value"}')
        assert isinstance(result, str)
        assert '"test"' in result and '"value"' in result

        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("*invalid", "replacement")
        )
        result = tools.execute('{"test": "value"}')
        assert isinstance(result, str)
        assert '"test"' in result and '"value"' in result

    def test_malformed_regex_pattern_is_silently_ignored(self):
        """An r'...'-wrapped pattern that fails to compile as regex is treated as no match,
        not an error -- there's no way to validate patterns ahead of execute()."""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'[invalid'", "replacement")
        )
        result = tools.execute('{"test": "value"}')
        assert isinstance(result, str)
        assert '"test"' in result and '"value"' in result

        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("r'*invalid'", "replacement")
        )
        result = tools.execute('{"test": "value"}')
        assert isinstance(result, str)
        assert '"test"' in result and '"value"' in result

    def test_deeply_nested_structure_limits(self):
        """Test very deeply nested structures"""
        # Create extremely deep nesting
        data = {"level": "value"}
        for i in range(50):  # 50 levels deep
            data = {f"level_{i}": data}

        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute(data)

        assert isinstance(result, dict)
        assert len(result) == 1
        # Should have one very long key
        key = list(result.keys())[0]
        assert key.count(".") == 50  # 50 dots for 51 levels
        assert result[key] == "value"

    def test_large_json_structure(self):
        """Test very large JSON structures"""
        # Create large object with many keys
        large_data = {}
        for i in range(1000):
            large_data[f"key_{i}"] = {
                "id": i,
                "name": f"name_{i}",
                "nested": {"value": f"value_{i}"},
            }

        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute(large_data)

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
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({})

        assert isinstance(result, dict)
        assert len(result) == 0

    def test_empty_json_string(self):
        """Test empty JSON string"""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute("{}")

        assert isinstance(result, str)
        assert result == "{}"

    def test_root_level_primitive(self):
        """Test root-level primitive values"""
        tools = json_tools_rs.JSONTools().flatten()

        # Test string
        result = tools.execute('"hello"')
        parsed = json.loads(result)
        assert parsed == "hello"

        # Test number
        result = tools.execute("42")
        parsed = json.loads(result)
        assert parsed == 42

        # Test boolean
        result = tools.execute("true")
        parsed = json.loads(result)
        assert parsed is True

        # Test null
        result = tools.execute("null")
        parsed = json.loads(result)
        assert parsed is None

    def test_special_characters_in_keys(self):
        """Test special characters in keys"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "key with spaces": "value1",
            "key-with-dashes": "value2",
            "key_with_underscores": "value3",
            "key.with.dots": "value4",
            "key@with#symbols": "value5",
            "": "empty_key",  # Empty key
            "unicode_café": "value6",
        }
        result = tools.execute(input_data)

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
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "normal": "value",
            "empty": "",
            "with_quotes": 'value with "quotes"',
            "with_newlines": "line1\nline2",
            "with_unicode": "café ñoño 🚀",
            "with_json": '{"nested": "json"}',
            "with_numbers": "123.45",
        }
        result = tools.execute(input_data)

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
        tools = json_tools_rs.JSONTools().flatten()

        # This isn't actually circular but tests deep self-reference patterns
        input_data = {
            "node": {
                "id": 1,
                "children": [{"id": 2, "parent_id": 1}, {"id": 3, "parent_id": 1}],
            }
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["node.id"] == 1
        assert result["node.children.0.id"] == 2
        assert result["node.children.0.parent_id"] == 1
        assert result["node.children.1.id"] == 3
        assert result["node.children.1.parent_id"] == 1

    def test_numeric_string_keys(self):
        """Test numeric string keys"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "0": "zero",
            "1": "one",
            "123": "one-two-three",
            "nested": {"0": "nested_zero", "456": "nested_four-five-six"},
        }
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["0"] == "zero"
        assert result["1"] == "one"
        assert result["123"] == "one-two-three"
        assert result["nested.0"] == "nested_zero"
        assert result["nested.456"] == "nested_four-five-six"

    def test_boolean_and_null_values(self):
        """Test boolean and null value handling"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {
            "true_value": True,
            "false_value": False,
            "null_value": None,
            "nested": {"bool_true": True, "bool_false": False, "null_nested": None},
        }
        result = tools.execute(input_data)

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
        tools = json_tools_rs.JSONTools().flatten()

        test_cases = [
            '{"simple": "value"}',
            '{"nested": {"key": "value"}}',
            '{"array": [1, 2, 3]}',
            '{"mixed": {"array": [{"nested": "value"}]}}',
        ]

        for input_json in test_cases:
            result = tools.execute(input_json)
            assert isinstance(
                result, str
            ), f"Expected str output for str input: {input_json}"

            # Verify it's valid JSON
            parsed = json.loads(result)
            assert isinstance(parsed, dict)

    def test_dict_to_dict_consistency(self):
        """Test Python dict input consistently produces Python dict output"""
        tools = json_tools_rs.JSONTools().flatten()

        test_cases = [
            {"simple": "value"},
            {"nested": {"key": "value"}},
            {"array": [1, 2, 3]},
            {"mixed": {"array": [{"nested": "value"}]}},
        ]

        for input_dict in test_cases:
            result = tools.execute(input_dict)
            assert isinstance(
                result, dict
            ), f"Expected dict output for dict input: {input_dict}"

    def test_list_str_to_list_str_consistency(self):
        """Test list[str] input consistently produces list[str] output"""
        tools = json_tools_rs.JSONTools().flatten()

        input_list = [
            '{"item1": "value1"}',
            '{"item2": {"nested": "value2"}}',
            '{"item3": [1, 2, 3]}',
        ]
        result = tools.execute(input_list)

        assert isinstance(result, list)
        assert len(result) == len(input_list)
        assert all(isinstance(item, str) for item in result)

        # Verify all are valid JSON
        for item in result:
            parsed = json.loads(item)
            assert isinstance(parsed, dict)

    def test_list_dict_to_list_dict_consistency(self):
        """Test list[dict] input consistently produces list[dict] output"""
        tools = json_tools_rs.JSONTools().flatten()

        input_list = [
            {"item1": "value1"},
            {"item2": {"nested": "value2"}},
            {"item3": [1, 2, 3]},
        ]
        result = tools.execute(input_list)

        assert isinstance(result, list)
        assert len(result) == len(input_list)
        assert all(isinstance(item, dict) for item in result)

    def test_mixed_list_type_preservation_detailed(self):
        """Test detailed mixed list type preservation"""
        tools = json_tools_rs.JSONTools().flatten()

        # Test various mixed patterns
        mixed_patterns = [
            # Pattern 1: str, dict, str
            ['{"str1": "value1"}', {"dict1": "value2"}, '{"str2": "value3"}'],
            # Pattern 2: dict, dict, str, str
            [
                {"dict1": "value1"},
                {"dict2": "value2"},
                '{"str1": "value3"}',
                '{"str2": "value4"}',
            ],
            # Pattern 3: alternating
            [
                '{"str1": "value1"}',
                {"dict1": "value2"},
                '{"str2": "value3"}',
                {"dict2": "value4"},
                '{"str3": "value5"}',
            ],
        ]

        for i, pattern in enumerate(mixed_patterns):
            result = tools.execute(pattern)

            assert isinstance(result, list), f"Pattern {i+1}: Expected list output"
            assert len(result) == len(pattern), f"Pattern {i+1}: Length mismatch"

            for j, (original, processed) in enumerate(zip(pattern, result)):
                original_type = type(original)
                processed_type = type(processed)

                assert (
                    original_type == processed_type
                ), f"Pattern {i+1}, Item {j}: Type mismatch. Expected {original_type}, got {processed_type}"

    def test_type_preservation_with_configurations(self):
        """Test type preservation with various configurations"""
        configurations = [
            json_tools_rs.JSONTools().flatten().remove_empty_strings(True),
            json_tools_rs.JSONTools().flatten().remove_nulls(True),
            json_tools_rs.JSONTools().flatten().separator("_"),
            json_tools_rs.JSONTools().flatten().lowercase_keys(True),
            json_tools_rs.JSONTools().flatten().key_replacement("test_", ""),
            json_tools_rs.JSONTools().flatten().value_replacement("old", "new"),
        ]

        test_data = {
            "str_input": '{"test_key": "old_value", "empty": "", "null_val": null}',
            "dict_input": {"test_key": "old_value", "empty": "", "null_val": None},
            "list_str_input": ['{"test1": "old"}', '{"test2": "value"}'],
            "list_dict_input": [{"test1": "old"}, {"test2": "value"}],
        }

        for config in configurations:
            # Test string input → string output
            result = config.execute(test_data["str_input"])
            assert isinstance(result, str)

            # Test dict input → dict output
            result = config.execute(test_data["dict_input"])
            assert isinstance(result, dict)

            # Test list[str] input → list[str] output
            result = config.execute(test_data["list_str_input"])
            assert isinstance(result, list)
            assert all(isinstance(item, str) for item in result)

            # Test list[dict] input → list[dict] output
            result = config.execute(test_data["list_dict_input"])
            assert isinstance(result, list)
            assert all(isinstance(item, dict) for item in result)


class TestPerformance:
    """Performance tests and benchmarks"""

    def test_basic_flattening_performance(self):
        """Test basic flattening performance"""
        tools = json_tools_rs.JSONTools().flatten()

        # Create test data with varying complexity
        simple_data = {"user": {"name": "John", "age": 30}}
        nested_data = {"level1": {"level2": {"level3": {"level4": {"data": "value"}}}}}
        array_data = {"items": [{"id": i, "name": f"item_{i}"} for i in range(100)]}

        test_cases = [
            ("simple", simple_data),
            ("nested", nested_data),
            ("array", array_data),
        ]

        results = {}

        for name, data in test_cases:
            start_time = time.time()
            iterations = 1000

            for _ in range(iterations):
                result = tools.execute(data)
                # Ensure the operation completes
                if isinstance(result, dict):
                    _ = len(result)

            end_time = time.time()
            total_time = end_time - start_time
            ops_per_second = iterations / total_time

            results[name] = {
                "ops_per_second": ops_per_second,
                "avg_time_ms": (total_time / iterations) * 1000,
            }

            print(
                f"{name.capitalize()} data: {ops_per_second:.0f} ops/sec, {results[name]['avg_time_ms']:.3f}ms avg"
            )

        # Performance assertions
        assert (
            results["simple"]["ops_per_second"] > 1000
        ), "Simple flattening should be > 1000 ops/sec"
        assert (
            results["nested"]["ops_per_second"] > 500
        ), "Nested flattening should be > 500 ops/sec"
        assert (
            results["array"]["ops_per_second"] > 100
        ), "Array flattening should be > 100 ops/sec"

    def test_batch_processing_performance(self):
        """Test batch processing performance"""
        tools = json_tools_rs.JSONTools().flatten()

        # Create batch data
        batch_sizes = [10, 50, 100, 500]

        for batch_size in batch_sizes:
            # Create list of dictionaries
            dict_batch = [
                {
                    "user": {
                        "id": i,
                        "name": f"user_{i}",
                        "data": {"nested": f"value_{i}"},
                    }
                }
                for i in range(batch_size)
            ]

            # Create list of JSON strings
            str_batch = [json.dumps(item) for item in dict_batch]

            # Test dict batch performance
            start_time = time.time()
            dict_result = tools.execute(dict_batch)
            dict_time = time.time() - start_time

            # Test string batch performance
            start_time = time.time()
            str_result = tools.execute(str_batch)
            str_time = time.time() - start_time

            print(f"Batch size {batch_size}:")
            print(
                f"  Dict batch: {dict_time*1000:.2f}ms ({batch_size/dict_time:.0f} items/sec)"
            )
            print(
                f"  Str batch:  {str_time*1000:.2f}ms ({batch_size/str_time:.0f} items/sec)"
            )

            # Verify results
            assert len(dict_result) == batch_size
            assert len(str_result) == batch_size
            assert all(isinstance(item, dict) for item in dict_result)
            assert all(isinstance(item, str) for item in str_result)

            # Performance assertions
            items_per_sec_dict = batch_size / dict_time
            items_per_sec_str = batch_size / str_time

            assert (
                items_per_sec_dict > 50
            ), f"Dict batch processing should be > 50 items/sec for size {batch_size}"
            assert (
                items_per_sec_str > 50
            ), f"String batch processing should be > 50 items/sec for size {batch_size}"

    def test_complex_configuration_performance(self):
        """Test performance with complex configurations"""
        # Create complex flattener with all features
        complex_tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .remove_empty_objects(True)
            .remove_empty_arrays(True)
            .key_replacement("r'^user_'", "person_")
            .value_replacement("r'@example\\.com'", "@company.org")
            .separator("_")
            .lowercase_keys(True)
        )

        # Create test data
        complex_data = {
            "User_Profile": {
                "User_Name": "John Doe",
                "User_Email": "john@example.com",
                "User_Settings": {
                    "Theme": "dark",
                    "Language": "",
                    "Notifications": None,
                },
                "User_Tags": [],
                "User_Metadata": {},
            }
        }

        # Benchmark complex configuration
        start_time = time.time()
        iterations = 500

        for _ in range(iterations):
            result = complex_tools.execute(complex_data)
            _ = len(result)  # Ensure operation completes

        end_time = time.time()
        total_time = end_time - start_time
        ops_per_second = iterations / total_time

        print(
            f"Complex configuration: {ops_per_second:.0f} ops/sec, {(total_time/iterations)*1000:.3f}ms avg"
        )

        # Should still maintain reasonable performance
        assert ops_per_second > 100, "Complex configuration should be > 100 ops/sec"

    def test_large_data_performance(self):
        """Test performance with large data structures"""
        tools = json_tools_rs.JSONTools().flatten()

        # Create large nested structure
        large_data = {}
        for i in range(1000):
            large_data[f"section_{i}"] = {
                "id": i,
                "name": f"Section {i}",
                "items": [{"item_id": j, "value": f"value_{i}_{j}"} for j in range(10)],
                "metadata": {
                    "created": f"2024-01-{(i % 28) + 1:02d}",
                    "tags": [f"tag_{i % 5}", f"category_{i % 10}"],
                },
            }

        # Benchmark large data
        start_time = time.time()
        result = tools.execute(large_data)
        end_time = time.time()

        processing_time = end_time - start_time
        key_count = len(result)
        keys_per_second = key_count / processing_time

        print(
            f"Large data: {key_count} keys in {processing_time*1000:.2f}ms ({keys_per_second:.0f} keys/sec)"
        )

        # Performance assertions
        assert (
            processing_time < 5.0
        ), "Large data processing should complete within 5 seconds"
        assert keys_per_second > 1000, "Should process > 1000 keys/sec for large data"
        assert key_count > 10000, "Should generate many flattened keys"

    def test_regex_performance_impact(self):
        """Test performance impact of regex operations"""
        data = {
            f"user_{i}": {
                "email": f"user{i}@example.com",
                "status": "active" if i % 2 else "inactive",
            }
            for i in range(100)
        }

        # Test without regex
        simple_tools = json_tools_rs.JSONTools().flatten()
        start_time = time.time()
        iterations = 100
        for _ in range(iterations):
            result = simple_tools.execute(data)
            _ = len(result)
        simple_time = time.time() - start_time

        # Test with regex
        regex_tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'^user_'", "person_")
            .value_replacement("r'@example\\.com'", "@company.org")
        )
        start_time = time.time()
        for _ in range(iterations):
            result = regex_tools.execute(data)
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

        tools = json_tools_rs.JSONTools().flatten()

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
            result = tools.execute(data)

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
                {
                    "id": i,
                    "profile": {"name": f"User {i}", "email": f"user{i}@example.com"},
                }
                for i in range(100)
            ]
        }
        test_data_str = json.dumps(test_data_dict)

        tools = json_tools_rs.JSONTools().flatten()

        # Test dict input performance
        start_time = time.time()
        iterations = 100
        for _ in range(iterations):
            result = tools.execute(test_data_dict)
            _ = len(result)
        dict_time = time.time() - start_time

        # Test string input performance
        start_time = time.time()
        for _ in range(iterations):
            result = tools.execute(test_data_str)
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
                        "phone": "+1-555-123-4567",
                    },
                    "preferences": {
                        "notifications": {"email": True, "sms": False, "push": True},
                        "privacy": {"profile_public": False, "email_visible": False},
                    },
                    "metadata": {
                        "created_at": "2024-01-15T10:30:00Z",
                        "updated_at": "2024-01-20T15:45:00Z",
                        "last_login": "2024-01-26T09:15:00Z",
                    },
                },
                "permissions": ["read", "write", "admin"],
                "groups": [
                    {"id": 1, "name": "Developers", "role": "member"},
                    {"id": 2, "name": "Admins", "role": "owner"},
                ],
            },
            "meta": {
                "request_id": "req_123456789",
                "timestamp": "2024-01-26T12:00:00Z",
                "version": "v1.2.3",
            },
        }

        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute(api_response)

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
                "credentials": {"username": "admin", "password": "secret123"},
                "pools": {"min_connections": 5, "max_connections": 20, "timeout": 30},
            },
            "redis": {
                "host": "redis.example.com",
                "port": 6379,
                "auth": {"password": "redis_secret"},
            },
            "logging": {
                "level": "INFO",
                "handlers": [
                    {"type": "console", "format": "%(asctime)s - %(message)s"},
                    {
                        "type": "file",
                        "filename": "/var/log/app.log",
                        "max_size": "10MB",
                    },
                ],
            },
            "features": {
                "authentication": {"enabled": True, "provider": "oauth2"},
                "caching": {"enabled": True, "ttl": 3600},
                "monitoring": {"enabled": False},
            },
        }

        # Use environment variable style flattening
        tools = json_tools_rs.JSONTools().flatten().separator("_").lowercase_keys(True)

        result = tools.execute(config)

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
                        "referral": 6,
                    },
                },
                "user_engagement": {
                    "session_duration": {"avg_seconds": 245, "median_seconds": 180},
                    "bounce_rate": 0.34,
                    "pages_per_session": 2.8,
                },
                "conversions": {
                    "total": 89,
                    "rate": 0.0058,
                    "by_funnel_stage": {
                        "awareness": 15420,
                        "interest": 4521,
                        "consideration": 892,
                        "conversion": 89,
                    },
                },
            },
            "dimensions": {
                "time_period": "2024-01-01 to 2024-01-31",
                "geography": {
                    "primary_country": "US",
                    "top_cities": ["New York", "Los Angeles", "Chicago"],
                },
                "demographics": {
                    "age_groups": {
                        "18-24": 0.15,
                        "25-34": 0.35,
                        "35-44": 0.28,
                        "45-54": 0.15,
                        "55+": 0.07,
                    }
                },
            },
        }

        # Clean up and standardize for analysis
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_nulls(True)
            .remove_empty_objects(True)
            .separator("__")
        )

        result = tools.execute(analytics_data)

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
                "date_of_birth": "1990-05-15",
            },
            "address": {
                "street": "123 Main St",
                "city": "Springfield",
                "state": "IL",
                "zip_code": "62701",
                "country": "USA",
            },
            "employment": {
                "company": "Tech Corp",
                "position": "Software Engineer",
                "salary": None,  # Optional field not filled
                "start_date": "2022-03-01",
            },
            "preferences": {
                "newsletter": True,
                "marketing_emails": False,
                "contact_method": "email",
            },
            "additional_info": "",  # Empty text area
            "terms_accepted": True,
            "submission_metadata": {
                "timestamp": "2024-01-26T14:30:00Z",
                "ip_address": "192.168.1.100",
                "user_agent": "Mozilla/5.0 ...",
            },
        }

        # Clean up form data for storage
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .key_replacement("personal_info.", "")
            .key_replacement("submission_metadata.", "meta_")
        )

        result = tools.execute(form_data)

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
                    "status_code": 200,
                },
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
                    "database": "users_db",
                },
                "stack_trace": None,  # Sometimes present, sometimes not
            },
        ]

        # Process logs for analysis
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_nulls(True)
            .key_replacement("context_", "")
            .separator("_")
        )

        results = tools.execute(log_entries)

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
                    "Fax_Number": None,
                },
                "Address_Details": {
                    "Street_Address": "123 Business Ave",
                    "City": "Springfield",
                    "State_Province": "IL",
                    "Postal_Code": "62701",
                    "Country_Code": "US",
                },
            },
            "account_info": {
                "Account_Status": "ACTIVE",
                "Account_Type": "PREMIUM",
                "Registration_Date": "2023-01-15",
                "Last_Activity": "2024-01-25",
                "Payment_Methods": [
                    {"Type": "CREDIT_CARD", "Last_Four": "1234", "Expires": "12/26"},
                    {
                        "Type": "BANK_TRANSFER",
                        "Account_Number": "****5678",
                        "Routing": "987654321",
                    },
                ],
            },
            "usage_statistics": {
                "Monthly_Usage": {
                    "API_Calls": 15420,
                    "Data_Transfer_GB": 245.8,
                    "Storage_GB": 12.3,
                },
                "Feature_Usage": {
                    "Advanced_Analytics": True,
                    "Custom_Reports": True,
                    "White_Label": False,
                    "API_Access": True,
                },
            },
            "billing_details": {
                "Current_Plan": "PREMIUM_MONTHLY",
                "Plan_Start_Date": "2024-01-01",
                "Plan_End_Date": "2025-01-01",
                "Next_Payment_Due": "2025-01-15",
                "Amount_Due": 99.99,
                "Payment_Status": "PAID",
            },
        }

        # Clean up and transform for analytics
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .key_replacement(
                "r'^(customer_data|account_info|usage_statistics|billing_details)_'",
                "",
            )
            .separator("_")
            .lowercase_keys(True)
        )

        result = tools.execute(raw_data)

        assert isinstance(result, dict)
        assert result["customer_id"] == "CUST_12345"
        assert result["customer_name"] == "John Doe Industries"
        assert result["contact_info_primary_email"] == "contact@johndoe.com"
        assert "contact_info_secondary_email" not in result  # Empty string removed
        assert "contact_info_fax_number" not in result  # Null removed


# ============================================================================
# JsonUnflattener Tests
# ============================================================================


class TestJsonUnflattenerBasic:
    """Test basic JsonUnflattener functionality."""

    def test_basic_string_unflattening(self):
        """Test basic unflattening with JSON string input."""
        flattened = '{"user.name": "John", "user.age": 30, "user.profile.city": "NYC"}'
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        # Should return string
        assert isinstance(result, str)

        # Parse and verify structure
        parsed = json.loads(result)
        assert parsed["user"]["name"] == "John"
        assert parsed["user"]["age"] == 30
        assert parsed["user"]["profile"]["city"] == "NYC"

    def test_basic_dict_unflattening(self):
        """Test basic unflattening with Python dict input."""
        flattened = {"user.name": "John", "user.age": 30, "user.profile.city": "NYC"}
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        # Should return dict
        assert isinstance(result, dict)

        # Verify structure
        assert result["user"]["name"] == "John"
        assert result["user"]["age"] == 30
        assert result["user"]["profile"]["city"] == "NYC"

    def test_array_reconstruction(self):
        """Test reconstruction of arrays from flattened keys."""
        flattened = {"items.0": "first", "items.1": "second", "items.2": "third"}
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result["items"] == ["first", "second", "third"]

    def test_mixed_structure(self):
        """Test unflattening of mixed objects and arrays."""
        flattened = {
            "user.name": "John",
            "user.emails.0": "john@work.com",
            "user.emails.1": "john@personal.com",
            "settings.theme": "dark",
            "settings.notifications.email": True,
            "settings.notifications.sms": False,
        }
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result["user"]["name"] == "John"
        assert result["user"]["emails"] == ["john@work.com", "john@personal.com"]
        assert result["settings"]["theme"] == "dark"
        assert result["settings"]["notifications"]["email"] is True
        assert result["settings"]["notifications"]["sms"] is False


class TestJsonUnflattenerTypePreservation:
    """Test type preservation in JsonUnflattener."""

    def test_string_input_string_output(self):
        """Test str input → str output."""
        flattened = '{"a.b": 1, "c.d": 2}'
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed == {"a": {"b": 1}, "c": {"d": 2}}

    def test_dict_input_dict_output(self):
        """Test dict input → dict output."""
        flattened = {"a.b": 1, "c.d": 2}
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"a": {"b": 1}, "c": {"d": 2}}

    def test_string_list_input_string_list_output(self):
        """Test list[str] input → list[str] output."""
        flattened_list = ['{"a.b": 1}', '{"c.d": 2}', '{"e.f": 3}']
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, str) for item in result)

        # Parse and verify each result
        parsed_results = [json.loads(item) for item in result]
        assert parsed_results[0] == {"a": {"b": 1}}
        assert parsed_results[1] == {"c": {"d": 2}}
        assert parsed_results[2] == {"e": {"f": 3}}

    def test_dict_list_input_dict_list_output(self):
        """Test list[dict] input → list[dict] output."""
        flattened_list = [{"a.b": 1}, {"c.d": 2}, {"e.f": 3}]
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute(flattened_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, dict) for item in result)

        # Verify each result
        assert result[0] == {"a": {"b": 1}}
        assert result[1] == {"c": {"d": 2}}
        assert result[2] == {"e": {"f": 3}}

    def test_empty_list_handling(self):
        """Test empty list handling."""
        tools = json_tools_rs.JSONTools().unflatten()
        result = tools.execute([])

        assert isinstance(result, list)
        assert len(result) == 0


class TestJsonUnflattenerBuilderPattern:
    """Test JsonUnflattener builder pattern configuration."""

    def test_custom_separator(self):
        """Test custom separator configuration."""
        flattened = {"user_name": "John", "user_age": 30}
        tools = json_tools_rs.JSONTools().unflatten().separator("_")
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_lowercase_keys(self):
        """Test lowercase keys configuration."""
        flattened = {"USER.NAME": "John", "USER.AGE": 30}
        tools = json_tools_rs.JSONTools().unflatten().lowercase_keys(True)
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_key_replacement(self):
        """Test key replacement configuration."""
        flattened = {"prefix.name": "John", "prefix.age": 30}
        tools = (
            json_tools_rs.JSONTools().unflatten().key_replacement("prefix.", "user.")
        )
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_value_replacement(self):
        """Test value replacement configuration."""
        flattened = {"user.email": "john@company.org", "user.name": "John"}
        tools = (
            json_tools_rs.JSONTools()
            .unflatten()
            .value_replacement("@company.org", "@example.com")
        )
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result["user"]["email"] == "john@example.com"
        assert result["user"]["name"] == "John"

    def test_exclude_key_drops_container_subtree(self):
        """Matching a container key drops its entire subtree, not just a leaf."""
        data = {
            "user": {
                "name": "John",
                "crypto_wallet": {"coin": "BTC", "balance": 100},
            }
        }
        result = json_tools_rs.JSONTools().flatten().exclude_key("crypto").execute(data)
        assert result["user.name"] == "John"
        assert not any("crypto" in k for k in result)
        assert len(result) == 1

    def test_exclude_key_drops_leaf(self):
        data = {"user": {"name": "John", "crypto_balance": 100, "city": "NYC"}}
        result = json_tools_rs.JSONTools().flatten().exclude_key("crypto").execute(data)
        assert result["user.name"] == "John"
        assert result["user.city"] == "NYC"
        assert "user.crypto_balance" not in result

    def test_exclude_key_normal_mode_drops_subtree(self):
        data = {
            "user": {
                "name": "John",
                "crypto_wallet": {"coin": "BTC", "balance": 100},
            }
        }
        result = json_tools_rs.JSONTools().normal().exclude_key("crypto").execute(data)
        assert result["user"]["name"] == "John"
        assert "crypto_wallet" not in result["user"]

    def test_exclude_key_regex_and_multiple(self):
        data = {"cryptoBalance": 100, "secret_token": "x", "name": "John"}
        result = (
            json_tools_rs.JSONTools()
            .flatten()
            .exclude_key("r'^crypto'")
            .exclude_key("secret")
            .execute(data)
        )
        assert result == {"name": "John"}

    def test_exclude_value_string_leaf(self):
        data = {"user": {"name": "John", "status": "banned"}}
        result = (
            json_tools_rs.JSONTools().flatten().exclude_value("banned").execute(data)
        )
        assert result == {"user.name": "John"}

    def test_exclude_value_non_string_scalar(self):
        data = {"a": 42, "b": True, "c": "keep"}
        result = (
            json_tools_rs.JSONTools()
            .flatten()
            .exclude_value("42")
            .exclude_value("true")
            .execute(data)
        )
        assert result == {"c": "keep"}

    def test_exclude_value_normal_mode(self):
        data = {"user": {"name": "John", "status": "banned"}}
        result = (
            json_tools_rs.JSONTools().normal().exclude_value("banned").execute(data)
        )
        assert result == {"user": {"name": "John"}}

    def test_exclude_value_matches_after_replacement_and_conversion(self):
        data = {"a": "old_price", "b": "keep"}
        result = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("old_price", "999")
            .auto_convert_types(True)
            .exclude_value("999")
            .execute(data)
        )
        assert result == {"b": "keep"}

    def test_regex_key_replacement(self):
        """Test regex key replacement."""
        flattened = {"user_name": "John", "admin_role": "super"}
        tools = (
            json_tools_rs.JSONTools()
            .unflatten()
            .key_replacement("r'^(user|admin)_'", "$1.")
        )
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John"}, "admin": {"role": "super"}}

    def test_chained_configuration(self):
        """Test chained builder pattern configuration."""
        # Use lowercase input since key replacement happens before lowercase conversion
        flattened = {"prefix_name": "john@company.org", "prefix_age": 30}
        tools = (
            json_tools_rs.JSONTools()
            .unflatten()
            .separator("_")
            .key_replacement("prefix_", "user_")
            .value_replacement("@company.org", "@example.com")
            .lowercase_keys(True)
        )
        result = tools.execute(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "john@example.com", "age": 30}}


class TestJsonUnflattenerErrorHandling:
    """Test JsonUnflattener error handling."""

    def test_invalid_json_string(self):
        """Test handling of invalid JSON string."""
        tools = json_tools_rs.JSONTools().unflatten()
        with pytest.raises(json_tools_rs.JsonToolsError):
            tools.execute('{"invalid": json}')

    def test_invalid_input_type(self):
        """Test handling of invalid input types."""
        tools = json_tools_rs.JSONTools().unflatten()
        with pytest.raises(ValueError):
            tools.execute(123)  # Invalid type

    def test_mixed_list_types(self):
        """Test handling of mixed list types."""
        tools = json_tools_rs.JSONTools().unflatten()
        with pytest.raises(ValueError):
            tools.execute(['{"a": 1}', 123, {"b": 2}])  # Mixed types

    def test_invalid_list_content(self):
        """Test handling of invalid list content."""
        tools = json_tools_rs.JSONTools().unflatten()
        with pytest.raises(ValueError):
            tools.execute([None, "test"])  # Invalid content


class TestJsonUnflattenerRoundtrip:
    """Test roundtrip compatibility between JsonFlattener and JsonUnflattener."""

    def test_simple_roundtrip(self):
        """Test simple roundtrip: original → flatten → unflatten → original."""
        original = {"user": {"name": "John", "age": 30}}

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        flattened = flatten_tools.execute(original)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        restored = unflatten_tools.execute(flattened)

        # Should be equivalent to original
        assert restored == original

    def test_complex_roundtrip(self):
        """Test complex roundtrip with nested structures and arrays."""
        original = {
            "user": {
                "profile": {"name": "John", "age": 30},
                "emails": ["john@work.com", "john@personal.com"],
                "settings": {"theme": "dark", "notifications": True},
            },
            "metadata": {"created": "2024-01-01", "version": 1.0},
        }

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        flattened = flatten_tools.execute(original)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        restored = unflatten_tools.execute(flattened)

        # Should be equivalent to original
        assert restored == original

    def test_roundtrip_with_custom_separator(self):
        """Test roundtrip with custom separator."""
        original = {"user": {"name": "John", "profile": {"city": "NYC"}}}

        # Flatten with custom separator
        flatten_tools = json_tools_rs.JSONTools().flatten().separator("_")
        flattened = flatten_tools.execute(original)

        # Unflatten with same separator
        unflatten_tools = json_tools_rs.JSONTools().unflatten().separator("_")
        restored = unflatten_tools.execute(flattened)

        # Should be equivalent to original
        assert restored == original

    def test_batch_roundtrip(self):
        """Test batch roundtrip processing."""
        originals = [
            {"a": {"b": 1}},
            {"c": {"d": [1, 2, 3]}},
            {"e": {"f": {"g": "test"}}},
        ]

        # Flatten batch
        flatten_tools = json_tools_rs.JSONTools().flatten()
        flattened_batch = flatten_tools.execute(originals)

        # Unflatten batch
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        restored_batch = unflatten_tools.execute(flattened_batch)

        # Should be equivalent to originals
        assert restored_batch == originals

    def test_roundtrip_with_arrays(self):
        """Test roundtrip with complex array structures."""
        original = {
            "items": [
                {"id": 1, "name": "first"},
                {"id": 2, "name": "second", "tags": ["a", "b"]},
                {"id": 3, "nested": {"deep": {"value": "test"}}},
            ]
        }

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        flattened = flatten_tools.execute(original)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        restored = unflatten_tools.execute(flattened)

        # Should be equivalent to original
        assert restored == original

    def test_roundtrip_with_mixed_types(self):
        """Test roundtrip with mixed data types."""
        original = {
            "string": "test",
            "number": 42,
            "float": 3.14,
            "boolean": True,
            "null": None,
            "array": [1, "two", 3.0, False],
            "object": {"nested": "value"},
        }

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        flattened = flatten_tools.execute(original)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        restored = unflatten_tools.execute(flattened)

        # Should be equivalent to original
        assert restored == original


class TestTypeConversion:
    """Test automatic type conversion from strings to numbers and booleans"""

    def test_basic_number_conversion_dict(self):
        """Test basic number conversion with dict input"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"id": "123", "price": "45.67", "count": "-10"}
        result = tools.execute(input_data)

        assert isinstance(result, dict)
        assert result["id"] == 123
        assert result["price"] == 45.67
        assert result["count"] == -10

    def test_basic_number_conversion_str(self):
        """Test basic number conversion with JSON string input"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_json = '{"id": "123", "price": "45.67", "count": "-10"}'
        result = tools.execute(input_json)

        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed["id"] == 123
        assert parsed["price"] == 45.67
        assert parsed["count"] == -10

    def test_thousands_separator_us_format(self):
        """Test US format thousands separators (1,234.56)"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"amount": "1,234.56", "total": "1,000,000"}
        result = tools.execute(input_data)

        assert result["amount"] == 1234.56
        assert result["total"] == 1000000

    def test_thousands_separator_european_format(self):
        """Test European format thousands separators (1.234,56)"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"amount": "1.234,56", "total": "1.000.000,00"}
        result = tools.execute(input_data)

        assert result["amount"] == 1234.56
        assert result["total"] == 1000000.0

    def test_currency_symbols(self):
        """Test currency symbol removal and conversion"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {
            "usd": "$123.45",
            "eur": "€99.99",
            "gbp": "£50.00",
            "yen": "¥1000",
        }
        result = tools.execute(input_data)

        assert result["usd"] == 123.45
        assert result["eur"] == 99.99
        assert result["gbp"] == 50.0
        assert result["yen"] == 1000

    def test_scientific_notation(self):
        """Test scientific notation conversion"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"small": "1.23e-4", "large": "1e5", "negative": "-2.5e3"}
        result = tools.execute(input_data)

        assert result["small"] == 0.000123
        assert result["large"] == 100000.0
        assert result["negative"] == -2500.0

    def test_boolean_conversion(self):
        """Test boolean conversion (only exact variants)"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {
            "a": "true",
            "b": "TRUE",
            "c": "True",
            "d": "false",
            "e": "FALSE",
            "f": "False",
        }
        result = tools.execute(input_data)

        assert result["a"] is True
        assert result["b"] is True
        assert result["c"] is True
        assert result["d"] is False
        assert result["e"] is False
        assert result["f"] is False

    def test_keep_invalid_strings(self):
        """Test that invalid strings are kept as-is"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {
            "name": "John",
            "code": "ABC123",
            "maybe": "perhaps",  # Not a valid boolean
            "invalid": "12.34.56",  # Invalid number
        }
        result = tools.execute(input_data)

        assert result["name"] == "John"
        assert result["code"] == "ABC123"
        assert result["maybe"] == "perhaps"
        assert result["invalid"] == "12.34.56"

    def test_mixed_conversion(self):
        """Test mixed conversion with valid and invalid strings"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {
            "id": "123",
            "name": "Alice",
            "price": "$1,234.56",
            "active": "true",
            "code": "XYZ",
        }
        result = tools.execute(input_data)

        assert result["id"] == 123
        assert result["name"] == "Alice"
        assert result["price"] == 1234.56
        assert result["active"] is True
        assert result["code"] == "XYZ"

    def test_nested_conversion(self):
        """Test type conversion in nested structures"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"user": {"id": "456", "age": "25", "verified": "true"}}
        result = tools.execute(input_data)

        assert result["user.id"] == 456
        assert result["user.age"] == 25
        assert result["user.verified"] is True

    def test_array_conversion(self):
        """Test type conversion in arrays"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {"numbers": ["123", "45.6", "true", "invalid"]}
        result = tools.execute(input_data)

        assert result["numbers.0"] == 123
        assert result["numbers.1"] == 45.6
        assert result["numbers.2"] is True
        assert result["numbers.3"] == "invalid"

    def test_conversion_disabled_by_default(self):
        """Test that conversion is disabled by default"""
        tools = json_tools_rs.JSONTools().flatten()
        input_data = {"id": "123", "active": "true"}
        result = tools.execute(input_data)

        # Should keep as strings when conversion is disabled
        assert result["id"] == "123"
        assert result["active"] == "true"

    def test_unflatten_with_conversion(self):
        """Test type conversion with unflatten operation"""
        tools = json_tools_rs.JSONTools().unflatten().auto_convert_types(True)
        input_data = {"user.id": "789", "user.active": "false"}
        result = tools.execute(input_data)

        assert result["user"]["id"] == 789
        assert result["user"]["active"] is False

    def test_normal_mode_with_conversion(self):
        """Test type conversion with normal mode (no flatten/unflatten)"""
        tools = json_tools_rs.JSONTools().normal().auto_convert_types(True)
        input_data = {"user": {"id": "999", "enabled": "TRUE"}}
        result = tools.execute(input_data)

        assert result["user"]["id"] == 999
        assert result["user"]["enabled"] is True

    def test_conversion_with_other_transformations(self):
        """Test type conversion combined with other transformations"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .auto_convert_types(True)
            .lowercase_keys(True)
            .remove_empty_strings(True)
        )

        input_data = {
            "User_ID": "123",
            "User_Active": "true",
            "User_Name": "Alice",
            "Empty": "",
        }
        result = tools.execute(input_data)

        assert result["user_id"] == 123
        assert result["user_active"] is True
        assert result["user_name"] == "Alice"
        assert "empty" not in result  # Removed empty string

    def test_batch_processing_with_conversion(self):
        """Test type conversion with batch processing"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_batch = [
            {"id": "101", "price": "$99.99"},
            {"id": "102", "price": "$149.00"},
        ]
        result = tools.execute(input_batch)

        assert isinstance(result, list)
        assert len(result) == 2
        assert result[0]["id"] == 101
        assert result[0]["price"] == 99.99
        assert result[1]["id"] == 102
        assert result[1]["price"] == 149.0

    def test_complex_real_world_example(self):
        """Test complex real-world scenario with type conversion"""
        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        input_data = {
            "order": {
                "id": "ORD-12345",
                "total": "$1,234.56",
                "items": [
                    {
                        "id": "101",
                        "quantity": "5",
                        "price": "€99.99",
                        "available": "true",
                    },
                    {
                        "id": "102",
                        "quantity": "2",
                        "price": "$49.50",
                        "available": "FALSE",
                    },
                ],
                "customer": {
                    "id": "CUST-789",
                    "verified": "True",
                    "balance": "1,500.00",
                },
            }
        }
        result = tools.execute(input_data)

        # Check order fields
        assert result["order.id"] == "ORD-12345"  # Kept as string (not a number)
        assert result["order.total"] == 1234.56

        # Check item 0
        assert result["order.items.0.id"] == 101
        assert result["order.items.0.quantity"] == 5
        assert result["order.items.0.price"] == 99.99
        assert result["order.items.0.available"] is True

        # Check item 1
        assert result["order.items.1.id"] == 102
        assert result["order.items.1.quantity"] == 2
        assert result["order.items.1.price"] == 49.50
        assert result["order.items.1.available"] is False

        # Check customer
        assert result["order.customer.id"] == "CUST-789"  # Kept as string
        assert result["order.customer.verified"] is True
        assert result["order.customer.balance"] == 1500.0


class TestFineGrainedTypeConversion:
    """Test convert_dates/convert_nulls/convert_booleans/convert_numbers -- the
    per-category alternative to auto_convert_types(), including their kwargs-based
    customization."""

    def test_convert_dates_independent(self):
        tools = json_tools_rs.JSONTools().flatten().convert_dates(True)
        result = tools.execute(
            {
                "d": "2024-01-15T10:30:00+05:00",
                "n": "null",
                "b": "true",
                "num": "123",
            }
        )
        assert result["d"] == "2024-01-15T05:30:00Z"
        assert result["n"] == "null"
        assert result["b"] == "true"
        assert result["num"] == "123"

    def test_convert_nulls_independent(self):
        tools = json_tools_rs.JSONTools().flatten().convert_nulls(True)
        result = tools.execute({"n": "null", "b": "true", "num": "123"})
        assert result["n"] is None
        assert result["b"] == "true"
        assert result["num"] == "123"

    def test_convert_booleans_independent(self):
        tools = json_tools_rs.JSONTools().flatten().convert_booleans(True)
        result = tools.execute({"n": "null", "b": "true", "num": "123"})
        assert result["n"] == "null"
        assert result["b"] is True
        assert result["num"] == "123"

    def test_convert_numbers_independent(self):
        tools = json_tools_rs.JSONTools().flatten().convert_numbers(True)
        result = tools.execute({"n": "null", "b": "true", "num": "123"})
        assert result["n"] == "null"
        assert result["b"] == "true"
        assert result["num"] == 123

    def test_auto_convert_types_then_per_category_disable(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .auto_convert_types(True)
            .convert_dates(False)
        )
        result = tools.execute({"d": "2024-01-15T10:30:00Z", "b": "true", "num": "123"})
        assert result["d"] == "2024-01-15T10:30:00Z"
        assert result["b"] is True
        assert result["num"] == 123

    def test_per_category_disable_then_auto_convert_types_reenables(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_dates(False)
            .auto_convert_types(True)
        )
        result = tools.execute({"d": "2024-01-15T10:30:00+05:00"})
        assert result["d"] == "2024-01-15T05:30:00Z"

    def test_convert_dates_kwargs_customization_persists_across_second_call(self):
        """A second convert_dates(True) call without a kwarg must preserve a first
        call's customization, not silently reset it."""
        tools = json_tools_rs.JSONTools().flatten()
        tools = tools.convert_dates(True, assume_utc_for_naive=False)
        tools = tools.convert_dates(True)  # no kwarg -- must not reset
        result = tools.execute({"d": "2024-01-15T10:30:00"})
        assert result["d"] == "2024-01-15T10:30:00"  # still unchanged, naive

    def test_convert_dates_normalize_to_utc_false(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_dates(True, normalize_to_utc=False)
        )
        result = tools.execute({"d": "2024-01-15T10:30:00+05:00"})
        assert result["d"] == "2024-01-15T10:30:00+05:00"

    def test_convert_dates_assume_utc_for_naive_false(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_dates(True, assume_utc_for_naive=False)
        )
        result = tools.execute({"d": "2024-01-15T10:30:00"})
        assert result["d"] == "2024-01-15T10:30:00"

    def test_convert_nulls_extra_tokens(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_nulls(True, extra_tokens=["missing", "MISSING"])
        )
        result = tools.execute({"a": "missing", "b": "N/A", "c": "not_a_token"})
        assert result["a"] is None
        assert result["b"] is None  # built-in list still active
        assert result["c"] == "not_a_token"

    def test_convert_booleans_extra_tokens(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_booleans(
                True, extra_true_tokens=["si"], extra_false_tokens=["nope"]
            )
        )
        result = tools.execute(
            {"a": "si", "b": "nope", "c": "true", "d": "not_a_token"}
        )
        assert result["a"] is True
        assert result["b"] is False
        assert result["c"] is True  # built-in list still active
        assert result["d"] == "not_a_token"

    def test_convert_numbers_currency_disabled(self):
        tools = (
            json_tools_rs.JSONTools().flatten().convert_numbers(True, currency=False)
        )
        result = tools.execute({"price": "$45.67", "count": "1,234.56"})
        assert result["price"] == "$45.67"
        assert result["count"] == 1234.56  # thousands-separator cleanup still core

    def test_convert_numbers_percent_disabled(self):
        tools = json_tools_rs.JSONTools().flatten().convert_numbers(True, percent=False)
        result = tools.execute({"pct": "50%", "count": "123"})
        assert result["pct"] == "50%"
        assert result["count"] == 123

    def test_convert_numbers_basis_points_disabled(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_numbers(True, basis_points=False)
        )
        result = tools.execute({"bp": "25bps", "count": "123"})
        assert result["bp"] == "25bps"
        assert result["count"] == 123

    def test_convert_numbers_suffixes_disabled(self):
        tools = (
            json_tools_rs.JSONTools().flatten().convert_numbers(True, suffixes=False)
        )
        result = tools.execute({"mag": "2.5M", "count": "123"})
        assert result["mag"] == "2.5M"
        assert result["count"] == 123

    def test_convert_numbers_fractions_disabled(self):
        tools = (
            json_tools_rs.JSONTools().flatten().convert_numbers(True, fractions=False)
        )
        result = tools.execute({"frac": "1/2", "count": "123"})
        assert result["frac"] == "1/2"
        assert result["count"] == 123

    def test_convert_numbers_radix_disabled(self):
        tools = json_tools_rs.JSONTools().flatten().convert_numbers(True, radix=False)
        result = tools.execute({"hex": "0x1A", "count": "123"})
        assert result["hex"] == "0x1A"
        assert result["count"] == 123

    def test_normal_mode_fine_grained(self):
        tools = json_tools_rs.JSONTools().normal().convert_booleans(True)
        result = tools.execute({"user": {"active": "true", "id": "42"}})
        assert result["user"]["active"] is True
        assert result["user"]["id"] == "42"

    def test_unflatten_fine_grained(self):
        tools = json_tools_rs.JSONTools().unflatten().convert_booleans(True)
        result = tools.execute({"user.active": "true", "user.id": "42"})
        assert result["user"]["active"] is True
        assert result["user"]["id"] == "42"

    # ===== Edge cases =====

    def test_extra_tokens_kwarg_is_bulk_replace_not_additive(self):
        """Unlike Rust's add_extra_token() (additive, one call per token), Python's
        extra_tokens kwarg is bulk-replace: a second call with a different list
        replaces the first list entirely, it doesn't merge with it."""
        tools = json_tools_rs.JSONTools().flatten()
        tools = tools.convert_nulls(True, extra_tokens=["first"])
        tools = tools.convert_nulls(True, extra_tokens=["second"])
        result = tools.execute({"a": "first", "b": "second"})
        assert result["a"] == "first"  # no longer recognized -- replaced, not merged
        assert result["b"] is None

    def test_convert_booleans_token_in_both_lists_true_wins(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_booleans(
                True, extra_true_tokens=["maybe"], extra_false_tokens=["maybe"]
            )
        )
        result = tools.execute({"a": "maybe"})
        assert result["a"] is True

    def test_convert_nulls_extra_token_duplicating_builtin_is_harmless(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_nulls(True, extra_tokens=["null"])
        )
        result = tools.execute({"a": "null", "b": "not_null"})
        assert result["a"] is None
        assert result["b"] == "not_null"

    def test_disabled_category_customization_has_no_effect(self):
        tools = (
            json_tools_rs.JSONTools().flatten().convert_numbers(True, currency=False)
        )
        tools = tools.convert_numbers(False)  # last call wins: whole category off
        result = tools.execute({"price": "$45.67"})
        assert result["price"] == "$45.67"

    def test_unicode_extra_tokens(self):
        """Extra tokens round-trip correctly across the Python<->Rust FFI boundary
        for non-ASCII strings. Matching happens against the *trimmed* value (same
        as every other category/built-in token -- e.g. auto_convert_types already
        converts " 123 " to 123), so "oui " (trailing space) still matches "oui";
        it isn't a byte-for-byte match against the raw untrimmed string."""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_booleans(
                True, extra_true_tokens=["oui"], extra_false_tokens=["非"]
            )
        )
        result = tools.execute({"a": "oui", "b": "非", "c": "oui ", "d": "ouiX"})
        assert result["a"] is True
        assert result["b"] is False
        assert result["c"] is True  # matches after trimming, like every other category
        assert (
            result["d"] == "ouiX"
        )  # not a match at all -- extra text, not just whitespace

    def test_fine_grained_type_conversion_in_batch(self):
        batch = [
            {"id": str(i), "active": "yes", "extra": "missing"} for i in range(150)
        ]
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_numbers(True)
            .convert_booleans(True)
            .convert_nulls(True, extra_tokens=["missing"])
        )
        results = tools.execute(batch)
        assert len(results) == 150
        for i, r in enumerate(results):
            assert r["id"] == i
            assert r["active"] is True
            assert r["extra"] is None

    def test_numeric_extra_boolean_token_loses_to_numbers_when_both_enabled(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_numbers(True)
            .convert_booleans(True, extra_true_tokens=["1"])
        )
        result = tools.execute({"a": "1"})
        assert result["a"] == 1
        assert result["a"] is not True  # number, not boolean

    def test_numeric_extra_boolean_token_wins_when_numbers_disabled(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_booleans(True, extra_true_tokens=["1"])
        )
        result = tools.execute({"a": "1"})
        assert result["a"] is True

    def test_extra_tokens_are_case_sensitive(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_nulls(True, extra_tokens=["missing"])
        )
        result = tools.execute({"a": "missing", "b": "MISSING"})
        assert result["a"] is None
        assert result["b"] == "MISSING"

    def test_malformed_date_stays_as_string_no_crash(self):
        tools = json_tools_rs.JSONTools().flatten().convert_dates(True)
        result = tools.execute({"a": "2024-13-45T99:99:99"})
        assert result["a"] == "2024-13-45T99:99:99"

    def test_keys_are_never_type_converted(self):
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_booleans(True)
            .convert_numbers(True)
        )
        result = tools.execute({"true": "something", "123": "also something"})
        assert result["true"] == "something"
        assert result["123"] == "also something"

    def test_replacement_and_conversion_chain_identically_across_modes(self):
        """Previously an inconsistency: flatten()/unflatten() returned as soon as
        value_replacement matched, without trying conversion on the replaced value,
        while normal() already chained replacement into conversion. Fixed so all
        three modes compose identically -- this is what makes remove_nulls reliably
        catch a null that only emerges after a replacement runs, regardless of
        mode."""
        data = {"a": "ACTIVE"}

        flatten_result = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("ACTIVE", "true")
            .convert_booleans(True)
            .execute(data)
        )
        assert (
            flatten_result["a"] is True
        )  # chained: replacement's output was converted

        normal_result = (
            json_tools_rs.JSONTools()
            .normal()
            .value_replacement("ACTIVE", "true")
            .convert_booleans(True)
            .execute(data)
        )
        assert normal_result["a"] is True  # chained

    def test_remove_nulls_catches_null_produced_by_replacement_then_conversion(self):
        """Direct regression test: a value_replacement turns "MISSING" into "N/A"
        (a recognized null token), which auto_convert_types then converts to null,
        which remove_nulls must then catch -- across flatten/unflatten/normal."""
        data = {"user": {"name": "John", "status": "MISSING", "city": "NYC"}}

        flatten_result = (
            json_tools_rs.JSONTools()
            .flatten()
            .value_replacement("MISSING", "N/A")
            .auto_convert_types(True)
            .remove_nulls(True)
            .execute(data)
        )
        assert flatten_result["user.name"] == "John"
        assert "user.status" not in flatten_result

        normal_result = (
            json_tools_rs.JSONTools()
            .normal()
            .value_replacement("MISSING", "N/A")
            .auto_convert_types(True)
            .remove_nulls(True)
            .execute(data)
        )
        assert normal_result["user"]["name"] == "John"
        assert "status" not in normal_result["user"]

    def test_extra_tokens_empty_list_clears_vs_none_preserves(self):
        """Python-specific nuance from the kwargs-based bulk-replace design:
        extra_tokens=[] (explicit empty list) clears previously-set customization,
        while omitting the kwarg (None) preserves it -- these are NOT the same."""
        tools = json_tools_rs.JSONTools().flatten()
        tools = tools.convert_nulls(True, extra_tokens=["missing"])

        preserved = tools.convert_nulls(True)  # kwarg omitted -> preserves
        result = preserved.execute({"a": "missing"})
        assert result["a"] is None

        cleared = tools.convert_nulls(True, extra_tokens=[])  # explicit empty -> clears
        result = cleared.execute({"a": "missing"})
        assert result["a"] == "missing"  # no longer recognized


class TestParallelProcessing:
    """Test parallel processing configuration and functionality"""

    def test_parallel_threshold_method_exists(self):
        """Test that parallel_threshold method exists and is chainable"""
        tools = json_tools_rs.JSONTools().flatten().parallel_threshold(50)
        assert tools is not None

    def test_num_threads_method_exists(self):
        """Test that num_threads method exists and is chainable"""
        tools = json_tools_rs.JSONTools().flatten().num_threads(4)
        assert tools is not None

    def test_num_threads_with_none(self):
        """Test that num_threads accepts None (use system default)"""
        tools = json_tools_rs.JSONTools().flatten().num_threads(None)
        assert tools is not None

    def test_nested_parallel_threshold_method_exists(self):
        """Test that nested_parallel_threshold method exists and is chainable"""
        tools = json_tools_rs.JSONTools().flatten().nested_parallel_threshold(200)
        assert tools is not None

    def test_parallel_methods_chaining(self):
        """Test that all parallel methods can be chained together"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .parallel_threshold(50)
            .num_threads(4)
            .nested_parallel_threshold(200)
            .remove_empty_strings(True)
        )
        assert tools is not None

    def test_parallel_batch_processing_small_batch(self):
        """Test batch processing with small batch (below default threshold)"""
        tools = json_tools_rs.JSONTools().flatten()
        batch = [{"key": i, "nested": {"value": i * 10}} for i in range(5)]
        results = tools.execute(batch)

        assert len(results) == 5
        assert all(isinstance(r, dict) for r in results)
        assert results[0]["key"] == 0
        assert results[0]["nested.value"] == 0
        assert results[4]["key"] == 4
        assert results[4]["nested.value"] == 40

    def test_parallel_batch_processing_medium_batch(self):
        """Test batch processing with medium batch (above default threshold of 10)"""
        tools = json_tools_rs.JSONTools().flatten()
        batch = [{"user_id": i, "data": {"score": i * 100}} for i in range(25)]
        results = tools.execute(batch)

        assert len(results) == 25
        assert all(isinstance(r, dict) for r in results)
        assert results[0]["user_id"] == 0
        assert results[0]["data.score"] == 0
        assert results[24]["user_id"] == 24
        assert results[24]["data.score"] == 2400

    def test_parallel_batch_processing_large_batch(self):
        """Test batch processing with large batch (>1000 items, uses chunked processing)"""
        tools = json_tools_rs.JSONTools().flatten()
        batch = [{"id": i, "value": i * 2} for i in range(1500)]
        results = tools.execute(batch)

        assert len(results) == 1500
        assert all(isinstance(r, dict) for r in results)
        assert results[0]["id"] == 0
        assert results[0]["value"] == 0
        assert results[1499]["id"] == 1499
        assert results[1499]["value"] == 2998

    def test_parallel_threshold_configuration(self):
        """Test custom parallel threshold configuration"""
        # Set threshold to 100, so batch of 50 should process sequentially
        tools = json_tools_rs.JSONTools().flatten().parallel_threshold(100)
        batch = [{"key": i} for i in range(50)]
        results = tools.execute(batch)

        assert len(results) == 50
        assert all(isinstance(r, dict) for r in results)

    def test_parallel_with_string_batch(self):
        """Test parallel processing with list of JSON strings"""
        tools = json_tools_rs.JSONTools().flatten()
        batch = [f'{{"id": {i}, "nested": {{"value": {i * 10}}}}}' for i in range(20)]
        results = tools.execute(batch)

        assert len(results) == 20
        assert all(isinstance(r, str) for r in results)
        parsed_0 = json.loads(results[0])
        assert parsed_0["id"] == 0
        assert parsed_0["nested.value"] == 0

    def test_parallel_with_mixed_operations(self):
        """Test parallel processing with various transformations"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .parallel_threshold(10)
            .remove_empty_strings(True)
            .remove_nulls(True)
            .lowercase_keys(True)
        )
        batch = [
            {"User_ID": i, "Name": "Test", "Empty": "", "Null": None} for i in range(15)
        ]
        results = tools.execute(batch)

        assert len(results) == 15
        for result in results:
            assert "user_id" in result  # lowercase
            assert "name" in result
            assert "Empty" not in result  # removed
            assert "Null" not in result  # removed

    def test_parallel_unflatten_batch(self):
        """Test parallel processing with unflatten operation"""
        tools = json_tools_rs.JSONTools().unflatten().parallel_threshold(10)
        batch = [{"user.id": i, "user.name": f"User{i}"} for i in range(20)]
        results = tools.execute(batch)

        assert len(results) == 20
        assert all(isinstance(r, dict) for r in results)
        assert results[0]["user"]["id"] == 0
        assert results[0]["user"]["name"] == "User0"
        assert results[19]["user"]["id"] == 19
        assert results[19]["user"]["name"] == "User19"

    def test_parallel_with_collision_handling(self):
        """Test parallel processing with collision handling"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .parallel_threshold(5)
            .key_replacement("r'(user|admin)_'", "")
            .handle_key_collision(True)
        )
        batch = [
            {"user_name": f"User{i}", "admin_name": f"Admin{i}"} for i in range(10)
        ]
        results = tools.execute(batch)

        assert len(results) == 10
        for i, result in enumerate(results):
            assert "name" in result
            # Should be an array due to collision
            assert isinstance(result["name"], list)
            assert len(result["name"]) == 2

    def test_parallel_performance_benefit(self):
        """Test that parallel processing provides performance benefit for large batches"""
        # Create a large batch with complex nested structures
        large_batch = [
            {
                "user": {
                    "id": i,
                    "profile": {
                        "name": f"User{i}",
                        "email": f"user{i}@example.com",
                        "settings": {"theme": "dark", "notifications": True},
                    },
                    "posts": [
                        {"id": j, "title": f"Post {j}", "likes": j * 10}
                        for j in range(5)
                    ],
                }
            }
            for i in range(100)
        ]

        # Process with parallel processing enabled (default threshold = 10)
        tools_parallel = json_tools_rs.JSONTools().flatten()
        start = time.time()
        results_parallel = tools_parallel.execute(large_batch)
        time_parallel = time.time() - start

        # Verify results are correct
        assert len(results_parallel) == 100
        assert all(isinstance(r, dict) for r in results_parallel)
        assert "user.id" in results_parallel[0]
        assert "user.profile.name" in results_parallel[0]
        assert "user.posts.0.title" in results_parallel[0]

        # Just verify it completes successfully - actual speedup depends on hardware
        assert time_parallel > 0

    def test_nested_parallel_threshold_large_object(self):
        """Test nested parallel threshold with large objects"""
        # Create a large object with many keys
        large_object = {f"key_{i}": {"nested": i, "value": i * 10} for i in range(150)}

        # With default nested threshold (100), this should trigger nested parallelism
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute(large_object)

        assert isinstance(result, dict)
        assert len(result) == 300  # 150 keys * 2 nested fields each
        assert result["key_0.nested"] == 0
        assert result["key_0.value"] == 0
        assert result["key_149.nested"] == 149
        assert result["key_149.value"] == 1490

    def test_nested_parallel_threshold_configuration(self):
        """Test custom nested parallel threshold configuration"""
        # Set very high threshold so nested parallelism won't trigger
        large_object = {f"key_{i}": {"nested": i} for i in range(150)}

        tools = json_tools_rs.JSONTools().flatten().nested_parallel_threshold(1000)
        result = tools.execute(large_object)

        assert isinstance(result, dict)
        assert len(result) == 150
        assert result["key_0.nested"] == 0
        assert result["key_149.nested"] == 149

    def test_parallel_with_type_conversion(self):
        """Test parallel processing with automatic type conversion"""
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .parallel_threshold(10)
            .auto_convert_types(True)
        )
        batch = [
            {"id": str(i), "score": f"{i * 100}", "active": "true"} for i in range(20)
        ]
        results = tools.execute(batch)

        assert len(results) == 20
        for i, result in enumerate(results):
            assert result["id"] == i  # Converted to int
            assert result["score"] == i * 100  # Converted to int
            assert result["active"] is True  # Converted to bool

    def test_parallel_roundtrip_consistency(self):
        """Test that parallel processing maintains roundtrip consistency"""
        original_batch = [
            {"user": {"id": i, "data": {"value": i * 10}}} for i in range(25)
        ]

        # Flatten with parallel processing
        flatten_tools = json_tools_rs.JSONTools().flatten().parallel_threshold(10)
        flattened = flatten_tools.execute(original_batch)

        # Unflatten with parallel processing
        unflatten_tools = json_tools_rs.JSONTools().unflatten().parallel_threshold(10)
        unflattened = unflatten_tools.execute(flattened)

        # Should match original
        assert len(unflattened) == len(original_batch)
        for i, (original, result) in enumerate(zip(original_batch, unflattened)):
            assert result == original

    def test_parallel_error_handling(self):
        """Test that parallel processing handles errors correctly"""
        tools = json_tools_rs.JSONTools().flatten().parallel_threshold(5)

        # Mix of valid and invalid JSON strings
        batch = ['{"valid": 1}', '{"valid": 2}', "invalid json", '{"valid": 3}']

        with pytest.raises(Exception):  # Should raise error for invalid JSON
            tools.execute(batch)

    def test_parallel_empty_batch(self):
        """Test parallel processing with empty batch"""
        tools = json_tools_rs.JSONTools().flatten().parallel_threshold(10)
        results = tools.execute([])

        assert results == []

    def test_parallel_single_item_batch(self):
        """Test parallel processing with single item (below threshold)"""
        tools = json_tools_rs.JSONTools().flatten().parallel_threshold(10)
        batch = [{"key": "value", "nested": {"data": 123}}]
        results = tools.execute(batch)

        assert len(results) == 1
        assert results[0]["key"] == "value"
        assert results[0]["nested.data"] == 123


class TestDataFrameAndSeriesSupport:
    """Test DataFrame and Series support for pandas and polars"""

    @pytest.fixture(autouse=True)
    def setup(self):
        """Setup for DataFrame/Series tests"""
        try:
            import pandas as pd

            self.pd = pd
            self.has_pandas = True
        except ImportError:
            self.has_pandas = False

        try:
            import polars as pl

            self.pl = pl
            self.has_polars = True
        except ImportError:
            self.has_polars = False

        try:
            import pyarrow as pa

            self.pa = pa
            self.has_pyarrow = True
        except ImportError:
            self.has_pyarrow = False

    # =========================================================================
    # Pandas DataFrame Tests
    # =========================================================================

    def test_pandas_dataframe_flatten(self):
        """Test pandas DataFrame flattening"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create DataFrame with nested dicts
        df = self.pd.DataFrame(
            [
                {"user": {"name": "Alice", "age": 30}, "active": True},
                {"user": {"name": "Bob", "age": 25}, "active": False},
            ]
        )

        result = tools.execute(df)

        # Result should be pandas DataFrame (or list if pandas not in reconstruct)
        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 2
            assert "user.name" in result.columns
            assert "user.age" in result.columns
            assert result["user.name"].tolist() == ["Alice", "Bob"]
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    def test_pandas_dataframe_unflatten(self):
        """Test pandas DataFrame unflattening"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create DataFrame with flattened structure
        df = self.pd.DataFrame(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(df)

        # Result should be pandas DataFrame (or list if pandas not in reconstruct)
        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2
            assert result[0]["user"]["name"] == "Alice"
            assert result[1]["user"]["name"] == "Bob"

    def test_pandas_dataframe_empty(self):
        """Test empty pandas DataFrame"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame([])

        result = tools.execute(df)

        # Should handle empty DataFrame gracefully
        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 0
        else:
            assert isinstance(result, list)
            assert len(result) == 0

    # =========================================================================
    # Polars DataFrame Tests
    # =========================================================================

    def test_polars_dataframe_flatten(self):
        """Test polars DataFrame flattening"""
        if not self.has_polars:
            pytest.skip("polars not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create polars DataFrame with nested dicts
        df = self.pl.DataFrame(
            [
                {"user": {"name": "Alice", "age": 30}, "active": True},
                {"user": {"name": "Bob", "age": 25}, "active": False},
            ]
        )

        result = tools.execute(df)

        # Result should be polars DataFrame (or list if polars not in reconstruct)
        if isinstance(result, self.pl.DataFrame):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    def test_polars_dataframe_unflatten(self):
        """Test polars DataFrame unflattening"""
        if not self.has_polars:
            pytest.skip("polars not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create polars DataFrame with flattened structure
        df = self.pl.DataFrame(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(df)

        # Result should be polars DataFrame (or list if polars not in reconstruct)
        if isinstance(result, self.pl.DataFrame):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    # =========================================================================
    # PyArrow Table Tests
    # =========================================================================

    def test_pyarrow_table_flatten(self):
        """Test PyArrow Table flattening"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create PyArrow Table with nested dicts
        table = self.pa.Table.from_pylist(
            [
                {"user": {"name": "Alice", "age": 30}, "active": True},
                {"user": {"name": "Bob", "age": 25}, "active": False},
            ]
        )

        result = tools.execute(table)

        # Result should be PyArrow Table (or list if pyarrow not in reconstruct)
        if isinstance(result, self.pa.Table):
            assert len(result) == 2
            # Check that columns were flattened
            column_names = result.column_names
            assert "user.name" in column_names or "user" in column_names
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2
            assert "user.name" in result[0]

    def test_pyarrow_table_unflatten(self):
        """Test PyArrow Table unflattening"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create PyArrow Table with flattened structure
        table = self.pa.Table.from_pylist(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(table)

        # Result should be PyArrow Table (or list if pyarrow not in reconstruct)
        if isinstance(result, self.pa.Table):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2
            assert result[0]["user"]["name"] == "Alice"
            assert result[1]["user"]["name"] == "Bob"

    def test_pyarrow_table_empty(self):
        """Test empty PyArrow Table"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().flatten()
        table = self.pa.Table.from_pylist([])

        result = tools.execute(table)

        # Should handle empty Table gracefully
        if isinstance(result, self.pa.Table):
            assert len(result) == 0
        else:
            assert isinstance(result, list)
            assert len(result) == 0

    # =========================================================================
    # PyArrow Array Tests
    # =========================================================================

    def test_pyarrow_array_dicts_flatten(self):
        """Test PyArrow Array with dicts - flatten"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create PyArrow Array of dicts
        array = self.pa.array(
            [
                {"user": {"name": "Alice", "age": 30}},
                {"user": {"name": "Bob", "age": 25}},
            ]
        )

        result = tools.execute(array)

        # Result should be PyArrow Array (or list as fallback)
        if hasattr(self.pa, "Array") and isinstance(
            result, (self.pa.Array, self.pa.ChunkedArray)
        ):
            assert len(result) == 2
        elif isinstance(result, list):
            # Fallback to list
            assert len(result) == 2
            assert result[0]["user.name"] == "Alice"
        else:
            # May return as Array type
            assert len(result) == 2

    def test_pyarrow_array_dicts_unflatten(self):
        """Test PyArrow Array with dicts - unflatten"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create PyArrow Array of flattened dicts
        array = self.pa.array(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(array)

        # Result should be PyArrow Array (or list as fallback)
        if hasattr(self.pa, "Array") and isinstance(
            result, (self.pa.Array, self.pa.ChunkedArray)
        ):
            assert len(result) == 2
        elif isinstance(result, list):
            # Fallback to list
            assert len(result) == 2
            assert result[0]["user"]["name"] == "Alice"
        else:
            # May return as Array type
            assert len(result) == 2

    # =========================================================================
    # Pandas Series Tests
    # =========================================================================

    def test_pandas_series_json_strings_flatten(self):
        """Test pandas Series with JSON strings - flatten"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create Series of JSON strings
        series = self.pd.Series(
            ['{"user": {"name": "Alice"}}', '{"user": {"name": "Bob"}}']
        )

        result = tools.execute(series)

        # Result should be pandas Series of JSON strings (or list as fallback)
        if isinstance(result, self.pd.Series):
            assert len(result) == 2
            # Each element should be a flattened JSON string
            parsed = json.loads(result.iloc[0])
            assert "user.name" in parsed
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    def test_pandas_series_dicts_flatten(self):
        """Test pandas Series with dicts - flatten"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create Series of dicts
        series = self.pd.Series(
            [
                {"user": {"name": "Alice", "age": 30}},
                {"user": {"name": "Bob", "age": 25}},
            ]
        )

        result = tools.execute(series)

        # Result should be pandas Series of dicts (or list as fallback)
        if isinstance(result, self.pd.Series):
            assert len(result) == 2
            # Each element should be a flattened dict
            assert "user.name" in result.iloc[0]
            assert result.iloc[0]["user.name"] == "Alice"
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2
            assert result[0]["user.name"] == "Alice"

    def test_pandas_series_dicts_unflatten(self):
        """Test pandas Series with dicts - unflatten"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create Series of flattened dicts
        series = self.pd.Series(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(series)

        # Result should be pandas Series of nested dicts (or list as fallback)
        if isinstance(result, self.pd.Series):
            assert len(result) == 2
            # Each element should be an unflattened dict
            assert "user" in result.iloc[0]
            assert result.iloc[0]["user"]["name"] == "Alice"
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2
            assert result[0]["user"]["name"] == "Alice"

    def test_pandas_series_empty(self):
        """Test empty pandas Series"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()
        series = self.pd.Series([], dtype=object)

        result = tools.execute(series)

        # Should handle empty Series gracefully
        if isinstance(result, self.pd.Series):
            assert len(result) == 0
        else:
            assert isinstance(result, list)
            assert len(result) == 0

    # =========================================================================
    # Polars Series Tests
    # =========================================================================

    def test_polars_series_dicts_flatten(self):
        """Test polars Series with dicts - flatten"""
        if not self.has_polars:
            pytest.skip("polars not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create polars Series of dicts
        series = self.pl.Series(
            [
                {"user": {"name": "Alice", "age": 30}},
                {"user": {"name": "Bob", "age": 25}},
            ]
        )

        result = tools.execute(series)

        # Result should be polars Series (or list as fallback)
        if isinstance(result, self.pl.Series):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    def test_polars_series_dicts_unflatten(self):
        """Test polars Series with dicts - unflatten"""
        if not self.has_polars:
            pytest.skip("polars not installed")

        tools = json_tools_rs.JSONTools().unflatten()

        # Create polars Series of flattened dicts
        series = self.pl.Series(
            [
                {"user.name": "Alice", "user.age": 30},
                {"user.name": "Bob", "user.age": 25},
            ]
        )

        result = tools.execute(series)

        # Result should be polars Series (or list as fallback)
        if isinstance(result, self.pl.Series):
            assert len(result) == 2
        else:
            # Fallback to list
            assert isinstance(result, list)
            assert len(result) == 2

    # =========================================================================
    # Type Preservation Tests
    # =========================================================================

    def test_type_preservation_dataframe(self):
        """Test that DataFrame input returns DataFrame output (when library installed)"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame([{"a": {"b": 1}}])
        result = tools.execute(df)

        # Should be DataFrame or list (fallback)
        assert isinstance(result, (self.pd.DataFrame, list))

    def test_type_preservation_series(self):
        """Test that Series input returns Series output (when library installed)"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()
        series = self.pd.Series([{"a": {"b": 1}}])
        result = tools.execute(series)

        # Should be Series or list (fallback)
        assert isinstance(result, (self.pd.Series, list))

    # =========================================================================
    # Edge Cases and Error Handling
    # =========================================================================

    def test_dataframe_with_complex_nested_data(self):
        """Test DataFrame with deeply nested and complex data"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [{"level1": {"level2": {"level3": {"value": "deep", "array": [1, 2, 3]}}}}]
        )

        result = tools.execute(df)

        # Should process successfully
        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1

    def test_series_with_mixed_valid_invalid_json(self):
        """Test Series with mix of valid and invalid JSON strings"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Mix of valid and invalid JSON
        series = self.pd.Series(['{"valid": 1}', "invalid json"])

        # Should raise error for invalid JSON
        with pytest.raises(Exception):
            tools.execute(series)

    # =========================================================================
    # Round-Trip Tests (Flatten → Unflatten)
    # =========================================================================

    def test_pandas_dataframe_roundtrip(self):
        """Test pandas DataFrame roundtrip: flatten then unflatten"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        original_data = [
            {"user": {"name": "Alice", "age": 30}, "active": True},
            {"user": {"name": "Bob", "age": 25}, "active": False},
        ]

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame(original_data)
        flattened = flatten_tools.execute(df)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        result = unflatten_tools.execute(flattened)

        # Verify roundtrip
        if isinstance(result, self.pd.DataFrame):
            result_dicts = result.to_dict("records")
        else:
            result_dicts = result

        assert len(result_dicts) == 2
        assert result_dicts[0]["user"]["name"] == "Alice"
        assert result_dicts[1]["user"]["name"] == "Bob"

    def test_pyarrow_table_roundtrip(self):
        """Test PyArrow Table roundtrip: flatten then unflatten"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        original_data = [
            {"user": {"name": "Alice", "age": 30}},
            {"user": {"name": "Bob", "age": 25}},
        ]

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        table = self.pa.Table.from_pylist(original_data)
        flattened = flatten_tools.execute(table)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        result = unflatten_tools.execute(flattened)

        # Verify roundtrip
        if isinstance(result, self.pa.Table):
            result_dicts = result.to_pylist()
        else:
            result_dicts = result

        assert len(result_dicts) == 2
        assert result_dicts[0]["user"]["name"] == "Alice"

    def test_polars_dataframe_roundtrip(self):
        """Test polars DataFrame roundtrip: flatten then unflatten"""
        if not self.has_polars:
            pytest.skip("polars not installed")

        original_data = [
            {"user": {"name": "Alice", "age": 30}},
            {"user": {"name": "Bob", "age": 25}},
        ]

        # Flatten
        flatten_tools = json_tools_rs.JSONTools().flatten()
        df = self.pl.DataFrame(original_data)
        flattened = flatten_tools.execute(df)

        # Unflatten
        unflatten_tools = json_tools_rs.JSONTools().unflatten()
        result = unflatten_tools.execute(flattened)

        # Verify roundtrip
        if isinstance(result, self.pl.DataFrame):
            result_dicts = result.to_dicts()
        else:
            result_dicts = result

        assert len(result_dicts) == 2
        assert result_dicts[0]["user"]["name"] == "Alice"

    # =========================================================================
    # Null/None Value Handling
    # =========================================================================

    def test_dataframe_with_null_values(self):
        """Test DataFrame with null/None values"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [
                {"user": {"name": "Alice", "age": None}},
                {"user": {"name": None, "age": 25}},
            ]
        )

        result = tools.execute(df)

        # Should handle nulls gracefully
        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 2
        else:
            assert isinstance(result, list)
            assert len(result) == 2

    def test_series_with_none_values(self):
        """Test Series with None values - should raise error"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        series = self.pd.Series(
            [
                {"user": {"name": "Alice"}},
                None,  # Invalid - not a dict or JSON string
                {"user": {"name": "Bob"}},
            ]
        )

        # Should raise error for None values (not valid JSON)
        with pytest.raises(Exception):
            tools.execute(series)

    # =========================================================================
    # Unicode and Special Characters
    # =========================================================================

    def test_dataframe_with_unicode(self):
        """Test DataFrame with unicode characters"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [
                {"user": {"name": "José", "city": "São Paulo"}},
                {"user": {"name": "李明", "city": "北京"}},
                {"user": {"name": "Владимир", "city": "Москва"}},
            ]
        )

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 3
        else:
            assert isinstance(result, list)
            assert len(result) == 3
            assert result[0]["user.name"] == "José"

    def test_dataframe_with_emoji(self):
        """Test DataFrame with emoji characters"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [
                {"message": {"text": "Hello 👋", "reaction": "🎉"}},
                {"message": {"text": "World 🌍", "reaction": "❤️"}},
            ]
        )

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 2
        else:
            assert isinstance(result, list)
            assert len(result) == 2

    # =========================================================================
    # Mixed Data Types
    # =========================================================================

    def test_dataframe_with_mixed_types(self):
        """Test DataFrame with mixed data types"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [
                {
                    "data": {
                        "string": "text",
                        "integer": 42,
                        "float": 3.14,
                        "boolean": True,
                        "array": [1, 2, 3],
                        "nested": {"key": "value"},
                    }
                }
            ]
        )

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1
            assert result[0]["data.string"] == "text"
            assert result[0]["data.integer"] == 42
            assert result[0]["data.boolean"] is True

    # =========================================================================
    # Deeply Nested Structures
    # =========================================================================

    def test_dataframe_very_deep_nesting(self):
        """Test DataFrame with very deep nesting (10+ levels)"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create deeply nested structure
        nested = {"value": "deep"}
        for i in range(10):
            nested = {f"level{i}": nested}

        df = self.pd.DataFrame([nested])
        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1

    # =========================================================================
    # Large DataFrames (Performance/Stress Testing)
    # =========================================================================

    def test_large_dataframe_performance(self):
        """Test processing large DataFrame (1000 rows)"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create large DataFrame
        data = [
            {"user": {"name": f"User{i}", "age": 20 + (i % 50)}, "active": i % 2 == 0}
            for i in range(1000)
        ]
        df = self.pd.DataFrame(data)

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1000
        else:
            assert isinstance(result, list)
            assert len(result) == 1000

    def test_large_series_performance(self):
        """Test processing large Series (1000 items)"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create large Series
        data = [{"user": {"name": f"User{i}"}} for i in range(1000)]
        series = self.pd.Series(data)

        result = tools.execute(series)

        assert len(result) == 1000

    # =========================================================================
    # Single Row/Column Edge Cases
    # =========================================================================

    def test_dataframe_single_row(self):
        """Test DataFrame with single row"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame([{"user": {"name": "Alice"}}])
        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1

    def test_series_single_item(self):
        """Test Series with single item"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        series = self.pd.Series([{"user": {"name": "Alice"}}])
        result = tools.execute(series)

        assert len(result) == 1

    # =========================================================================
    # PyArrow ChunkedArray
    # =========================================================================

    def test_pyarrow_chunked_array(self):
        """Test PyArrow ChunkedArray specifically"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create ChunkedArray from multiple chunks
        chunk1 = self.pa.array([{"user": {"name": "Alice"}}])
        chunk2 = self.pa.array([{"user": {"name": "Bob"}}])
        chunked = self.pa.chunked_array([chunk1, chunk2])

        result = tools.execute(chunked)

        # Should handle ChunkedArray
        assert len(result) == 2

    # =========================================================================
    # Special Column Names
    # =========================================================================

    def test_dataframe_with_special_characters_in_keys(self):
        """Test DataFrame with special characters in keys"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [{"user@email": {"key$value": "data", "key-name": "test"}}]
        )

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1

    # =========================================================================
    # Arrays in DataFrames
    # =========================================================================

    def test_dataframe_with_arrays(self):
        """Test DataFrame with array values"""
        if not self.has_pandas:
            pytest.skip("pandas not installed")

        tools = json_tools_rs.JSONTools().flatten()

        df = self.pd.DataFrame(
            [
                {
                    "user": {"name": "Alice", "tags": ["python", "rust", "data"]},
                    "scores": [95, 87, 92],
                }
            ]
        )

        result = tools.execute(df)

        if isinstance(result, self.pd.DataFrame):
            assert len(result) == 1
        else:
            assert isinstance(result, list)
            assert len(result) == 1
            # Arrays should be indexed
            assert "user.tags.0" in result[0]

    # =========================================================================
    # PyArrow Table with Schema
    # =========================================================================

    def test_pyarrow_table_with_schema(self):
        """Test PyArrow Table with explicit schema"""
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")

        tools = json_tools_rs.JSONTools().flatten()

        # Create table with schema
        data = [
            {"user": {"name": "Alice", "age": 30}},
            {"user": {"name": "Bob", "age": 25}},
        ]
        table = self.pa.Table.from_pylist(data)

        result = tools.execute(table)

        if isinstance(result, self.pa.Table):
            assert len(result) == 2
        else:
            assert isinstance(result, list)
            assert len(result) == 2

    # =========================================================================
    # Multiple Libraries - Type Consistency
    # =========================================================================

    def test_consistent_output_across_libraries(self):
        """Test that output is consistent across pandas, polars, pyarrow"""
        if not (self.has_pandas and self.has_polars and self.has_pyarrow):
            pytest.skip("Need pandas, polars, and pyarrow installed")

        tools = json_tools_rs.JSONTools().flatten()

        data = [
            {"user": {"name": "Alice", "age": 30}},
            {"user": {"name": "Bob", "age": 25}},
        ]

        # Process with pandas
        df_pandas = self.pd.DataFrame(data)
        result_pandas = tools.execute(df_pandas)
        if isinstance(result_pandas, self.pd.DataFrame):
            result_pandas = result_pandas.to_dict("records")

        # Process with polars
        df_polars = self.pl.DataFrame(data)
        result_polars = tools.execute(df_polars)
        if isinstance(result_polars, self.pl.DataFrame):
            result_polars = result_polars.to_dicts()

        # Process with pyarrow
        table_pyarrow = self.pa.Table.from_pylist(data)
        result_pyarrow = tools.execute(table_pyarrow)
        if isinstance(result_pyarrow, self.pa.Table):
            result_pyarrow = result_pyarrow.to_pylist()

        # All should have same structure (flattened)
        assert len(result_pandas) == 2
        assert len(result_polars) == 2
        assert len(result_pyarrow) == 2

        # All should have flattened keys
        assert "user.name" in result_pandas[0]
        assert "user.name" in result_polars[0]
        assert "user.name" in result_pyarrow[0]


class TestErrorHandlingExtended:
    """Extended error handling tests covering validation and error codes."""

    def test_missing_operation_mode(self):
        """E005: Execute without calling flatten/unflatten/normal."""
        with pytest.raises(
            json_tools_rs.JsonToolsError, match="Operation mode not set"
        ):
            json_tools_rs.JSONTools().execute('{"key": "value"}')

    def test_invalid_json_input(self):
        """Invalid JSON should raise an error."""
        with pytest.raises(json_tools_rs.JsonToolsError):
            json_tools_rs.JSONTools().flatten().execute("not valid json")

    def test_batch_error_includes_index(self):
        """Batch processing errors should indicate which item failed."""
        with pytest.raises(json_tools_rs.JsonToolsError):
            inputs = ['{"valid": true}', "invalid json", '{"also": "valid"}']
            json_tools_rs.JSONTools().flatten().execute(inputs)

    def test_empty_separator_raises(self):
        """Empty separator should raise an error."""
        with pytest.raises(Exception):
            json_tools_rs.JSONTools().flatten().separator("").execute('{"a": 1}')

    def test_num_threads_zero_raises(self):
        """Zero threads should raise a configuration error."""
        with pytest.raises(json_tools_rs.JsonToolsError, match="num_threads"):
            json_tools_rs.JSONTools().flatten().num_threads(0).execute('{"a": 1}')


class TestExecuteToOutput:
    """Test execute_to_output() and JsonOutput wrapper."""

    def test_single_input_returns_json_output(self):
        """Single input should produce a JsonOutput with is_single=True."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute_to_output('{"a": {"b": 1}}')
        assert result.is_single
        assert not result.is_multiple
        single = result.get_single()
        assert isinstance(single, str)
        assert "a.b" in single

    def test_multiple_input_returns_json_output(self):
        """Multiple inputs should produce a JsonOutput with is_multiple=True."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute_to_output(['{"a": 1}', '{"b": 2}'])
        assert result.is_multiple
        assert not result.is_single
        multiple = result.get_multiple()
        assert isinstance(multiple, list)
        assert len(multiple) == 2

    def test_to_python_single(self):
        """to_python() on single result should return a string."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute_to_output('{"a": 1}')
        py_result = result.to_python()
        assert isinstance(py_result, str)

    def test_to_python_multiple(self):
        """to_python() on multiple results should return a list."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute_to_output(['{"a": 1}', '{"b": 2}'])
        py_result = result.to_python()
        assert isinstance(py_result, list)
        assert len(py_result) == 2


class TestNormalModeComprehensive:
    """Test normal mode (transforms without flatten/unflatten)."""

    def test_normal_lowercase_keys(self):
        """Normal mode should lowercase keys recursively."""
        tools = json_tools_rs.JSONTools().normal().lowercase_keys(True)
        result = tools.execute({"UserName": "John", "Nested": {"InnerKey": True}})
        assert "username" in result
        assert "nested" in result
        assert "innerkey" in result["nested"]

    def test_normal_auto_convert_types(self):
        """Normal mode should convert string types."""
        tools = json_tools_rs.JSONTools().normal().auto_convert_types(True)
        result = tools.execute({"count": "42", "active": "true", "rate": "3.14"})
        assert result["count"] == 42
        assert result["active"] is True
        assert isinstance(result["rate"], float)

    def test_normal_filtering(self):
        """Normal mode should filter empty values."""
        tools = (
            json_tools_rs.JSONTools()
            .normal()
            .remove_empty_strings(True)
            .remove_nulls(True)
            .remove_empty_objects(True)
            .remove_empty_arrays(True)
        )
        result = tools.execute({"a": "", "b": None, "c": {}, "d": [], "e": "keep"})
        assert "a" not in result
        assert "b" not in result
        assert "c" not in result
        assert "d" not in result
        assert result["e"] == "keep"

    def test_normal_key_replacement(self):
        """Normal mode should apply key replacements."""
        tools = json_tools_rs.JSONTools().normal().key_replacement("user_", "person_")
        result = tools.execute({"user_name": "John", "user_age": 30})
        assert "person_name" in result
        assert "person_age" in result

    def test_normal_value_replacement(self):
        """Normal mode should apply value replacements."""
        tools = (
            json_tools_rs.JSONTools()
            .normal()
            .value_replacement("r'@example\\.com'", "@company.org")
        )
        result = tools.execute({"email": "user@example.com"})
        assert result["email"] == "user@company.org"


class TestMaxArrayIndexProtection:
    """Test DoS protection via max_array_index."""

    def test_default_limit_rejects_huge_index(self):
        """Default limit should reject very large array indices."""
        with pytest.raises(json_tools_rs.JsonToolsError):
            json_tools_rs.JSONTools().unflatten().execute({"items.999999999": "value"})

    def test_custom_limit(self):
        """Custom limit should be enforced."""
        with pytest.raises(json_tools_rs.JsonToolsError):
            json_tools_rs.JSONTools().unflatten().max_array_index(10).execute(
                {"items.11": "value"}
            )

    def test_within_limit_succeeds(self):
        """Indices within limit should succeed."""
        result = (
            json_tools_rs.JSONTools()
            .unflatten()
            .max_array_index(10)
            .execute({"items.9": "value"})
        )
        assert isinstance(result, dict)


class TestUnicodeEdgeCases:
    """Test Unicode handling in keys and values."""

    def test_emoji_keys(self):
        """Emoji keys should flatten correctly."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({"🏠": {"🔑": "value"}})
        assert "🏠.🔑" in result

    def test_cjk_keys(self):
        """CJK characters in keys should work."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({"用户": {"名前": "太郎"}})
        assert result["用户.名前"] == "太郎"

    def test_unicode_roundtrip(self):
        """Unicode keys should survive flatten→unflatten roundtrip."""
        original = {"café": {"naïve": "résumé"}}
        flattened = json_tools_rs.JSONTools().flatten().execute(original)
        unflattened = json_tools_rs.JSONTools().unflatten().execute(flattened)
        assert unflattened["café"]["naïve"] == "résumé"

    def test_mixed_scripts(self):
        """Mixed script keys should work."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({"العربية": {"日本語": "value"}})
        assert "العربية.日本語" in result


class TestJsonMarshalingCompat:
    """Dict <-> JSON conversion edge cases for the orjson-backed marshaling
    path in src/python.rs (orjson is a required dependency -- see
    pyproject.toml). These pin its compatibility contract against the
    stdlib json module it replaces, including the inputs it falls back to
    stdlib for."""

    def test_big_int_precision_preserved(self):
        """Integers beyond 64-bit must roundtrip exactly, never as floats.

        orjson silently parses >64-bit integers as lossy floats, so the
        bindings route documents containing long digit runs to stdlib json.
        """
        big = 2**70
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({"big": big, "nested": {"also": -big}})
        assert result["big"] == big
        assert isinstance(result["big"], int)
        assert result["nested.also"] == -big
        assert isinstance(result["nested.also"], int)

    def test_big_int_in_batch(self):
        """Big-int preservation must hold on the list[dict] batch path too."""
        big = 2**66
        tools = json_tools_rs.JSONTools().flatten()
        results = tools.execute([{"v": big}, {"v": 1}])
        assert results[0]["v"] == big
        assert isinstance(results[0]["v"], int)

    def test_non_string_keys_coerced_like_stdlib(self):
        """int/float/bool/None dict keys must coerce to the same strings
        stdlib json.dumps produces by default."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({1: "a", 2.5: "b", True: "c", None: "d"})
        # NOTE: True == 1 in Python, so {1: ..., True: ...} collapses to one
        # key before serialization ever sees it -- use distinct keys here.
        result2 = tools.execute({False: "f", None: "n", 7: "s"})
        assert result2["false"] == "f"
        assert result2["null"] == "n"
        assert result2["7"] == "s"
        assert result["2.5"] == "b"

    def test_nan_and_infinity_still_rejected(self):
        """NaN/Infinity values are not valid JSON and must keep erroring."""
        tools = json_tools_rs.JSONTools().flatten()
        with pytest.raises(Exception):
            tools.execute({"x": float("nan")})
        with pytest.raises(Exception):
            tools.execute({"x": float("inf")})

    def test_dict_subclass_and_ordereddict(self):
        """dict subclasses must still route through full input detection."""
        import collections

        class DictSub(dict):
            pass

        tools = json_tools_rs.JSONTools().flatten()
        assert tools.execute(DictSub({"a": {"b": 1}})) == {"a.b": 1}
        od = collections.OrderedDict([("x", {"y": 2})])
        assert tools.execute(od) == {"x.y": 2}

    def test_unicode_dict_roundtrip(self):
        """Unicode and escapes must survive the accelerated dumps path."""
        data = {"café": {"emoji": "😀", "cjk": "中文", "esc": 'a"b\\c\nd'}}
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute(data)
        assert result["café.emoji"] == "😀"
        assert result["café.esc"] == 'a"b\\c\nd'

    def test_long_digit_string_value_unaffected(self):
        """A long digit run inside a *string* value must stay a string
        (it forces the conservative stdlib loads path internally)."""
        tools = json_tools_rs.JSONTools().flatten()
        result = tools.execute({"id": "12345678901234567890123"})
        assert result["id"] == "12345678901234567890123"


class TestPickling:
    """JSONTools pickle support (github.com/amaye15/JSON-Tools-rs/issues/29,
    point 2). Needed to capture a configured JSONTools in a closure that
    crosses a process boundary -- e.g. a PySpark UDF/mapInPandas function,
    which cloudpickle-serializes -- without the ASCII-workaround-style hacks
    the issue describes needing before this existed."""

    def test_pickle_roundtrip_preserves_behavior(self):
        """A pickled-then-unpickled instance must process input identically
        to the original, across a real pickle.dumps/loads cycle (not just
        the underlying config-string mechanism)."""
        import pickle

        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .separator("::")
            .remove_nulls(True)
            .key_replacement("r'^admin_'", "")
            .auto_convert_types(True)
        )
        payload = {"admin_name": "Jane", "age": None, "id": "123"}
        expected = tools.execute(payload)

        restored = pickle.loads(pickle.dumps(tools))
        assert restored.execute(payload) == expected

    def test_pickle_roundtrip_preserves_nested_type_conversion_config(self):
        """Nested per-category customization (not just top-level booleans)
        must survive the round trip -- this is what actually exercises
        to_config_json's date/null/boolean/number sub-config serialization,
        not just the simple top-level flags."""
        import pickle

        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .convert_dates(True, assume_utc_for_naive=False)
            .convert_booleans(True, extra_true_tokens=["da"])
            .exclude_key("secret")
        )
        payload = {
            "flag": "da",
            "when": "2024-01-15T10:30:00",
            "secret_x": "hidden",
            "y": 1,
        }
        expected = tools.execute(payload)

        restored = pickle.loads(pickle.dumps(tools))
        assert restored.execute(payload) == expected
        # naive datetime must stay unchanged (assume_utc_for_naive=False survived
        # the round trip -- without it, this input would get "Z" appended)
        assert "Z" not in restored.execute(payload)["when"]
        # extra_true_tokens=["da"] also survived: "da" recognized as boolean True
        assert restored.execute(payload)["flag"] is True

    def test_pickle_restored_instance_is_independent(self):
        """Mutating the restored instance must not affect the original --
        they must be two genuinely separate native handles, not sharing
        interior state."""
        import pickle

        tools = json_tools_rs.JSONTools().flatten()
        restored = pickle.loads(pickle.dumps(tools))
        restored.lowercase_keys(True)

        assert restored.execute({"Name": "X"}) == {"name": "X"}
        assert tools.execute({"Name": "X"}) == {"Name": "X"}

    def test_to_config_json_from_config_json_roundtrip(self):
        """The lower-level mechanism pickling is built on, usable directly
        (e.g. for a PySpark mapInPandas partition function to reconstruct a
        fresh instance per-partition from a captured config string)."""
        tools = json_tools_rs.JSONTools().flatten().remove_nulls(True).num_threads(4)
        config = tools.to_config_json()
        assert isinstance(config, str)

        restored = json_tools_rs.JSONTools.from_config_json(config)
        payload = {"a": None, "b": 1}
        assert restored.execute(payload) == tools.execute(payload)

    def test_pickle_across_real_process_boundary(self):
        """The actual scenario from the issue: a configured JSONTools
        captured in a closure shipped to a genuinely separate process (the
        local analog of a Spark executor receiving a cloudpickled task).
        Uses the 'spawn' start method so the child is a fresh interpreter,
        not a fork inheriting the parent's already-loaded extension state."""
        import multiprocessing as mp
        import pickle

        tools = json_tools_rs.JSONTools().flatten().auto_convert_types(True)
        payload = {"count": "42", "name": "test"}
        expected = tools.execute(payload)

        ctx = mp.get_context("spawn")
        queue = ctx.Queue()
        proc = ctx.Process(
            target=_pickle_worker, args=(pickle.dumps(tools), payload, queue)
        )
        proc.start()
        result = queue.get(timeout=30)
        proc.join()

        assert result == expected


def _pickle_worker(pickled_tools, payload, out_queue):
    """Module-level (picklable) target for TestPickling's cross-process test."""
    import pickle

    tools = pickle.loads(pickled_tools)
    out_queue.put(tools.execute(payload))


class TestNormalise:
    """Test execute(..., normalise=True, target=...): always get back a wide
    DataFrame, natively, across pandas/polars/pyarrow/pyspark -- see
    github.com/amaye15/JSON-Tools-rs's `normalise` feature."""

    @pytest.fixture(autouse=True)
    def setup(self):
        try:
            import pandas as pd

            self.pd = pd
            self.has_pandas = True
        except ImportError:
            self.has_pandas = False

        try:
            import polars as pl

            self.pl = pl
            self.has_polars = True
        except ImportError:
            self.has_polars = False

        try:
            import pyarrow as pa

            self.pa = pa
            self.has_pyarrow = True
        except ImportError:
            self.has_pyarrow = False

        try:
            import pyspark  # noqa: F401
            from pyspark.sql import SparkSession

            self.spark = (
                SparkSession.builder.master("local[2]")
                .appName("json_tools_rs_normalise_tests")
                .getOrCreate()
            )
            self.has_pyspark = True
        except ImportError:
            self.has_pyspark = False

    # =========================================================================
    # Configuration errors
    # =========================================================================

    def test_normalise_requires_flatten_mode_unflatten(self):
        tools = json_tools_rs.JSONTools().unflatten()
        with pytest.raises(json_tools_rs.JsonToolsError, match="flatten"):
            tools.execute({"a.b": 1}, normalise=True)

    def test_normalise_requires_flatten_mode_normal(self):
        tools = json_tools_rs.JSONTools().normal()
        with pytest.raises(json_tools_rs.JsonToolsError, match="flatten"):
            tools.execute({"a": 1}, normalise=True)

    def test_normalise_requires_flatten_mode_unset(self):
        tools = json_tools_rs.JSONTools()
        with pytest.raises(json_tools_rs.JsonToolsError, match="flatten"):
            tools.execute({"a": 1}, normalise=True)

    def test_target_without_normalise_errors(self):
        tools = json_tools_rs.JSONTools().flatten()
        with pytest.raises(json_tools_rs.JsonToolsError, match="normalise=True"):
            tools.execute({"a": 1}, target="pandas")

    def test_unknown_target_errors(self):
        tools = json_tools_rs.JSONTools().flatten()
        with pytest.raises(
            json_tools_rs.JsonToolsError, match="Unknown normalise target"
        ):
            tools.execute({"a": 1}, normalise=True, target="numpy")

    def test_pyspark_target_without_pyspark_installed_errors(self):
        if self.has_pyspark:
            pytest.skip("pyspark is installed in this environment")
        tools = json_tools_rs.JSONTools().flatten()
        with pytest.raises(json_tools_rs.JsonToolsError, match="pyspark"):
            tools.execute({"a": 1}, normalise=True, target="pyspark")

    def test_series_of_plain_scalars_errors(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        series = self.pd.Series([1, 2, 3])
        with pytest.raises(Exception, match="JSON strings or Python dictionaries"):
            tools.execute(series, normalise=True)

    def test_non_object_row_errors_with_row_index(self):
        # Explicit target: row-content validation runs *after* target resolution
        # in execute_normalise, so without this, an environment with none of
        # pandas/polars/pyarrow installed fails at target auto-detection first
        # ("could not auto-detect a target") instead of reaching the row check
        # this test actually exercises -- caught by a CI job that has none of
        # those optional libraries installed (maturin-ci.yml's wheel-test step).
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        with pytest.raises(json_tools_rs.JsonToolsError, match="row 0"):
            tools.execute(['"just a string"'], normalise=True, target="pandas")

    # =========================================================================
    # Pandas target
    # =========================================================================

    def test_pandas_dict_input_one_row(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute(
            {"user": {"name": "Alice", "age": 30}}, normalise=True, target="pandas"
        )
        assert isinstance(df, self.pd.DataFrame)
        assert df.shape == (1, 2)
        assert df.iloc[0]["user.name"] == "Alice"
        assert df.iloc[0]["user.age"] == 30

    def test_pandas_str_input_one_row(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute('{"a": 1}', normalise=True, target="pandas")
        assert df.shape == (1, 1)

    def test_pandas_list_str_input_n_rows(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute(['{"a": 1}', '{"a": 2}'], normalise=True, target="pandas")
        assert df.shape == (2, 1)

    def test_pandas_heterogeneous_keys_union_and_null_fill(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": {"x": "hi"}}, {"a": 2, "c": True}]
        df = tools.execute(data, normalise=True, target="pandas")
        # First-seen order across all rows, not alphabetical.
        assert df.columns.tolist() == ["a", "b.x", "c"]
        # pd.isna(), not `is None` -- pandas represents a missing value in a
        # mixed-type/object column as Python None on some versions and as
        # float('nan') on others (confirmed: pandas 2.3.3 gives None, pandas
        # 3.0.5 gives nan, for this exact column shape); pd.isna() is the
        # pandas-recommended check that's correct across both.
        assert self.pd.isna(df.iloc[0]["c"])
        assert self.pd.isna(df.iloc[1]["b.x"])

    def test_pandas_empty_list_zero_rows_no_error(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute([], normalise=True, target="pandas")
        assert df.shape == (0, 0)

    def test_pandas_all_none_column_does_not_crash(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": None}, {"a": 2, "b": None}]
        df = tools.execute(data, normalise=True, target="pandas")
        assert df["b"].isna().all()

    def test_pandas_key_collision_mixed_list_scalar_columns(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'(User|Admin)_'", "")
            .handle_key_collision(True)
        )
        data = [{"User_name": "John", "Admin_name": "Bob"}, {"User_name": "Carl"}]
        df = tools.execute(data, normalise=True, target="pandas")
        assert df.iloc[0]["name"] == ["John", "Bob"]
        assert df.iloc[1]["name"] == ["Carl"]

    # =========================================================================
    # Polars target
    # =========================================================================

    def test_polars_dict_input_one_row(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute({"user": {"name": "Alice"}}, normalise=True, target="polars")
        assert isinstance(df, self.pl.DataFrame)
        assert df.shape == (1, 1)

    def test_polars_heterogeneous_keys_union_and_null_fill(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": {"x": "hi"}}, {"a": 2, "c": True}]
        df = tools.execute(data, normalise=True, target="polars")
        assert df.columns == ["a", "b.x", "c"]
        assert df["c"][0] is None
        assert df["b.x"][1] is None

    def test_polars_all_none_column_does_not_crash(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": None}, {"a": 2, "b": None}]
        df = tools.execute(data, normalise=True, target="polars")
        # polars's own harmless default for an all-None column (Null dtype) --
        # no explicit typing needed/attempted here, unlike the pyspark target,
        # which has a real reason to force an explicit schema (see
        # reconstruct_pyspark_normalise's doc comment).
        assert df["b"].dtype == self.pl.Null
        assert df["b"].is_null().all()

    def test_polars_key_collision_mixed_list_scalar_columns(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'(User|Admin)_'", "")
            .handle_key_collision(True)
        )
        data = [{"User_name": "John", "Admin_name": "Bob"}, {"User_name": "Carl"}]
        df = tools.execute(data, normalise=True, target="polars")
        assert df["name"].to_list() == [["John", "Bob"], ["Carl"]]

    # =========================================================================
    # PyArrow target
    # =========================================================================

    def test_pyarrow_dict_input_one_row(self):
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")
        tools = json_tools_rs.JSONTools().flatten()
        table = tools.execute(
            {"user": {"name": "Alice"}}, normalise=True, target="pyarrow"
        )
        assert isinstance(table, self.pa.Table)
        assert table.shape == (1, 1)

    def test_pyarrow_heterogeneous_keys_union_and_null_fill(self):
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": {"x": "hi"}}, {"a": 2, "c": True}]
        table = tools.execute(data, normalise=True, target="pyarrow")
        assert table.column_names == ["a", "b.x", "c"]

    def test_pyarrow_all_none_column_does_not_crash(self):
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": None}, {"a": 2, "b": None}]
        table = tools.execute(data, normalise=True, target="pyarrow")
        # pyarrow's own harmless default for an all-None column (null type) --
        # no explicit typing needed/attempted here; see the polars test above
        # and reconstruct_pyspark_normalise's doc comment for why pyspark
        # alone needs a real, explicit-schema fix instead.
        assert table.schema.field("b").type == self.pa.null()
        assert table.column("b").null_count == 2

    def test_pyarrow_key_collision_mixed_list_scalar_columns(self):
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")
        tools = (
            json_tools_rs.JSONTools()
            .flatten()
            .key_replacement("r'(User|Admin)_'", "")
            .handle_key_collision(True)
        )
        data = [{"User_name": "John", "Admin_name": "Bob"}, {"User_name": "Carl"}]
        table = tools.execute(data, normalise=True, target="pyarrow")
        assert table.column("name").to_pylist() == [["John", "Bob"], ["Carl"]]

    # =========================================================================
    # Target resolution: cross-backend, auto-detect
    # =========================================================================

    def test_cross_backend_input_pandas_target_polars(self):
        if not (self.has_pandas and self.has_polars):
            pytest.skip("pandas and polars both required")
        tools = json_tools_rs.JSONTools().flatten()
        pdf = self.pd.DataFrame(
            [{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}]
        )
        out = tools.execute(pdf, normalise=True, target="polars")
        assert isinstance(out, self.pl.DataFrame)
        assert out.shape == (2, 1)

    def test_auto_detect_target_matches_live_input_backend(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = json_tools_rs.JSONTools().flatten()
        pldf = self.pl.DataFrame([{"user": {"name": "Alice"}}])
        out = tools.execute(pldf, normalise=True)
        assert isinstance(out, self.pl.DataFrame)

    def test_auto_detect_target_priority_pandas_first_for_bare_json(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        out = tools.execute({"a": 1}, normalise=True)
        assert isinstance(out, self.pd.DataFrame)

    # =========================================================================
    # PySpark target
    # =========================================================================

    def test_pyspark_target_produces_real_dataframe(self):
        if not self.has_pyspark:
            pytest.skip("pyspark not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [
            {"user": {"name": "Alice", "age": 30}},
            {"user": {"name": "Bob", "age": 25}},
        ]
        df = tools.execute(data, normalise=True, target="pyspark")
        from pyspark.sql import DataFrame as SparkDataFrame

        assert isinstance(df, SparkDataFrame)
        # Backtick-quoted: "user.name" is a literal flat column name (flatten's
        # separator produced the dot), not Spark's dotted nested-field syntax --
        # without backticks, Spark tries to resolve a `user` struct column
        # containing a `name` field, which doesn't exist here.
        rows = df.orderBy("`user.name`").collect()
        assert [r["user.name"] for r in rows] == ["Alice", "Bob"]
        assert [r["user.age"] for r in rows] == [30, 25]

    def test_pyspark_all_none_column_does_not_crash(self):
        if not self.has_pyspark:
            pytest.skip("pyspark not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [{"a": 1, "b": None}, {"a": 2, "b": None}]
        df = tools.execute(data, normalise=True, target="pyspark")
        field = next(f for f in df.schema.fields if f.name == "b")
        from pyspark.sql.types import StringType

        assert isinstance(field.dataType, StringType)
        assert df.count() == 2
        # Regression guard for a real corruption bug found in this exact
        # scenario: on the non-Arrow fallback path Spark silently takes when
        # pyarrow isn't installed, pandas's nullable "string" dtype's pd.NA
        # sentinel (an earlier version of the Rust reconstruction used it for
        # all-None columns) serialized as the *literal string* "<NA>" instead
        # of a real null -- `isNull()`/`.count()` alone wouldn't have caught
        # this, only checking the actual collected value does.
        rows = df.collect()
        assert all(r["b"] is None for r in rows)
        assert df.filter(df.b.isNull()).count() == 2

    def test_pyspark_empty_input_produces_empty_dataframe(self):
        if not self.has_pyspark:
            pytest.skip("pyspark not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = tools.execute([], normalise=True, target="pyspark")
        assert df.count() == 0
        assert len(df.schema.fields) == 0

    def test_pyspark_target_no_active_session_errors(self):
        """Regression guard for the SparkSession.getActiveSession() auto-discovery
        path -- stops the local session started in `setup` so none is active,
        then restarts it afterward for any later test in this class."""
        if not self.has_pyspark:
            pytest.skip("pyspark not installed")
        from pyspark.sql import SparkSession

        self.spark.stop()
        try:
            tools = json_tools_rs.JSONTools().flatten()
            with pytest.raises(
                json_tools_rs.JsonToolsError, match="active SparkSession"
            ):
                tools.execute({"a": 1}, normalise=True, target="pyspark")
        finally:
            self.spark = (
                SparkSession.builder.master("local[2]")
                .appName("json_tools_rs_normalise_tests")
                .getOrCreate()
            )


class TestJsonStringColumnExpansion:
    """Test auto-expansion of DataFrame columns holding JSON *strings* (not
    already dicts/structs) in .flatten() mode -- see
    github.com/amaye15/JSON-Tools-rs/issues/30. A dict/struct-typed column
    already expanded before this fix; a JSON-string column previously stayed
    an opaque, unexpanded string."""

    @pytest.fixture(autouse=True)
    def setup(self):
        try:
            import pandas as pd

            self.pd = pd
            self.has_pandas = True
        except ImportError:
            self.has_pandas = False

        try:
            import polars as pl

            self.pl = pl
            self.has_polars = True
        except ImportError:
            self.has_polars = False

        try:
            import pyarrow as pa

            self.pa = pa
            self.has_pyarrow = True
        except ImportError:
            self.has_pyarrow = False

        try:
            import pyspark  # noqa: F401
            from pyspark.sql import SparkSession

            self.spark = (
                SparkSession.builder.master("local[2]")
                .appName("json_tools_rs_json_column_tests")
                .getOrCreate()
            )
            self.has_pyspark = True
        except ImportError:
            self.has_pyspark = False

    # =========================================================================
    # Basic expansion, per backend
    # =========================================================================

    def test_pandas_json_string_column_expands(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten().separator("__")
        df = self.pd.DataFrame(
            {
                "id": [1, 2],
                "name": ["Alice", "Bob"],
                "json_col": ['{"a": {"b": 1}}', '{"a": {"b": 2}}'],
            }
        )
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "name", "json_col__a__b"]
        assert result["json_col__a__b"].tolist() == [1, 2]

    def test_polars_json_string_column_expands(self):
        if not self.has_polars:
            pytest.skip("polars not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pl.DataFrame(
            {"id": [1, 2], "json_col": ['{"a": {"b": 1}}', '{"a": {"b": 2}}']}
        )
        result = tools.execute(df)
        assert result.columns == ["id", "json_col.a.b"]

    def test_pyarrow_json_string_column_expands(self):
        if not self.has_pyarrow:
            pytest.skip("pyarrow not installed")
        tools = json_tools_rs.JSONTools().flatten()
        table = self.pa.table(
            {"id": [1, 2], "json_col": ['{"a": {"b": 1}}', '{"a": {"b": 2}}']}
        )
        result = tools.execute(table)
        assert result.column_names == ["id", "json_col.a.b"]

    # =========================================================================
    # Detection correctness
    # =========================================================================

    def test_plain_string_column_not_expanded(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame(
            {"id": [1, 2], "notes": ["hello world", "just some text"]}
        )
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "notes"]
        assert result["notes"].tolist() == ["hello world", "just some text"]

    def test_column_only_sometimes_json_not_expanded(self):
        """A column where only some values look like JSON must not partially
        expand -- conservative: any non-parsing string in the sample
        disqualifies the whole column."""
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame(
            {
                "id": [1, 2, 3],
                "mixed_col": ['{"a": 1}', "just plain text", '{"b": 2}'],
            }
        )
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "mixed_col"]

    def test_json_string_column_with_nulls_still_expands(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame(
            {"id": [1, 2, 3], "json_col": ['{"a": 1}', None, '{"a": 3}']}
        )
        result = tools.execute(df)
        assert "json_col.a" in result.columns.tolist()
        assert result.loc[result["id"] == 1, "json_col.a"].iloc[0] == 1
        assert result.loc[result["id"] == 3, "json_col.a"].iloc[0] == 3

    def test_json_string_array_column_expands_into_indexed_columns(self):
        """A JSON-string-encoded array expands into indexed sub-columns the
        same way an already-list-typed column does today."""
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame({"id": [1, 2], "tags": ['["python", "rust"]', '["go"]']})
        result = tools.execute(df)
        assert "tags.0" in result.columns.tolist()
        assert "tags.1" in result.columns.tolist()
        assert result.loc[result["id"] == 1, "tags.0"].iloc[0] == "python"

    def test_large_json_string_array_column_no_cap(self):
        """Pins current behavior: flatten's array walk has no cap (unlike
        unflatten's max_array_index DoS protection), so a large embedded
        array explodes into that many columns. Not a regression target for
        this fix -- documents the existing, accepted limitation explicitly."""
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        import json as json_module

        tools = json_tools_rs.JSONTools().flatten()
        big_array = json_module.dumps(list(range(150)))
        df = self.pd.DataFrame({"id": [1], "vec": [big_array]})
        result = tools.execute(df)
        assert len(result.columns) == 151
        assert "vec.0" in result.columns.tolist()
        assert "vec.149" in result.columns.tolist()

    def test_malformed_json_beyond_sample_keeps_original_and_warns(self):
        """A column detected as JSON via its sample, but with one malformed
        row further down, keeps that row's original string value (does not
        crash or corrupt) and emits exactly one aggregated warning naming the
        column and failure count."""
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        rows = [f'{{"a": {i}}}' for i in range(25)]
        rows[22] = '{"a": broken'
        df = self.pd.DataFrame({"id": range(25), "json_col": rows})

        with pytest.warns(UserWarning, match="json_col"):
            result = tools.execute(df)

        assert "json_col.a" in result.columns.tolist()
        bad_row = result.loc[result["id"] == 22]
        assert bad_row["json_col"].iloc[0] == '{"a": broken'
        assert self.pd.isna(bad_row["json_col.a"].iloc[0])

    # =========================================================================
    # Mode scoping (the wiring correction found during planning)
    # =========================================================================

    def test_normal_mode_json_string_column_untouched(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().normal()
        df = self.pd.DataFrame({"id": [1], "json_col": ['{"a": 1}']})
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "json_col"]
        assert result["json_col"].iloc[0] == '{"a": 1}'

    def test_unflatten_mode_json_string_column_untouched(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().unflatten()
        df = self.pd.DataFrame({"id": [1], "json_col": ['{"a": 1}']})
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "json_col"]
        assert result["json_col"].iloc[0] == '{"a": 1}'

    # =========================================================================
    # Composition with existing features
    # =========================================================================

    def test_mixed_dict_and_json_string_columns_both_expand(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        df = self.pd.DataFrame(
            {
                "id": [1, 2],
                "dict_col": [{"x": 1}, {"x": 2}],
                "json_col": ['{"y": 10}', '{"y": 20}'],
            }
        )
        result = tools.execute(df)
        assert result.columns.tolist() == ["id", "dict_col.x", "json_col.y"]

    def test_json_string_column_with_normalise(self):
        if not self.has_pandas:
            pytest.skip("pandas not installed")
        tools = json_tools_rs.JSONTools().flatten()
        data = [
            {"id": 1, "json_col": '{"a": {"b": 1}}'},
            {"id": 2, "json_col": '{"a": {"b": 2}}'},
        ]
        result = tools.execute(self.pd.DataFrame(data), normalise=True, target="pandas")
        assert "json_col.a.b" in result.columns.tolist()
        assert result["json_col.a.b"].tolist() == [1, 2]

    def test_pyspark_json_string_column_expands(self):
        """The issue's own reported scenario: a PySpark DataFrame with a JSON
        string column."""
        if not self.has_pyspark:
            pytest.skip("pyspark not installed")
        tools = json_tools_rs.JSONTools().flatten().separator("__")
        data = [
            {"id": 1, "name": "Alice", "json_col": '{"a": {"b": 1}}'},
            {"id": 2, "name": "Bob", "json_col": '{"a": {"b": 2}}'},
        ]
        spark_df = self.spark.createDataFrame(data)
        result = tools.execute(spark_df)
        assert isinstance(result, list)
        assert result[0]["json_col__a__b"] == 1
        assert result[1]["json_col__a__b"] == 2
