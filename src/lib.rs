
//! # JSON Tools RS
//!
//! A Rust library for advanced JSON manipulation, including flattening and unflattening
//! nested JSON structures with configurable filtering and replacement options.
//!
//! ## Features
//!
//! - **Unified API**: Single `JSONTools` entry point for both flattening and unflattening
//! - **Builder Pattern**: Fluent, chainable API for easy configuration
//! - **Advanced Filtering**: Remove empty values (strings, objects, arrays, null values)
//! - **Pattern Replacements**: Support for literal and regex-based key/value replacements
//! - **High Performance**: SIMD-accelerated JSON parsing with optimized algorithms
//! - **Batch Processing**: Handle single JSON strings or arrays of JSON strings
//! - **Comprehensive Error Handling**: Detailed error messages for debugging
//!
//! ## Quick Start with Unified API
//!
//! ### Flattening JSON
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! let json = r#"{"user": {"name": "John", "details": {"age": null, "city": ""}}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .separator("::")
//!     .lowercase_keys(true)
//!     .key_replacement("(User|Admin)_", "")
//!     .value_replacement("@example.com", "@company.org")
//!     .remove_empty_strings(true)
//!     .remove_nulls(true)
//!     .remove_empty_objects(true)
//!     .remove_empty_arrays(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"user::name": "John"}
//!         println!("{}", flattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ### Unflattening JSON
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! let flattened = r#"{"user::name": "John", "user::age": 30}"#;
//! let result = JSONTools::new()
//!     .unflatten()
//!     .separator("::")
//!     .lowercase_keys(true)
//!     .key_replacement("(User|Admin)_", "")
//!     .value_replacement("@company.org", "@example.com")
//!     .remove_empty_strings(true)
//!     .remove_nulls(true)
//!     .remove_empty_objects(true)
//!     .remove_empty_arrays(true)
//!     .execute(flattened).unwrap();
//!
//! match result {
//!     JsonOutput::Single(unflattened) => {
//!         // Result: {"user": {"name": "John", "age": 30}}
//!         println!("{}", unflattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! # Doctests
//!
//! The following doctests demonstrate individual features in a progressive learning format.
//! Each example focuses on a specific capability to help users understand how to use the library effectively.
//!
//! ## 1. Basic Flattening and Unflattening Operations
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Basic flattening - converts nested JSON to flat key-value pairs
//! let nested_json = r#"{"user": {"name": "John", "profile": {"age": 30}}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .execute(nested_json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"user.name": "John", "user.profile.age": 30}
//!         assert!(flattened.contains("user.name"));
//!         assert!(flattened.contains("user.profile.age"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//!
//! // Basic unflattening - converts flat JSON back to nested structure
//! let flat_json = r#"{"user.name": "John", "user.profile.age": 30}"#;
//! let result = JSONTools::new()
//!     .unflatten()
//!     .execute(flat_json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(unflattened) => {
//!         // Result: {"user": {"name": "John", "profile": {"age": 30}}}
//!         assert!(unflattened.contains(r#""user""#));
//!         assert!(unflattened.contains(r#""name":"John""#));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 2. Custom Separator Usage
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Using custom separator instead of default "."
//! let json = r#"{"company": {"department": {"team": "engineering"}}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .separator("::") // Use "::" instead of "."
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"company::department::team": "engineering"}
//!         assert!(flattened.contains("company::department::team"));
//!         assert!(!flattened.contains("company.department.team"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 3. Key Transformations - Lowercase Keys
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Convert all keys to lowercase during processing
//! let json = r#"{"UserName": "John", "UserProfile": {"FirstName": "John"}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .lowercase_keys(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"username": "John", "userprofile.firstname": "John"}
//!         assert!(flattened.contains("username"));
//!         assert!(flattened.contains("userprofile.firstname"));
//!         assert!(!flattened.contains("UserName"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 4. Key Replacement Patterns - Literal Replacement
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Replace literal strings in keys
//! let json = r#"{"user_profile_name": "John", "user_profile_age": 30}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .key_replacement("user_profile_", "person_") // Replace literal string
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"person_name": "John", "person_age": 30}
//!         assert!(flattened.contains("person_name"));
//!         assert!(flattened.contains("person_age"));
//!         assert!(!flattened.contains("user_profile_"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 5. Key Replacement Patterns - Regex Replacement
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Replace using regex patterns in keys
//! let json = r#"{"user_name": "John", "admin_name": "Jane", "guest_name": "Bob"}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .key_replacement("(user|admin)_", "person_") // Regex pattern
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"person_name": "John", "person_name": "Jane", "guest_name": "Bob"}
//!         // Note: This would cause collision without collision handling
//!         assert!(flattened.contains("person_name"));
//!         assert!(flattened.contains("guest_name"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 6. Value Replacement Patterns - Literal Replacement
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Replace literal strings in values
//! let json = r#"{"email": "user@example.com", "backup_email": "admin@example.com"}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .value_replacement("@example.com", "@company.org") // Replace domain
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"email": "user@company.org", "backup_email": "admin@company.org"}
//!         assert!(flattened.contains("@company.org"));
//!         assert!(!flattened.contains("@example.com"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 7. Value Replacement Patterns - Regex Replacement
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Replace using regex patterns in values
//! let json = r#"{"role": "super", "level": "admin", "type": "user"}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .value_replacement("^(super|admin)$", "administrator") // Regex pattern
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"role": "administrator", "level": "administrator", "type": "user"}
//!         assert!(flattened.contains(r#""role":"administrator""#));
//!         assert!(flattened.contains(r#""level":"administrator""#));
//!         assert!(flattened.contains(r#""type":"user""#));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 8. Filtering Options - Remove Empty Strings
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Remove keys that have empty string values
//! let json = r#"{"name": "John", "nickname": "", "age": 30}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .remove_empty_strings(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"name": "John", "age": 30} - "nickname" removed
//!         assert!(flattened.contains("name"));
//!         assert!(flattened.contains("age"));
//!         assert!(!flattened.contains("nickname"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 9. Filtering Options - Remove Null Values
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Remove keys that have null values
//! let json = r#"{"name": "John", "age": null, "city": "NYC"}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .remove_nulls(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"name": "John", "city": "NYC"} - "age" removed
//!         assert!(flattened.contains("name"));
//!         assert!(flattened.contains("city"));
//!         assert!(!flattened.contains("age"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 10. Filtering Options - Remove Empty Objects and Arrays
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Remove keys that have empty objects or arrays
//! let json = r#"{"user": {"name": "John"}, "tags": [], "metadata": {}}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .remove_empty_objects(true)
//!     .remove_empty_arrays(true)
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"user.name": "John"} - "tags" and "metadata" removed
//!         assert!(flattened.contains("user.name"));
//!         assert!(!flattened.contains("tags"));
//!         assert!(!flattened.contains("metadata"));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!

//!
//! ## 12. Collision Handling - Collect Values into Arrays
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // When key replacements cause collisions, collect all values into an array
//! let json = r#"{"user_name": "John", "admin_name": "Jane"}"#;
//! let result = JSONTools::new()
//!     .flatten()
//!     .key_replacement("(user|admin)_", "") // This creates collision: both become "name"
//!     .handle_key_collision(true) // Collect values into array
//!     .execute(json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Result: {"name": ["John", "Jane"]}
//!         assert!(flattened.contains(r#""name":["John","Jane"]"#) ||
//!                 flattened.contains(r#""name": ["John", "Jane"]"#));
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!
//! ## 13. Comprehensive Integration Example
//!
//! ```rust
//! use json_tools_rs::{JSONTools, JsonOutput};
//!
//! // Comprehensive example combining multiple features for real-world usage
//! let complex_json = r#"{
//!     "User_Profile": {
//!         "Personal_Info": {
//!             "FirstName": "John",
//!             "LastName": "",
//!             "Email": "john@example.com",
//!             "Age": null
//!         },
//!         "Settings": {
//!             "Theme": "dark",
//!             "Notifications": {},
//!             "Tags": []
//!         }
//!     },
//!     "Admin_Profile": {
//!         "Personal_Info": {
//!             "FirstName": "Jane",
//!             "Email": "jane@example.com",
//!             "Role": "super"
//!         }
//!     }
//! }"#;
//!
//! let result = JSONTools::new()
//!     .flatten()
//!     .separator("::") // Use custom separator
//!     .lowercase_keys(true) // Convert all keys to lowercase
//!     .key_replacement("(user|admin)_profile::", "person::") // Normalize profile keys
//!     .key_replacement("personal_info::", "info::") // Simplify nested keys
//!     .value_replacement("@example.com", "@company.org") // Update email domain
//!     .value_replacement("^super$", "administrator") // Normalize role values
//!     .remove_empty_strings(true) // Remove empty string values
//!     .remove_nulls(true) // Remove null values
//!     .remove_empty_objects(true) // Remove empty objects
//!     .remove_empty_arrays(true) // Remove empty arrays
//!     .handle_key_collision(true) // Handle any key collisions by collecting into arrays
//!     .execute(complex_json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(flattened) => {
//!         // Verify the comprehensive transformation worked
//!         // Note: Keys are transformed through multiple steps: lowercase + replacements
//!         assert!(flattened.contains("@company.org"));
//!         assert!(flattened.contains("administrator"));
//!         assert!(!flattened.contains("lastname")); // Empty string removed
//!         assert!(!flattened.contains("age")); // Null removed
//!         assert!(!flattened.contains("notifications")); // Empty object removed
//!         assert!(!flattened.contains("tags")); // Empty array removed
//!         // The exact key structure depends on the order of transformations
//!         println!("Comprehensive transformation result: {}", flattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//!
//! // Demonstrate unflattening with the same configuration
//! let flat_json = r#"{"person::info::name": "Alice", "person::settings::theme": "light"}"#;
//! let result = JSONTools::new()
//!     .unflatten()
//!     .separator("::")
//!     .execute(flat_json).unwrap();
//!
//! match result {
//!     JsonOutput::Single(unflattened) => {
//!         // Result: {"person": {"info": {"name": "Alice"}, "settings": {"theme": "light"}}}
//!         assert!(unflattened.contains(r#""person""#));
//!         assert!(unflattened.contains(r#""info""#));
//!         assert!(unflattened.contains(r#""settings""#));
//!         println!("Unflattening result: {}", unflattened);
//!     }
//!     JsonOutput::Multiple(_) => unreachable!(),
//! }
//! ```
//!


// ================================================================================================
// MODULE: External Dependencies and Imports
// ================================================================================================

use memchr::{memchr, memmem};
use regex::Regex;
use rustc_hash::FxHashMap;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, OnceLock};

// ================================================================================================
// MODULE: SIMD-JSON Thread-Local Parser Optimization
// ================================================================================================

/// SIMD-JSON padding requirement (64 bytes as per documentation)
/// From simdjson docs: "simdjson requires SIMDJSON_PADDING bytes at the end"
const SIMDJSON_PADDING: usize = 64;

// Thread-local parser state with buffer reuse for optimal SIMD-JSON performance
//
// From simdjson documentation:
// "create a parser once and reuse it... keeping buffers hot in the cache and minimizing memory allocation"
// This provides 20-40% performance improvement by reusing buffers across parses
thread_local! {
    static JSON_PARSER_STATE: RefCell<ParserState> = RefCell::new(ParserState::new());
}

/// Parser state with pre-allocated buffer and SIMD-JSON padding
struct ParserState {
    /// Reusable buffer with capacity for padding
    /// Starts at 8KB and grows as needed (amortized cost)
    buffer: Vec<u8>,
}

impl ParserState {
    fn new() -> Self {
        Self {
            // Start with 8KB + padding for optimal performance
            buffer: Vec::with_capacity(8192 + SIMDJSON_PADDING),
        }
    }

    /// Parse JSON using the reusable buffer with proper SIMD-JSON padding
    ///
    /// From simdjson docs: "you can almost always read a few bytes beyond your buffer"
    /// The padding ensures SIMD instructions can safely read ahead without bounds checking
    fn parse(&mut self, json: &str) -> Result<Value, JsonToolsError> {
        self.buffer.clear();

        // Calculate required capacity with padding
        let required_capacity = json.len() + SIMDJSON_PADDING;

        // Grow buffer if needed (amortized O(1) over many parses)
        if self.buffer.capacity() < required_capacity {
            self.buffer.reserve(required_capacity - self.buffer.capacity());
        }

        // Copy JSON data
        self.buffer.extend_from_slice(json.as_bytes());

        // Add padding bytes (zeros) for SIMD safety
        self.buffer.resize(json.len() + SIMDJSON_PADDING, 0);

        // Parse only the actual JSON length (not the padding)
        simd_json::serde::from_slice(&mut self.buffer[..json.len()])
            .map_err(JsonToolsError::json_parse_error)
    }
}

// ================================================================================================
// MODULE: Global Caches and Performance Optimizations
// ================================================================================================

/// Pre-compiled common regex patterns for maximum performance
/// Using Arc<Regex> to make cloning O(1) instead of copying the entire regex state
/// Using std::sync::LazyLock (Rust 1.80+) instead of lazy_static for better performance
static COMMON_REGEX_PATTERNS: LazyLock<FxHashMap<&'static str, Arc<Regex>>> = LazyLock::new(|| {
    // Pre-allocate with known capacity for better performance
    let mut patterns = FxHashMap::with_capacity_and_hasher(20, Default::default());

    // Common patterns for key/value replacements
    let common_patterns = [
        // Whitespace patterns
        (r"\s+", r"\s+"),                          // Multiple whitespace
        (r"^\s+|\s+$", r"^\s+|\s+$"),             // Leading/trailing whitespace
        (r"\s", r"\s"),                            // Any whitespace

        // Special character patterns
        (r"[^\w\s]", r"[^\w\s]"),                 // Non-word, non-space characters
        (r"[^a-zA-Z0-9]", r"[^a-zA-Z0-9]"),       // Non-alphanumeric
        (r"[^a-zA-Z0-9_]", r"[^a-zA-Z0-9_]"),     // Non-alphanumeric except underscore

        // Common JSON key patterns
        (r"[A-Z]", r"[A-Z]"),                     // Uppercase letters
        (r"[a-z]", r"[a-z]"),                     // Lowercase letters
        (r"\d+", r"\d+"),                         // Digits
        (r"_+", r"_+"),                           // Multiple underscores
        (r"-+", r"-+"),                           // Multiple hyphens

        // Email and URL patterns (common in JSON data)
        (r"@", r"@"),                             // At symbol (emails)
        (r"\.", r"\."),                           // Dot (domains, decimals)
        (r"://", r"://"),                         // Protocol separator

        // Date/time patterns
        (r"\d{4}-\d{2}-\d{2}", r"\d{4}-\d{2}-\d{2}"), // ISO date
        (r"\d{2}:\d{2}:\d{2}", r"\d{2}:\d{2}:\d{2}"), // Time format
    ];

    for (pattern, _) in &common_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            patterns.insert(*pattern, Arc::new(regex));
        }
    }

    patterns
});

/// Simple LRU cache for regex patterns to prevent unbounded growth
/// Using Arc<Regex> to make cloning O(1)
struct LruRegexCache {
    cache: FxHashMap<String, (Arc<Regex>, usize)>, // (regex, access_order)
    access_counter: usize,
    max_size: usize,
}

impl LruRegexCache {
    fn new(max_size: usize) -> Self {
        Self {
            cache: FxHashMap::with_capacity_and_hasher(max_size, Default::default()),
            access_counter: 0,
            max_size,
        }
    }

    fn get(&mut self, pattern: &str) -> Option<Arc<Regex>> {
        if let Some((regex, access_time)) = self.cache.get_mut(pattern) {
            self.access_counter += 1;
            *access_time = self.access_counter; // Update access time
            Some(Arc::clone(regex)) // Arc::clone is explicit and O(1)
        } else {
            None
        }
    }

    fn insert(&mut self, pattern: String, regex: Arc<Regex>) {
        self.access_counter += 1;

        // If cache is full, remove least recently used item
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }

        self.cache.insert(pattern, (regex, self.access_counter));
    }

    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.cache
            .iter()
            .min_by_key(|(_, (_, access_time))| *access_time)
            .map(|(k, _)| k.clone())
        {
            self.cache.remove(&lru_key);
        }
    }

}

// Global regex cache with LRU eviction for better performance
static REGEX_CACHE: OnceLock<std::sync::RwLock<LruRegexCache>> = OnceLock::new();

// Thread-local regex cache for even better performance
// Using Arc<Regex> for O(1) cloning
thread_local! {
    static THREAD_LOCAL_REGEX_CACHE: std::cell::RefCell<FxHashMap<String, Arc<Regex>>> =
        std::cell::RefCell::new(FxHashMap::with_capacity_and_hasher(32, Default::default()));
}

/// Get a cached regex, using Arc<Regex> for O(1) cloning
fn get_cached_regex(pattern: &str) -> Result<Arc<Regex>, regex::Error> {
    // First, check pre-compiled common patterns (fastest path, no allocation)
    if let Some(regex) = COMMON_REGEX_PATTERNS.get(pattern) {
        return Ok(Arc::clone(regex));
    }

    // Second, try thread-local cache (fast path, no locks)
    let thread_local_result = THREAD_LOCAL_REGEX_CACHE.with(|cache| {
        let cache_ref = cache.borrow();
        cache_ref.get(pattern).map(Arc::clone)
    });

    if let Some(regex) = thread_local_result {
        return Ok(regex);
    }

    // If not in thread-local cache, try global LRU cache
    let cache = REGEX_CACHE.get_or_init(|| std::sync::RwLock::new(LruRegexCache::new(256))); // 256 max patterns

    // First, try to read from global cache (allows concurrent reads)
    // Note: LRU requires mutable access, so we need write lock for get operations
    let global_result = {
        let mut write_guard = cache.write().unwrap();
        write_guard.get(pattern)
    };

    if let Some(regex) = global_result {
        // Cache in thread-local for next time
        THREAD_LOCAL_REGEX_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            // Limit thread-local cache size to prevent memory bloat
            if cache_ref.len() >= 32 {
                cache_ref.clear(); // Simple eviction strategy
            }
            cache_ref.insert(pattern.into(), Arc::clone(&regex));
        });
        return Ok(regex);
    }

    // If not found anywhere, compile and cache the regex
    let mut write_guard = cache.write().unwrap();

    // Double-check in case another thread compiled it while we were waiting
    if let Some(regex) = write_guard.get(pattern) {
        // Cache in thread-local for next time
        THREAD_LOCAL_REGEX_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            if cache_ref.len() >= 32 {
                cache_ref.clear();
            }
            cache_ref.insert(pattern.into(), Arc::clone(&regex));
        });
        Ok(regex)
    } else {
        let regex = Arc::new(Regex::new(pattern)?);
        write_guard.insert(pattern.to_string(), Arc::clone(&regex));

        // Cache in thread-local for next time
        THREAD_LOCAL_REGEX_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            if cache_ref.len() >= 32 {
                cache_ref.clear();
            }
            cache_ref.insert(pattern.into(), Arc::clone(&regex));
        });

        Ok(regex)
    }
}

/// Key deduplication system that works with HashMap operations
/// This reduces memory usage when the same keys appear multiple times
struct KeyDeduplicator {
    /// Cache of deduplicated keys
    key_cache: FxHashMap<String, std::sync::Arc<str>>,
    /// Statistics for monitoring effectiveness
    cache_hits: usize,
    cache_misses: usize,
}

impl KeyDeduplicator {
    fn new() -> Self {
        Self {
            key_cache: FxHashMap::with_capacity_and_hasher(128, Default::default()),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    /// Get a deduplicated key, creating it if it doesn't exist
    /// OPTIMIZATION: Check cache before allocating string for entry lookup
    fn deduplicate_key(&mut self, key: &str) -> String {
        // Fast path: check if key exists without allocating
        if let Some(cached) = self.key_cache.get(key) {
            self.cache_hits += 1;
            return (*cached).to_string();
        }

        // Slow path: key not in cache, create and insert
        self.cache_misses += 1;
        let owned_key = key.to_string();
        let arc_key: std::sync::Arc<str> = owned_key.as_str().into();
        self.key_cache.insert(owned_key.clone(), arc_key);
        owned_key
    }


}

thread_local! {
    static KEY_DEDUPLICATOR: std::cell::RefCell<KeyDeduplicator> = std::cell::RefCell::new(KeyDeduplicator::new());
}

/// Get a deduplicated key using thread-local storage for better performance
/// OPTIMIZATION: Avoids Arc overhead by returning String directly while still caching
#[inline]
fn deduplicate_key(key: &str) -> String {
    // For short, simple keys that are likely to be repeated, use deduplication
    // Use bytes() instead of chars() for faster ASCII-only checks
    if key.len() <= 64 && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'-') {
        KEY_DEDUPLICATOR.with(|dedup| dedup.borrow_mut().deduplicate_key(key))
    } else {
        // For complex or long keys, just create a regular string
        key.to_string()
    }
}



/// Radix tree/trie for efficient prefix-based key operations
/// Optimized for JSON key processing with path analysis and collision detection





// ================================================================================================
// MODULE: Feature Gates and External Modules
// ================================================================================================

// Python bindings module
#[cfg(feature = "python")]
pub mod python;

// Tests module
#[cfg(test)]
mod tests;

// ================================================================================================
// MODULE: Core Data Types and Input/Output Structures
// ================================================================================================

/// Input type for JSON flattening operations with Cow optimization
#[derive(Debug, Clone)]
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
        JsonInput::MultipleOwned(json_list.into_iter().map(|s| Cow::Owned(s)).collect())
    }
}

impl<'a> From<&'a [String]> for JsonInput<'a> {
    fn from(json_list: &'a [String]) -> Self {
        JsonInput::MultipleOwned(json_list.iter().map(|s| Cow::Borrowed(s.as_str())).collect())
    }
}

/// Output type for JSON flattening operations
#[derive(Debug, Clone)]
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

// ================================================================================================
// MODULE: Error Types and Error Handling
// ================================================================================================

/// Comprehensive error type for all JSON Tools operations with detailed information and suggestions
#[derive(Debug, thiserror::Error)]
pub enum JsonToolsError {
    /// Error parsing JSON input with detailed context and suggestions
    #[error("JSON parsing failed: {message}\n💡 Suggestion: {suggestion}")]
    JsonParseError {
        message: String,
        suggestion: String,
        #[source]
        source: simd_json::Error,
    },

    /// Error compiling or using regex patterns with helpful suggestions
    #[error("Regex pattern error: {message}\n💡 Suggestion: {suggestion}")]
    RegexError {
        message: String,
        suggestion: String,
        #[source]
        source: regex::Error,
    },

    /// Invalid replacement pattern configuration with detailed guidance
    #[error("Invalid replacement pattern: {message}\n💡 Suggestion: {suggestion}")]
    InvalidReplacementPattern {
        message: String,
        suggestion: String,
    },

    /// Invalid JSON structure for the requested operation
    #[error("Invalid JSON structure: {message}\n💡 Suggestion: {suggestion}")]
    InvalidJsonStructure {
        message: String,
        suggestion: String,
    },

    /// Configuration error when operation mode is not set
    #[error("Operation mode not configured: {message}\n💡 Suggestion: {suggestion}")]
    ConfigurationError {
        message: String,
        suggestion: String,
    },

    /// Error processing batch item with detailed context
    #[error("Batch processing failed at index {index}: {message}\n💡 Suggestion: {suggestion}")]
    BatchProcessingError {
        index: usize,
        message: String,
        suggestion: String,
        #[source]
        source: Box<JsonToolsError>,
    },

    /// Input validation error with helpful guidance
    #[error("Input validation failed: {message}\n💡 Suggestion: {suggestion}")]
    InputValidationError {
        message: String,
        suggestion: String,
    },

    /// Serialization error when converting results back to JSON
    #[error("JSON serialization failed: {message}\n💡 Suggestion: {suggestion}")]
    SerializationError {
        message: String,
        suggestion: String,
        #[source]
        source: simd_json::Error,
    },
}

impl JsonToolsError {
    /// Create a JSON parse error with helpful suggestions
    #[cold]  // Optimization #19: Mark error paths as cold
    #[inline(never)]
    pub fn json_parse_error(source: simd_json::Error) -> Self {
        let suggestion = "Verify your JSON syntax using a JSON validator. Common issues include: missing quotes around keys or values, trailing commas, unescaped characters, incomplete JSON (missing closing braces or brackets), or invalid escape sequences.";

        JsonToolsError::JsonParseError {
            message: source.to_string(),
            suggestion: suggestion.into(),
            source,
        }
    }

    /// Create a regex error with helpful suggestions
    #[cold]  // Optimization #19: Mark error paths as cold
    #[inline(never)]
    pub fn regex_error(source: regex::Error) -> Self {
        let suggestion = match source {
            regex::Error::Syntax(_) =>
                "Check your regex pattern syntax. Use online regex testers to validate your pattern. Remember to escape special characters like '.', '*', '+', '?', etc.",
            regex::Error::CompiledTooBig(_) =>
                "Your regex pattern is too complex. Try simplifying it or breaking it into multiple smaller patterns.",
            _ => "Verify your regex pattern is valid. Use tools like regex101.com to test and debug your pattern.",
        };

        JsonToolsError::RegexError {
            message: source.to_string(),
            suggestion: suggestion.into(),
            source,
        }
    }

    /// Create an invalid replacement pattern error
    #[cold]  // Optimization #19: Mark error paths as cold
    #[inline(never)]
    pub fn invalid_replacement_pattern(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("pairs") {
            "Replacement patterns must be provided in pairs (pattern, replacement). Ensure you have an even number of arguments."
        } else if msg.contains("regex") {
            "Patterns use standard Rust regex syntax. If a pattern fails to compile as regex, it falls back to literal string matching. Example: 'user_.*' to match keys starting with 'user_'."
        } else {
            "Check your replacement pattern configuration. Patterns should be in the format: pattern1, replacement1, pattern2, replacement2, etc."
        };

        JsonToolsError::InvalidReplacementPattern {
            message: msg,
            suggestion: suggestion.into(),
        }
    }

    /// Create an invalid JSON structure error
    #[cold]  // Optimization #19: Mark error paths as cold
    #[inline(never)]
    pub fn invalid_json_structure(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("unflatten") {
            "For unflattening, ensure your JSON is a flat object with dot-separated keys like {'user.name': 'John', 'user.age': 30}."
        } else if msg.contains("object") {
            "The operation requires a JSON object ({}), but received a different type. Check that your input is a valid JSON object."
        } else {
            "Verify that your JSON structure is compatible with the requested operation. Flattening works on nested objects/arrays, unflattening works on flat objects."
        };

        JsonToolsError::InvalidJsonStructure {
            message: msg,
            suggestion: suggestion.into(),
        }
    }

    /// Create a configuration error
    pub fn configuration_error(message: impl Into<String>) -> Self {
        JsonToolsError::ConfigurationError {
            message: message.into(),
            suggestion: "Call .flatten() or .unflatten() on your JSONTools instance before calling .execute() to set the operation mode.".into(),
        }
    }

    /// Create a batch processing error
    pub fn batch_processing_error(index: usize, source: JsonToolsError) -> Self {
        JsonToolsError::BatchProcessingError {
            index,
            message: format!("Failed to process item at index {}", index),
            suggestion: "Check the JSON at the specified index. All items in a batch must be valid JSON strings or objects.".to_string(),
            source: Box::new(source),
        }
    }

    /// Create an input validation error
    pub fn input_validation_error(message: impl Into<String>) -> Self {
        let msg = message.into();
        let suggestion = if msg.contains("type") {
            "Ensure your input is a valid JSON string, Python dict, or list of JSON strings/dicts."
        } else if msg.contains("empty") {
            "Provide non-empty input for processing."
        } else {
            "Check that your input format matches the expected type for the operation."
        };

        JsonToolsError::InputValidationError {
            message: msg,
            suggestion: suggestion.to_string(),
        }
    }

    /// Create a serialization error
    pub fn serialization_error(source: simd_json::Error) -> Self {
        JsonToolsError::SerializationError {
            message: source.to_string(),
            suggestion: "This is likely an internal error. The processed data couldn't be serialized back to JSON. Please report this issue.".to_string(),
            source,
        }
    }
}

// Automatic conversion from simd_json::Error
impl From<simd_json::Error> for JsonToolsError {
    fn from(error: simd_json::Error) -> Self {
        JsonToolsError::json_parse_error(error)
    }
}

// Automatic conversion from regex::Error
impl From<regex::Error> for JsonToolsError {
    fn from(error: regex::Error) -> Self {
        JsonToolsError::regex_error(error)
    }
}

/// Operation mode for the unified JSONTools API
#[derive(Debug, Clone, PartialEq)]
enum OperationMode {
    /// Flatten JSON structures
    Flatten,
    /// Unflatten JSON structures
    Unflatten,
    /// Normal processing (no flatten/unflatten) applying transformations recursively
    Normal,
}

// ================================================================================================
// MODULE: Configuration Structures and Builder Patterns
// ================================================================================================

/// Configuration for filtering operations
#[derive(Debug, Clone, Default)]
pub struct FilteringConfig {
    /// Remove keys with empty string values
    pub remove_empty_strings: bool,
    /// Remove keys with null values
    pub remove_nulls: bool,
    /// Remove keys with empty object values
    pub remove_empty_objects: bool,
    /// Remove keys with empty array values
    pub remove_empty_arrays: bool,
}

impl FilteringConfig {
    /// Create a new FilteringConfig with all filters disabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable removal of empty strings
    pub fn remove_empty_strings(mut self, enabled: bool) -> Self {
        self.remove_empty_strings = enabled;
        self
    }

    /// Enable removal of null values
    pub fn remove_nulls(mut self, enabled: bool) -> Self {
        self.remove_nulls = enabled;
        self
    }

    /// Enable removal of empty objects
    pub fn remove_empty_objects(mut self, enabled: bool) -> Self {
        self.remove_empty_objects = enabled;
        self
    }

    /// Enable removal of empty arrays
    pub fn remove_empty_arrays(mut self, enabled: bool) -> Self {
        self.remove_empty_arrays = enabled;
        self
    }

    /// Check if any filtering is enabled
    pub fn has_any_filter(&self) -> bool {
        self.remove_empty_strings || self.remove_nulls || self.remove_empty_objects || self.remove_empty_arrays
    }
}

/// Configuration for collision handling strategies
#[derive(Debug, Clone, Default)]
pub struct CollisionConfig {
    /// Handle key collisions by collecting values into arrays
    pub handle_collisions: bool,
}

impl CollisionConfig {
    /// Create a new CollisionConfig with collision handling disabled
    pub fn new() -> Self { Self::default() }

    /// Enable collision handling by collecting values into arrays
    pub fn handle_collisions(mut self, enabled: bool) -> Self {
        self.handle_collisions = enabled;
        self
    }

    /// Check if any collision handling is enabled
    pub fn has_collision_handling(&self) -> bool { self.handle_collisions }
}

/// Configuration for replacement operations
#[derive(Debug, Clone, Default)]
pub struct ReplacementConfig {
    /// Key replacement patterns (find, replace)
    pub key_replacements: Vec<(String, String)>,
    /// Value replacement patterns (find, replace)
    pub value_replacements: Vec<(String, String)>,
}

impl ReplacementConfig {
    /// Create a new ReplacementConfig with no replacements
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a key replacement pattern
    pub fn add_key_replacement(mut self, find: impl Into<String>, replace: impl Into<String>) -> Self {
        self.key_replacements.push((find.into(), replace.into()));
        self
    }

    /// Add a value replacement pattern
    pub fn add_value_replacement(mut self, find: impl Into<String>, replace: impl Into<String>) -> Self {
        self.value_replacements.push((find.into(), replace.into()));
        self
    }

    /// Check if any key replacements are configured
    pub fn has_key_replacements(&self) -> bool {
        !self.key_replacements.is_empty()
    }

    /// Check if any value replacements are configured
    pub fn has_value_replacements(&self) -> bool {
        !self.value_replacements.is_empty()
    }
}

/// Comprehensive configuration for JSON processing operations
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    /// Separator for nested keys (default: ".")
    pub separator: String,
    /// Convert all keys to lowercase
    pub lowercase_keys: bool,
    /// Filtering configuration
    pub filtering: FilteringConfig,
    /// Collision handling configuration
    pub collision: CollisionConfig,
    /// Replacement configuration
    pub replacements: ReplacementConfig,
    /// Automatically convert string values to numbers and booleans
    pub auto_convert_types: bool,
    /// Minimum batch size for parallel processing
    pub parallel_threshold: usize,
    /// Number of threads for parallel processing (None = use Rayon default)
    pub num_threads: Option<usize>,
    /// Minimum object/array size for nested parallel processing within a single JSON document
    /// Only objects/arrays with more than this many keys/items will be processed in parallel
    /// Default: 100 (can be overridden with JSON_TOOLS_NESTED_PARALLEL_THRESHOLD environment variable)
    pub nested_parallel_threshold: usize,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            separator: ".".to_string(),
            lowercase_keys: false,
            filtering: FilteringConfig::default(),
            collision: CollisionConfig::default(),
            replacements: ReplacementConfig::default(),
            auto_convert_types: false,
            parallel_threshold: 10,
            num_threads: None, // Use Rayon default (number of logical CPUs)
            nested_parallel_threshold: std::env::var("JSON_TOOLS_NESTED_PARALLEL_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
        }
    }
}

impl ProcessingConfig {
    /// Create a new ProcessingConfig with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the separator for nested keys
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Enable lowercase key conversion
    pub fn lowercase_keys(mut self, enabled: bool) -> Self {
        self.lowercase_keys = enabled;
        self
    }

    /// Configure filtering options
    pub fn filtering(mut self, filtering: FilteringConfig) -> Self {
        self.filtering = filtering;
        self
    }

    /// Configure collision handling options
    pub fn collision(mut self, collision: CollisionConfig) -> Self {
        self.collision = collision;
        self
    }

    /// Configure replacement options
    pub fn replacements(mut self, replacements: ReplacementConfig) -> Self {
        self.replacements = replacements;
        self
    }

    /// Create a ProcessingConfig from JSONTools instance
    pub fn from_json_tools(tools: &JSONTools) -> Self {
        Self {
            separator: tools.separator.clone(),
            lowercase_keys: tools.lower_case_keys,
            filtering: FilteringConfig {
                remove_empty_strings: tools.remove_empty_string_values,
                remove_nulls: tools.remove_null_values,
                remove_empty_objects: tools.remove_empty_dict,
                remove_empty_arrays: tools.remove_empty_list,
            },
            collision: CollisionConfig {
                handle_collisions: tools.handle_key_collision,
            },
            replacements: ReplacementConfig {
                key_replacements: tools.key_replacements.clone(),
                value_replacements: tools.value_replacements.clone(),
            },
            auto_convert_types: tools.auto_convert_types,
            parallel_threshold: tools.parallel_threshold,
            num_threads: tools.num_threads,
            nested_parallel_threshold: tools.nested_parallel_threshold,
        }
    }
}

// ================================================================================================
// MODULE: Public API and Main JSONTools Interface
// ================================================================================================

/// Unified JSON Tools API with builder pattern for both flattening and unflattening operations
///
/// This is the unified interface for all JSON manipulation operations.
/// It provides a single entry point for all JSON manipulation operations with a consistent builder pattern.
///
/// Fields are ordered by size for better memory alignment and cache locality:
/// - Large fields (24 bytes each): Vec, String
/// - Medium fields (2 bytes): Option<OperationMode>
/// - Small fields (1 byte each): bool flags
#[derive(Debug, Clone)]
pub struct JSONTools {
    // Large fields first (24 bytes each on 64-bit systems)
    /// Key replacement patterns (find, replace)
    key_replacements: Vec<(String, String)>,
    /// Value replacement patterns (find, replace)
    value_replacements: Vec<(String, String)>,
    /// Separator for nested keys (default: ".")
    separator: String,

    // Medium fields (8 bytes on 64-bit systems)
    /// Minimum batch size to use parallel processing (default: 10)
    parallel_threshold: usize,
    /// Number of threads for parallel processing (None = use Rayon default)
    num_threads: Option<usize>,
    /// Minimum object/array size for nested parallel processing within a single JSON document
    nested_parallel_threshold: usize,

    // Medium fields (2 bytes)
    /// Current operation mode (flatten or unflatten)
    mode: Option<OperationMode>,

    // Small fields (1 byte each) - grouped together to minimize padding
    /// Remove keys with empty string values
    remove_empty_string_values: bool,
    /// Remove keys with null values
    remove_null_values: bool,
    /// Remove keys with empty object values
    remove_empty_dict: bool,
    /// Remove keys with empty array values
    remove_empty_list: bool,
    /// Convert all keys to lowercase
    lower_case_keys: bool,
    /// Handle key collisions by collecting values into arrays
    handle_key_collision: bool,
    /// Automatically convert string values to numbers and booleans
    auto_convert_types: bool,
}

impl Default for JSONTools {
    fn default() -> Self {
        Self {
            // Large fields
            key_replacements: Vec::with_capacity(4),
            value_replacements: Vec::with_capacity(4),
            separator: ".".to_string(),
            // Medium fields
            parallel_threshold: std::env::var("JSON_TOOLS_PARALLEL_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            num_threads: std::env::var("JSON_TOOLS_NUM_THREADS")
                .ok()
                .and_then(|v| v.parse().ok()),
            nested_parallel_threshold: std::env::var("JSON_TOOLS_NESTED_PARALLEL_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            mode: None,
            // Small fields
            remove_empty_string_values: false,
            remove_null_values: false,
            remove_empty_dict: false,
            remove_empty_list: false,
            lower_case_keys: false,
            handle_key_collision: false,
            auto_convert_types: false,
        }
    }
}


impl JSONTools {
    /// Create a new JSONTools instance with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the operation mode to flatten
    pub fn flatten(mut self) -> Self {
        self.mode = Some(OperationMode::Flatten);
        self
    }

    /// Set the operation mode to unflatten
    pub fn unflatten(mut self) -> Self {
        self.mode = Some(OperationMode::Unflatten);
        self
    }

    /// Set the operation mode to normal (no flatten/unflatten)
    pub fn normal(mut self) -> Self {
        self.mode = Some(OperationMode::Normal);
        self
    }

    /// Set the separator used for nested keys (default: ".")
    pub fn separator<S: Into<Cow<'static, str>>>(mut self, separator: S) -> Self {
        let sep_cow = separator.into();
        // Only allocate if we need to own the string
        self.separator = match sep_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        self
    }

    /// Convert all keys to lowercase
    pub fn lowercase_keys(mut self, value: bool) -> Self {
        self.lower_case_keys = value;
        self
    }

    /// Add a key replacement pattern
    ///
    /// Patterns are treated as regex patterns using standard Rust regex syntax.
    /// If a pattern fails to compile as regex, it falls back to literal string replacement.
    /// Works for both flatten and unflatten operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// // Regex pattern (standard Rust regex syntax)
    /// let json = r#"{"user_name": "John", "admin_name": "Jane"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .key_replacement("(user|admin)_", "person_")
    ///     .execute(json).unwrap();
    ///
    /// // Literal pattern (if regex compilation fails)
    /// let result2 = JSONTools::new()
    ///     .flatten()
    ///     .key_replacement("user_", "person_")
    ///     .execute(json).unwrap();
    /// ```
    pub fn key_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        let find_cow = find.into();
        let replace_cow = replace.into();

        // Only allocate when necessary
        let find_string = match find_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        let replace_string = match replace_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };

        self.key_replacements.push((find_string, replace_string));
        self
    }

    /// Add a value replacement pattern
    ///
    /// Patterns are treated as regex patterns using standard Rust regex syntax.
    /// If a pattern fails to compile as regex, it falls back to literal string replacement.
    /// Works for both flatten and unflatten operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// // Regex pattern (standard Rust regex syntax)
    /// let json = r#"{"role": "super", "level": "admin"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .value_replacement("^(super|admin)$", "administrator")
    ///     .execute(json).unwrap();
    ///
    /// // Literal pattern (if regex compilation fails)
    /// let result2 = JSONTools::new()
    ///     .flatten()
    ///     .value_replacement("@example.com", "@company.org")
    ///     .execute(json).unwrap();
    /// ```
    pub fn value_replacement<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        mut self,
        find: S1,
        replace: S2,
    ) -> Self {
        let find_cow = find.into();
        let replace_cow = replace.into();

        // Only allocate when necessary
        let find_string = match find_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        let replace_string = match replace_cow {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };

        self.value_replacements.push((find_string, replace_string));
        self
    }

    /// Remove keys with empty string values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty string values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty string values
    pub fn remove_empty_strings(mut self, value: bool) -> Self {
        self.remove_empty_string_values = value;
        self
    }

    /// Remove keys with null values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have null values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have null values
    pub fn remove_nulls(mut self, value: bool) -> Self {
        self.remove_null_values = value;
        self
    }

    /// Remove keys with empty object values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty object values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty object values
    pub fn remove_empty_objects(mut self, value: bool) -> Self {
        self.remove_empty_dict = value;
        self
    }

    /// Remove keys with empty array values
    ///
    /// Works for both flatten and unflatten operations:
    /// - In flatten mode: removes flattened keys that have empty array values
    /// - In unflatten mode: removes keys from the unflattened JSON structure that have empty array values
    pub fn remove_empty_arrays(mut self, value: bool) -> Self {
        self.remove_empty_list = value;
        self
    }

    /// Handle key collisions by collecting values into arrays
    ///
    /// When enabled, collect all values that would have the same key into an array.
    /// Works for all operations (flatten, unflatten, normal).
    pub fn handle_key_collision(mut self, value: bool) -> Self {
        self.handle_key_collision = value;
        self
    }

    /// Enable automatic type conversion from strings to numbers and booleans
    ///
    /// When enabled, the library will attempt to convert string values to numbers or booleans:
    /// - **Numbers**: "123" → 123, "1,234.56" → 1234.56, "$99.99" → 99.99, "1e5" → 100000
    /// - **Booleans**: "true"/"TRUE"/"True" → true, "false"/"FALSE"/"False" → false
    ///
    /// If conversion fails, the original string value is kept. No errors are thrown.
    ///
    /// Works for all operations (flatten, unflatten, normal).
    ///
    /// # Example
    /// ```
    /// use json_tools_rs::{JSONTools, JsonOutput};
    ///
    /// let json = r#"{"id": "123", "price": "1,234.56", "active": "true"}"#;
    /// let result = JSONTools::new()
    ///     .flatten()
    ///     .auto_convert_types(true)
    ///     .execute(json)
    ///     .unwrap();
    ///
    /// match result {
    ///     JsonOutput::Single(output) => {
    ///         // Result: {"id": 123, "price": 1234.56, "active": true}
    ///         assert!(output.contains(r#""id":123"#));
    ///         assert!(output.contains(r#""price":1234.56"#));
    ///         assert!(output.contains(r#""active":true"#));
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn auto_convert_types(mut self, enable: bool) -> Self {
        self.auto_convert_types = enable;
        self
    }

    /// Set the minimum batch size for parallel processing (only available with 'parallel' feature)
    ///
    /// When processing multiple JSON documents, this threshold determines when to use
    /// parallel processing. Batches smaller than this threshold will be processed sequentially
    /// to avoid the overhead of thread spawning.
    ///
    /// Default: 10 items (can be overridden with JSON_TOOLS_PARALLEL_THRESHOLD environment variable)
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum number of items in a batch to trigger parallel processing
    ///
    /// # Example
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .parallel_threshold(50); // Only use parallelism for batches of 50+ items
    /// ```
    pub fn parallel_threshold(mut self, threshold: usize) -> Self {
        self.parallel_threshold = threshold;
        self
    }

    /// Configure the number of threads for parallel processing
    ///
    /// By default, Rayon uses the number of logical CPUs. This method allows you to override
    /// that behavior for specific workloads or resource constraints.
    ///
    /// # Arguments
    ///
    /// * `num_threads` - Number of threads to use (None = use Rayon default)
    ///
    /// # Examples
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .num_threads(Some(4)); // Use exactly 4 threads
    /// ```
    pub fn num_threads(mut self, num_threads: Option<usize>) -> Self {
        self.num_threads = num_threads;
        self
    }

    /// Configure the threshold for nested parallel processing within individual JSON documents
    ///
    /// When flattening or unflattening a single large JSON document, this threshold determines
    /// when to parallelize the processing of objects and arrays. Only objects/arrays with more
    /// than this many keys/items will be processed in parallel.
    ///
    /// Default: 100 (can be overridden with JSON_TOOLS_NESTED_PARALLEL_THRESHOLD environment variable)
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum number of keys/items to trigger nested parallelism
    ///
    /// # Examples
    ///
    /// ```
    /// use json_tools_rs::JSONTools;
    ///
    /// let tools = JSONTools::new()
    ///     .flatten()
    ///     .nested_parallel_threshold(200); // Only parallelize objects/arrays with 200+ items
    /// ```
    pub fn nested_parallel_threshold(mut self, threshold: usize) -> Self {
        self.nested_parallel_threshold = threshold;
        self
    }

    /// Execute the configured operation on the provided JSON input
    ///
    /// This method performs the selected operation based on the mode set by calling
    /// `.flatten()`, `.unflatten()`, or `.normal()`. If no mode was set, an error is returned.
    ///
    /// # Arguments
    /// * `json_input` - JSON input that can be a single string, multiple strings, or other supported types
    ///
    /// # Returns
    /// * `Result<JsonOutput, Box<dyn Error>>` - The processed JSON result or an error
    ///
    /// # Errors
    /// * Returns an error if no operation mode has been set
    /// * Returns an error if the JSON input is invalid
    /// * Returns an error if processing fails for any other reason
    pub fn execute<'a, T>(&self, json_input: T) -> Result<JsonOutput, JsonToolsError>
    where
        T: Into<JsonInput<'a>>,
    {
        let mode = self.mode.as_ref().ok_or_else(|| {
            JsonToolsError::configuration_error(
                "Operation mode not set. Call .flatten(), .unflatten(), or .normal() before .execute()"
            )
        })?;

        let input = json_input.into();
        match mode {
            OperationMode::Flatten => self.execute_flatten(input),
            OperationMode::Unflatten => self.execute_unflatten(input),
            OperationMode::Normal => self.execute_normal(input),
        }
    }

    /// Generic batch processing helper that eliminates code duplication
    /// Processes single or multiple JSON inputs using the provided processor function
    #[inline]
    fn execute_with_processor<'a, F>(
        input: JsonInput<'a>,
        config: &ProcessingConfig,
        processor: F,
    ) -> Result<JsonOutput, JsonToolsError>
    where
        F: Fn(&Cow<str>, &ProcessingConfig) -> Result<String, JsonToolsError> + Sync,
    {
        match input {
            JsonInput::Single(json_cow) => {
                let result = processor(&json_cow, config)?;
                Ok(JsonOutput::Single(result))
            }
            JsonInput::Multiple(json_list) => {
                // Use parallel processing if batch size meets threshold
                if json_list.len() >= config.parallel_threshold {
                    use rayon::prelude::*;

                    // For very large batches (>1000), use chunked processing for better cache locality
                    if json_list.len() > 1000 {
                            // Calculate optimal chunk size: aim for ~100-200 items per chunk
                            // This balances parallelism with cache locality
                            let num_cpus = rayon::current_num_threads();
                            let chunk_size = (json_list.len() / num_cpus).max(100).min(200);

                            // Process chunks in parallel, then flatten results
                            let chunk_results: Result<Vec<Vec<String>>, _> = json_list
                                .par_chunks(chunk_size)
                                .enumerate()
                                .map(|(chunk_idx, chunk)| {
                                    let base_index = chunk_idx * chunk_size;
                                    chunk.iter().enumerate()
                                        .map(|(item_idx, json)| {
                                            let json_cow = Cow::Borrowed(*json);
                                            processor(&json_cow, config)
                                                .map_err(|e| JsonToolsError::batch_processing_error(base_index + item_idx, e))
                                        })
                                        .collect()
                                })
                                .collect();

                            // Flatten the results
                            let results: Vec<String> = chunk_results?.into_iter().flatten().collect();
                            return Ok(JsonOutput::Multiple(results));
                        } else {
                            // For smaller batches, use simple par_iter for maximum parallelism
                            let results: Result<Vec<_>, _> = json_list
                                .par_iter()
                                .enumerate()
                                .map(|(index, json)| {
                                    let json_cow = Cow::Borrowed(*json);
                                    processor(&json_cow, config)
                                        .map_err(|e| JsonToolsError::batch_processing_error(index, e))
                                })
                                .collect();

                            return Ok(JsonOutput::Multiple(results?));
                        }
                    }

                // Sequential processing (default or below threshold)
                let mut results = Vec::with_capacity(json_list.len());
                for (index, json) in json_list.iter().enumerate() {
                    let json_cow = Cow::Borrowed(*json);
                    match processor(&json_cow, config) {
                        Ok(result) => results.push(result),
                        Err(e) => return Err(JsonToolsError::batch_processing_error(index, e)),
                    }
                }
                Ok(JsonOutput::Multiple(results))
            }
            JsonInput::MultipleOwned(vecs) => {
                // Use parallel processing if batch size meets threshold
                if vecs.len() >= config.parallel_threshold {
                    use rayon::prelude::*;

                    // For very large batches (>1000), use chunked processing for better cache locality
                    if vecs.len() > 1000 {
                            // Calculate optimal chunk size: aim for ~100-200 items per chunk
                            let num_cpus = rayon::current_num_threads();
                            let chunk_size = (vecs.len() / num_cpus).max(100).min(200);

                            // Process chunks in parallel, then flatten results
                            let chunk_results: Result<Vec<Vec<String>>, _> = vecs
                                .par_chunks(chunk_size)
                                .enumerate()
                                .map(|(chunk_idx, chunk)| {
                                    let base_index = chunk_idx * chunk_size;
                                    chunk.iter().enumerate()
                                        .map(|(item_idx, json_cow)| {
                                            processor(json_cow, config)
                                                .map_err(|e| JsonToolsError::batch_processing_error(base_index + item_idx, e))
                                        })
                                        .collect()
                                })
                                .collect();

                            // Flatten the results
                            let results: Vec<String> = chunk_results?.into_iter().flatten().collect();
                            return Ok(JsonOutput::Multiple(results));
                        } else {
                            // For smaller batches, use simple par_iter for maximum parallelism
                            let results: Result<Vec<_>, _> = vecs
                                .par_iter()
                                .enumerate()
                                .map(|(index, json_cow)| {
                                    processor(json_cow, config)
                                        .map_err(|e| JsonToolsError::batch_processing_error(index, e))
                                })
                                .collect();

                            return Ok(JsonOutput::Multiple(results?));
                        }
                    }

                // Sequential processing (default or below threshold)
                let mut results = Vec::with_capacity(vecs.len());
                for (index, json_cow) in vecs.iter().enumerate() {
                    match processor(json_cow, config) {
                        Ok(result) => results.push(result),
                        Err(e) => return Err(JsonToolsError::batch_processing_error(index, e)),
                    }
                }
                Ok(JsonOutput::Multiple(results))
            }
        }
    }

    fn execute_flatten<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json)
    }

    fn execute_unflatten<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json_for_unflatten)
    }

    fn execute_normal<'a>(&self, input: JsonInput<'a>) -> Result<JsonOutput, JsonToolsError> {
        let config = ProcessingConfig::from_json_tools(self);
        Self::execute_with_processor(input, &config, process_single_json_normal)
    }

}



/// Handle root-level primitive values and empty containers for unflattening
#[inline]
fn handle_root_level_primitives_unflatten(
    value: &Value,
    value_replacements: &[(String, String)],
) -> Result<Option<String>, JsonToolsError> {
    match value {
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // For root-level primitives, apply value replacements if any, then return
            let mut single_value = value.clone();
            if !value_replacements.is_empty() {
                apply_value_replacement_patterns(&mut single_value, value_replacements)?;
            }


            Ok(Some(simd_json::serde::to_string(&single_value)?))
        }
        Value::Object(obj) if obj.is_empty() => {
            // Empty object should remain empty object
            Ok(Some("{}".to_string()))
        }
        Value::Array(_) => {
            // Arrays at root level are not valid flattened JSON - convert to empty object
            Ok(Some("{}".to_string()))
        }
        _ => {
            // Continue with normal unflattening for objects with content
            Ok(None)
        }
    }
}

/// Extract flattened object from parsed JSON value
#[inline]
fn extract_flattened_object(flattened: Value) -> Result<Map<String, Value>, JsonToolsError> {
    match flattened {
        Value::Object(obj) => Ok(obj),
        _ => Err(JsonToolsError::invalid_json_structure(
            "Expected object for unflattening"
        ))
    }
}

/// Apply all transformations (key replacements, value replacements, lowercase) for unflattening
/// Optimized to avoid unnecessary clone by consuming the input
#[inline]
fn apply_transformations_unflatten(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Result<Map<String, Value>, JsonToolsError> {
    // Consume the input instead of cloning
    let mut processed_obj = flattened_obj;

    // Apply key replacements with collision detection if provided
    if config.replacements.has_key_replacements() {
        // Use optimized version when collision handling is disabled for better performance
        if !config.collision.handle_collisions {
            // Pass ownership to avoid cloning all values
            processed_obj = apply_key_replacements_for_unflatten(processed_obj, &config.replacements.key_replacements)?;
        } else {
            processed_obj = apply_key_replacements_unflatten_with_collisions(
                processed_obj,
                config,
            )?;
        }
    }

    // Apply value replacements
    if config.replacements.has_value_replacements() {
        apply_value_replacements_for_unflatten(&mut processed_obj, &config.replacements.value_replacements)?;
    }

    // Apply lowercase conversion if specified
    if config.lowercase_keys {
        processed_obj = apply_lowercase_keys_for_unflatten(processed_obj);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if config.collision.handle_collisions && !config.replacements.has_key_replacements() {
            processed_obj = handle_key_collisions_for_unflatten(
                processed_obj,
                config,
            );
        }
    } else if config.collision.handle_collisions && !config.replacements.has_key_replacements() {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        processed_obj = handle_key_collisions_for_unflatten(
            processed_obj,
            config,
        );
    }

    Ok(processed_obj)
}

/// Perform unflattening and apply filtering to the result
#[inline]
fn perform_unflattening_and_filtering(
    processed_obj: Map<String, Value>,
    separator: &str,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> Result<Value, JsonToolsError> {
    // Perform the actual unflattening (takes ownership to avoid cloning values)
    let mut unflattened = unflatten_object(processed_obj, separator)?;

    // Apply filtering to the unflattened result
    if remove_empty_string_values || remove_null_values || remove_empty_dict || remove_empty_list {
        filter_nested_value(
            &mut unflattened,
            remove_empty_string_values,
            remove_null_values,
            remove_empty_dict,
            remove_empty_list,
        );
    }

    Ok(unflattened)
}

// ================================================================================================
// MODULE: Core Processing Functions - Unflattening Operations
// ================================================================================================

/// Core unflattening logic for a single JSON string with Cow optimization
#[inline]
fn process_single_json_for_unflatten(
    json: &Cow<str>,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // Parse JSON using optimized SIMD parsing
    let mut flattened = parse_json_optimized(json)?;

    // Apply type conversion FIRST (before other transformations)
    if config.auto_convert_types {
        apply_type_conversion_recursive(&mut flattened);
    }

    // Handle root-level primitives and empty containers
    if let Some(result) = handle_root_level_primitives_unflatten(&flattened, &config.replacements.value_replacements)? {
        return Ok(result);
    }

    // Extract the flattened object
    let flattened_obj = extract_flattened_object(flattened)?;

    // Apply key and value transformations
    let processed_obj = apply_transformations_unflatten(
        flattened_obj,
        config,
    )?;

    // Perform the actual unflattening and apply filtering
    // Pass ownership to avoid cloning values during unflatten
    let unflattened = perform_unflattening_and_filtering(
        processed_obj,
        &config.separator,
        config.filtering.remove_empty_strings,
        config.filtering.remove_nulls,
        config.filtering.remove_empty_objects,
        config.filtering.remove_empty_arrays,
    )?;

    // Serialize the result
    Ok(simd_json::serde::to_string(&unflattened)?)
}

/// Handle root-level primitive values and empty containers for flattening
#[inline]
fn handle_root_level_primitives_flatten(
    value: &Value,
    value_replacements: Option<&[(String, String)]>,
) -> Result<Option<String>, JsonToolsError> {
    match value {
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {
            // For root-level primitives, apply value replacements if any, then return
            let mut single_value = value.clone();
            if let Some(patterns) = value_replacements {
                apply_value_replacement_patterns(&mut single_value, patterns)?;
            }
            Ok(Some(simd_json::serde::to_string(&single_value)?))
        }
        Value::Object(obj) if obj.is_empty() => {
            // Empty object should remain empty object
            Ok(Some("{}".to_string()))
        }
        Value::Array(arr) if arr.is_empty() => {
            // Empty array at root level should become empty object
            Ok(Some("{}".to_string()))
        }
        _ => {
            // Continue with normal flattening for objects and arrays with content
            Ok(None)
        }
    }
}

/// Parse JSON string using optimized SIMD parsing
/// Optimized JSON parsing using thread-local buffer reuse with SIMD-JSON padding
///
/// From simdjson docs: "create a parser once and reuse it... keeping buffers hot in the cache"
/// This provides 20-40% performance improvement over creating new buffers each time
#[inline]
fn parse_json_optimized(json: &Cow<str>) -> Result<Value, JsonToolsError> {
    JSON_PARSER_STATE.with(|state| state.borrow_mut().parse(json))
}

/// Initialize flattened HashMap with optimized capacity
#[inline]
fn initialize_flattened_map(value: &Value) -> FxHashMap<String, Value> {
    let estimated_size = estimate_flattened_size(value);
    let optimal_capacity = calculate_optimal_capacity(estimated_size);

    // Use FxHashMap for better performance with string keys
    FxHashMap::with_capacity_and_hasher(optimal_capacity, Default::default())
}

/// Perform the core flattening operation
#[inline]
fn perform_flattening(value: &Value, separator: &str, nested_threshold: usize) -> FxHashMap<String, Value> {
    let mut flattened = initialize_flattened_map(value);

    // Ultra-aggressive string builder capacity for SIMD performance
    let max_key_length = estimate_max_key_length(value);
    // Use massive extra capacity to ensure zero reallocations for SIMD efficiency
    let builder_capacity = std::cmp::max(max_key_length * 4, 512);
    let mut builder = FastStringBuilder::with_capacity_and_separator(builder_capacity, separator);
    flatten_value_with_threshold(value, &mut builder, &mut flattened, nested_threshold);

    flattened
}


/// Apply key transformations including replacements and lowercase conversion for flattening
#[inline]
fn apply_key_transformations_flatten(
    mut flattened: FxHashMap<String, Value>,
    config: &ProcessingConfig,
) -> Result<FxHashMap<String, Value>, JsonToolsError> {
    // Apply key replacements with collision detection if provided
    if config.replacements.has_key_replacements() {
        // Convert tuple format to the internal vector format
        let key_patterns = convert_tuples_to_patterns(&config.replacements.key_replacements);

        // Use the consolidated function that handles both optimized and collision scenarios
        flattened = apply_key_replacements_with_collision_handling(
            flattened,
            &key_patterns,
            config,
        )?;
    }

    // Apply lowercase conversion to keys if requested
    if config.lowercase_keys {
        flattened = apply_lowercase_keys(flattened);

        // If collision handling is enabled but no key replacements were applied,
        // we need to check for collisions after lowercase conversion
        if config.collision.has_collision_handling() && !config.replacements.has_key_replacements() {
            flattened = handle_key_collisions(
                flattened,
                config.collision.handle_collisions,
            );
        }
    } else if config.collision.has_collision_handling() && !config.replacements.has_key_replacements() {
        // If collision handling is enabled but no transformations were applied,
        // we still need to check for collisions (though this would be rare)
        flattened = handle_key_collisions(
            flattened,
            config.collision.handle_collisions,
        );
    }

    Ok(flattened)
}

/// Apply value replacements to flattened data
#[inline]
fn apply_value_replacements_flatten(
    flattened: &mut FxHashMap<String, Value>,
    config: &ProcessingConfig,
) -> Result<(), JsonToolsError> {
    if config.replacements.has_value_replacements() {
        // Convert tuple format to the internal vector format
        let value_patterns = convert_tuples_to_patterns(&config.replacements.value_replacements);
        apply_value_replacements(flattened, &value_patterns)?;
    }
    Ok(())
}

/// Apply filtering to flattened data after replacements
#[inline]
fn apply_filtering_flatten(
    flattened: &mut FxHashMap<String, Value>,
    config: &ProcessingConfig,
) {
    if !config.filtering.has_any_filter() {
        return;
    }

    // First pass: filter inside arrays that were created by collision handling
    if config.collision.handle_collisions {
        for (_, v) in flattened.iter_mut() {
            if let Some(arr) = v.as_array_mut() {
                // Filter elements inside collision-created arrays
                arr.retain(|element| {
                    should_include_value(
                        element,
                        config.filtering.remove_empty_strings,
                        config.filtering.remove_nulls,
                        config.filtering.remove_empty_objects,
                        config.filtering.remove_empty_arrays,
                    )
                });
            }
        }
    }

    // Second pass: filter top-level key-value pairs
    flattened.retain(|_, v| {
        should_include_value(
            v,
            config.filtering.remove_empty_strings,
            config.filtering.remove_nulls,
            config.filtering.remove_empty_objects,
            config.filtering.remove_empty_arrays,
        )
    });
}

/// Core key replacement logic that works with both string keys and Cow<str>
/// This eliminates duplication between flatten and unflatten key replacement functions
/// Optimized to minimize string allocations by using efficient Cow operations
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
#[inline]
fn apply_key_replacement_patterns(
    key: &str,
    patterns: &[(String, String)],
) -> Result<Option<String>, JsonToolsError> {
    let mut new_key = Cow::Borrowed(key);
    let mut changed = false;

    // Apply each replacement pattern
    for (pattern, replacement) in patterns {
        // Try to compile as regex first
        match get_cached_regex(pattern) {
            Ok(regex) => {
                // Successfully compiled as regex - use regex replacement
                if regex.is_match(&new_key) {
                    new_key = Cow::Owned(regex.replace_all(&new_key, replacement).into_owned());
                    changed = true;
                }
            }
            Err(_) => {
                // Failed to compile as regex - fall back to literal replacement
                if new_key.contains(pattern) {
                    new_key = Cow::Owned(new_key.replace(pattern, replacement));
                    changed = true;
                }
            }
        }
    }

    if changed {
        Ok(Some(new_key.into_owned()))
    } else {
        Ok(None)
    }
}

/// Core value replacement logic that works with any Value type
/// This eliminates duplication between flatten and unflatten value replacement functions
/// Optimized to minimize string allocations by using efficient Cow operations
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
#[inline]
fn apply_value_replacement_patterns(
    value: &mut Value,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    if let Value::String(s) = value {
        let mut current_value = Cow::Borrowed(s.as_str());
        let mut changed = false;

        // Apply each replacement pattern
        for (pattern, replacement) in patterns {
            // Try to compile as regex first
            match get_cached_regex(pattern) {
                Ok(regex) => {
                    // Successfully compiled as regex - use regex replacement
                    if regex.is_match(&current_value) {
                        current_value = Cow::Owned(regex.replace_all(&current_value, replacement).into_owned());
                        changed = true;
                    }
                }
                Err(_) => {
                    // Failed to compile as regex - fall back to literal replacement
                    if current_value.contains(pattern) {
                        current_value = Cow::Owned(current_value.replace(pattern, replacement));
                        changed = true;
                    }
                }
            }
        }

        if changed {
            *s = current_value.into_owned();
        }
    }
    Ok(())
}

/// Core collision detection and grouping logic
/// This eliminates duplication between flatten and unflatten collision handling
#[inline]
fn group_items_by_key<K, V>(items: impl Iterator<Item = (K, V)>) -> FxHashMap<K, Vec<V>>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    // Pre-allocate with estimated capacity for better performance
    let mut key_groups: FxHashMap<K, Vec<V>> = FxHashMap::with_capacity_and_hasher(64, Default::default());
    for (key, value) in items {
        key_groups.entry(key).or_default().push(value);
    }
    key_groups
}


/// Apply collision handling strategy by collecting values into arrays
#[inline]
fn apply_collision_handling(
    key: String,
    values: Vec<Value>,
    filter_config: Option<&FilteringConfig>,
) -> Option<(String, Value)> {
    let filtered_values: Vec<Value> = if let Some(config) = filter_config {
        values.into_iter().filter(|value| {
            should_include_value(
                value,
                config.remove_empty_strings,
                config.remove_nulls,
                config.remove_empty_objects,
                config.remove_empty_arrays,
            )
        }).collect()
    } else {
        values
    };

    if !filtered_values.is_empty() {
        Some((key, Value::Array(filtered_values)))
    } else {
        None
    }
}

// ================================================================================================
// MODULE: Core Processing Functions - Flattening Operations
// ================================================================================================

/// Core flattening logic for a single JSON string with Cow optimization
#[inline]
fn process_single_json(
    json: &Cow<str>,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // Parse JSON using optimized SIMD parsing
    let mut value = parse_json_optimized(json)?;

    // Apply type conversion FIRST (before other transformations)
    if config.auto_convert_types {
        apply_type_conversion_recursive(&mut value);
    }

    // Handle root-level primitives and empty containers
    if let Some(result) = handle_root_level_primitives_flatten(&value, Some(&config.replacements.value_replacements))? {
        return Ok(result);
    }

    // Perform the core flattening operation
    let mut flattened = perform_flattening(&value, &config.separator, config.nested_parallel_threshold);

    // Apply key transformations (replacements and lowercase conversion)
    flattened = apply_key_transformations_flatten(flattened, &config)?;

    // Apply value replacements if provided
    apply_value_replacements_flatten(&mut flattened, &config)?;

    // Apply filtering AFTER replacements to catch newly created empty values
    apply_filtering_flatten(&mut flattened, &config);

    // Convert back to JSON string using simd-json serialization
    serialize_flattened(&flattened).map_err(JsonToolsError::serialization_error)
}

/// Convert tuple-based replacement patterns to the internal vector format
/// This converts the intuitive tuple format to the internal representation used by replacement functions
/// Optimized to use string references instead of cloning to reduce memory allocations
#[inline]
fn convert_tuples_to_patterns(tuples: &[(String, String)]) -> Vec<&str> {
    let mut patterns = Vec::with_capacity(tuples.len() * 2);
    for (pattern, replacement) in tuples {
        patterns.push(pattern.as_str());
        patterns.push(replacement.as_str());
    }
    patterns
}

/// Apply lowercase conversion to all keys in the flattened HashMap
/// This function creates a new HashMap with all keys converted to lowercase
/// Optimized with Cow to avoid unnecessary allocations when keys are already lowercase
/// Uses Entry API for potential collision handling
#[inline]
fn apply_lowercase_keys(flattened: FxHashMap<String, Value>) -> FxHashMap<String, Value> {
    let optimal_capacity = calculate_optimal_capacity(flattened.len());
    let mut result = FxHashMap::with_capacity_and_hasher(optimal_capacity, Default::default());

    for (key, value) in flattened {
        // SIMD-optimized lowercase conversion (zero-copy if already lowercase)
        let lowercase_key = to_lowercase_simd(&key);

        let final_key = match lowercase_key {
            Cow::Borrowed(_) => key, // Key was already lowercase, reuse original
            Cow::Owned(lower) => lower, // Key was converted to lowercase
        };

        // Use Entry API to handle potential collisions more efficiently
        match result.entry(final_key) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                // Handle collision by converting to array
                let existing_value = entry.get_mut();
                match existing_value {
                    Value::Array(arr) => {
                        arr.push(value);
                    }
                    _ => {
                        let old_value = std::mem::replace(existing_value, Value::Null);
                        *existing_value = Value::Array(vec![old_value, value]);
                    }
                }
            }
        }
    }
    result
}

/// Estimates the flattened size to pre-allocate HashMap capacity
/// Improved algorithm that considers nesting depth and provides more accurate estimates
fn estimate_flattened_size(value: &Value) -> usize {
    estimate_flattened_size_with_depth(value, 0)
}

/// Internal function that tracks depth for more accurate capacity estimation
fn estimate_flattened_size_with_depth(value: &Value, depth: usize) -> usize {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                1
            } else {
                // For deeply nested objects, the flattening factor increases
                let depth_multiplier = if depth > 3 { 1.2 } else { 1.0 };
                let base_size: usize = obj.iter()
                    .map(|(_, v)| estimate_flattened_size_with_depth(v, depth + 1))
                    .sum();
                (base_size as f64 * depth_multiplier).ceil() as usize
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                1
            } else {
                // Arrays contribute more to flattened size due to index keys
                let depth_multiplier = if depth > 2 { 1.3 } else { 1.1 };
                let base_size: usize = arr.iter()
                    .map(|v| estimate_flattened_size_with_depth(v, depth + 1))
                    .sum();
                (base_size as f64 * depth_multiplier).ceil() as usize
            }
        }
        _ => 1,
    }
}

/// Estimates optimal HashMap capacity based on expected size and load factor
fn calculate_optimal_capacity(estimated_size: usize) -> usize {
    if estimated_size == 0 {
        return 16; // Minimum reasonable capacity
    }

    // Use a load factor of 0.75 to minimize rehashing while not wasting too much memory
    let target_capacity = (estimated_size as f64 / 0.75).ceil() as usize;

    // Round up to next power of 2 for optimal HashMap performance
    let next_power_of_2 = target_capacity.next_power_of_two();

    // Cap the maximum initial capacity to prevent excessive memory usage
    let max_capacity = 8192;
    std::cmp::min(next_power_of_2, max_capacity)
}

/// Estimates the maximum key length for string pre-allocation
fn estimate_max_key_length(value: &Value) -> usize {
    fn estimate_depth_and_width(value: &Value, current_depth: usize) -> (usize, usize) {
        match value {
            Value::Object(obj) => {
                if obj.is_empty() {
                    (current_depth, 0)
                } else {
                    let max_key_len = obj.keys().map(|k| k.len()).max().unwrap_or(0);
                    let (max_child_depth, max_child_width) = obj
                        .values()
                        .map(|v| estimate_depth_and_width(v, current_depth + 1))
                        .fold((current_depth, max_key_len), |(max_d, max_w), (d, w)| {
                            (max_d.max(d), max_w.max(w))
                        });
                    (max_child_depth, max_child_width)
                }
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    (current_depth, 0)
                } else {
                    let max_index_len = (arr.len() - 1).to_string().len();
                    let (max_child_depth, max_child_width) = arr
                        .iter()
                        .map(|v| estimate_depth_and_width(v, current_depth + 1))
                        .fold((current_depth, max_index_len), |(max_d, max_w), (d, w)| {
                            (max_d.max(d), max_w.max(w))
                        });
                    (max_child_depth, max_child_width)
                }
            }
            _ => (current_depth, 0),
        }
    }

    let (max_depth, max_width) = estimate_depth_and_width(value, 0);
    // Estimate: max_depth * (max_width + 1 for dot) + some buffer
    max_depth * (max_width + 1) + 50
}



// ================================================================================================
// MODULE: Core Processing Functions - Normal (No Flatten/Unflatten) Operations
// ================================================================================================

/// Core normal-mode logic for a single JSON string with Cow optimization
/// Applies key/value transformations and filtering recursively without changing structure
#[inline]
fn process_single_json_normal(
    json: &Cow<str>,
    config: &ProcessingConfig,
) -> Result<String, JsonToolsError> {
    // Parse JSON using optimized SIMD parsing
    let mut value = parse_json_optimized(json)?;

    // Apply type conversion FIRST (before other transformations)
    if config.auto_convert_types {
        apply_type_conversion_recursive(&mut value);
    }

    // Apply value replacements recursively to all strings
    if config.replacements.has_value_replacements() {
        apply_value_replacements_recursive(&mut value, &config.replacements.value_replacements)?;
    }

    // Apply key transformations (key replacements and lowercase), with collision handling
    if config.replacements.has_key_replacements() || config.lowercase_keys || config.collision.has_collision_handling() {
        value = apply_key_transformations_normal(value, config)?;
    }

    // Apply filtering recursively after replacements and key transformations
    if config.filtering.has_any_filter() {
        filter_nested_value(
            &mut value,
            config.filtering.remove_empty_strings,
            config.filtering.remove_nulls,
            config.filtering.remove_empty_objects,
            config.filtering.remove_empty_arrays,
        );
    }

    // Serialize back to JSON
    Ok(simd_json::serde::to_string(&value)?)
}

/// Recursively apply value replacements to all string values
fn apply_value_replacements_recursive(value: &mut Value, patterns: &[(String, String)]) -> Result<(), JsonToolsError> {
    match value {
        Value::Object(map) => {
            for v in map.values_mut() {
                apply_value_replacements_recursive(v, patterns)?;
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                apply_value_replacements_recursive(v, patterns)?;
            }
        }
        _ => {
            // Apply to primitive string values
            apply_value_replacement_patterns(value, patterns)?;
        }
    }
    Ok(())
}

/// Apply key replacements and lowercase to all object keys recursively, with collision handling
fn apply_key_transformations_normal(value: Value, config: &ProcessingConfig) -> Result<Value, JsonToolsError> {
    match value {
        Value::Object(map) => {
            // Transform this level's keys first
            let mut transformed: Map<String, Value> = Map::with_capacity(map.len());
            for (key, v) in map.into_iter() {
                // Recurse into child first
                let v = apply_key_transformations_normal(v, config)?;

                // Apply key replacement patterns
                let new_key: String = if config.replacements.has_key_replacements() {
                    if let Some(repl) = apply_key_replacement_patterns(&key, &config.replacements.key_replacements)? {
                        repl
                    } else { key }
                } else { key };

                // Apply lowercase if needed
                let final_key = if config.lowercase_keys { new_key.to_lowercase() } else { new_key };

                // Insert; we'll handle collisions later
                transformed.insert(final_key, v);
            }

            // Handle key collisions if requested
            let result_map = if config.collision.has_collision_handling() {
                handle_key_collisions_for_unflatten(transformed, config)
            } else {
                transformed
            };

            Ok(Value::Object(result_map))
        }
        Value::Array(mut arr) => {
            for i in 0..arr.len() {
                let v = std::mem::take(&mut arr[i]);
                arr[i] = apply_key_transformations_normal(v, config)?;
            }
            Ok(Value::Array(arr))
        }
        other => Ok(other),
    }
}

/// Value replacement with regex caching - optimized to use string references
///
/// Patterns are treated as regex patterns. If a pattern fails to compile as regex,
/// it falls back to literal string replacement.
fn apply_value_replacements(
    flattened: &mut FxHashMap<String, Value>,
    patterns: &[&str],
) -> Result<(), JsonToolsError> {
    if patterns.len() % 2 != 0 {
        return Err(JsonToolsError::invalid_replacement_pattern(
            "Value replacement patterns must be provided in pairs (pattern, replacement)"
        ));
    }

    // Pre-compile all regex patterns (or mark as literal if compilation fails)
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        // Try to compile as regex
        match get_cached_regex(pattern) {
            Ok(regex) => compiled_patterns.push((Some(regex), replacement)),
            Err(_) => compiled_patterns.push((None, replacement)),
        }
    }

    for (_, value) in flattened.iter_mut() {
        if let Value::String(s) = value {
            let mut new_value = Cow::Borrowed(s.as_str());
            let mut changed = false;

            // Apply each compiled pattern
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_value) {
                        new_value = Cow::Owned(
                            regex
                                .replace_all(&new_value, *replacement)
                                .to_string(),
                        );
                        changed = true;
                    }
                } else if new_value.contains(pattern) {
                    new_value = Cow::Owned(new_value.replace(pattern, *replacement));
                    changed = true;
                }
            }

            if changed {
                *value = Value::String(new_value.into_owned());
            }
        }
    }

    Ok(())
}

/// Ultra-fast JSON serialization using direct writer API
///
/// From serde-json docs: "to_writer which serializes to any io::Write... avoiding string allocations"
/// This provides 15-25% improvement over intermediate Value creation
#[inline]
fn serialize_flattened(
    flattened: &FxHashMap<String, Value>,
) -> Result<String, simd_json::Error> {
    // Estimate capacity based on map contents
    let estimated_size = estimate_serialized_size(flattened);
    let mut buffer = Vec::with_capacity(estimated_size);

    // Write directly to buffer, avoiding intermediate String allocation
    // This is faster than building a String char-by-char
    simd_json::serde::to_writer(&mut buffer, &flattened)?;

    // SAFETY: simd_json guarantees valid UTF-8 output
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
}

/// Estimate the serialized JSON size for optimal buffer pre-allocation
#[inline]
fn estimate_serialized_size(map: &FxHashMap<String, Value>) -> usize {
    // Heuristic based on typical JSON structure:
    // - Average key length: ~20 chars
    // - Average value overhead: ~30 chars (quotes, colons, commas, etc.)
    // - Base object overhead: 100 chars
    map.len() * 50 + 100
}




// ================================================================================================
// MODULE: Performance Utilities and Optimized Data Structures
// ================================================================================================

/// Cached separator information for operations with Cow optimization
#[derive(Clone)]
struct SeparatorCache {
    separator: Cow<'static, str>,    // Cow for efficient memory usage
    is_single_char: bool,            // True if separator is a single character
    single_char: Option<char>,       // The character if single-char separator
    length: usize,                   // Pre-computed length
}

impl SeparatorCache {
    #[inline]
    fn new(separator: &str) -> Self {
        // Check for common static separators to avoid heap allocations
        let separator_cow = match separator {
            "." => Cow::Borrowed("."),
            "_" => Cow::Borrowed("_"),
            "::" => Cow::Borrowed("::"),
            "/" => Cow::Borrowed("/"),
            "-" => Cow::Borrowed("-"),
            "|" => Cow::Borrowed("|"),
            _ => Cow::Owned(separator.to_string()),
        };

        let is_single_char = separator.len() == 1;
        let single_char = if is_single_char {
            separator.chars().next()
        } else {
            None
        };

        Self {
            separator: separator_cow,
            is_single_char,
            single_char,
            length: separator.len(),
        }
    }

    #[inline]
    fn append_to_buffer(&self, buffer: &mut String) {
        if self.is_single_char {
            // Ultra-fast path for single characters - direct byte manipulation
            let ch = self.single_char.unwrap();
            // Compile-time optimization for the most common separators
            match ch {
                '.' => buffer.push('.'), // Most common case
                '_' => buffer.push('_'), // Second most common
                '/' => buffer.push('/'), // Third most common
                '|' => buffer.push('|'), // Fourth most common
                '-' => buffer.push('-'), // Fifth most common
                _ => buffer.push(ch),    // Fallback for other single chars
            }
        } else {
            // Use Cow for efficient string handling
            buffer.push_str(&self.separator);
        }
    }

    #[inline]
    fn reserve_capacity_for_append(&self, buffer: &mut String, additional_content_len: usize) {
        // Pre-calculate total capacity needed to avoid multiple reallocations
        let needed_capacity = buffer.len() + self.length + additional_content_len;
        if buffer.capacity() < needed_capacity {
            buffer.reserve(needed_capacity - buffer.len());
        }
    }
}

/// High-performance string builder with advanced caching and optimization
struct FastStringBuilder {
    buffer: String,
    stack: Vec<usize>, // Stack of prefix lengths for efficient truncation
    separator_cache: SeparatorCache, // Cached separator information
}

impl FastStringBuilder {
    #[inline]
    fn with_capacity_and_separator(capacity: usize, separator: &str) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            stack: Vec::with_capacity(32), // Reasonable depth for most JSON
            separator_cache: SeparatorCache::new(separator),
        }
    }

    #[inline(always)]  // Optimization #13: Force inline for hot path
    fn push_level(&mut self) {
        self.stack.push(self.buffer.len());
    }

    #[inline(always)]  // Optimization #13: Force inline for hot path
    fn pop_level(&mut self) {
        if let Some(len) = self.stack.pop() {
            self.buffer.truncate(len);
        }
    }

    #[inline(always)]  // Optimization #13: Force inline for hot path
    fn append_key(&mut self, key: &str, needs_separator: bool) {
        if needs_separator {
            // Pre-allocate capacity to avoid reallocations with extra buffer
            self.separator_cache
                .reserve_capacity_for_append(&mut self.buffer, key.len() + 8); // Extra buffer for future appends
            self.separator_cache.append_to_buffer(&mut self.buffer);
        } else {
            // Reserve capacity for the key plus some extra buffer
            let needed = key.len() + 16; // Extra buffer for future operations
            if self.buffer.capacity() < self.buffer.len() + needed {
                self.buffer.reserve(needed);
            }
        }
        self.buffer.push_str(key);
    }

    #[inline]
    fn append_index(&mut self, index: usize, needs_separator: bool) {
        // Pre-calculate index string length for capacity optimization
        let index_len = if index < 10 {
            1
        } else if index < 100 {
            2
        } else if index < 1000 {
            3
        } else if index < 10000 {
            4
        } else {
            // For very large indices, calculate the length
            (index as f64).log10().floor() as usize + 1
        };

        if needs_separator {
            self.separator_cache
                .reserve_capacity_for_append(&mut self.buffer, index_len);
            self.separator_cache.append_to_buffer(&mut self.buffer);
        } else {
            // Reserve capacity for just the index
            if self.buffer.capacity() < self.buffer.len() + index_len {
                self.buffer.reserve(index_len);
            }
        }

        // Ultra-fast path for single digits
        if index < 10 {
            self.buffer.push(char::from(b'0' + index as u8));
        } else if index < 100 {
            // Fast path for two digits
            let tens = index / 10;
            let ones = index % 10;
            self.buffer.push(char::from(b'0' + tens as u8));
            self.buffer.push(char::from(b'0' + ones as u8));
        } else {
            // Fallback for larger numbers
            use std::fmt::Write;
            write!(self.buffer, "{}", index).unwrap();
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        &self.buffer
    }



    #[inline]
    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

}

// ================================================================================================
// MODULE: Core Algorithms - Flattening Implementation
// ================================================================================================

/// Flattening with nested parallelism support
/// When objects/arrays exceed the threshold, they are processed in parallel
/// Pass usize::MAX as nested_threshold to disable nested parallelism
#[inline]
fn flatten_value_with_threshold(
    value: &Value,
    builder: &mut FastStringBuilder,
    result: &mut FxHashMap<String, Value>,
    nested_threshold: usize,
) {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                result.insert(builder.as_str().to_string(), Value::Object(Map::new()));
            } else if obj.len() > nested_threshold {
                // PARALLEL PATH: Large object - process keys in parallel
                use rayon::prelude::*;

                let prefix = builder.as_str().to_string();
                let separator = builder.separator_cache.separator.clone();
                let needs_dot = !builder.is_empty();

                // Convert to Vec for parallel iteration (serde_json::Map doesn't implement ParallelIterator)
                let entries: Vec<_> = obj.iter().collect();

                // Process each key-value pair in parallel and collect results
                let partial_results: Vec<FxHashMap<String, Value>> = entries
                    .par_iter()
                    .map(|(key, val)| {
                        // Create a new builder for this branch
                        let mut branch_builder = FastStringBuilder::with_capacity_and_separator(
                            prefix.len() + key.len() + 10,
                            &separator,
                        );

                        // Build the prefix for this branch
                        if !prefix.is_empty() {
                            branch_builder.buffer.push_str(&prefix);
                        }
                        branch_builder.push_level();
                        branch_builder.append_key(key, needs_dot);

                        // Recursively flatten this branch
                        let mut branch_result = FxHashMap::with_capacity_and_hasher(16, Default::default());
                        flatten_value_with_threshold(val, &mut branch_builder, &mut branch_result, nested_threshold);
                        branch_result
                    })
                    .collect();

                // Merge all partial results into the main result
                for partial in partial_results {
                    result.extend(partial);
                }
            } else {
                // SEQUENTIAL PATH: Small object
                let needs_dot = !builder.is_empty();
                for (key, val) in obj {
                    builder.push_level();
                    builder.append_key(key, needs_dot);
                    flatten_value_with_threshold(val, builder, result, nested_threshold);
                    builder.pop_level();
                }
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                result.insert(builder.as_str().to_string(), Value::Array(vec![]));
            } else if arr.len() > nested_threshold {
                // PARALLEL PATH: Large array - process indices in parallel
                use rayon::prelude::*;

                let prefix = builder.as_str().to_string();
                let separator = builder.separator_cache.separator.clone();
                let needs_dot = !builder.is_empty();

                // Process each array element in parallel and collect results
                let partial_results: Vec<FxHashMap<String, Value>> = arr
                    .par_iter()
                    .enumerate()
                    .map(|(index, val)| {
                        // Create a new builder for this branch
                        let mut branch_builder = FastStringBuilder::with_capacity_and_separator(
                            prefix.len() + 10,
                            &separator,
                        );

                        // Build the prefix for this branch
                        if !prefix.is_empty() {
                            branch_builder.buffer.push_str(&prefix);
                        }
                        branch_builder.push_level();
                        branch_builder.append_index(index, needs_dot);

                        // Recursively flatten this branch
                        let mut branch_result = FxHashMap::with_capacity_and_hasher(16, Default::default());
                        flatten_value_with_threshold(val, &mut branch_builder, &mut branch_result, nested_threshold);
                        branch_result
                    })
                    .collect();

                // Merge all partial results into the main result
                for partial in partial_results {
                    result.extend(partial);
                }
            } else {
                // SEQUENTIAL PATH: Small array
                let needs_dot = !builder.is_empty();
                for (index, val) in arr.iter().enumerate() {
                    builder.push_level();
                    builder.append_index(index, needs_dot);
                    flatten_value_with_threshold(val, builder, result, nested_threshold);
                    builder.pop_level();
                }
            }
        }
        _ => {
            // Optimized key creation with deduplication for memory efficiency
            let key_str = builder.as_str();
            let key = deduplicate_key(key_str);

            // Clone is necessary here as we're borrowing from the input Value
            // The clone is unavoidable since we need to own the value for the HashMap
            result.insert(key, value.clone());
        }
    }
}

// ================================================================================================
// MODULE: Replacement and Transformation Operations
// ================================================================================================

/// Apply key replacements for unflattening (works on Map<String, Value>)
/// This version is used when collision handling is NOT enabled for better performance
/// Takes ownership to avoid cloning values
fn apply_key_replacements_for_unflatten(
    obj: Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<Map<String, Value>, JsonToolsError> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        // Use the unified key replacement logic
        let final_key = if let Some(new_key) = apply_key_replacement_patterns(&key, patterns)? {
            new_key
        } else {
            key
        };

        // No clone needed - we own the value!
        new_obj.insert(final_key, value);
    }

    Ok(new_obj)
}

/// Apply value replacements for unflattening (works on Map<String, Value>)
/// Optimized with Cow to avoid unnecessary string allocations
fn apply_value_replacements_for_unflatten(
    obj: &mut Map<String, Value>,
    patterns: &[(String, String)],
) -> Result<(), JsonToolsError> {
    for (_, value) in obj.iter_mut() {
        // Use the unified value replacement logic
        apply_value_replacement_patterns(value, patterns)?;
    }
    Ok(())
}

/// Apply lowercase conversion to keys for unflattening
/// Optimized with Cow to avoid unnecessary allocations when keys are already lowercase
fn apply_lowercase_keys_for_unflatten(obj: Map<String, Value>) -> Map<String, Value> {
    let mut new_obj = Map::with_capacity(obj.len());

    for (key, value) in obj {
        // SIMD-optimized lowercase conversion (zero-copy if already lowercase)
        let lowercase_key = to_lowercase_simd(&key);

        let final_key = match lowercase_key {
            Cow::Borrowed(_) => key, // Key was already lowercase, reuse original
            Cow::Owned(lower) => lower, // Key was converted to lowercase
        };

        new_obj.insert(final_key, value);
    }

    new_obj
}

// ================================================================================================
// MODULE: Type Conversion Functions
// ================================================================================================

/// Try to parse a string into a number, handling various formats
/// Returns None if the string cannot be parsed as a valid number
///
/// Supports:
/// - Basic numbers: "123", "45.67", "-10"
/// - Scientific notation: "1e5", "1.23e-4"
/// - Thousands separators: "1,234.56" (US), "1.234,56" (EU), "1 234.56" (FR)
/// - Currency symbols: "$123.45", "€99.99", "£50.00"
/// - Percentages: "50%" → 50.0 (not as decimal)
///
/// Optimized version that accepts already-trimmed string and has fast-path for clean numbers
#[inline]
fn try_parse_number_optimized(trimmed: &str) -> Option<f64> {
    // Early exit for empty strings
    if trimmed.is_empty() {
        return None;
    }

    // Fast path: try direct parse first (handles basic numbers and scientific notation)
    // This catches ~90% of cases with minimal overhead
    if let Ok(num) = fast_float::parse(trimmed) {
        return Some(num);
    }

    // Handle percentage strings (e.g., "50%" → 50.0)
    if let Some(stripped) = trimmed.strip_suffix('%') {
        if let Ok(num) = fast_float::parse(stripped) {
            return Some(num);
        }
    }

    // OPTIMIZATION: Check if the string needs expensive cleaning
    // If it only contains digits, '.', '-', '+', 'e', 'E', '%' but failed to parse,
    // it might be a malformed format that needs cleaning (like trailing minus)
    // Only skip cleaning if there are definitely complex formats (currency, commas, etc.)
    let has_complex_formatting = trimmed.bytes().any(|b| {
        matches!(b, b',' | b'$' | b' ' | b'\'' | b'_' | b'(' | b')' | b'[' | b']')
            || b > 127 // Unicode characters (€, £, ¥, etc.)
    });

    if !has_complex_formatting {
        // No complex formatting detected, but parse still failed
        // This might be a malformed number (like "abc" or invalid scientific notation)
        // Call clean_number_string as a fallback - it handles edge cases like trailing minus
    }

    // Slow path: clean common number formats and try again
    let cleaned = clean_number_string(trimmed);
    fast_float::parse(&cleaned).ok()
}

/// Clean a number string by removing common formatting characters
/// Handles: currencies, thousands separators, negative formats, and more
/// Supports: $, €, £, ¥, ₹, ₽, ₩, ₺, R$, A$, C$, Fr, kr, zł, Kč, USD/EUR/GBP codes
/// Negative formats: -123, (123), [123], 123-, 123 CR/DR
/// Separators: comma, dot, space, apostrophe, underscore
/// Optimized with single-pass filtering and comprehensive format detection
#[inline(always)]  // Optimization #13: Force inline for hot path
fn clean_number_string(s: &str) -> String {
    let trimmed = s.trim();

    // Early exit for empty strings
    if trimmed.is_empty() {
        return String::new();
    }

    // Detect negative number formats
    let is_negative = trimmed.starts_with('-')
        || trimmed.starts_with('(') && trimmed.ends_with(')') // Accounting format: (123.45)
        || trimmed.starts_with('[') && trimmed.ends_with(']') // Bracket format: [123.45]
        || trimmed.ends_with('-'); // Trailing minus: 123.45-

    // Remove negative indicators temporarily for processing
    let working_str = if is_negative {
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            &trimmed[1..trimmed.len()-1]
        } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
            &trimmed[1..trimmed.len()-1]
        } else if trimmed.ends_with('-') {
            &trimmed[..trimmed.len()-1]
        } else {
            &trimmed[1..] // Remove leading minus
        }
    } else {
        trimmed
    }.trim();

    // Remove leading plus sign if present
    let working_str = working_str.strip_prefix('+').unwrap_or(working_str).trim();

    // Remove currency symbols and codes
    // Extended currency support: $, €, £, ¥, ₹, ₽, ₩, ₺, R$, A$, C$, Fr, kr, zł, Kč
    let mut without_currency = working_str;

    // Remove multi-character currency prefixes first (R$, A$, C$, AU$, CA$, US$)
    if without_currency.len() > 2 {
        if let Some(rest) = without_currency.strip_prefix("R$")
            .or_else(|| without_currency.strip_prefix("A$"))
            .or_else(|| without_currency.strip_prefix("C$"))
            .or_else(|| without_currency.strip_prefix("AU$"))
            .or_else(|| without_currency.strip_prefix("CA$"))
            .or_else(|| without_currency.strip_prefix("US$"))
            .or_else(|| without_currency.strip_prefix("Fr"))
            .or_else(|| without_currency.strip_prefix("kr"))
            .or_else(|| without_currency.strip_prefix("zł"))
            .or_else(|| without_currency.strip_prefix("Kč"))
        {
            without_currency = rest.trim();
        }
    }

    // Remove single-character currency symbols from start
    without_currency = without_currency
        .trim_start_matches(&['$', '€', '£', '¥', '₹', '₽', '₩', '₺'][..])
        .trim();

    // Remove currency codes (USD, EUR, GBP, etc.) - 3 letter codes at start
    // Only remove if followed by a space to avoid false positives like "ABC123"
    if without_currency.len() > 4 {
        let first_three = &without_currency[..3];
        if first_three.chars().all(|c| c.is_ascii_uppercase()) {
            let potential_code = &without_currency[3..];
            // Only strip if followed by space (USD 123, EUR 45.67)
            if potential_code.starts_with(' ') {
                without_currency = potential_code.trim();
            }
        }
    }

    // Remove trailing currency indicators and credit/debit markers
    without_currency = without_currency
        .trim_end_matches(&['$', '€', '£', '¥', '₹', '₽', '₩', '₺'][..])
        .trim_end_matches("CR") // Credit
        .trim_end_matches("DR") // Debit
        .trim_end_matches("cr")
        .trim_end_matches("dr")
        .trim();

    // Early exit for simple cases (no special characters)
    if !without_currency.contains(&[',', '.', ' ', '\'', '_'][..]) {
        return if is_negative {
            format!("-{}", without_currency)
        } else {
            without_currency.to_string()
        };
    }

    // Find positions of commas and dots to determine format
    let last_comma_pos = without_currency.rfind(',');
    let last_dot_pos = without_currency.rfind('.');
    let comma_count = without_currency.matches(',').count();
    let dot_count = without_currency.matches('.').count();

    // Pre-allocate with capacity to avoid reallocations
    let mut result = String::with_capacity(without_currency.len() + 1);

    // Add negative sign if needed
    if is_negative {
        result.push('-');
    }

    match (last_comma_pos, last_dot_pos, comma_count, dot_count) {
        // Both comma and dot present
        (Some(comma_pos), Some(dot_pos), _, _) => {
            if dot_pos > comma_pos {
                // US format: 1,234.56 - keep dot, remove commas
                for ch in without_currency.chars() {
                    match ch {
                        ',' | ' ' | '\'' | '_' => continue,  // Skip thousands separators
                        _ => result.push(ch),
                    }
                }
            } else {
                // European format: 1.234,56 - keep comma as decimal, remove dots
                for ch in without_currency.chars() {
                    match ch {
                        '.' | ' ' | '\'' | '_' => continue,  // Skip thousands separators
                        ',' => result.push('.'),              // Convert decimal comma to dot
                        _ => result.push(ch),
                    }
                }
            }
        }
        // Only comma present
        (Some(_), None, 1, 0) => {
            // Single comma - likely decimal separator (European format: 12,34)
            for ch in without_currency.chars() {
                match ch {
                    ',' => result.push('.'),            // Convert decimal comma to dot
                    ' ' | '\'' | '_' => continue,       // Skip separators
                    _ => result.push(ch),
                }
            }
        }
        (Some(_), None, _, 0) => {
            // Multiple commas - could be thousands separators (US format: 1,234,567)
            // Validate the format - commas should be every 3 digits from right
            let segments: Vec<&str> = without_currency.split(',').collect();
            let is_valid_thousands = segments.len() > 1
                && segments[1..].iter().all(|seg| seg.len() == 3 && seg.chars().all(|c| c.is_ascii_digit()));

            if is_valid_thousands {
                // Valid thousands separators - remove commas
                for ch in without_currency.chars() {
                    match ch {
                        ',' | ' ' | '\'' | '_' => continue,   // Skip thousands separators
                        _ => result.push(ch),
                    }
                }
            } else {
                // Invalid format (like "12,34,56") - keep as-is and let it fail to parse
                return without_currency.to_string();
            }
        }
        // Only dot present (multiple dots means thousands separators in EU format)
        (None, Some(_), 0, count) if count > 1 => {
            // Multiple dots - could be thousands separators (European format: 1.234.567)
            // But need to validate the format - dots should be every 3 digits from right
            // Split by dots and check if all segments (except first) have 3 digits
            let segments: Vec<&str> = without_currency.split('.').collect();
            let is_valid_thousands = segments.len() > 1
                && segments[1..].iter().all(|seg| seg.len() == 3 && seg.chars().all(|c| c.is_ascii_digit()));

            if is_valid_thousands {
                // Valid thousands separators - remove dots
                for ch in without_currency.chars() {
                    match ch {
                        '.' | ' ' | '\'' | '_' => continue,   // Skip thousands separators
                        _ => result.push(ch),
                    }
                }
            } else {
                // Invalid format (like "12.34.56") - keep as-is and let it fail to parse
                return without_currency.to_string();
            }
        }
        // Default case: just remove spaces, apostrophes, and underscores
        _ => {
            for ch in without_currency.chars() {
                match ch {
                    ' ' | '\'' | '_' => continue,         // Skip separators
                    _ => result.push(ch),
                }
            }
        }
    }

    result
}

/// Fast version that accepts already-trimmed string (no trim() overhead)
#[inline(always)]
fn try_parse_bool_fast(s: &str) -> Option<bool> {
    match s {
        // Standard true/false variants
        "true" | "TRUE" | "True" => Some(true),
        "false" | "FALSE" | "False" => Some(false),

        // Yes/No variants
        "yes" | "YES" | "Yes" => Some(true),
        "no" | "NO" | "No" => Some(false),

        // Y/N variants
        "y" | "Y" => Some(true),
        "n" | "N" => Some(false),

        // On/Off variants
        "on" | "ON" | "On" => Some(true),
        "off" | "OFF" | "Off" => Some(false),

        _ => None,
    }
}

/// Convert f64 to JSON Number value
/// Returns None for NaN or Infinity (invalid JSON numbers)
/// Converts to integer if the number has no fractional part
#[inline]
fn f64_to_json_number(num: f64) -> Option<Value> {
    // Check if the number is an integer (no fractional part)
    if num.is_finite() && num.fract() == 0.0 {
        // Try to convert to i64 first for better representation
        if num >= i64::MIN as f64 && num <= i64::MAX as f64 {
            return Some(Value::Number(serde_json::Number::from(num as i64)));
        }
        // If it's too large for i64, try u64
        if num >= 0.0 && num <= u64::MAX as f64 {
            return Some(Value::Number(serde_json::Number::from(num as u64)));
        }
    }

    // Otherwise, use f64
    serde_json::Number::from_f64(num).map(Value::Number)
}

/// Fast version that accepts already-trimmed string (no trim() overhead)
#[inline(always)]
fn is_null_string_fast(s: &str) -> bool {
    matches!(
        s,
        "null" | "NULL" | "Null" |
        "nil" | "NIL" | "Nil" |
        "none" | "NONE" | "None" |
        "N/A" | "n/a" | "NA" | "na"
    )
}

/// Apply automatic type conversion to a single value
/// Tries conversions in order: null strings, booleans, numbers
/// Booleans checked first since "1"/"0" are commonly used as boolean indicators
/// Keeps original string if no conversion succeeds
#[inline]
fn apply_type_conversion_to_value(value: &mut Value) {
    if let Value::String(s) = value {
        // Early exit for empty strings (most common case that won't convert)
        if s.is_empty() {
            return;
        }

        // OPTIMIZATION: Trim once and reuse to avoid multiple trim() calls
        let trimmed = s.trim();

        // Early exit if trimming removed everything
        if trimmed.is_empty() {
            return;
        }

        // Check for null string representations first (fast match)
        if is_null_string_fast(trimmed) {
            *value = Value::Null;
            return;
        }

        // Try boolean conversion first (more specific for "1"/"0" cases)
        if let Some(b) = try_parse_bool_fast(trimmed) {
            *value = Value::Bool(b);
            return;
        }

        // Try number conversion (pass trimmed string to avoid re-trimming)
        if let Some(num) = try_parse_number_optimized(trimmed) {
            if let Some(num_value) = f64_to_json_number(num) {
                *value = num_value;
                return;
            }
        }

        // Keep as string if no conversion succeeded
    }
}

/// Recursively apply type conversion to all string values in a JSON structure
fn apply_type_conversion_recursive(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for v in map.values_mut() {
                apply_type_conversion_recursive(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                apply_type_conversion_recursive(v);
            }
        }
        Value::String(_) => {
            apply_type_conversion_to_value(value);
        }
        _ => {} // Leave other types unchanged
    }
}

// ================================================================================================
// MODULE: Core Algorithms - Unflattening Implementation
// ================================================================================================

/// Core unflattening algorithm that reconstructs nested JSON from flattened keys
fn unflatten_object(obj: Map<String, Value>, separator: &str) -> Result<Value, JsonToolsError> {
    // OPTIMIZATION: Single-pass unflatten algorithm with zero-copy values
    // Process keys in sorted order so parents are guaranteed to exist before children.
    // Takes ownership of the map to avoid cloning values during unflatten.

    let mut result = Map::with_capacity(obj.len() / 2); // Estimate final size

    // Pre-analyze path types before consuming the map
    let path_types = analyze_path_types(&obj, separator);

    // Sort keys to ensure parents are processed before children
    // Convert to owned entries to avoid cloning values later
    let mut sorted_entries: Vec<_> = obj.into_iter().collect();
    sorted_entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    // Process entries in order, taking ownership (no clone needed!)
    for (key, value) in sorted_entries {
        set_nested_value_with_types(&mut result, &key, value, separator, &path_types)?;
    }

    Ok(Value::Object(result))
}

/// Analyze all flattened keys to determine whether each path should be an array or object
fn analyze_path_types(obj: &Map<String, Value>, separator: &str) -> FxHashMap<String, bool> {
    analyze_path_types_optimized(obj, separator)
}

/// Optimized path analysis using radix tree and efficient data structures
fn analyze_path_types_optimized(obj: &Map<String, Value>, separator: &str) -> FxHashMap<String, bool> {
    // Use a more efficient approach with pre-allocated capacity and optimized string operations
    let estimated_paths = obj.len() * 2; // Rough estimate of path count
    let mut state: FxHashMap<String, u8> = FxHashMap::with_capacity_and_hasher(estimated_paths, Default::default());

    // Pre-compile separator for faster matching
    let sep_bytes = separator.as_bytes();
    let sep_len = separator.len();

    for key in obj.keys() {
        analyze_key_path_optimized(key, sep_bytes, sep_len, &mut state);
    }

    // Convert bitmask state to final decision with pre-allocated result
    let mut result: FxHashMap<String, bool> = FxHashMap::with_capacity_and_hasher(state.len(), Default::default());
    for (k, mask) in state.into_iter() {
        let is_array = (mask & 0b10 == 0) && (mask & 0b01 != 0);
        result.insert(k, is_array);
    }
    result
}

/// Optimized key path analysis with efficient string operations
#[inline]
fn analyze_key_path_optimized(key: &str, sep_bytes: &[u8], sep_len: usize, state: &mut FxHashMap<String, u8>) {
    let key_bytes = key.as_bytes();
    let mut search_start = 0;

    // Use Boyer-Moore-like approach for separator finding
    while search_start < key_bytes.len() {
        // Find next separator using optimized byte search
        let next_sep = find_separator_optimized(key_bytes, sep_bytes, search_start);

        if next_sep == key_bytes.len() {
            break; // No more separators
        }

        // Extract parent path efficiently
        let parent = &key[..next_sep];

        // Look ahead to classify child
        let child_start = next_sep + sep_len;
        if child_start < key_bytes.len() {
            let child_end = find_separator_optimized(key_bytes, sep_bytes, child_start)
                .min(key_bytes.len());
            let child = &key[child_start..child_end];

            // Optimized numeric check
            let is_numeric = is_valid_array_index(child);

            // Update state with efficient entry handling
            match state.get_mut(parent) {
                Some(entry) => {
                    if is_numeric { *entry |= 0b01; } else { *entry |= 0b10; }
                }
                None => {
                    let initial_value = if is_numeric { 0b01 } else { 0b10 };
                    state.insert(parent.to_string(), initial_value);
                }
            }
        }

        search_start = next_sep + sep_len;
    }
}

/// SIMD-optimized separator finding using memchr crate
///
/// Uses hardware-accelerated SIMD instructions (SSE2/AVX2/NEON) for byte searching
/// Provides 3-10x speedup over naive byte-by-byte search
#[inline]
fn find_separator_optimized(haystack: &[u8], needle: &[u8], start: usize) -> usize {
    if needle.len() == 1 {
        // Single byte separator - use memchr's SIMD-optimized byte search
        // This uses SSE2/AVX2 on x86 and NEON on ARM for 3-10x speedup
        memchr(needle[0], &haystack[start..])
            .map(|pos| start + pos)
            .unwrap_or(haystack.len())
    } else {
        // Multi-byte separator - use memmem's SIMD-optimized substring search
        // Uses Two-Way algorithm with SIMD acceleration
        memmem::find(&haystack[start..], needle)
            .map(|pos| start + pos)
            .unwrap_or(haystack.len())
    }
}

/// SIMD-optimized lowercase conversion using Cow for zero-copy when possible
///
/// Uses byte-level SIMD operations to detect uppercase ASCII characters
/// If no uppercase found, returns Borrowed (zero-copy)
/// Otherwise, converts to lowercase and returns Owned
#[inline]
fn to_lowercase_simd(s: &str) -> Cow<'_, str> {
    let bytes = s.as_bytes();

    // SIMD-friendly check: scan for uppercase ASCII bytes (A-Z = 0x41-0x5A)
    // This allows compiler to vectorize the loop
    let has_uppercase = bytes.iter().any(|&b| matches!(b, b'A'..=b'Z'));

    if has_uppercase {
        Cow::Owned(s.to_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

/// Optimized check for valid array index
#[inline]
fn is_valid_array_index(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Fast path for single digit
    if s.len() == 1 {
        return s.as_bytes()[0].is_ascii_digit();
    }

    // Check for leading zero (invalid except for "0")
    if s.starts_with('0') {
        return s == "0";
    }

    // Check if all characters are digits (vectorizable)
    s.bytes().all(|b| b.is_ascii_digit())
}


/// Set a nested value using pre-analyzed path types to handle conflicts
fn set_nested_value_with_types(
    result: &mut Map<String, Value>,
    key_path: &str,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
) -> Result<(), JsonToolsError> {
    let parts: Vec<&str> = key_path.split(separator).collect();

    if parts.is_empty() {
        return Err(JsonToolsError::invalid_json_structure("Empty key path"));
    }

    if parts.len() == 1 {
        // Simple key, just insert
        result.insert(parts[0].to_string(), value);
        return Ok(());
    }

    // OPTIMIZATION: Pre-allocate path buffer to avoid repeated allocations
    let mut path_buffer = String::with_capacity(key_path.len());
    set_nested_value_recursive_with_types_optimized(
        result, &parts, 0, value, separator, path_types, &mut path_buffer
    )
}

/// Optimized recursive helper that reuses a path buffer to avoid allocations
fn set_nested_value_recursive_with_types_optimized(
    current: &mut Map<String, Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
) -> Result<(), JsonToolsError> {
    let part = parts[index];

    if index == parts.len() - 1 {
        // Last part, insert the value
        current.insert(part.to_string(), value);
        return Ok(());
    }

    // Build the current path in the buffer
    let buffer_start_len = path_buffer.len();
    if buffer_start_len > 0 {
        path_buffer.push_str(separator);
    }
    path_buffer.push_str(part);

    let should_be_array = path_types.get(path_buffer.as_str()).copied().unwrap_or(false);

    // Get or create the nested structure based on the determined type
    let entry = current.entry(part.to_string()).or_insert_with(|| {
        if should_be_array {
            Value::Array(vec![])
        } else {
            Value::Object(Map::new())
        }
    });

    let result = match entry {
        Value::Object(ref mut obj) => set_nested_value_recursive_with_types_optimized(
            obj,
            parts,
            index + 1,
            value,
            separator,
            path_types,
            path_buffer,
        ),
        Value::Array(ref mut arr) => {
            // Handle array indexing
            let next_part = parts[index + 1];
            if let Ok(array_index) = next_part.parse::<usize>() {
                // Ensure array is large enough
                while arr.len() <= array_index {
                    arr.push(Value::Null);
                }

                if index + 2 == parts.len() {
                    // Last part, set the value
                    arr[array_index] = value;
                    Ok(())
                } else {
                    // Build next path in buffer for type lookup
                    path_buffer.push_str(separator);
                    path_buffer.push_str(next_part);
                    let next_should_be_array = path_types.get(path_buffer.as_str()).copied().unwrap_or(false);

                    if arr[array_index].is_null() {
                        arr[array_index] = if next_should_be_array {
                            Value::Array(vec![])
                        } else {
                            Value::Object(Map::new())
                        };
                    }

                    match &mut arr[array_index] {
                        Value::Object(ref mut obj) => set_nested_value_recursive_with_types_optimized(
                            obj,
                            parts,
                            index + 2,
                            value,
                            separator,
                            path_types,
                            path_buffer,
                        ),
                        Value::Array(ref mut nested_arr) => {
                            set_nested_value_recursive_for_array_with_types_optimized(
                                nested_arr,
                                parts,
                                index + 2,
                                value,
                                separator,
                                path_types,
                                path_buffer,
                            )
                        }
                        _ => Err(JsonToolsError::invalid_json_structure(format!(
                            "Array element at index {} has incompatible type",
                            array_index
                        ))),
                    }
                }
            } else {
                // Non-numeric key in array context - treat as object key
                // Convert array to object
                let mut obj = Map::new();
                for (i, item) in arr.iter().enumerate() {
                    if !item.is_null() {
                        obj.insert(i.to_string(), item.clone());
                    }
                }
                obj.insert(next_part.to_string(), Value::Null); // Placeholder
                *entry = Value::Object(obj);

                // Now continue as object
                if let Value::Object(ref mut obj) = entry {
                    set_nested_value_recursive_with_types_optimized(
                        obj,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                        path_buffer,
                    )
                } else {
                    unreachable!()
                }
            }
        }
        _ => Err(JsonToolsError::invalid_json_structure(format!(
            "Cannot navigate into non-object/non-array value at key: {}",
            part
        ))),
    };

    // Restore buffer to its state before this call
    path_buffer.truncate(buffer_start_len);
    result
}

/// Optimized recursive helper for setting nested values in arrays with type awareness
fn set_nested_value_recursive_for_array_with_types_optimized(
    arr: &mut Vec<Value>,
    parts: &[&str],
    index: usize,
    value: Value,
    separator: &str,
    path_types: &FxHashMap<String, bool>,
    path_buffer: &mut String,
) -> Result<(), JsonToolsError> {
    if index >= parts.len() {
        return Err(JsonToolsError::invalid_json_structure(
            "Invalid path for array"
        ));
    }

    let part = parts[index];

    if let Ok(array_index) = part.parse::<usize>() {
        while arr.len() <= array_index {
            arr.push(Value::Null);
        }

        if index == parts.len() - 1 {
            arr[array_index] = value;
            Ok(())
        } else {
            // Build path in buffer for type lookup
            let buffer_start_len = path_buffer.len();
            if buffer_start_len > 0 {
                path_buffer.push_str(separator);
            }
            path_buffer.push_str(part);

            let next_should_be_array = path_types.get(path_buffer.as_str()).copied().unwrap_or(false);

            if arr[array_index].is_null() {
                arr[array_index] = if next_should_be_array {
                    Value::Array(vec![])
                } else {
                    Value::Object(Map::new())
                };
            }

            let result = match &mut arr[array_index] {
                Value::Object(ref mut obj) => set_nested_value_recursive_with_types_optimized(
                    obj,
                    parts,
                    index + 1,
                    value,
                    separator,
                    path_types,
                    path_buffer,
                ),
                Value::Array(ref mut nested_arr) => {
                    set_nested_value_recursive_for_array_with_types_optimized(
                        nested_arr,
                        parts,
                        index + 1,
                        value,
                        separator,
                        path_types,
                        path_buffer,
                    )
                }
                _ => Err(JsonToolsError::invalid_json_structure(format!(
                    "Array element at index {} has incompatible type",
                    array_index
                ))),
            };

            // Restore buffer to its state before this call
            path_buffer.truncate(buffer_start_len);
            result
        }
    } else {
        Err(JsonToolsError::invalid_json_structure(format!(
            "Expected array index but got: {}",
            part
        )))
    }
}

/// Helper function to check if a value should be filtered out based on criteria
/// Consolidates the filtering logic used by both objects and arrays
#[inline]
fn should_filter_value(
    v: &Value,
    remove_empty_strings: bool,
    remove_nulls: bool,
    remove_empty_objects: bool,
    remove_empty_arrays: bool,
) -> bool {
    if remove_empty_strings {
        if let Some(s) = v.as_str() {
            if s.is_empty() {
                return true;
            }
        }
    }
    if remove_nulls && v.is_null() {
        return true;
    }
    if remove_empty_objects {
        if let Some(obj) = v.as_object() {
            if obj.is_empty() {
                return true;
            }
        }
    }
    if remove_empty_arrays {
        if let Some(arr) = v.as_array() {
            if arr.is_empty() {
                return true;
            }
        }
    }
    false
}

/// Recursively filter nested JSON values based on the specified criteria
/// This function removes empty strings, nulls, empty objects, and empty arrays from nested JSON structures
fn filter_nested_value(
    value: &mut Value,
    remove_empty_strings: bool,
    remove_nulls: bool,
    remove_empty_objects: bool,
    remove_empty_arrays: bool,
) {
    match value {
        Value::Object(ref mut obj) => {
            // First, recursively filter all nested values
            for (_, v) in obj.iter_mut() {
                filter_nested_value(v, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays);
            }

            // Then remove keys that match our filtering criteria
            obj.retain(|_, v| {
                !should_filter_value(v, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays)
            });
        }
        Value::Array(ref mut arr) => {
            // First, recursively filter all nested values
            for item in arr.iter_mut() {
                filter_nested_value(item, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays);
            }

            // Then remove array elements that match our filtering criteria
            arr.retain(|v| {
                !should_filter_value(v, remove_empty_strings, remove_nulls, remove_empty_objects, remove_empty_arrays)
            });
        }
        _ => {
            // For primitive values (strings, numbers, booleans, null), no filtering needed
            // The filtering will be handled by the parent container
        }
    }
}

// ================================================================================================
// MODULE: Collision Handling and Resolution Strategies
// ================================================================================================

/// Handle key collisions in a flattened map
///
/// This function processes a HashMap to handle cases where multiple keys would collide
/// after key replacements and transformations. It supports two strategies:
///
/// Only supported strategy: `handle_key_collision` to collect values into arrays for duplicate keys
fn handle_key_collisions(
    mut flattened: FxHashMap<String, Value>,
    handle_key_collision: bool,
) -> FxHashMap<String, Value> {
    // If option is disabled, return as-is
    if !handle_key_collision {
        return flattened;
    }

    // Use the unified collision detection logic
    let key_groups = group_items_by_key(flattened.drain());
    let mut result = FxHashMap::with_capacity_and_hasher(key_groups.len(), Default::default());

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected: collect values into array
            if let Some((final_key, array_value)) = apply_collision_handling(key, values, None) {
                result.insert(final_key, array_value);
            }
        }
    }

    result
}

/// Handle key collisions for unflattening operations
///
/// This function processes a Map<String, Value> (flattened object) to handle cases where
/// multiple keys would collide after key replacements and transformations.
/// Only supported strategy: collect values into arrays when enabled.
fn handle_key_collisions_for_unflatten(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Map<String, Value> {
    // If option is disabled, return as-is
    if !config.collision.handle_collisions {
        return flattened_obj;
    }

    // Use the unified collision detection logic
    let key_groups = group_items_by_key(flattened_obj.into_iter());
    let mut result = Map::new();

    for (key, values) in key_groups {
        if values.len() == 1 {
            // No collision, keep the original key-value pair
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            // Collision detected: collect values into array, with filtering
            if let Some((final_key, array_value)) = apply_collision_handling(key, values, Some(&config.filtering)) {
                result.insert(final_key, array_value);
            }
        }
    }

    result
}

/// Helper function to determine if a value should be included based on filtering criteria
/// This ensures consistent filtering logic across both flatten and unflatten operations
#[inline]
fn should_include_value(
    value: &Value,
    remove_empty_string_values: bool,
    remove_null_values: bool,
    remove_empty_dict: bool,
    remove_empty_list: bool,
) -> bool {
    // Check for empty strings
    if remove_empty_string_values {
        if let Some(s) = value.as_str() {
            if s.is_empty() {
                return false;
            }
        }
    }

    // Check for nulls
    if remove_null_values && value.is_null() {
        return false;
    }

    // Check for empty objects
    if remove_empty_dict {
        if let Some(obj) = value.as_object() {
            if obj.is_empty() {
                return false;
            }
        }
    }

    // Check for empty arrays
    if remove_empty_list {
        if let Some(arr) = value.as_array() {
            if arr.is_empty() {
                return false;
            }
        }
    }

    true
}

/// Apply key replacements with collision handling for flattening operations
///
/// This function combines key replacement and collision detection with performance optimizations
/// including regex pre-compilation, early exit checks, and efficient string handling.
/// It properly handles cases where multiple keys would map to the same result after replacement.
fn apply_key_replacements_with_collision_handling(
    flattened: FxHashMap<String, Value>,
    patterns: &[&str],
    config: &ProcessingConfig,
) -> Result<FxHashMap<String, Value>, JsonToolsError> {
    if patterns.is_empty() {
        return Ok(flattened);
    }

    if patterns.len() % 2 != 0 {
        return Err(JsonToolsError::invalid_replacement_pattern(
            "Patterns must be provided in pairs (find, replace)"
        ));
    }

    // Pre-compile all regex patterns to avoid repeated compilation
    // Patterns are treated as regex. If compilation fails, fall back to literal matching.
    let mut compiled_patterns = Vec::with_capacity(patterns.len() / 2);
    for chunk in patterns.chunks(2) {
        let pattern = chunk[0];
        let replacement = chunk[1];

        // Try to compile as regex
        match get_cached_regex(pattern) {
            Ok(regex) => compiled_patterns.push((Some(regex), replacement)),
            Err(_) => compiled_patterns.push((None, replacement)),
        }
    }

    // Early exit optimization: check if any keys need replacement to avoid unnecessary allocation
    if !config.collision.handle_collisions {
        // When collision handling is disabled, we can use the optimized path
        let needs_replacement = flattened.keys().any(|key| {
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = &chunk[0];
                let (compiled_regex, _) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(key) {
                        return true;
                    }
                } else if key.contains(pattern) {
                    return true;
                }
            }
            false
        });

        if !needs_replacement {
            return Ok(flattened);
        }

        // Use optimized path for non-collision scenarios with Cow for efficient string handling
        let mut new_flattened = FxHashMap::with_capacity_and_hasher(flattened.len(), Default::default());

        for (old_key, value) in flattened {
            let mut new_key = Cow::Borrowed(old_key.as_str());

            // Apply each compiled pattern
            for (i, chunk) in patterns.chunks(2).enumerate() {
                let pattern = chunk[0];
                let (compiled_regex, replacement) = &compiled_patterns[i];

                if let Some(regex) = compiled_regex {
                    if regex.is_match(&new_key) {
                        new_key = Cow::Owned(
                            regex
                                .replace_all(&new_key, *replacement)
                                .to_string(),
                        );
                    }
                } else if new_key.contains(pattern) {
                    new_key = Cow::Owned(new_key.replace(pattern, *replacement));
                }
            }

            new_flattened.insert(new_key.into_owned(), value);
        }

        return Ok(new_flattened);
    }

    // Collision handling path: apply replacements and track what each original key maps to
    let flattened_len = flattened.len();
    let mut key_mapping: FxHashMap<String, String> = FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());
    let mut original_values: FxHashMap<String, Value> = FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());

    for (original_key, value) in flattened {
        let mut new_key = Cow::Borrowed(original_key.as_str());

        // Apply all key replacement patterns using pre-compiled patterns
        for (i, chunk) in patterns.chunks(2).enumerate() {
            let pattern = chunk[0];
            let (compiled_regex, replacement) = &compiled_patterns[i];

            if let Some(regex) = compiled_regex {
                if regex.is_match(&new_key) {
                    new_key = Cow::Owned(
                        regex
                            .replace_all(&new_key, *replacement)
                            .to_string(),
                    );
                }
            } else if new_key.contains(pattern) {
                new_key = Cow::Owned(new_key.replace(pattern, *replacement));
            }
        }

        let new_key_string = new_key.into_owned();
        key_mapping.insert(original_key.clone(), new_key_string.clone());
        original_values.insert(original_key, value);
    }

    // Second pass: group by target key to detect collisions
    // OPTIMIZATION: Consume key_mapping to avoid cloning
    let mut target_groups: FxHashMap<String, Vec<String>> = FxHashMap::with_capacity_and_hasher(key_mapping.len(), Default::default());
    for (original_key, target_key) in key_mapping {
        target_groups.entry(target_key).or_insert_with(Vec::new).push(original_key);
    }

    // Third pass: build result with collision handling
    let mut result = FxHashMap::with_capacity_and_hasher(target_groups.len(), Default::default());

    for (target_key, original_keys) in target_groups {
        if original_keys.len() == 1 {
            // No collision
            let original_key = &original_keys[0];
            let value = original_values.remove(original_key).unwrap();
            result.insert(target_key, value);
        } else {
            // Collision detected: Only supported strategy is collecting into arrays
            let mut values = Vec::with_capacity(original_keys.len());
            for original_key in &original_keys {
                let value = original_values.remove(original_key).unwrap();

                // Apply filtering to values before adding to collision array
                let should_include = should_include_value(
                    &value,
                    config.filtering.remove_empty_strings,
                    config.filtering.remove_nulls,
                    config.filtering.remove_empty_objects,
                    config.filtering.remove_empty_arrays,
                );

                if should_include {
                    values.push(value);
                }
            }

            // Only create the array if we have values after filtering
            if !values.is_empty() {
                result.insert(target_key, Value::Array(values));
            }
            // If all values were filtered out, don't insert anything
        }
    }

    Ok(result)
}

/// Apply key replacements with collision handling for unflattening operations
///
/// This function combines key replacement and collision detection for Map<String, Value>
/// to properly handle cases where multiple keys would map to the same result after replacement.
fn apply_key_replacements_unflatten_with_collisions(
    flattened_obj: Map<String, Value>,
    config: &ProcessingConfig,
) -> Result<Map<String, Value>, JsonToolsError> {
    if config.replacements.key_replacements.is_empty() {
        return Ok(flattened_obj);
    }

    // First pass: apply replacements and track what each original key maps to
    let flattened_len = flattened_obj.len();
    let mut key_mapping: FxHashMap<String, String> = FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());
    let mut original_values: FxHashMap<String, Value> = FxHashMap::with_capacity_and_hasher(flattened_len, Default::default());

    for (original_key, value) in flattened_obj {
        // Apply all key replacement patterns using Cow to avoid allocation if no replacement
        let mut new_key = Cow::Borrowed(original_key.as_str());

        // Apply all key replacement patterns
        // Patterns are treated as regex. If compilation fails, fall back to literal matching.
        for (find, replace) in &config.replacements.key_replacements {
            // Try to compile as regex first
            match get_cached_regex(find) {
                Ok(regex) => {
                    // Successfully compiled as regex - use regex replacement
                    if regex.is_match(&new_key) {
                        new_key = Cow::Owned(regex.replace_all(&new_key, replace).to_string());
                    }
                }
                Err(_) => {
                    // Failed to compile as regex - fall back to literal replacement
                    if new_key.contains(find) {
                        new_key = Cow::Owned(new_key.replace(find, replace));
                    }
                }
            }
        }

        let new_key_string = new_key.into_owned();
        key_mapping.insert(original_key.clone(), new_key_string.clone());
        original_values.insert(original_key, value);
    }

    // Second pass: group by target key to detect collisions
    // OPTIMIZATION: Consume key_mapping to avoid cloning target_key
    let mut target_groups: FxHashMap<String, Vec<String>> = FxHashMap::with_capacity_and_hasher(key_mapping.len(), Default::default());
    for (original_key, target_key) in key_mapping {
        target_groups.entry(target_key).or_insert_with(Vec::new).push(original_key);
    }

    // Third pass: build result with collision handling
    let mut result = Map::with_capacity(target_groups.len());

    for (target_key, original_keys) in target_groups {
        if original_keys.len() == 1 {
            // No collision
            let original_key = &original_keys[0];
            let value = original_values.remove(original_key).unwrap();
            result.insert(target_key, value);
        } else {
            // Collision detected: Only supported strategy is collecting into arrays
            let mut values = Vec::with_capacity(original_keys.len());
            for original_key in &original_keys {
                let value = original_values.remove(original_key).unwrap();

                // Apply filtering to values before adding to collision array
                let should_include = should_include_value(
                    &value,
                    config.filtering.remove_empty_strings,
                    config.filtering.remove_nulls,
                    config.filtering.remove_empty_objects,
                    config.filtering.remove_empty_arrays,
                );

                if should_include {
                    values.push(value);
                }
            }

            // Only create the array if we have values after filtering
            if !values.is_empty() {
                result.insert(target_key, Value::Array(values));
            }
            // If all values were filtered out, don't insert anything
        }
    }

    Ok(result)
}


