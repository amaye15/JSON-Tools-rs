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
