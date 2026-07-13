package io.github.amaye15.jsontoolsrs;

import java.util.concurrent.ConcurrentHashMap;

/**
 * Static, executor-JVM-wide cache of {@link JsonToolsHandle}s keyed by their
 * canonical config JSON ({@link JsonTools#toConfigJson()}), used by the Tier-1 row
 * UDFs ({@code spark.FlattenUDF}, {@code spark.UnflattenUDF}).
 *
 * <p>{@code UDF1} has no destroy/lifecycle hook, and whether Spark shares one
 * deserialized UDF instance across concurrent task threads or deserializes a fresh
 * copy per task is not something to assume either way. A handle owned privately by a
 * UDF instance would either leak one native handle per task forever (if
 * deserialized per task, since nothing would ever call {@code close()}) or need a
 * lock for no reason (if shared). This cache sidesteps both: bounded to the number of
 * distinct configs actually registered in a pipeline (typically a handful), safe to
 * call concurrently with no lock (see {@link JsonToolsHandle}'s own thread-safety
 * note), and deliberately never evicted or closed in normal operation -- cleanup
 * happens for free at executor JVM process exit.
 *
 * <p><b>Do not</b> wire {@code close()} to any plausible-looking lifecycle event
 * (e.g. an executor shutdown listener): other tasks may still be using the same
 * shared handle at that point, and closing it out from under them is a
 * use-after-free race. Tier-2 ({@code spark.BatchTransform}) does not use this cache
 * -- it creates and closes one handle per partition directly, since {@code
 * mapPartitions} already gives it an explicit, safe teardown point.
 *
 * <p>Public because the same pattern is the right choice for anyone writing their own
 * row UDF against this library, not just the two shipped here.
 */
public final class NativeHandleCache {

    private static final ConcurrentHashMap<String, JsonToolsHandle> HANDLES = new ConcurrentHashMap<>();

    private NativeHandleCache() {}

    public static JsonToolsHandle get(String configJson) {
        return HANDLES.computeIfAbsent(configJson, JsonToolsHandle::new);
    }
}
