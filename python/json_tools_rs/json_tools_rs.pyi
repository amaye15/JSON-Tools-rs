"""Type stubs for the json_tools_rs native extension module."""

from typing import Any, Optional, Union

class JsonToolsError(Exception):
    """Exception raised by JSON Tools operations."""

    ...

class JsonOutput:
    """Wrapper for JSON processing results (single or multiple)."""

    @property
    def is_single(self) -> bool:
        """True if this contains a single result."""
        ...

    @property
    def is_multiple(self) -> bool:
        """True if this contains multiple results."""
        ...

    def get_single(self) -> str:
        """Get the single result string. Raises ValueError if multiple."""
        ...

    def get_multiple(self) -> list[str]:
        """Get the multiple result strings. Raises ValueError if single."""
        ...

    def to_python(self) -> Union[str, list[str]]:
        """Get the result as a native Python object."""
        ...

class JSONTools:
    """High-performance JSON flattening/unflattening with builder pattern API.

    Supports str, dict, list[str], list[dict], DataFrame, and Series inputs.
    Output type matches input type automatically.

    Example::

        import json_tools_rs
        result = json_tools_rs.JSONTools().flatten().execute({"a": {"b": 1}})
        # result == {"a.b": 1}
    """

    def __init__(self) -> None: ...
    def flatten(self) -> "JSONTools":
        """Set operation mode to flatten nested JSON into dot-separated keys."""
        ...

    def unflatten(self) -> "JSONTools":
        """Set operation mode to unflatten dot-separated keys back into nested JSON."""
        ...

    def normal(self) -> "JSONTools":
        """Set operation mode to normal (apply transformations without flatten/unflatten)."""
        ...

    def separator(self, separator: str) -> "JSONTools":
        """Set the separator for nested keys (default: '.')."""
        ...

    def lowercase_keys(self, value: bool) -> "JSONTools":
        """Enable or disable lowercase key conversion."""
        ...

    def remove_empty_strings(self, value: bool) -> "JSONTools":
        """Enable or disable removal of keys with empty string values."""
        ...

    def remove_nulls(self, value: bool) -> "JSONTools":
        """Enable or disable removal of keys with null values."""
        ...

    def remove_empty_objects(self, value: bool) -> "JSONTools":
        """Enable or disable removal of keys with empty object values ({})."""
        ...

    def remove_empty_arrays(self, value: bool) -> "JSONTools":
        """Enable or disable removal of keys with empty array values ([])."""
        ...

    def key_replacement(self, find: str, replace: str) -> "JSONTools":
        """Add a key replacement pattern (regex or literal fallback)."""
        ...

    def value_replacement(self, find: str, replace: str) -> "JSONTools":
        """Add a value replacement pattern (regex or literal fallback)."""
        ...

    def handle_key_collision(self, value: bool) -> "JSONTools":
        """Enable collision handling by collecting duplicate keys into arrays."""
        ...

    def auto_convert_types(self, enable: bool) -> "JSONTools":
        """Enable automatic type conversion from strings to numbers and booleans."""
        ...

    def parallel_threshold(self, threshold: int) -> "JSONTools":
        """Set the minimum batch size for parallel processing."""
        ...

    def num_threads(self, num_threads: Optional[int]) -> "JSONTools":
        """Configure the number of threads for parallel processing."""
        ...

    def nested_parallel_threshold(self, threshold: int) -> "JSONTools":
        """Configure the threshold for nested parallel processing."""
        ...

    def max_array_index(self, max: int) -> "JSONTools":
        """Set the maximum array index allowed during unflattening (DoS protection)."""
        ...

    def execute(self, json_input: Any) -> Any:
        """Execute the configured JSON operation.

        Args:
            json_input: JSON input as str, dict, list[str], list[dict],
                DataFrame (pandas/polars/pyarrow/pyspark), or
                Series (pandas/polars/pyarrow).

        Returns:
            Output type matches input type automatically.

        Raises:
            JsonToolsError: If operation mode is not set or processing fails.
        """
        ...

    def execute_to_output(self, json_input: Any) -> JsonOutput:
        """Execute and return a JsonOutput wrapper instead of auto-detecting type."""
        ...
