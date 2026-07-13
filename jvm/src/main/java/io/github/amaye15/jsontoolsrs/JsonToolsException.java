package io.github.amaye15.jsontoolsrs;

/**
 * Thrown for any error surfaced by the native json-tools-rs core, and for any panic
 * caught at the JNI boundary. The message is the Rust error's formatted display text
 * (which embeds a machine-readable {@code [E00x]} code), matching the message format
 * already used by this library's Python bindings.
 */
public class JsonToolsException extends RuntimeException {

    private static final long serialVersionUID = 1L;

    public JsonToolsException(String message) {
        super(message);
    }
}
