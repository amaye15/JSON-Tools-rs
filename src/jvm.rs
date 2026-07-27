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
use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::strings::JNIString;
use jni::sys::{jbyteArray, jlong, jobjectArray, jsize};
use jni::{Env, EnvUnowned};

use crate::builder::JSONTools;
use crate::config_json::build_tools;
use crate::error::JsonToolsError;

/// Binary name (slashes, not dots) of the Java exception thrown for any error or panic.
const EXCEPTION_CLASS: &str = "io/github/amaye15/jsontoolsrs/JsonToolsException";

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
///
/// The Java caller passes UTF-8 bytes
/// (`String.getBytes(UTF_8)`) and receives UTF-8 bytes back (`new String(bytes,
/// UTF_8)`). Those two Java-side conversions are JIT-intrinsified, and the JNI
/// transfer becomes two plain array copies, replacing `GetStringUTFChars`/
/// `NewStringUTF`'s UTF-16 <-> modified-UTF-8 conversions (plus the cesu8
/// re-conversions on the Rust side) that measured ~1.2us of a ~2-3us call on a
/// medium document -- see `JsonToolsHandle.execute` for the Java half.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecuteBytes<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json: JByteArray<'local>,
) -> jbyteArray {
    env.with_env(|env| -> Result<jbyteArray, JsonToolsError> {
        // SAFETY: `handle` is a pointer previously returned by `nativeCreate` and not
        // yet passed to `nativeDestroy` -- the Java-side handle owner enforces this.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let bytes = env.convert_byte_array(&json)?;
        let json_str = String::from_utf8(bytes).map_err(|e| {
            JsonToolsError::input_validation_error(format!("input is not valid UTF-8: {e}"))
        })?;
        let result = tools.execute(json_str.as_str())?.try_into_single()?;
        Ok(env.byte_array_from_slice(result.as_bytes())?.into_raw())
    })
    .resolve::<ThrowJsonToolsException>()
}

/// Process a batch of JSON documents in one native call, using the existing
/// rayon-parallel batch path (`JSONTools::execute` with a `Vec<String>`) with no
/// additional parallelism logic needed here. UTF-8 `byte[]` elements in and out
/// for the same reason as `nativeExecuteBytes`, applied per element.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_io_github_amaye15_jsontoolsrs_JsonToolsNative_nativeExecuteBatchBytes<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>,
    handle: jlong,
    json_array: JObjectArray<'local, JByteArray<'local>>,
) -> jobjectArray {
    env.with_env(|env| -> Result<jobjectArray, JsonToolsError> {
        // SAFETY: see nativeExecuteBytes.
        let tools = unsafe { &*(handle as *const JSONTools) };
        let len = json_array.len(env)?;

        // JNI's default local-reference-table capacity (~512 on most JVMs) is
        // trivially exhausted by extracting/constructing thousands of local
        // array refs in one call otherwise -- request enough up front for both
        // the input reads and the output writes below.
        env.ensure_local_capacity(2 * len + 16)?;

        let mut inputs: Vec<String> = Vec::with_capacity(len);
        for i in 0..len {
            let jarr = json_array.get_element(env, i)?;
            let bytes = env.convert_byte_array(&jarr)?;
            inputs.push(String::from_utf8(bytes).map_err(|e| {
                JsonToolsError::input_validation_error(format!("input is not valid UTF-8: {e}"))
            })?);
        }

        let results = tools.execute(inputs)?.try_into_multiple()?;

        let out_array = env.new_object_array(
            results.len() as jsize,
            JNIString::from("[B"),
            JObject::null(),
        )?;
        for (i, s) in results.into_iter().enumerate() {
            let jb = env.byte_array_from_slice(s.as_bytes())?;
            out_array.set_element(env, i, &jb)?;
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
            // SAFETY: see nativeExecuteBytes; this is the one call site allowed to
            // invalidate the pointer, and callers must not use it afterward.
            unsafe {
                drop(Box::from_raw(handle as *mut JSONTools));
            }
        }
        Ok(())
    })
    .resolve::<ThrowJsonToolsException>()
}
