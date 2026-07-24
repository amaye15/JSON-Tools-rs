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

    /**
     * Processes a single JSON document; throws {@link JsonToolsException} on error.
     * Takes and returns UTF-8 bytes: the {@code String.getBytes(UTF_8)} /
     * {@code new String(bytes, UTF_8)} conversions on the Java side are
     * JIT-intrinsified, making the JNI crossing two plain array copies instead of
     * the UTF-16 &lt;-&gt; modified-UTF-8 conversions a String signature pays.
     */
    static native byte[] nativeExecuteBytes(long handle, byte[] json);

    /**
     * Processes a batch of JSON documents in one native call, UTF-8 bytes per
     * element (see {@link #nativeExecuteBytes}). Throws {@link JsonToolsException}
     * if any element in the batch fails to process -- the whole batch is rejected
     * as a unit (see {@code spark.BatchTransform}'s batch-failure isolation for
     * how callers recover the offending row).
     */
    static native byte[][] nativeExecuteBatchBytes(long handle, byte[][] jsonArray);

    /** Frees a handle previously returned by {@link #nativeCreate}. */
    static native void nativeDestroy(long handle);
}
