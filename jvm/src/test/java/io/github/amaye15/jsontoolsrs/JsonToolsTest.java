package io.github.amaye15.jsontoolsrs;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

/** Tests for the {@link JsonTools} fluent builder, exercising config-blob marshaling end to end. */
class JsonToolsTest {

    @Test
    void basicFlatten() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
            String result = tools.execute("{\"user\":{\"name\":\"John\",\"profile\":{\"age\":30}}}");
            assertTrue(result.contains("\"user.name\":\"John\""));
            assertTrue(result.contains("\"user.profile.age\":30"));
        }
    }

    @Test
    void basicUnflatten() {
        try (JsonToolsHandle tools = JsonTools.builder().unflatten().build()) {
            String result = tools.execute("{\"user.name\":\"John\",\"user.age\":30}");
            assertTrue(result.contains("\"name\":\"John\""));
            assertTrue(result.contains("\"age\":30"));
        }
    }

    @Test
    void customSeparator() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().separator("::").build()) {
            String result = tools.execute("{\"a\":{\"b\":1}}");
            assertEquals("{\"a::b\":1}", result);
        }
    }

    @Test
    void bareKeyReplacementIsLiteral() {
        // Bare patterns match literally, even when they contain regex metacharacters.
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().keyReplacement("admin_", "").build()) {
            String result = tools.execute("{\"admin_name\":\"Jane\"}");
            assertEquals("{\"name\":\"Jane\"}", result);
        }
    }

    @Test
    void rQuoteKeyReplacementIsRegex() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .keyReplacement("r'^(user|admin)_'", "person_")
                .build()) {
            String result = tools.execute("{\"admin_name\":\"Jane\",\"user_id\":1}");
            assertTrue(result.contains("\"person_name\":\"Jane\""));
            assertTrue(result.contains("\"person_id\":1"));
        }
    }

    @Test
    void valueReplacement() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .valueReplacement("@example.com", "@company.org")
                .build()) {
            String result = tools.execute("{\"email\":\"john@example.com\"}");
            assertEquals("{\"email\":\"john@company.org\"}", result);
        }
    }

    @Test
    void removeNullsAndEmptyStrings() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .removeNulls(true)
                .removeEmptyStrings(true)
                .build()) {
            String result = tools.execute("{\"a\":null,\"b\":\"\",\"c\":\"kept\"}");
            assertEquals("{\"c\":\"kept\"}", result);
        }
    }

    @Test
    void lowercaseKeys() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().lowercaseKeys(true).build()) {
            String result = tools.execute("{\"UserName\":\"John\"}");
            assertEquals("{\"username\":\"John\"}", result);
        }
    }

    @Test
    void autoConvertTypes() {
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().autoConvertTypes(true).build()) {
            String result = tools.execute("{\"id\":\"123\",\"active\":\"true\"}");
            assertTrue(result.contains("\"id\":123"));
            assertTrue(result.contains("\"active\":true"));
        }
    }

    @Test
    void convertDatesIndependent() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().convertDates(true).build()) {
            String result = tools.execute(
                    "{\"d\":\"2024-01-15T10:30:00+05:00\",\"b\":\"true\",\"num\":\"123\"}");
            assertTrue(result.contains("\"d\":\"2024-01-15T05:30:00Z\""));
            assertTrue(result.contains("\"b\":\"true\"")); // still a string
            assertTrue(result.contains("\"num\":\"123\"")); // still a string
        }
    }

    @Test
    void convertNullsIndependent() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().convertNulls(true).build()) {
            String result = tools.execute("{\"n\":\"null\",\"b\":\"true\"}");
            assertTrue(result.contains("\"n\":null"));
            assertTrue(result.contains("\"b\":\"true\""));
        }
    }

    @Test
    void convertBooleansIndependent() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().convertBooleans(true).build()) {
            String result = tools.execute("{\"n\":\"null\",\"b\":\"true\"}");
            assertTrue(result.contains("\"n\":\"null\""));
            assertTrue(result.contains("\"b\":true"));
        }
    }

    @Test
    void convertNumbersIndependent() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().convertNumbers(true).build()) {
            String result = tools.execute("{\"b\":\"true\",\"num\":\"123\"}");
            assertTrue(result.contains("\"b\":\"true\""));
            assertTrue(result.contains("\"num\":123"));
        }
    }

    @Test
    void autoConvertTypesThenPerCategoryDisable() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .autoConvertTypes(true)
                .convertDates(false)
                .build()) {
            String result = tools.execute("{\"d\":\"2024-01-15T10:30:00Z\",\"b\":\"true\"}");
            assertTrue(result.contains("\"d\":\"2024-01-15T10:30:00Z\"")); // unchanged string
            assertTrue(result.contains("\"b\":true"));
        }
    }

    @Test
    void dateAssumeUtcForNaiveFalseLeavesNaiveDatetimeUnchanged() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertDates(true)
                .dateAssumeUtcForNaive(false)
                .build()) {
            String result = tools.execute("{\"d\":\"2024-01-15T10:30:00\"}");
            assertEquals("{\"d\":\"2024-01-15T10:30:00\"}", result);
        }
    }

    @Test
    void nullExtraTokenIsAdditive() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNulls(true)
                .nullExtraToken("missing")
                .build()) {
            String result = tools.execute("{\"a\":\"missing\",\"b\":\"N/A\"}");
            assertTrue(result.contains("\"a\":null")); // extra token
            assertTrue(result.contains("\"b\":null")); // built-in list still active
        }
    }

    @Test
    void booleanExtraTokensAreAdditive() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .booleanExtraTrueToken("si")
                .booleanExtraFalseToken("nope")
                .build()) {
            String result = tools.execute("{\"a\":\"si\",\"b\":\"nope\",\"c\":\"true\"}");
            assertTrue(result.contains("\"a\":true"));
            assertTrue(result.contains("\"b\":false"));
            assertTrue(result.contains("\"c\":true")); // built-in list still active
        }
    }

    @Test
    void numberCurrencyDisabled() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNumbers(true)
                .numberCurrency(false)
                .build()) {
            String result = tools.execute("{\"price\":\"$45.67\",\"count\":\"1,234.56\"}");
            assertTrue(result.contains("\"price\":\"$45.67\""));
            assertTrue(result.contains("\"count\":1234.56")); // thousands-separator cleanup still core
        }
    }

    @Test
    void configJsonNestedObjectShapeForCustomizedCategory() {
        // Locks in the wire format for a customized category -- matters for
        // NativeHandleCache's cache-key determinism (see JsonTools's class doc).
        String json = JsonTools.builder()
                .flatten()
                .convertDates(true)
                .dateAssumeUtcForNaive(false)
                .toConfigJson();
        assertEquals(
                "{\"mode\":\"flatten\",\"convert_dates\":true,"
                        + "\"date_conversion_config\":{\"assume_utc_for_naive\":false}}",
                json);
    }

    @Test
    void booleanTokenInBothListsTrueWins() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .booleanExtraTrueToken("maybe")
                .booleanExtraFalseToken("maybe")
                .build()) {
            String result = tools.execute("{\"a\":\"maybe\"}");
            assertEquals("{\"a\":true}", result);
        }
    }

    @Test
    void nullExtraTokenDuplicatingBuiltinIsHarmless() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNulls(true)
                .nullExtraToken("null")
                .build()) {
            String result = tools.execute("{\"a\":\"null\",\"b\":\"not_null\"}");
            assertTrue(result.contains("\"a\":null"));
            assertTrue(result.contains("\"b\":\"not_null\""));
        }
    }

    @Test
    void nullExtraTokenIsAdditiveAcrossMultipleCalls() {
        // Java's nullExtraToken(String) is additive per call (like Rust's
        // add_extra_token(), unlike Python's bulk-replace extra_tokens kwarg) --
        // confirms repeated calls accumulate rather than each replacing the last.
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNulls(true)
                .nullExtraToken("missing")
                .nullExtraToken("unavailable")
                .build()) {
            String result = tools.execute("{\"a\":\"missing\",\"b\":\"unavailable\",\"c\":\"present\"}");
            assertTrue(result.contains("\"a\":null"));
            assertTrue(result.contains("\"b\":null"));
            assertTrue(result.contains("\"c\":\"present\""));
        }
    }

    @Test
    void disabledCategoryCustomizationHasNoEffect() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNumbers(false)
                .numberCurrency(false) // inert: category itself is off
                .convertBooleans(true) // ensures the fine-grained code path is reached
                .build()) {
            String result = tools.execute("{\"price\":\"$45.67\",\"active\":\"true\"}");
            assertTrue(result.contains("\"price\":\"$45.67\""));
            assertTrue(result.contains("\"active\":true"));
        }
    }

    @Test
    void datePriorityWinsOverNullExtraToken() {
        // Dates are checked before nulls in priority order -- a string that's both
        // a valid recognized date AND configured as an extra null token must still
        // be date-normalized, matching the same cross-language guarantee tested in
        // the Rust and Python suites.
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertDates(true)
                .convertNulls(true)
                .nullExtraToken("2024-01-15T10:30:00+05:00")
                .build()) {
            String result = tools.execute("{\"d\":\"2024-01-15T10:30:00+05:00\"}");
            assertEquals("{\"d\":\"2024-01-15T05:30:00Z\"}", result);
        }
    }

    @Test
    void extraTokenMatchesAfterTrimming() {
        // Extra tokens match against the trimmed value, consistent with every
        // other category -- not a byte-for-byte match against the raw string.
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .booleanExtraTrueToken("si")
                .build()) {
            String result = tools.execute("{\"a\":\"si \",\"b\":\"siX\"}");
            assertTrue(result.contains("\"a\":true"));
            assertTrue(result.contains("\"b\":\"siX\""));
        }
    }

    @Test
    void fineGrainedTypeConversionInBatch() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNumbers(true)
                .convertBooleans(true)
                .convertNulls(true)
                .nullExtraToken("missing")
                .build()) {
            String[] input = new String[150];
            for (int i = 0; i < input.length; i++) {
                input[i] = "{\"id\":\"" + i + "\",\"active\":\"yes\",\"extra\":\"missing\"}";
            }
            String[] results = tools.executeBatch(input);
            assertEquals(150, results.length);
            assertEquals("{\"id\":0,\"active\":true,\"extra\":null}", results[0]);
            assertEquals("{\"id\":149,\"active\":true,\"extra\":null}", results[149]);
        }
    }

    @Test
    void numericExtraBooleanTokenLosesToNumbersWhenBothEnabled() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNumbers(true)
                .convertBooleans(true)
                .booleanExtraTrueToken("1")
                .build()) {
            String result = tools.execute("{\"a\":\"1\"}");
            assertEquals("{\"a\":1}", result); // number, not boolean
        }
    }

    @Test
    void numericExtraBooleanTokenWinsWhenNumbersDisabled() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .booleanExtraTrueToken("1")
                .build()) {
            String result = tools.execute("{\"a\":\"1\"}");
            assertEquals("{\"a\":true}", result);
        }
    }

    @Test
    void extraTokensAreCaseSensitive() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNulls(true)
                .nullExtraToken("missing")
                .build()) {
            String result = tools.execute("{\"a\":\"missing\",\"b\":\"MISSING\"}");
            assertTrue(result.contains("\"a\":null"));
            assertTrue(result.contains("\"b\":\"MISSING\""));
        }
    }

    @Test
    void malformedDateStaysAsStringNoCrash() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().convertDates(true).build()) {
            String result = tools.execute("{\"a\":\"2024-13-45T99:99:99\"}");
            assertEquals("{\"a\":\"2024-13-45T99:99:99\"}", result);
        }
    }

    @Test
    void keysAreNeverTypeConverted() {
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .convertNumbers(true)
                .build()) {
            String result = tools.execute("{\"true\":\"something\",\"123\":\"also something\"}");
            assertTrue(result.contains("\"true\":\"something\""));
            assertTrue(result.contains("\"123\":\"also something\""));
        }
    }

    @Test
    void replacementAndConversionChainingDiffersByMode() {
        // Same pre-existing cross-mode behavior confirmed in the Rust and Python
        // suites: flatten() returns as soon as valueReplacement matches, without
        // trying conversion on the replaced value; normal() chains the two.
        try (JsonToolsHandle flattenTools = JsonTools.builder()
                .flatten()
                .valueReplacement("ACTIVE", "true")
                .convertBooleans(true)
                .build()) {
            String result = flattenTools.execute("{\"a\":\"ACTIVE\"}");
            assertEquals("{\"a\":\"true\"}", result); // still a string
        }
        try (JsonToolsHandle normalTools = JsonTools.builder()
                .normal()
                .valueReplacement("ACTIVE", "true")
                .convertBooleans(true)
                .build()) {
            String result = normalTools.execute("{\"a\":\"ACTIVE\"}");
            assertEquals("{\"a\":true}", result); // chained
        }
    }

    @Test
    void executeBatch() {
        try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
            String[] results =
                    tools.executeBatch(new String[] {"{\"a\":{\"b\":1}}", "{\"c\":{\"d\":2}}"});
            assertEquals("{\"a.b\":1}", results[0]);
            assertEquals("{\"c.d\":2}", results[1]);
        }
    }

    @Test
    void toConfigJsonOmitsUnsetFields() {
        String json = JsonTools.builder().flatten().separator("::").toConfigJson();
        assertEquals("{\"mode\":\"flatten\",\"separator\":\"::\"}", json);
    }

    @Test
    void toConfigJsonIsDeterministicAcrossIdenticalBuilders() {
        String a = JsonTools.builder()
                .flatten()
                .separator("::")
                .keyReplacement("r'^admin_'", "")
                .removeNulls(true)
                .toConfigJson();
        String b = JsonTools.builder()
                .flatten()
                .separator("::")
                .keyReplacement("r'^admin_'", "")
                .removeNulls(true)
                .toConfigJson();
        assertEquals(a, b);
        assertFalse(a.isEmpty());
    }
}
