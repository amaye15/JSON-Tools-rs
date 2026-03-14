//! Multi-tier caching for regex patterns.
//!
//! Three-tier regex cache: compile-time table for common patterns, thread-local
//! FxHashMap for recent patterns, and global DashMap for shared access.

use dashmap::DashMap;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::sync::{Arc, LazyLock};

/// Maximum number of patterns in the thread-local cache before eviction.
const THREAD_LOCAL_CACHE_CAPACITY: usize = 128;

/// Maximum number of patterns in the global DashMap cache.
const GLOBAL_CACHE_CAPACITY: usize = 512;

/// Pre-compiled common regex patterns for maximum performance
/// Using Arc<Regex> to make cloning O(1) instead of copying the entire regex state
/// Using std::sync::LazyLock (Rust 1.80+) instead of lazy_static for better performance
static COMMON_REGEX_PATTERNS: LazyLock<FxHashMap<&'static str, Arc<Regex>>> = LazyLock::new(|| {
    // Pre-allocate with increased capacity for expanded pattern set
    let mut patterns = FxHashMap::with_capacity_and_hasher(50, Default::default());

    // Common patterns for key/value replacements
    let common_patterns = [
        // Whitespace patterns
        (r"\s+", "Multiple whitespace"),
        (r"^\s+|\s+$", "Leading/trailing whitespace"),
        (r"\s", "Any whitespace"),
        (r"\n+", "Multiple newlines"),
        (r"\r\n", "Windows line ending"),
        // Special character patterns
        (r"[^\w\s]", "Non-word, non-space characters"),
        (r"[^a-zA-Z0-9]", "Non-alphanumeric"),
        (r"[^a-zA-Z0-9_]", "Non-alphanumeric except underscore"),
        (
            r"[^a-zA-Z0-9_-]",
            "Non-alphanumeric except underscore and hyphen",
        ),
        // Common JSON key patterns
        (r"[A-Z]", "Uppercase letters"),
        (r"[a-z]", "Lowercase letters"),
        (r"\d+", "Digits"),
        (r"_+", "Multiple underscores"),
        (r"-+", "Multiple hyphens"),
        (r"\.+", "Multiple dots"),
        // Email and URL patterns (common in JSON data)
        (r"@", "At symbol (emails)"),
        (r"\.", "Dot (domains, decimals)"),
        (r"://", "Protocol separator"),
        (r"https?://", "HTTP/HTTPS protocol"),
        // Date/time patterns
        (r"\d{4}-\d{2}-\d{2}", "ISO date (YYYY-MM-DD)"),
        (r"\d{2}:\d{2}:\d{2}", "Time format (HH:MM:SS)"),
        (r"\d{4}/\d{2}/\d{2}", "US date format (YYYY/MM/DD)"),
        (r"\d{2}/\d{2}/\d{4}", "Date format (MM/DD/YYYY)"),
        // UUID patterns
        (
            r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
            "UUID format",
        ),
        (r"[0-9a-fA-F]{32}", "UUID without hyphens"),
        // IP address patterns
        (r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}", "IPv4 address"),
        (r"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}", "IPv6 address"),
        // Naming convention patterns
        (r"[a-z]+([A-Z][a-z]+)*", "camelCase"),
        (r"[a-z]+(_[a-z]+)*", "snake_case"),
        (r"[a-z]+(-[a-z]+)*", "kebab-case"),
        (r"[A-Z]+([A-Z][a-z]+)*", "PascalCase"),
        // Currency patterns
        (r"\$\d+(\.\d{2})?", "USD currency"),
        (r"€\d+(\.\d{2})?", "EUR currency"),
        (r"£\d+(\.\d{2})?", "GBP currency"),
        (r"\d+(\.\d{2})?\s*(USD|EUR|GBP)", "Currency with code"),
        // Version number patterns
        (r"\d+\.\d+\.\d+", "Semantic version"),
        (r"v\d+\.\d+", "Version prefix"),
        // File and path patterns
        (r"\.\w+$", "File extension"),
        (r"/[^/]+", "Path segment"),
        (r"\\[^\\]+", "Windows path segment"),
        // Bracket and quote patterns
        (r"\[.*?\]", "Square brackets with content"),
        (r"\{.*?\}", "Curly braces with content"),
        (r"\(.*?\)", "Parentheses with content"),
        (r#"".*?""#, "Double quoted string"),
        (r"'.*?'", "Single quoted string"),
        // Common user patterns for key/value replacements
        (r"^(user|admin)_", "User/admin prefix"),
        (r"^(User|Admin)_", "User/Admin prefix (capitalized)"),
        (r"(user|admin)_", "User/admin anywhere"),
        (r"(User|Admin)_", "User/Admin anywhere (capitalized)"),
        (r"@example\.com", "Example email domain"),
        (r"@example\.org", "Example org domain"),
        (r"@company\.org", "Company org domain"),
        (r"@company\.com", "Company com domain"),
        (r"_id$", "Trailing _id suffix"),
        (r"_ids$", "Trailing _ids suffix"),
        (r"^id_", "Leading id_ prefix"),
        (r"_at$", "Timestamp suffix (_at)"),
        (r"_on$", "Date suffix (_on)"),
        (r"^created_", "Created prefix"),
        (r"^updated_", "Updated prefix"),
        (r"^deleted_", "Deleted prefix"),
        (r"^is_", "Boolean prefix (is_)"),
        (r"^has_", "Boolean prefix (has_)"),
        (r"^can_", "Boolean prefix (can_)"),
    ];

    for (pattern, _) in &common_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            patterns.insert(*pattern, Arc::new(regex));
        }
    }

    patterns
});

/// Lock-free concurrent hashmap for regex caching using DashMap.
/// Eliminates write-lock contention that existed with RwLock<LruCache>.
static REGEX_CACHE: LazyLock<DashMap<Arc<str>, Arc<Regex>>> =
    LazyLock::new(|| DashMap::with_capacity(GLOBAL_CACHE_CAPACITY));

thread_local! {
    static THREAD_LOCAL_REGEX_CACHE: std::cell::RefCell<FxHashMap<Arc<str>, Arc<Regex>>> =
        std::cell::RefCell::new(FxHashMap::with_capacity_and_hasher(THREAD_LOCAL_CACHE_CAPACITY, Default::default()));
}

/// Evict approximately half the entries from a thread-local cache.
/// Uses alternating retain to preserve ~50% of entries.
#[inline]
fn evict_thread_local_half(cache: &mut FxHashMap<Arc<str>, Arc<Regex>>) {
    let mut keep = true;
    cache.retain(|_, _| {
        keep = !keep;
        keep
    });
}

/// Insert a regex into the thread-local cache, evicting if at capacity.
#[inline]
fn insert_thread_local(pattern: &str, regex: &Arc<Regex>) {
    THREAD_LOCAL_REGEX_CACHE.with(|cache| {
        let mut cache_ref = cache.borrow_mut();
        if cache_ref.len() >= THREAD_LOCAL_CACHE_CAPACITY {
            evict_thread_local_half(&mut cache_ref);
        }
        cache_ref.insert(Arc::from(pattern), Arc::clone(regex));
    });
}

/// Get a cached regex, using Arc<Regex> for O(1) cloning
///
/// Three-tier caching strategy (optimized for both latency and concurrency):
/// 1. Pre-compiled common patterns (static, no allocation)
/// 2. Thread-local cache (lock-free, thread-specific)
/// 3. Global DashMap (lock-free concurrent, shared across threads)
pub(crate) fn get_cached_regex(pattern: &str) -> Result<Arc<Regex>, regex::Error> {
    // TIER 1: Check pre-compiled common patterns (fastest path, no allocation)
    if let Some(regex) = COMMON_REGEX_PATTERNS.get(pattern) {
        return Ok(Arc::clone(regex));
    }

    // TIER 2: Try thread-local cache (fast path, no locks)
    let thread_local_result = THREAD_LOCAL_REGEX_CACHE.with(|cache| {
        let cache_ref = cache.borrow();
        cache_ref.get(pattern).map(Arc::clone)
    });

    if let Some(regex) = thread_local_result {
        return Ok(regex);
    }

    // TIER 3: Try global DashMap cache (lock-free concurrent access)
    if let Some(regex) = REGEX_CACHE.get(pattern) {
        let regex_arc = Arc::clone(regex.value());
        insert_thread_local(pattern, &regex_arc);
        return Ok(regex_arc);
    }

    // NOT FOUND: Compile new regex and cache it
    let regex = Arc::new(Regex::new(pattern)?);

    // Bounded cache: evict one entry if at capacity
    if REGEX_CACHE.len() >= GLOBAL_CACHE_CAPACITY {
        if let Some(entry) = REGEX_CACHE.iter().next() {
            let key_to_remove = entry.key().clone();
            drop(entry);
            REGEX_CACHE.remove(&key_to_remove);
        }
    }

    REGEX_CACHE.insert(Arc::from(pattern), Arc::clone(&regex));
    insert_thread_local(pattern, &regex);

    Ok(regex)
}
