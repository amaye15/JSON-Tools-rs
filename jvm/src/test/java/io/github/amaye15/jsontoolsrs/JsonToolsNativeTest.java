package io.github.amaye15.jsontoolsrs;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.nio.charset.StandardCharsets;

import org.junit.jupiter.api.Test;

/** Direct round-trip tests against the native bridge, bypassing the fluent builder. */
class JsonToolsNativeTest {

    private static byte[] utf8(String s) {
        return s.getBytes(StandardCharsets.UTF_8);
    }

    private static String str(byte[] bytes) {
        return new String(bytes, StandardCharsets.UTF_8);
    }

    @Test
    void createExecuteAndDestroyRoundTrip() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            byte[] result =
                    JsonToolsNative.nativeExecuteBytes(handle, utf8("{\"user\":{\"name\":\"John\"}}"));
            assertEquals("{\"user.name\":\"John\"}", str(result));
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void executeBatchProcessesAllElements() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            byte[][] results = JsonToolsNative.nativeExecuteBatchBytes(
                    handle,
                    new byte[][] {
                        utf8("{\"a\":{\"b\":1}}"), utf8("{\"c\":{\"d\":2}}"), utf8("{\"e\":{\"f\":3}}")
                    });
            String[] decoded = new String[results.length];
            for (int i = 0; i < results.length; i++) {
                decoded[i] = str(results[i]);
            }
            assertArrayEquals(
                    new String[] {"{\"a.b\":1}", "{\"c.d\":2}", "{\"e.f\":3}"}, decoded);
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
                    () -> JsonToolsNative.nativeExecuteBytes(handle, utf8("not valid json")));
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
                    () -> JsonToolsNative.nativeExecuteBytes(handle, utf8("{\"a\":1}")));
        } finally {
            JsonToolsNative.nativeDestroy(handle);
        }
    }

    @Test
    void invalidUtf8InputThrowsJsonToolsException() {
        long handle = JsonToolsNative.nativeCreate("{\"mode\":\"flatten\"}");
        try {
            // 0xFF is never valid in UTF-8 -- must surface as JsonToolsException,
            // not a panic or garbage output.
            assertThrows(
                    JsonToolsException.class,
                    () -> JsonToolsNative.nativeExecuteBytes(
                            handle, new byte[] {'{', (byte) 0xFF, '}'}));
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
