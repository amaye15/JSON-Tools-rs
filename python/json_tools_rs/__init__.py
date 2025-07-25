"""
JSON Tools RS - Python Bindings

A Python library for advanced JSON manipulation, including flattening nested JSON structures
with configurable filtering and replacement options.

This package provides Python bindings for the high-performance Rust implementation of JSON
flattening functionality.

Features:
- Flatten nested JSON structures using dot notation
- Remove empty values (strings, objects, arrays, null values)
- Replace keys and values using literal strings or regex patterns
- Comprehensive error handling
- High performance through Rust implementation

Example:
    >>> import json_tools_rs
    >>> result = json_tools_rs.flatten_json('{"user": {"name": "John", "age": 30}}')
    >>> result.get_single()
    '{"user.name": "John", "user.age": 30}'
"""

from typing import List, Optional, Tuple, Union

# Import the Rust extension module
try:
    from .json_tools_rs import flatten_json as _flatten_json, JsonOutput, JsonFlattenError
except ImportError:
    # Fallback for development/testing
    def _flatten_json(*args, **kwargs):
        raise ImportError("Rust extension module not available. Please build the package with maturin.")
    
    class JsonOutput:
        """Placeholder JsonOutput class for when Rust module is not available"""
        pass
    
    class JsonFlattenError(Exception):
        """Placeholder exception class for when Rust module is not available"""
        pass

__version__ = "0.1.0"
__author__ = "JSON Tools RS Contributors"
__all__ = ["flatten_json", "JsonOutput", "JsonFlattenError"]


def flatten_json(
    json_input: Union[str, List[str]],
    remove_empty_string_values: bool = False,
    remove_null_values: bool = False,
    remove_empty_dict: bool = False,
    remove_empty_list: bool = False,
    key_replacements: Optional[List[Tuple[str, str]]] = None,
    value_replacements: Optional[List[Tuple[str, str]]] = None,
    separator: Optional[str] = None,
    lower_case_keys: bool = False,
) -> JsonOutput:
    """
    Flatten JSON with comprehensive options.
    
    This function flattens nested JSON structures into a single-level dictionary
    using dot notation for keys. It supports various filtering options and
    key/value replacement patterns.
    
    Args:
        json_input: JSON string or list of JSON strings to flatten
        remove_empty_string_values: Remove keys with empty string values
        remove_null_values: Remove keys with null values  
        remove_empty_dict: Remove keys with empty object values
        remove_empty_list: Remove keys with empty array values
        key_replacements: List of (old, new) tuples for key replacements.
                         Supports literal strings and regex patterns (prefix with "regex:")
        value_replacements: List of (old, new) tuples for value replacements.
                           Supports literal strings and regex patterns (prefix with "regex:")
        separator: Custom separator for flattened keys (default: ".")
        lower_case_keys: Convert all keys to lowercase after flattening and regex transformations
    
    Returns:
        JsonOutput: Object containing flattened JSON result(s).
                   Use .get_single() for single JSON input or .get_multiple() for multiple inputs.
    
    Raises:
        JsonFlattenError: If JSON parsing or processing fails
        ValueError: If arguments are invalid
    
    Examples:
        Basic flattening:
        >>> result = flatten_json('{"a": {"b": {"c": 1}}}')
        >>> result.get_single()
        '{"a.b.c": 1}'
        
        Remove empty values:
        >>> json_str = '{"user": {"name": "John", "email": "", "age": null}}'
        >>> result = flatten_json(json_str, remove_empty_string_values=True, remove_null_values=True)
        >>> result.get_single()
        '{"user.name": "John"}'
        
        Multiple JSON strings:
        >>> json_list = ['{"a": 1}', '{"b": 2}']
        >>> result = flatten_json(json_list)
        >>> result.get_multiple()
        ['{"a": 1}', '{"b": 2}']
        
        Key and value replacements:
        >>> json_str = '{"user_name": "john@example.com"}'
        >>> key_replacements = [("user_", "person_")]
        >>> value_replacements = [("@example.com", "@company.org")]
        >>> result = flatten_json(json_str, key_replacements=key_replacements, value_replacements=value_replacements)
        >>> result.get_single()
        '{"person_name": "john@company.org"}'
        
        Custom separator:
        >>> result = flatten_json('{"a": {"b": 1}}', separator="_")
        >>> result.get_single()
        '{"a_b": 1}'
        
        Array flattening:
        >>> result = flatten_json('{"items": [1, 2, {"nested": "value"}]}')
        >>> result.get_single()
        '{"items.0": 1, "items.1": 2, "items.2.nested": "value"}'

        Lowercase keys:
        >>> result = flatten_json('{"User": {"Name": "John", "Email": "john@example.com"}}', lower_case_keys=True)
        >>> result.get_single()
        '{"user.name": "John", "user.email": "john@example.com"}'
    """
    return _flatten_json(
        json_input,
        remove_empty_string_values,
        remove_null_values,
        remove_empty_dict,
        remove_empty_list,
        key_replacements,
        value_replacements,
        separator,
        lower_case_keys,
    )


# Re-export the main classes and exceptions
JsonOutput = JsonOutput
JsonFlattenError = JsonFlattenError
