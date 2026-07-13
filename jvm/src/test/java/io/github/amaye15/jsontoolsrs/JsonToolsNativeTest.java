package io.github.amaye15.jsontoolsrs;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

/** Direct round-trip tests against the native bridge, bypassing the fluent builder. */
class JsonToolsNativeTest {

    @Test
    void createExecuteAndDestroyRoundTrip() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            String result = JsonToolsNative.nativeExecute(handle, "{\"user\":{\"name\":\"John\"}}");
            assertEquals("{\"user.name\":\"John\"}", result);
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void executeBatchProcessesAllElements() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            String[] results = JsonToolsNative.nativeExecuteBatch(
                    handle,
                    new String[] {
                        "{\"a\":{\"b\":1}}", "{\"c\":{\"d\":2}}", "{\"e\":{\"f\":3}}"
                    });
            assertArrayEquals(
                    new String[] {"{\"a.b\":1}", "{\"c.d\":2}", "{\"e.f\":3}"}, results);
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void invalidJsonThrowsJsonToolsException() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            assertThrows(
                    JsonToolsException.class,
                    () -> JsonToolsNative.nativeExecute(handle, "not valid json"));
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void missingModeThrowsOnExecute() {
        long handle = JsonToolsNative.nativeCreate("{}");
        try {
            assertThrows(
                    JsonToolsException.class,
                    () -> JsonToolsNative.nativeExecute(handle, "{\"a\":1}"));
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void unknownConfigFieldThrowsOnCreate() {
        assertThrows(
                JsonToolsException.class,
                () -> JsonToolsNative.nativeCreate("{\"not_a_real_field\":true}"));
    }
}
