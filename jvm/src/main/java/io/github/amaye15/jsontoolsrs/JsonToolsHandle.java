package io.github.amaye15.jsontoolsrs;

/**
 * Owns one native {@code JSONTools} handle. Thread-safe: the underlying Rust value
 * has no interior mutability and is immutable after construction (config is fixed at
 * {@link JsonTools#build()} time), so concurrent {@link #execute}/{@link
 * #executeBatch} calls from multiple threads are safe with no external locking.
 *
 * <p>{@link #close()} frees the native handle; using this instance afterward is
 * undefined behavior. See {@link NativeHandleCache} (Tier-1 row UDFs: shared, never
 * closed in normal operation) and {@code spark.BatchTransform} (Tier-2 batched
 * transform: one handle per partition, closed at iterator exhaustion) for the two
 * lifecycle patterns this library uses -- pick whichever matches your use case rather
 * than closing a cache-resident handle out from under concurrently-running callers.
 *
 * <p>The constructor is public so that code holding a raw config JSON string already
 * (e.g. {@code spark.BatchTransform}, or a caller on the other side of the {@code
 * spark._jvm} escape hatch) can build a handle directly, without needing to
 * reconstruct a {@link JsonTools} fluent builder just to get back to the same string.
 */
public final class JsonToolsHandle implements AutoCloseable {

    private final long handle;
    private volatile boolean closed = false;

    public JsonToolsHandle(String configJson) {
        this.handle = JsonToolsNative.nativeCreate(configJson);
    }

    public String execute(String json) {
        // UTF-8 bytes across the boundary: getBytes/new String are
        // JIT-intrinsified, so this is measurably faster than letting JNI do
        // UTF-16 <-> modified-UTF-8 conversions on a String argument. See
        // JsonToolsNative.nativeExecuteBytes.
        byte[] out =
                JsonToolsNative.nativeExecuteBytes(
                        handle, json.getBytes(java.nio.charset.StandardCharsets.UTF_8));
        return new String(out, java.nio.charset.StandardCharsets.UTF_8);
    }

    public String[] executeBatch(String[] jsonArray) {
        byte[][] in = new byte[jsonArray.length][];
        for (int i = 0; i < jsonArray.length; i++) {
            in[i] = jsonArray[i].getBytes(java.nio.charset.StandardCharsets.UTF_8);
        }
        byte[][] out = JsonToolsNative.nativeExecuteBatchBytes(handle, in);
        String[] results = new String[out.length];
        for (int i = 0; i < out.length; i++) {
            results[i] = new String(out[i], java.nio.charset.StandardCharsets.UTF_8);
        }
        return results;
    }

    @Override
    public void close() {
        if (!closed) {
            closed = true;
            JsonToolsNative.nativeDestroy(handle);
        }
    }
}
