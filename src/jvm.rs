//! JNI bindings for JVM (Java/Scala) integration, primarily for use as Spark UDFs.
//!
//! Unlike `python.rs`'s `Mutex<JSONTools>` + `mem::take`-per-builder-call pattern
//! (needed there because Python mutates a handle step by step), the JVM side never
//! mutates a `JSONTools` after construction: the Java-side fluent builder assembles
//! its whole configuration into one JSON blob and hands it over exactly once via
//! `nativeCreate`. `JSONTools` has no interior mutability and is `Send + Sync`, so
//! the boxed handle returned here is immutable and safe to call concurrently from
//! multiple threads with no lock.
//!
//! Every exported `Java_...` entry point is routed through [`guard`], which catches
//! panics and converts any error into a thrown `JsonToolsException` -- an unwinding
//! panic that crosses the JNI boundary into JVM frames is undefined behavior, so
//! every single entry point must go through it, not just the ones that look risky.

use std::panic::{catch_unwind, AssertUnwindSafe};

use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::{jlong, jobjectArray, jsize, jstring};
use jni::JNIEnv;
use serde::Deserialize;

use crate::builder::JSONTools;
use crate::config::{
    BooleanConversionConfig, DateConversionConfig, NullConversionConfig, NumberConversionConfig,
};
use crate::error::JsonToolsError;

/// Binary name (slashes, not dots) of the Java exception thrown for any error or panic.
const EXCEPTION_CLASS: &str = "io/github/amaye15/jsontoolsrs/JsonToolsException";

/// Mirrors `JSONTools`'s builder options. Every field is optional so that a field
/// left unset by the Java-side builder falls through to `JSONTools::new()`'s own
/// defaults -- this keeps `builder.rs` the single source of truth for defaults
/// across every language binding, rather than duplicating them here.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
struct JvmConfig {
    mode: Option<String>,
    separator: Option<String>,
    lowercase_keys: Option<bool>,
    #[serde(default)]
    key_replacements: Vec<(String, String)>,
    #[serde(default)]
    value_replacements: Vec<(String, String)>,
    remove_empty_strings: Option<bool>,
    remove_nulls: Option<bool>,
    remove_empty_objects: Option<bool>,
    remove_empty_arrays: Option<bool>,
    handle_key_collision: Option<bool>,
    auto_convert_types: Option<bool>,
    convert_dates: Option<bool>,
    date_conversion_config: Option<JvmDateConversionConfig>,
    convert_nulls: Option<bool>,
    null_conversion_config: Option<JvmNullConversionConfig>,
    convert_booleans: Option<bool>,
    boolean_conversion_config: Option<JvmBooleanConversionConfig>,
    convert_numbers: Option<bool>,
    number_conversion_config: Option<JvmNumberConversionConfig>,
    parallel_threshold: Option<usize>,
    num_threads: Option<usize>,
    nested_parallel_threshold: Option<usize>,
    max_array_index: Option<usize>,
}

/// Per-category customization mirrors of `DateConversionConfig`/etc. -- kept as
/// separate wire types (rather than deserializing the public config structs
/// directly) since `#[non_exhaustive]` blocks constructing those outside this
/// crate, and because every field here is optional (unset = "don't override this
/// knob"), unlike the public structs' plain bools.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
struct JvmDateConversionConfig {
    normalize_to_utc: Option<bool>,
    assume_utc_for_naive: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
struct JvmNullConversionConfig {
    #[serde(default)]
    extra_tokens: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
struct JvmBooleanConversionConfig {
    #[serde(default)]
    extra_true_tokens: Vec<String>,
    #[serde(default)]
    extra_false_tokens: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
struct JvmNumberConversionConfig {
    currency: Option<bool>,
    percent: Option<bool>,
    basis_points: Option<bool>,
    suffixes: Option<bool>,
    fractions: Option<bool>,
    radix: Option<bool>,
}

fn build_tools(config_json: &str) -> Result<JSONTools, JsonToolsError> {
    let config: JvmConfig = serde_json::from_str(config_json).map_err(|e| {
        JsonToolsError::input_validation_error(format!("invalid JVM config JSON: {e}"))
    })?;

    let mut tools = JSONTools::new();
    tools = match config.mode.as_deref() {
        Some("flatten") => tools.flatten(),
        Some("unflatten") => tools.unflatten(),
        Some("normal") => tools.normal(),
        Some(other) => {
            return Err(JsonToolsError::configuration_error(format!(
                "unknown mode '{other}': expected 'flatten', 'unflatten', or 'normal'"
            )));
        }
        None => tools,
    };
    if let Some(separator) = config.separator {
        tools = tools.separator(separator);
    }
    if let Some(v) = config.lowercase_keys {
        tools = tools.lowercase_keys(v);
    }
    for (find, replace) in config.key_replacements {
        tools = tools.key_replacement(find, replace);
    }
    for (find, replace) in config.value_replacements {
        tools = tools.value_replacement(find, replace);
    }
    if let Some(v) = config.remove_empty_strings {
        tools = tools.remove_empty_strings(v);
    }
    if let Some(v) = config.remove_nulls {
        tools = tools.remove_nulls(v);
    }
    if let Some(v) = config.remove_empty_objects {
        tools = tools.remove_empty_objects(v);
    }
    if let Some(v) = config.remove_empty_arrays {
        tools = tools.remove_empty_arrays(v);
    }
    if let Some(v) = config.handle_key_collision {
        tools = tools.handle_key_collision(v);
    }
    if let Some(v) = config.auto_convert_types {
        tools = tools.auto_convert_types(v);
    }
    // Nested customization applied first, then the top-level bool -- the bool only
    // ever touches `enabled`, so applying it last preserves customization already
    // set via the nested config block (same ordering principle as the Rust/Python
    // builders' `_config` methods).
    if let Some(date_cfg) = config.date_conversion_config {
        let mut cfg = DateConversionConfig::new();
        if let Some(v) = date_cfg.normalize_to_utc {
            cfg = cfg.normalize_to_utc(v);
        }
        if let Some(v) = date_cfg.assume_utc_for_naive {
            cfg = cfg.assume_utc_for_naive(v);
        }
        tools = tools.convert_dates_config(cfg);
    }
    if let Some(v) = config.convert_dates {
        tools = tools.convert_dates(v);
    }
    if let Some(null_cfg) = config.null_conversion_config {
        let mut cfg = NullConversionConfig::new();
        for token in null_cfg.extra_tokens {
            cfg = cfg.add_extra_token(token);
        }
        tools = tools.convert_nulls_config(cfg);
    }
    if let Some(v) = config.convert_nulls {
        tools = tools.convert_nulls(v);
    }
    if let Some(bool_cfg) = config.boolean_conversion_config {
        let mut cfg = BooleanConversionConfig::new();
        for token in bool_cfg.extra_true_tokens {
            cfg = cfg.add_extra_true_token(token);
        }
        for token in bool_cfg.extra_false_tokens {
            cfg = cfg.add_extra_false_token(token);
        }
        tools = tools.convert_booleans_config(cfg);
    }
    if let Some(v) = config.convert_booleans {
        tools = tools.convert_booleans(v);
    }
    if let Some(num_cfg) = config.number_conversion_config {
        let mut cfg = NumberConversionConfig::new();
        if let Some(v) = num_cfg.currency {
            cfg = cfg.currency(v);
        }
        if let Some(v) = num_cfg.percent {
            cfg = cfg.percent(v);
        }
        if let Some(v) = num_cfg.basis_points {
            cfg = cfg.basis_points(v);
        }
        if let Some(v) = num_cfg.suffixes {
            cfg = cfg.suffixes(v);
        }
        if let Some(v) = num_cfg.fractions {
            cfg = cfg.fractions(v);
        }
        if let Some(v) = num_cfg.radix {
            cfg = cfg.radix(v);
        }
        tools = tools.convert_numbers_config(cfg);
    }
    if let Some(v) = config.convert_numbers {
        tools = tools.convert_numbers(v);
    }
    if let Some(v) = config.parallel_threshold {
        tools = tools.parallel_threshold(v);
    }
    if let Some(v) = config.num_threads {
        tools = tools.num_threads(Some(v));
    }
    if let Some(v) = config.nested_parallel_threshold {
        tools = tools.nested_parallel_threshold(v);
    }
    if let Some(v) = config.max_array_index {
        tools = tools.max_array_index(v);
    }
    Ok(tools)
}

fn throw(env: &mut JNIEnv, message: &str) {
    // If throwing itself fails (e.g. OOM), there's nothing more we can do here; the
    // JVM will surface whatever pending-exception state exists on return.
    let _ = env.throw_new(EXCEPTION_CLASS, message);
}

fn panic_message(panic: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

/// Run `body`, catching panics and converting any `Err`/panic into a thrown
/// `JsonToolsException`, returning `default` in both cases.
fn guard<'local, R>(
    env: &mut JNIEnv<'local>,
    default: R,
    body: impl FnOnce(&mut JNIEnv<'local>) -> Result<R, JsonToolsError>,
) -> R {
    match catch_unwind(AssertUnwindSafe(|| body(env))) {
        Ok(Ok(value)) => value,
        Ok(Err(e)) => {
            throw(env, &e.to_string());
            default
        }
        Err(panic) => {
            let message = panic_message(&*panic);
            throw(env, &format!("internal panic in native code: {message}"));
            default
        }
    }
}

/// Build a configured `JSONTools` handle from a JSON config blob. Returns an opaque
/// pointer (as `jlong`) that must later be passed to `nativeDestroy`.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeCreate<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    config_json: JString<'local>,
) -> jlong {
    guard(&mut env, 0i64, |env| {
        let config_str: String = env.get_string(&config_json)?.into();
        let tools = build_tools(&config_str)?;
        Ok(Box::into_raw(Box::new(tools)) as jlong)
    })
}

/// Process a single JSON document through the handle built by `nativeCreate`.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecute<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json: JString<'local>,
) -> jstring {
    guard(&mut env, std::ptr::null_mut(), |env| {
        // SAFETY: `handle` is a pointer previously returned by `nativeCreate` and not
        // yet passed to `nativeDestroy` -- the Java-side handle owner enforces this.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let json_str: String = env.get_string(&json)?.into();
        let result = tools.execute(json_str.as_str())?.try_into_single()?;
        Ok(env.new_string(result)?.into_raw())
    })
}

/// Process a batch of JSON documents in one native call, using the existing
/// rayon-parallel batch path (`JSONTools::execute` with a `Vec<String>`) with no
/// additional parallelism logic needed here.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecuteBatch<
    'local,
>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json_array: JObjectArray<'local>,
) -> jobjectArray {
    guard(&mut env, std::ptr::null_mut(), |env| {
        // SAFETY: see nativeExecute.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let len = env.get_array_length(&json_array)?.max(0) as usize;

        // JNI's default local-reference-table capacity (~512 on most JVMs) is
        // trivially exhausted by extracting/constructing thousands of JStrings in
        // one call otherwise -- request enough up front for both the input reads
        // and the output writes below.
        env.ensure_local_capacity(2 * (len as jsize) + 16)?;

        let mut inputs: Vec<String> = Vec::with_capacity(len);
        for i in 0..len {
            let element = env.get_object_array_element(&json_array, i as jsize)?;
            let jstr = JString::from(element);
            inputs.push(env.get_string(&jstr)?.into());
        }

        let results = tools.execute(inputs)?.try_into_multiple()?;

        let out_array =
            env.new_object_array(results.len() as jsize, "java/lang/String", JObject::null())?;
        for (i, s) in results.into_iter().enumerate() {
            let js = env.new_string(s)?;
            env.set_object_array_element(&out_array, i as jsize, &js)?;
        }
        Ok(out_array.into_raw())
    })
}

/// Free a handle previously returned by `nativeCreate`. Tier-1 (row UDF) handles are
/// deliberately shared/cached for the lifetime of the executor JVM and never call
/// this in normal operation (see `NativeHandleCache.java`); Tier-2 (batched
/// `mapPartitions` transform) calls this once per partition at iterator exhaustion.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeDestroy<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
) {
    guard(&mut env, (), |_env| {
        if handle != 0 {
            // SAFETY: see nativeExecute; this is the one call site allowed to
            // invalidate the pointer, and callers must not use it afterward.
            unsafe {
                drop(Box::from_raw(handle as *mut JSONTools));
            }
        }
        Ok(())
    })
}
