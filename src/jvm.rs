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

use jni::errors::ErrorPolicy;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::strings::JNIString;
use jni::sys::{jlong, jobjectArray, jsize, jstring};
use jni::{Env, EnvUnowned};
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

fn throw(env: &mut Env, message: &str) {
    // If throwing itself fails (e.g. OOM), there's nothing more we can do here; the
    // JVM will surface whatever pending-exception state exists on return.
    // `throw_new` needs `AsRef<JNIStr>` args, not plain `&str` (since jni 0.22).
    let _ = env.throw_new(JNIString::from(EXCEPTION_CLASS), JNIString::from(message));
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

/// Error policy for [`EnvOutcome::resolve`]: converts any `Err`/panic from a
/// native entry point's body into a thrown `JsonToolsException`, returning
/// `T::default()` in both cases. `EnvUnowned::with_env` already wraps the body
/// in `catch_unwind` for us, so this only needs to handle the two outcomes.
struct ThrowJsonToolsException;

impl<T: Default> ErrorPolicy<T, JsonToolsError> for ThrowJsonToolsException {
    type Captures<'unowned_env_local, 'native_method>
        = ()
    where
        'unowned_env_local: 'native_method;

    fn on_error<'unowned_env_local, 'native_method>(
        env: &mut Env<'unowned_env_local>,
        _cap: &mut Self::Captures<'unowned_env_local, 'native_method>,
        err: JsonToolsError,
    ) -> jni::errors::Result<T>
    where
        'unowned_env_local: 'native_method,
    {
        throw(env, &err.to_string());
        Ok(T::default())
    }

    fn on_panic<'unowned_env_local, 'native_method>(
        env: &mut Env<'unowned_env_local>,
        _cap: &mut Self::Captures<'unowned_env_local, 'native_method>,
        payload: Box<dyn std::any::Any + Send + 'static>,
    ) -> jni::errors::Result<T>
    where
        'unowned_env_local: 'native_method,
    {
        let message = panic_message(&*payload);
        throw(env, &format!("internal panic in native code: {message}"));
        Ok(T::default())
    }
}

/// Build a configured `JSONTools` handle from a JSON config blob. Returns an opaque
/// pointer (as `jlong`) that must later be passed to `nativeDestroy`.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeCreate<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    config_json: JString<'local>,
) -> jlong {
    env.with_env(|env| -> Result<jlong, JsonToolsError> {
        let config_str = config_json.try_to_string(env)?;
        let tools = build_tools(&config_str)?;
        Ok(Box::into_raw(Box::new(tools)) as jlong)
    })
    .resolve::<ThrowJsonToolsException>()
}

/// Process a single JSON document through the handle built by `nativeCreate`.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecute<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json: JString<'local>,
) -> jstring {
    env.with_env(|env| -> Result<jstring, JsonToolsError> {
        // SAFETY: `handle` is a pointer previously returned by `nativeCreate` and not
        // yet passed to `nativeDestroy` -- the Java-side handle owner enforces this.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let json_str = json.try_to_string(env)?;
        let result = tools.execute(json_str.as_str())?.try_into_single()?;
        Ok(env.new_string(result)?.into_raw())
    })
    .resolve::<ThrowJsonToolsException>()
}

/// Process a batch of JSON documents in one native call, using the existing
/// rayon-parallel batch path (`JSONTools::execute` with a `Vec<String>`) with no
/// additional parallelism logic needed here.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecuteBatch<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json_array: JObjectArray<'local, JString<'local>>,
) -> jobjectArray {
    env.with_env(|env| -> Result<jobjectArray, JsonToolsError> {
        // SAFETY: see nativeExecute.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let len = json_array.len(env)?;

        // JNI's default local-reference-table capacity (~512 on most JVMs) is
        // trivially exhausted by extracting/constructing thousands of JStrings in
        // one call otherwise -- request enough up front for both the input reads
        // and the output writes below.
        env.ensure_local_capacity(2 * len + 16)?;

        let mut inputs: Vec<String> = Vec::with_capacity(len);
        for i in 0..len {
            let jstr = json_array.get_element(env, i)?;
            inputs.push(jstr.try_to_string(env)?);
        }

        let results = tools.execute(inputs)?.try_into_multiple()?;

        let out_array = env.new_object_array(
            results.len() as jsize,
            JNIString::from("java/lang/String"),
            JObject::null(),
        )?;
        for (i, s) in results.into_iter().enumerate() {
            let js = env.new_string(s)?;
            out_array.set_element(env, i, &js)?;
        }
        Ok(out_array.into_raw())
    })
    .resolve::<ThrowJsonToolsException>()
}

/// Free a handle previously returned by `nativeCreate`. Tier-1 (row UDF) handles are
/// deliberately shared/cached for the lifetime of the executor JVM and never call
/// this in normal operation (see `NativeHandleCache.java`); Tier-2 (batched
/// `mapPartitions` transform) calls this once per partition at iterator exhaustion.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeDestroy<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    handle: jlong,
) {
    env.with_env(|_env| -> Result<(), JsonToolsError> {
        if handle != 0 {
            // SAFETY: see nativeExecute; this is the one call site allowed to
            // invalidate the pointer, and callers must not use it afterward.
            unsafe {
                drop(Box::from_raw(handle as *mut JSONTools));
            }
        }
        Ok(())
    })
    .resolve::<ThrowJsonToolsException>()
}
