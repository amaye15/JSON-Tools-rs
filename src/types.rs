use rustc_hash::FxHashMap;
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;

// OPTIMIZATION: Use Arc<str> for HashMap keys to enable zero-copy sharing of repeated keys
// This significantly reduces memory allocations for large datasets with repeated field names
pub(crate) type FlatMap = FxHashMap<Arc<str>, Value>;

/// Input type for JSON flattening operations with Cow optimization
#[derive(Debug, Clone)]
#[repr(u8)] // OPTIMIZATION: Smaller discriminant for better cache locality
pub enum JsonInput<'a> {
    /// Single JSON string with Cow for efficient memory usage
    Single(Cow<'a, str>),
    /// Multiple JSON strings (borrowing)
    Multiple(&'a [&'a str]),
    /// Multiple JSON strings (owned)
    MultipleOwned(Vec<Cow<'a, str>>),
}

impl<'a> From<&'a str> for JsonInput<'a> {
    fn from(json: &'a str) -> Self {
        JsonInput::Single(Cow::Borrowed(json))
    }
}

impl<'a> From<&'a String> for JsonInput<'a> {
    fn from(json: &'a String) -> Self {
        JsonInput::Single(Cow::Borrowed(json.as_str()))
    }
}

impl<'a> From<&'a [&'a str]> for JsonInput<'a> {
    fn from(json_list: &'a [&'a str]) -> Self {
        JsonInput::Multiple(json_list)
    }
}

impl<'a> From<Vec<&'a str>> for JsonInput<'a> {
    fn from(json_list: Vec<&'a str>) -> Self {
        JsonInput::MultipleOwned(json_list.into_iter().map(Cow::Borrowed).collect())
    }
}

impl<'a> From<Vec<String>> for JsonInput<'a> {
    fn from(json_list: Vec<String>) -> Self {
        JsonInput::MultipleOwned(json_list.into_iter().map(Cow::Owned).collect())
    }
}

impl<'a> From<&'a [String]> for JsonInput<'a> {
    fn from(json_list: &'a [String]) -> Self {
        JsonInput::MultipleOwned(
            json_list
                .iter()
                .map(|s| Cow::Borrowed(s.as_str()))
                .collect(),
        )
    }
}

/// Output type for JSON flattening operations
#[derive(Debug, Clone)]
#[repr(u8)] // OPTIMIZATION: Smaller discriminant for better cache locality
pub enum JsonOutput {
    /// Single flattened JSON string
    Single(String),
    /// Multiple flattened JSON strings
    Multiple(Vec<String>),
}

impl JsonOutput {
    /// Extract single result, panicking if multiple results
    pub fn into_single(self) -> String {
        match self {
            JsonOutput::Single(result) => result,
            JsonOutput::Multiple(_) => panic!("Expected single result but got multiple"),
        }
    }

    /// Extract multiple results, panicking if single result
    pub fn into_multiple(self) -> Vec<String> {
        match self {
            JsonOutput::Single(_) => panic!("Expected multiple results but got single"),
            JsonOutput::Multiple(results) => results,
        }
    }

    /// Get results as vector (single result becomes vec with one element)
    pub fn into_vec(self) -> Vec<String> {
        match self {
            JsonOutput::Single(result) => vec![result],
            JsonOutput::Multiple(results) => results,
        }
    }
}
