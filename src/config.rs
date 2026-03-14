//! Configuration types for JSONTools operations.
//!
//! Defines operation modes, filtering flags, collision handling, replacement
//! patterns, and processing thresholds. Supports environment variable overrides
//! for parallelism settings.

use smallvec::SmallVec;
use std::sync::LazyLock;

/// Parse an environment variable once at process startup.
pub(crate) fn parse_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Parse an optional environment variable (returns None if unset).
pub(crate) fn parse_env_usize_opt(name: &str) -> Option<usize> {
    std::env::var(name).ok().and_then(|v| v.parse().ok())
}

pub(crate) static DEFAULT_PARALLEL_THRESHOLD: LazyLock<usize> =
    LazyLock::new(|| parse_env_usize("JSON_TOOLS_PARALLEL_THRESHOLD", 100));
pub(crate) static DEFAULT_NESTED_PARALLEL_THRESHOLD: LazyLock<usize> =
    LazyLock::new(|| parse_env_usize("JSON_TOOLS_NESTED_PARALLEL_THRESHOLD", 100));
pub(crate) static DEFAULT_MAX_ARRAY_INDEX: LazyLock<usize> =
    LazyLock::new(|| parse_env_usize("JSON_TOOLS_MAX_ARRAY_INDEX", 100_000));
pub(crate) static DEFAULT_NUM_THREADS: LazyLock<Option<usize>> =
    LazyLock::new(|| parse_env_usize_opt("JSON_TOOLS_NUM_THREADS"));

/// Operation mode for the unified JSONTools API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // Smaller discriminant for better cache locality
pub(crate) enum OperationMode {
    /// Flatten JSON structures
    Flatten,
    /// Unflatten JSON structures
    Unflatten,
    /// Normal processing (no flatten/unflatten) applying transformations recursively
    Normal,
}

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
    #[must_use]
    pub fn remove_empty_strings(mut self, enabled: bool) -> Self {
        self.remove_empty_strings = enabled;
        self
    }

    /// Enable removal of null values
    #[must_use]
    pub fn remove_nulls(mut self, enabled: bool) -> Self {
        self.remove_nulls = enabled;
        self
    }

    /// Enable removal of empty objects
    #[must_use]
    pub fn remove_empty_objects(mut self, enabled: bool) -> Self {
        self.remove_empty_objects = enabled;
        self
    }

    /// Enable removal of empty arrays
    #[must_use]
    pub fn remove_empty_arrays(mut self, enabled: bool) -> Self {
        self.remove_empty_arrays = enabled;
        self
    }

    /// Check if any filtering is enabled
    pub fn has_any_filter(&self) -> bool {
        self.remove_empty_strings
            || self.remove_nulls
            || self.remove_empty_objects
            || self.remove_empty_arrays
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable collision handling by collecting values into arrays
    #[must_use]
    pub fn handle_collisions(mut self, enabled: bool) -> Self {
        self.handle_collisions = enabled;
        self
    }

    /// Check if any collision handling is enabled
    pub fn has_collision_handling(&self) -> bool {
        self.handle_collisions
    }
}

/// Configuration for replacement operations
#[derive(Debug, Clone, Default)]
pub struct ReplacementConfig {
    /// Key replacement patterns (find, replace)
    /// Uses SmallVec to avoid heap allocation for 0-2 replacements (common case)
    pub key_replacements: SmallVec<[(String, String); 2]>,
    /// Value replacement patterns (find, replace)
    /// Uses SmallVec to avoid heap allocation for 0-2 replacements (common case)
    pub value_replacements: SmallVec<[(String, String); 2]>,
}

impl ReplacementConfig {
    /// Create a new ReplacementConfig with no replacements
    pub fn new() -> Self {
        Self {
            key_replacements: SmallVec::new(),
            value_replacements: SmallVec::new(),
        }
    }

    /// Add a key replacement pattern
    #[must_use]
    pub fn add_key_replacement(
        mut self,
        find: impl Into<String>,
        replace: impl Into<String>,
    ) -> Self {
        self.key_replacements.push((find.into(), replace.into()));
        self
    }

    /// Add a value replacement pattern
    #[must_use]
    pub fn add_value_replacement(
        mut self,
        find: impl Into<String>,
        replace: impl Into<String>,
    ) -> Self {
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
    /// Number of threads for parallel processing (None = use system default)
    pub num_threads: Option<usize>,
    /// Minimum object/array size for nested parallel processing within a single JSON document
    /// Only objects/arrays with more than this many keys/items will be processed in parallel
    /// Default: 100 (can be overridden with JSON_TOOLS_NESTED_PARALLEL_THRESHOLD environment variable)
    pub nested_parallel_threshold: usize,
    /// Maximum array index allowed during unflattening to prevent DoS via malicious keys
    /// Default: 100,000 (can be overridden with JSON_TOOLS_MAX_ARRAY_INDEX environment variable)
    pub max_array_index: usize,
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
            parallel_threshold: *DEFAULT_PARALLEL_THRESHOLD,
            num_threads: None, // Use system default (number of logical CPUs)
            nested_parallel_threshold: *DEFAULT_NESTED_PARALLEL_THRESHOLD,
            max_array_index: *DEFAULT_MAX_ARRAY_INDEX,
        }
    }
}

impl ProcessingConfig {
    /// Create a new ProcessingConfig with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the separator for nested keys
    #[must_use]
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Enable lowercase key conversion
    #[must_use]
    pub fn lowercase_keys(mut self, enabled: bool) -> Self {
        self.lowercase_keys = enabled;
        self
    }

    /// Configure filtering options
    #[must_use]
    pub fn filtering(mut self, filtering: FilteringConfig) -> Self {
        self.filtering = filtering;
        self
    }

    /// Configure collision handling options
    #[must_use]
    pub fn collision(mut self, collision: CollisionConfig) -> Self {
        self.collision = collision;
        self
    }

    /// Configure replacement options
    #[must_use]
    pub fn replacements(mut self, replacements: ReplacementConfig) -> Self {
        self.replacements = replacements;
        self
    }
}
