"""
Type stubs for json_tools_rs

This file provides type hints for the json_tools_rs Python package.
"""

from typing import List, Optional, Tuple, Union

__version__: str
__author__: str

class JsonFlattenError(Exception):
    """Exception raised when JSON flattening operations fail."""
    pass

class JsonOutput:
    """
    Container for JSON flattening results.
    
    This class wraps the result of JSON flattening operations and provides
    methods to access single or multiple results based on the input type.
    """
    
    @property
    def is_single(self) -> bool:
        """Check if this contains a single result."""
        ...
    
    @property
    def is_multiple(self) -> bool:
        """Check if this contains multiple results."""
        ...
    
    def get_single(self) -> str:
        """
        Get the single flattened JSON result.
        
        Returns:
            str: The flattened JSON string
            
        Raises:
            ValueError: If the result contains multiple JSON strings
        """
        ...
    
    def get_multiple(self) -> List[str]:
        """
        Get the multiple flattened JSON results.
        
        Returns:
            List[str]: List of flattened JSON strings
            
        Raises:
            ValueError: If the result contains a single JSON string
        """
        ...
    
    def to_python(self) -> Union[str, List[str]]:
        """
        Get the result as a Python object.
        
        Returns:
            Union[str, List[str]]: String for single result, list for multiple results
        """
        ...

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
    ...
