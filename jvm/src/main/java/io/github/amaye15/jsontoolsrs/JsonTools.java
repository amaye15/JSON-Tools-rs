package io.github.amaye15.jsontoolsrs;

import java.util.ArrayList;
import java.util.List;

/**
 * Fluent builder mirroring the Rust {@code JSONTools} builder
 * ({@code JSONTools::new().flatten().separator("::")...}). Accumulates configuration
 * purely in Java; {@link #build()} serializes it to a small, deterministic JSON blob
 * and hands it to the native core exactly once via {@code nativeCreate}.
 *
 * <p>Unset fields are left out of the serialized config entirely, so the underlying
 * Rust {@code JSONTools::new()} defaults apply -- this class must never hardcode its
 * own default values, to avoid two independently-drifting sources of truth for
 * defaults across language bindings.
 *
 * <pre>{@code
 * try (JsonToolsHandle tools = JsonTools.builder()
 *         .flatten()
 *         .separator("::")
 *         .keyReplacement("r'^admin_'", "")
 *         .removeNulls(true)
 *         .build()) {
 *     String result = tools.execute("{\"admin_name\": \"Jane\", \"age\": null}");
 * }
 * }</pre>
 */
public final class JsonTools {

    private String mode;
    private String separator;
    private Boolean lowercaseKeys;
    private final List<String[]> keyReplacements = new ArrayList<>();
    private final List<String[]> valueReplacements = new ArrayList<>();
    private Boolean removeEmptyStrings;
    private Boolean removeNulls;
    private Boolean removeEmptyObjects;
    private Boolean removeEmptyArrays;
    private Boolean handleKeyCollision;
    private Boolean autoConvertTypes;
    private Integer parallelThreshold;
    private Integer numThreads;
    private Integer nestedParallelThreshold;
    private Integer maxArrayIndex;

    private JsonTools() {}

    public static JsonTools builder() {
        return new JsonTools();
    }

    public JsonTools flatten() {
        this.mode = "flatten";
        return this;
    }

    public JsonTools unflatten() {
        this.mode = "unflatten";
        return this;
    }

    public JsonTools normal() {
        this.mode = "normal";
        return this;
    }

    public JsonTools separator(String separator) {
        this.separator = separator;
        return this;
    }

    public JsonTools lowercaseKeys(boolean value) {
        this.lowercaseKeys = value;
        return this;
    }

    /**
     * Adds a key replacement pattern. Bare patterns match literally; wrap in
     * {@code r'...'} (e.g. {@code "r'^admin_'"}) to match as regex. See the core
     * library's replacement pattern docs for the full {@code r'...'} convention.
     */
    public JsonTools keyReplacement(String find, String replace) {
        this.keyReplacements.add(new String[] {find, replace});
        return this;
    }

    /** Adds a value replacement pattern; same literal/{@code r'...'} convention as {@link #keyReplacement}. */
    public JsonTools valueReplacement(String find, String replace) {
        this.valueReplacements.add(new String[] {find, replace});
        return this;
    }

    public JsonTools removeEmptyStrings(boolean value) {
        this.removeEmptyStrings = value;
        return this;
    }

    public JsonTools removeNulls(boolean value) {
        this.removeNulls = value;
        return this;
    }

    public JsonTools removeEmptyObjects(boolean value) {
        this.removeEmptyObjects = value;
        return this;
    }

    public JsonTools removeEmptyArrays(boolean value) {
        this.removeEmptyArrays = value;
        return this;
    }

    public JsonTools handleKeyCollision(boolean value) {
        this.handleKeyCollision = value;
        return this;
    }

    public JsonTools autoConvertTypes(boolean value) {
        this.autoConvertTypes = value;
        return this;
    }

    public JsonTools parallelThreshold(int threshold) {
        this.parallelThreshold = threshold;
        return this;
    }

    public JsonTools numThreads(int numThreads) {
        this.numThreads = numThreads;
        return this;
    }

    public JsonTools nestedParallelThreshold(int threshold) {
        this.nestedParallelThreshold = threshold;
        return this;
    }

    public JsonTools maxArrayIndex(int max) {
        this.maxArrayIndex = max;
        return this;
    }

    /**
     * Serializes the accumulated configuration to a small, deterministic JSON blob
     * (fixed field order, fields omitted when unset). Deterministic output matters
     * beyond wire format: {@link NativeHandleCache} uses this string as a cache key,
     * so two logically-identical builders must produce byte-identical JSON.
     */
    public String toConfigJson() {
        StringBuilder json = new StringBuilder();
        json.append('{');
        boolean[] first = {true};
        writeStringField(json, first, "mode", mode);
        writeStringField(json, first, "separator", separator);
        writeBooleanField(json, first, "lowercase_keys", lowercaseKeys);
        writePairListField(json, first, "key_replacements", keyReplacements);
        writePairListField(json, first, "value_replacements", valueReplacements);
        writeBooleanField(json, first, "remove_empty_strings", removeEmptyStrings);
        writeBooleanField(json, first, "remove_nulls", removeNulls);
        writeBooleanField(json, first, "remove_empty_objects", removeEmptyObjects);
        writeBooleanField(json, first, "remove_empty_arrays", removeEmptyArrays);
        writeBooleanField(json, first, "handle_key_collision", handleKeyCollision);
        writeBooleanField(json, first, "auto_convert_types", autoConvertTypes);
        writeIntField(json, first, "parallel_threshold", parallelThreshold);
        writeIntField(json, first, "num_threads", numThreads);
        writeIntField(json, first, "nested_parallel_threshold", nestedParallelThreshold);
        writeIntField(json, first, "max_array_index", maxArrayIndex);
        json.append('}');
        return json.toString();
    }

    /**
     * Builds the native handle for this configuration. The caller owns the returned
     * handle and is responsible for {@link JsonToolsHandle#close() closing} it
     * (try-with-resources is the simplest way) unless it's being registered into
     * {@link NativeHandleCache}, which owns cached handles for the life of the JVM.
     */
    public JsonToolsHandle build() {
        return new JsonToolsHandle(toConfigJson());
    }

    private static void writeComma(StringBuilder json, boolean[] first) {
        if (!first[0]) {
            json.append(',');
        }
        first[0] = false;
    }

    private static void writeStringField(StringBuilder json, boolean[] first, String key, String value) {
        if (value == null) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(':');
        writeJsonString(json, value);
    }

    private static void writeBooleanField(StringBuilder json, boolean[] first, String key, Boolean value) {
        if (value == null) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(':').append(value.booleanValue());
    }

    private static void writeIntField(StringBuilder json, boolean[] first, String key, Integer value) {
        if (value == null) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(':').append(value.intValue());
    }

    private static void writePairListField(
            StringBuilder json, boolean[] first, String key, List<String[]> pairs) {
        if (pairs.isEmpty()) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(":[");
        for (int i = 0; i < pairs.size(); i++) {
            if (i > 0) {
                json.append(',');
            }
            String[] pair = pairs.get(i);
            json.append('[');
            writeJsonString(json, pair[0]);
            json.append(',');
            writeJsonString(json, pair[1]);
            json.append(']');
        }
        json.append(']');
    }

    private static void writeJsonString(StringBuilder json, String value) {
        json.append('"');
        for (int i = 0; i < value.length(); i++) {
            char c = value.charAt(i);
            switch (c) {
                case '"':
                    json.append("\\\"");
                    break;
                case '\\':
                    json.append("\\\\");
                    break;
                case '\n':
                    json.append("\\n");
                    break;
                case '\r':
                    json.append("\\r");
                    break;
                case '\t':
                    json.append("\\t");
                    break;
                default:
                    if (c < 0x20) {
                        json.append(String.format("\\u%04x", (int) c));
                    } else {
                        json.append(c);
                    }
            }
        }
        json.append('"');
    }
}
