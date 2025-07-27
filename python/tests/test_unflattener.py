#!/usr/bin/env python3
"""
Comprehensive tests for JsonUnflattener Python bindings

This test suite covers:
- Basic unflattening functionality
- Type preservation (str→str, dict→dict, list[str]→list[str], list[dict]→list[dict])
- Builder pattern configuration
- Error handling
- Roundtrip compatibility with JsonFlattener
"""

import json
import pytest
import json_tools_rs


class TestJsonUnflattenerBasic:
    """Test basic JsonUnflattener functionality."""

    def test_basic_string_unflattening(self):
        """Test basic unflattening with JSON string input."""
        flattened = '{"user.name": "John", "user.age": 30, "user.profile.city": "NYC"}'
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

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
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

        # Should return dict
        assert isinstance(result, dict)

        # Verify structure
        assert result["user"]["name"] == "John"
        assert result["user"]["age"] == 30
        assert result["user"]["profile"]["city"] == "NYC"

    def test_array_reconstruction(self):
        """Test reconstruction of arrays from flattened keys."""
        flattened = {"items.0": "first", "items.1": "second", "items.2": "third"}
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

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
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

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
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

        assert isinstance(result, str)
        parsed = json.loads(result)
        assert parsed == {"a": {"b": 1}, "c": {"d": 2}}

    def test_dict_input_dict_output(self):
        """Test dict input → dict output."""
        flattened = {"a.b": 1, "c.d": 2}
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"a": {"b": 1}, "c": {"d": 2}}

    def test_string_list_input_string_list_output(self):
        """Test list[str] input → list[str] output."""
        flattened_list = ['{"a.b": 1}', '{"c.d": 2}', '{"e.f": 3}']
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened_list)

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
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten(flattened_list)

        assert isinstance(result, list)
        assert len(result) == 3
        assert all(isinstance(item, dict) for item in result)

        # Verify each result
        assert result[0] == {"a": {"b": 1}}
        assert result[1] == {"c": {"d": 2}}
        assert result[2] == {"e": {"f": 3}}

    def test_empty_list_handling(self):
        """Test empty list handling."""
        unflattener = json_tools_rs.JsonUnflattener()
        result = unflattener.unflatten([])

        assert isinstance(result, list)
        assert len(result) == 0


class TestJsonUnflattenerBuilderPattern:
    """Test JsonUnflattener builder pattern configuration."""

    def test_custom_separator(self):
        """Test custom separator configuration."""
        flattened = {"user_name": "John", "user_age": 30}
        unflattener = json_tools_rs.JsonUnflattener().separator("_")
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_lowercase_keys(self):
        """Test lowercase keys configuration."""
        flattened = {"USER.NAME": "John", "USER.AGE": 30}
        unflattener = json_tools_rs.JsonUnflattener().lowercase_keys(True)
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_key_replacement(self):
        """Test key replacement configuration."""
        flattened = {"prefix.name": "John", "prefix.age": 30}
        unflattener = json_tools_rs.JsonUnflattener().key_replacement(
            "prefix.", "user."
        )
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John", "age": 30}}

    def test_value_replacement(self):
        """Test value replacement configuration."""
        flattened = {"user.email": "john@company.org", "user.name": "John"}
        unflattener = json_tools_rs.JsonUnflattener().value_replacement(
            "@company.org", "@example.com"
        )
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result["user"]["email"] == "john@example.com"
        assert result["user"]["name"] == "John"

    def test_regex_key_replacement(self):
        """Test regex key replacement."""
        flattened = {"user_name": "John", "admin_role": "super"}
        unflattener = json_tools_rs.JsonUnflattener().key_replacement(
            "regex:^(user|admin)_", "$1."
        )
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "John"}, "admin": {"role": "super"}}

    def test_chained_configuration(self):
        """Test chained builder pattern configuration."""
        flattened = {"PREFIX_NAME": "john@company.org", "PREFIX_AGE": 30}
        unflattener = (
            json_tools_rs.JsonUnflattener()
            .separator("_")
            .lowercase_keys(True)
            .key_replacement("prefix_", "user_")
            .value_replacement("@company.org", "@example.com")
        )
        result = unflattener.unflatten(flattened)

        assert isinstance(result, dict)
        assert result == {"user": {"name": "john@example.com", "age": 30}}


class TestJsonUnflattenerErrorHandling:
    """Test JsonUnflattener error handling."""

    def test_invalid_json_string(self):
        """Test handling of invalid JSON string."""
        unflattener = json_tools_rs.JsonUnflattener()
        with pytest.raises(json_tools_rs.JsonFlattenError):
            unflattener.unflatten('{"invalid": json}')

    def test_invalid_input_type(self):
        """Test handling of invalid input types."""
        unflattener = json_tools_rs.JsonUnflattener()
        with pytest.raises(ValueError):
            unflattener.unflatten(123)  # Invalid type

    def test_mixed_list_types(self):
        """Test handling of mixed list types."""
        unflattener = json_tools_rs.JsonUnflattener()
        with pytest.raises(ValueError):
            unflattener.unflatten(['{"a": 1}', 123, {"b": 2}])  # Mixed types

    def test_invalid_list_content(self):
        """Test handling of invalid list content."""
        unflattener = json_tools_rs.JsonUnflattener()
        with pytest.raises(ValueError):
            unflattener.unflatten([None, "test"])  # Invalid content


class TestJsonUnflattenerRoundtrip:
    """Test roundtrip compatibility between JsonFlattener and JsonUnflattener."""

    def test_simple_roundtrip(self):
        """Test simple roundtrip: original → flatten → unflatten → original."""
        original = {"user": {"name": "John", "age": 30}}

        # Flatten
        flattener = json_tools_rs.JsonFlattener()
        flattened = flattener.flatten(original)

        # Unflatten
        unflattener = json_tools_rs.JsonUnflattener()
        restored = unflattener.unflatten(flattened)

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
        flattener = json_tools_rs.JsonFlattener()
        flattened = flattener.flatten(original)

        # Unflatten
        unflattener = json_tools_rs.JsonUnflattener()
        restored = unflattener.unflatten(flattened)

        # Should be equivalent to original
        assert restored == original

    def test_roundtrip_with_custom_separator(self):
        """Test roundtrip with custom separator."""
        original = {"user": {"name": "John", "profile": {"city": "NYC"}}}

        # Flatten with custom separator
        flattener = json_tools_rs.JsonFlattener().separator("_")
        flattened = flattener.flatten(original)

        # Unflatten with same separator
        unflattener = json_tools_rs.JsonUnflattener().separator("_")
        restored = unflattener.unflatten(flattened)

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
        flattener = json_tools_rs.JsonFlattener()
        flattened_batch = flattener.flatten(originals)

        # Unflatten batch
        unflattener = json_tools_rs.JsonUnflattener()
        restored_batch = unflattener.unflatten(flattened_batch)

        # Should be equivalent to originals
        assert restored_batch == originals


if __name__ == "__main__":
    pytest.main([__file__])
