package io.github.amaye15.jsontoolsrs;

/**
 * Direct 1:1 mirror of the JNI entry points exported by {@code src/jvm.rs}. Package
 * private -- use {@link JsonTools} / {@link JsonToolsHandle} instead.
 */
final class JsonToolsNative {

    static {
        NativeLibraryLoader.load("json_tools_rs");
    }

    private JsonToolsNative() {}

    /**
     * Builds a configured handle from a JSON config blob (see {@link
     * JsonTools#toConfigJson()}). The returned handle must later be freed via {@link
     * #nativeDestroy}, and throws {@link JsonToolsException} on invalid config.
     */
    static native long nativeCreate(String configJson);

    /** Processes a single JSON document; throws {@link JsonToolsException} on error. */
    static native String nativeExecute(long handle, String json);

    /**
     * Processes a batch of JSON documents in one native call. Throws {@link
     * JsonToolsException} if any element in the batch fails to process -- the whole
     * batch is rejected as a unit (see {@code spark.BatchTransform}'s batch-failure
     * isolation for how callers recover the offending row).
     */
    static native String[] nativeExecuteBatch(long handle, String[] jsonArray);

    /** Frees a handle previously returned by {@link #nativeCreate}. */
    static native void nativeDestroy(long handle);
}
