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
    private final List<String> keyExclusions = new ArrayList<>();
    private final List<String> valueExclusions = new ArrayList<>();
    private Boolean removeEmptyStrings;
    private Boolean removeNulls;
    private Boolean removeEmptyObjects;
    private Boolean removeEmptyArrays;
    private Boolean handleKeyCollision;
    private Boolean autoConvertTypes;
    private Boolean convertDates;
    private Boolean dateNormalizeToUtc;
    private Boolean dateAssumeUtcForNaive;
    private Boolean convertNulls;
    private final List<String> nullExtraTokens = new ArrayList<>();
    private Boolean convertBooleans;
    private final List<String> booleanExtraTrueTokens = new ArrayList<>();
    private final List<String> booleanExtraFalseTokens = new ArrayList<>();
    private Boolean convertNumbers;
    private Boolean numberCurrency;
    private Boolean numberPercent;
    private Boolean numberBasisPoints;
    private Boolean numberSuffixes;
    private Boolean numberFractions;
    private Boolean numberRadix;
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

    /**
     * Excludes any key (and its entire value/subtree) whose name contains
     * {@code pattern}. Literal substring match by default; wrap in {@code r'...'}
     * for regex, matching {@link #keyReplacement}'s convention. Additive -- call
     * once per keyword to exclude multiple. Matching a container key drops its
     * entire subtree.
     */
    public JsonTools excludeKey(String pattern) {
        this.keyExclusions.add(pattern);
        return this;
    }

    /**
     * Drops a key-value pair whose value contains {@code pattern}. Literal substring
     * match by default; wrap in {@code r'...'} for regex, matching {@link
     * #excludeKey}'s convention. Additive -- call once per pattern to exclude
     * multiple. Only applies to scalar leaf values (strings/numbers/booleans/null).
     */
    public JsonTools excludeValue(String pattern) {
        this.valueExclusions.add(pattern);
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

    /**
     * Enables all four type-conversion categories (dates, nulls, booleans, numbers)
     * with default behavior. Equivalent to calling {@link #convertDates},
     * {@link #convertNulls}, {@link #convertBooleans}, and {@link #convertNumbers}
     * all with the same value -- only flips each category's own on/off switch,
     * preserving any per-category customization already configured via the other
     * {@code convert*}/{@code *Extra*Token}/{@code number*} methods. For independent
     * control or customization of a single category, use the {@code convert*}
     * methods directly instead of this one.
     */
    public JsonTools autoConvertTypes(boolean value) {
        this.autoConvertTypes = value;
        return this;
    }

    /**
     * Enables or disables date/datetime string conversion independently of the other
     * type-conversion categories. See {@link #dateNormalizeToUtc} and
     * {@link #dateAssumeUtcForNaive} to customize its behavior.
     */
    public JsonTools convertDates(boolean value) {
        this.convertDates = value;
        return this;
    }

    /**
     * Configures whether recognized dates/datetimes are normalized to UTC (default:
     * {@code true}). When {@code false}, a recognized date is left unchanged but is
     * still protected from being misread as a number.
     */
    public JsonTools dateNormalizeToUtc(boolean value) {
        this.dateNormalizeToUtc = value;
        return this;
    }

    /**
     * Configures whether timezone-less datetimes (e.g. {@code "2024-01-15T10:30:00"})
     * are assumed to be UTC and get a {@code Z} appended (default: {@code true}).
     * When {@code false}, naive datetimes are left unchanged.
     */
    public JsonTools dateAssumeUtcForNaive(boolean value) {
        this.dateAssumeUtcForNaive = value;
        return this;
    }

    /**
     * Enables or disables null-string conversion independently of the other
     * type-conversion categories. See {@link #nullExtraToken} to recognize
     * additional tokens.
     */
    public JsonTools convertNulls(boolean value) {
        this.convertNulls = value;
        return this;
    }

    /**
     * Adds an additional string to recognize as null, beyond the built-in list
     * ({@code "null"}, {@code "NULL"}, {@code "nil"}, {@code "none"}, {@code "N/A"},
     * {@code "NA"}, etc.). Additive only -- repeatable, matches
     * {@link #keyReplacement}'s one-call-per-item idiom.
     */
    public JsonTools nullExtraToken(String token) {
        this.nullExtraTokens.add(token);
        return this;
    }

    /**
     * Enables or disables boolean-string conversion independently of the other
     * type-conversion categories. See {@link #booleanExtraTrueToken} and
     * {@link #booleanExtraFalseToken} to recognize additional tokens.
     */
    public JsonTools convertBooleans(boolean value) {
        this.convertBooleans = value;
        return this;
    }

    /**
     * Adds an additional string to recognize as {@code true}, beyond the built-in
     * list ({@code "true"}, {@code "yes"}, {@code "on"}, {@code "y"}, etc.).
     * Additive only -- repeatable.
     */
    public JsonTools booleanExtraTrueToken(String token) {
        this.booleanExtraTrueTokens.add(token);
        return this;
    }

    /**
     * Adds an additional string to recognize as {@code false}, beyond the built-in
     * list ({@code "false"}, {@code "no"}, {@code "off"}, {@code "n"}, etc.).
     * Additive only -- repeatable.
     */
    public JsonTools booleanExtraFalseToken(String token) {
        this.booleanExtraFalseTokens.add(token);
        return this;
    }

    /**
     * Enables or disables numeric-string conversion independently of the other
     * type-conversion categories. Plain integers/decimals, scientific notation, and
     * thousands-separator cleanup are always applied when enabled; the remaining
     * sub-formats can each be disabled independently via {@link #numberCurrency},
     * {@link #numberPercent}, {@link #numberBasisPoints}, {@link #numberSuffixes},
     * {@link #numberFractions}, and {@link #numberRadix}.
     */
    public JsonTools convertNumbers(boolean value) {
        this.convertNumbers = value;
        return this;
    }

    /** Configures currency symbol/code/credit-debit-suffix stripping (default: {@code true}). */
    public JsonTools numberCurrency(boolean value) {
        this.numberCurrency = value;
        return this;
    }

    /** Configures {@code %}/permille/per-ten-thousand suffix parsing (default: {@code true}). */
    public JsonTools numberPercent(boolean value) {
        this.numberPercent = value;
        return this;
    }

    /** Configures text basis-point suffix parsing, e.g. {@code "25bps"} (default: {@code true}). */
    public JsonTools numberBasisPoints(boolean value) {
        this.numberBasisPoints = value;
        return this;
    }

    /** Configures K/M/B/T magnitude suffix parsing (default: {@code true}). */
    public JsonTools numberSuffixes(boolean value) {
        this.numberSuffixes = value;
        return this;
    }

    /** Configures fraction parsing, e.g. {@code "1/2"} (default: {@code true}). */
    public JsonTools numberFractions(boolean value) {
        this.numberFractions = value;
        return this;
    }

    /** Configures hex/binary/octal literal parsing (default: {@code true}). */
    public JsonTools numberRadix(boolean value) {
        this.numberRadix = value;
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
        writeStringListField(json, first, "key_exclusions", keyExclusions);
        writeStringListField(json, first, "value_exclusions", valueExclusions);
        writeBooleanField(json, first, "remove_empty_strings", removeEmptyStrings);
        writeBooleanField(json, first, "remove_nulls", removeNulls);
        writeBooleanField(json, first, "remove_empty_objects", removeEmptyObjects);
        writeBooleanField(json, first, "remove_empty_arrays", removeEmptyArrays);
        writeBooleanField(json, first, "handle_key_collision", handleKeyCollision);
        writeBooleanField(json, first, "auto_convert_types", autoConvertTypes);
        writeBooleanField(json, first, "convert_dates", convertDates);
        writeObjectField(json, first, "date_conversion_config", dateConversionConfigJson());
        writeBooleanField(json, first, "convert_nulls", convertNulls);
        writeObjectField(json, first, "null_conversion_config", nullConversionConfigJson());
        writeBooleanField(json, first, "convert_booleans", convertBooleans);
        writeObjectField(json, first, "boolean_conversion_config", booleanConversionConfigJson());
        writeBooleanField(json, first, "convert_numbers", convertNumbers);
        writeObjectField(json, first, "number_conversion_config", numberConversionConfigJson());
        writeIntField(json, first, "parallel_threshold", parallelThreshold);
        writeIntField(json, first, "num_threads", numThreads);
        writeIntField(json, first, "nested_parallel_threshold", nestedParallelThreshold);
        writeIntField(json, first, "max_array_index", maxArrayIndex);
        json.append('}');
        return json.toString();
    }

    /** Builds the {@code date_conversion_config} nested object, or {@code null} if unset. */
    private String dateConversionConfigJson() {
        if (dateNormalizeToUtc == null && dateAssumeUtcForNaive == null) {
            return null;
        }
        StringBuilder sub = new StringBuilder("{");
        boolean[] subFirst = {true};
        writeBooleanField(sub, subFirst, "normalize_to_utc", dateNormalizeToUtc);
        writeBooleanField(sub, subFirst, "assume_utc_for_naive", dateAssumeUtcForNaive);
        return sub.append('}').toString();
    }

    /** Builds the {@code null_conversion_config} nested object, or {@code null} if unset. */
    private String nullConversionConfigJson() {
        if (nullExtraTokens.isEmpty()) {
            return null;
        }
        StringBuilder sub = new StringBuilder("{");
        boolean[] subFirst = {true};
        writeStringListField(sub, subFirst, "extra_tokens", nullExtraTokens);
        return sub.append('}').toString();
    }

    /** Builds the {@code boolean_conversion_config} nested object, or {@code null} if unset. */
    private String booleanConversionConfigJson() {
        if (booleanExtraTrueTokens.isEmpty() && booleanExtraFalseTokens.isEmpty()) {
            return null;
        }
        StringBuilder sub = new StringBuilder("{");
        boolean[] subFirst = {true};
        writeStringListField(sub, subFirst, "extra_true_tokens", booleanExtraTrueTokens);
        writeStringListField(sub, subFirst, "extra_false_tokens", booleanExtraFalseTokens);
        return sub.append('}').toString();
    }

    /** Builds the {@code number_conversion_config} nested object, or {@code null} if unset. */
    private String numberConversionConfigJson() {
        if (numberCurrency == null
                && numberPercent == null
                && numberBasisPoints == null
                && numberSuffixes == null
                && numberFractions == null
                && numberRadix == null) {
            return null;
        }
        StringBuilder sub = new StringBuilder("{");
        boolean[] subFirst = {true};
        writeBooleanField(sub, subFirst, "currency", numberCurrency);
        writeBooleanField(sub, subFirst, "percent", numberPercent);
        writeBooleanField(sub, subFirst, "basis_points", numberBasisPoints);
        writeBooleanField(sub, subFirst, "suffixes", numberSuffixes);
        writeBooleanField(sub, subFirst, "fractions", numberFractions);
        writeBooleanField(sub, subFirst, "radix", numberRadix);
        return sub.append('}').toString();
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

    /** Writes a pre-built raw JSON object (or array) value verbatim, e.g. from one of
     * the {@code *ConfigJson()} nested-object builders. No-op if {@code rawJson} is
     * {@code null} (the sub-config had nothing set). */
    private static void writeObjectField(
            StringBuilder json, boolean[] first, String key, String rawJson) {
        if (rawJson == null) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(':').append(rawJson);
    }

    private static void writeStringListField(
            StringBuilder json, boolean[] first, String key, List<String> values) {
        if (values.isEmpty()) {
            return;
        }
        writeComma(json, first);
        writeJsonString(json, key);
        json.append(":[");
        for (int i = 0; i < values.size(); i++) {
            if (i > 0) {
                json.append(',');
            }
            writeJsonString(json, values.get(i));
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
