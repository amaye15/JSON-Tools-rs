"""
Tests for json_tools_rs Python bindings

This module contains comprehensive tests for the Python bindings of the JSON Tools RS library.
"""

import pytest
import json
from typing import List, Dict, Any

try:
    import json_tools_rs
    from json_tools_rs import flatten_json, JsonOutput, JsonFlattenError
    RUST_MODULE_AVAILABLE = True
except ImportError:
    RUST_MODULE_AVAILABLE = False
    pytest.skip("Rust module not available", allow_module_level=True)


class TestBasicFlattening:
    """Test basic JSON flattening functionality."""
    
    def test_simple_object_flattening(self):
        """Test flattening a simple nested object."""
        json_str = '{"a": {"b": {"c": 1}}}'
        result = flatten_json(json_str)
        
        assert result.is_single
        assert not result.is_multiple
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"a.b.c": 1}
    
    def test_array_flattening(self):
        """Test flattening arrays with indices."""
        json_str = '{"items": [1, 2, {"nested": "value"}]}'
        result = flatten_json(json_str)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "items.0": 1,
            "items.1": 2,
            "items.2.nested": "value"
        }
        assert parsed == expected
    
    def test_matrix_flattening(self):
        """Test flattening nested arrays (matrix)."""
        json_str = '{"matrix": [[1, 2], [3, 4]]}'
        result = flatten_json(json_str)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "matrix.0.0": 1,
            "matrix.0.1": 2,
            "matrix.1.0": 3,
            "matrix.1.1": 4
        }
        assert parsed == expected
    
    def test_custom_separator(self):
        """Test flattening with custom separator."""
        json_str = '{"a": {"b": 1}}'
        result = flatten_json(json_str, separator="_")
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"a_b": 1}
    
    def test_double_colon_separator(self):
        """Test flattening with double colon separator."""
        json_str = '{"user": {"profile": {"name": "John"}}}'
        result = flatten_json(json_str, separator="::")
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"user::profile::name": "John"}


class TestFiltering:
    """Test filtering options for removing empty values."""
    
    def test_remove_empty_strings(self):
        """Test removing empty string values."""
        json_str = '{"name": "John", "email": "", "city": "NYC"}'
        result = flatten_json(json_str, remove_empty_string_values=True)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"name": "John", "city": "NYC"}
        assert "email" not in parsed
    
    def test_remove_null_values(self):
        """Test removing null values."""
        json_str = '{"name": "John", "age": null, "city": "NYC"}'
        result = flatten_json(json_str, remove_null_values=True)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"name": "John", "city": "NYC"}
        assert "age" not in parsed
    
    def test_remove_empty_objects(self):
        """Test removing empty object values."""
        json_str = '{"user": {"name": "John"}, "metadata": {}, "settings": {"theme": "dark"}}'
        result = flatten_json(json_str, remove_empty_dict=True)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {"user.name": "John", "settings.theme": "dark"}
        assert parsed == expected
        assert "metadata" not in flattened
    
    def test_remove_empty_arrays(self):
        """Test removing empty array values."""
        json_str = '{"items": [1, 2], "empty": [], "tags": ["tag1"]}'
        result = flatten_json(json_str, remove_empty_list=True)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {"items.0": 1, "items.1": 2, "tags.0": "tag1"}
        assert parsed == expected
        assert "empty" not in flattened
    
    def test_combined_filtering(self):
        """Test combining multiple filtering options."""
        json_str = '{"user": {"name": "John", "email": "", "age": null}, "metadata": {}, "items": []}'
        result = flatten_json(
            json_str,
            remove_empty_string_values=True,
            remove_null_values=True,
            remove_empty_dict=True,
            remove_empty_list=True
        )
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"user.name": "John"}


class TestLowercaseKeys:
    """Test lowercase key conversion functionality."""

    def test_basic_lowercase_conversion(self):
        """Test basic lowercase key conversion."""
        json_str = '{"User": {"Name": "John", "Email": "john@example.com"}}'
        result = flatten_json(json_str, lower_case_keys=True)

        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "user.name": "John",
            "user.email": "john@example.com"
        }
        assert parsed == expected

    def test_lowercase_disabled(self):
        """Test that keys preserve case when lowercase is disabled."""
        json_str = '{"User": {"Name": "John", "Email": "john@example.com"}}'
        result = flatten_json(json_str, lower_case_keys=False)

        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "User.Name": "John",
            "User.Email": "john@example.com"
        }
        assert parsed == expected

    def test_lowercase_with_arrays(self):
        """Test lowercase conversion with array indices."""
        json_str = '{"Users": [{"Name": "John"}, {"Name": "Jane"}]}'
        result = flatten_json(json_str, lower_case_keys=True)

        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "users.0.name": "John",
            "users.1.name": "Jane"
        }
        assert parsed == expected

    def test_lowercase_with_key_replacements(self):
        """Test lowercase conversion combined with key replacements."""
        json_str = '{"User_Name": "John", "Admin_Role": "super", "Temp_Data": "test"}'
        key_replacements = [("regex:^(User|Admin)_", "")]
        result = flatten_json(
            json_str,
            key_replacements=key_replacements,
            lower_case_keys=True
        )

        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "name": "John",      # User_ removed, then lowercased
            "role": "super",     # Admin_ removed, then lowercased
            "temp_data": "test"  # Only lowercased (no regex match)
        }
        assert parsed == expected

    def test_lowercase_with_custom_separator(self):
        """Test lowercase conversion with custom separator."""
        json_str = '{"User": {"Profile": {"Name": "John"}}}'
        result = flatten_json(json_str, separator="_", lower_case_keys=True)

        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"user_profile_name": "John"}

    def test_lowercase_with_filtering(self):
        """Test lowercase conversion with value filtering."""
        json_str = '{"User": {"Name": "John", "Email": "", "Age": null}}'
        result = flatten_json(
            json_str,
            remove_empty_string_values=True,
            remove_null_values=True,
            lower_case_keys=True
        )

        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"user.name": "John"}

    def test_lowercase_values_unchanged(self):
        """Test that lowercase conversion only affects keys, not values."""
        json_str = '{"User": {"Name": "JOHN DOE", "Status": "ACTIVE"}}'
        result = flatten_json(json_str, lower_case_keys=True)

        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {
            "user.name": "JOHN DOE",    # Value case preserved
            "user.status": "ACTIVE"     # Value case preserved
        }
        assert parsed == expected


class TestReplacements:
    """Test key and value replacement functionality."""
    
    def test_key_replacements(self):
        """Test replacing keys with literal strings."""
        json_str = '{"user_name": "John", "user_email": "john@example.com"}'
        key_replacements = [("user_", "person_")]
        result = flatten_json(json_str, key_replacements=key_replacements)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {"person_name": "John", "person_email": "john@example.com"}
        assert parsed == expected
    
    def test_value_replacements(self):
        """Test replacing values with literal strings."""
        json_str = '{"email": "john@example.com", "backup_email": "john@example.com"}'
        value_replacements = [("@example.com", "@company.org")]
        result = flatten_json(json_str, value_replacements=value_replacements)
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        expected = {"email": "john@company.org", "backup_email": "john@company.org"}
        assert parsed == expected
    
    def test_combined_replacements(self):
        """Test combining key and value replacements."""
        json_str = '{"user_email": "john@example.com"}'
        key_replacements = [("user_", "person_")]
        value_replacements = [("@example.com", "@company.org")]
        result = flatten_json(
            json_str,
            key_replacements=key_replacements,
            value_replacements=value_replacements
        )
        
        flattened = result.get_single()
        parsed = json.loads(flattened)
        assert parsed == {"person_email": "john@company.org"}


class TestMultipleJsonInputs:
    """Test handling multiple JSON strings."""
    
    def test_multiple_json_strings(self):
        """Test flattening multiple JSON strings."""
        json_list = [
            '{"user1": {"name": "Alice"}}',
            '{"user2": {"name": "Bob"}}'
        ]
        result = flatten_json(json_list)
        
        assert not result.is_single
        assert result.is_multiple
        
        results = result.get_multiple()
        assert len(results) == 2
        
        parsed1 = json.loads(results[0])
        parsed2 = json.loads(results[1])
        assert parsed1 == {"user1.name": "Alice"}
        assert parsed2 == {"user2.name": "Bob"}
    
    def test_multiple_with_filtering(self):
        """Test multiple JSON strings with filtering."""
        json_list = [
            '{"name": "Alice", "email": ""}',
            '{"name": "Bob", "age": null}'
        ]
        result = flatten_json(
            json_list,
            remove_empty_string_values=True,
            remove_null_values=True
        )
        
        results = result.get_multiple()
        parsed1 = json.loads(results[0])
        parsed2 = json.loads(results[1])
        assert parsed1 == {"name": "Alice"}
        assert parsed2 == {"name": "Bob"}


class TestErrorHandling:
    """Test error handling and edge cases."""
    
    def test_invalid_json(self):
        """Test handling invalid JSON input."""
        with pytest.raises((JsonFlattenError, ValueError)):
            flatten_json('{"invalid": json}')
    
    def test_invalid_input_type(self):
        """Test handling invalid input types."""
        with pytest.raises(ValueError):
            flatten_json(123)  # type: ignore
    
    def test_get_single_on_multiple(self):
        """Test error when calling get_single on multiple results."""
        json_list = ['{"a": 1}', '{"b": 2}']
        result = flatten_json(json_list)
        
        with pytest.raises(ValueError, match="multiple JSON strings"):
            result.get_single()
    
    def test_get_multiple_on_single(self):
        """Test error when calling get_multiple on single result."""
        result = flatten_json('{"a": 1}')
        
        with pytest.raises(ValueError, match="single JSON string"):
            result.get_multiple()


class TestJsonOutputMethods:
    """Test JsonOutput class methods."""
    
    def test_to_python_single(self):
        """Test to_python method with single result."""
        result = flatten_json('{"a": 1}')
        python_result = result.to_python()
        
        assert isinstance(python_result, str)
        parsed = json.loads(python_result)
        assert parsed == {"a": 1}
    
    def test_to_python_multiple(self):
        """Test to_python method with multiple results."""
        json_list = ['{"a": 1}', '{"b": 2}']
        result = flatten_json(json_list)
        python_result = result.to_python()
        
        assert isinstance(python_result, list)
        assert len(python_result) == 2
        
        parsed1 = json.loads(python_result[0])
        parsed2 = json.loads(python_result[1])
        assert parsed1 == {"a": 1}
        assert parsed2 == {"b": 2}
