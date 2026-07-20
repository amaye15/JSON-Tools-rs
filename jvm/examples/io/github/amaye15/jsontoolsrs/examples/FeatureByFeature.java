package io.github.amaye15.jsontoolsrs.examples;

import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsException;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;

/**
 * One isolated example per {@link JsonTools} builder feature. Companion to {@link
 * FeatureCombinations}, which shows curated multi-feature pipelines. Mirrors
 * ../../../../../../../../examples/feature_by_feature.rs and
 * python/examples/feature_by_feature.py.
 *
 * <p>Run with:
 *
 * <pre>
 * mvn -P examples compile exec:java \
 *     -Dexec.mainClass=io.github.amaye15.jsontoolsrs.examples.FeatureByFeature
 * </pre>
 */
public final class FeatureByFeature {

    private FeatureByFeature() {}

    public static void main(String[] args) {
        System.out.println("JSON Tools RS - Feature by Feature");
        System.out.println("===================================\n");

        // 1. Mode: flatten
        System.out.println("1. Mode: .flatten()");
        String input = "{\"user\":{\"name\":\"John\",\"address\":{\"city\":\"NYC\",\"zip\":\"10001\"}}}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 2. Mode: unflatten
        System.out.println("2. Mode: .unflatten()");
        input = "{\"user.name\":\"John\",\"user.address.city\":\"NYC\"}";
        try (JsonToolsHandle tools = JsonTools.builder().unflatten().build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 3. Mode: normal (transform in place, no restructuring)
        System.out.println("3. Mode: .normal()");
        input = "{\"user\":{\"name\":\"John\",\"age\":null}}";
        try (JsonToolsHandle tools = JsonTools.builder().normal().removeNulls(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println("   Note: nulls removed but nesting preserved (no dot notation)\n");
        }

        // 4. .separator()
        System.out.println("4. .separator()");
        input = "{\"user\":{\"profile\":{\"city\":\"NYC\"}}}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().separator("::").build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 5. .lowercaseKeys()
        System.out.println("5. .lowercaseKeys()");
        input = "{\"User\":{\"Name\":\"John\"}}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().lowercaseKeys(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 6. .keyReplacement() - literal
        System.out.println("6. .keyReplacement() - literal match");
        input = "{\"user_name\":\"John\"}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().keyReplacement("user_", "").build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 7. .keyReplacement() - regex (wrap pattern in r'...')
        System.out.println("7. .keyReplacement() - regex match");
        input = "{\"user_id\":1,\"account_id\":2}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().keyReplacement("r'_id$'", "_key").build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 8. .valueReplacement() - literal
        System.out.println("8. .valueReplacement() - literal match");
        input = "{\"email\":\"john@example.com\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .valueReplacement("@example.com", "@company.org")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 9. .valueReplacement() - regex
        System.out.println("9. .valueReplacement() - regex match");
        input = "{\"phone\":\"555-1234\",\"fax\":\"555-5678\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .valueReplacement("r'^555-'", "10-555-")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 10. .removeEmptyStrings()
        System.out.println("10. .removeEmptyStrings()");
        input = "{\"name\":\"John\",\"bio\":\"\"}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().removeEmptyStrings(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 11. .removeNulls()
        System.out.println("11. .removeNulls()");
        input = "{\"name\":\"John\",\"age\":null}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().removeNulls(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 12. .removeEmptyObjects()
        System.out.println("12. .removeEmptyObjects()");
        input = "{\"name\":\"John\",\"meta\":{}}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().removeEmptyObjects(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 13. .removeEmptyArrays()
        System.out.println("13. .removeEmptyArrays()");
        input = "{\"name\":\"John\",\"tags\":[]}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().removeEmptyArrays(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 14. .handleKeyCollision()
        System.out.println("14. .handleKeyCollision()");
        input = "{\"user_name\":\"John\",\"admin_name\":\"Jane\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .keyReplacement("r'^(user|admin)_'", "")
                .handleKeyCollision(true)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println("   Note: colliding keys are collected into an array\n");
        }

        // 15. .autoConvertTypes()
        System.out.println("15. .autoConvertTypes()");
        input = "{\"id\":\"123\",\"price\":\"$19.99\",\"active\":\"true\"}";
        try (JsonToolsHandle tools =
                JsonTools.builder().flatten().autoConvertTypes(true).build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 16. .convertDates() - independent date/datetime conversion
        System.out.println("16. .convertDates() / .dateAssumeUtcForNaive()");
        input = "{\"d\":\"2024-01-15T10:30:00\",\"b\":\"true\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertDates(true)
                .dateAssumeUtcForNaive(false)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println(
                    "   Note: only dates convert (dateAssumeUtcForNaive(false) keeps this one unchanged)\n");
        }

        // 17. .convertNulls() - independent null conversion, with extra tokens
        System.out.println("17. .convertNulls() / .nullExtraToken()");
        input = "{\"a\":\"missing\",\"b\":\"N/A\",\"c\":\"not_a_token\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNulls(true)
                .nullExtraToken("missing")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println("   Note: 'missing' is a custom extra token; built-in 'N/A' still works\n");
        }

        // 18. .convertBooleans() - independent boolean conversion, with extra tokens
        System.out.println("18. .convertBooleans() / .booleanExtraTrueToken() / .booleanExtraFalseToken()");
        input = "{\"a\":\"si\",\"b\":\"nope\",\"c\":\"true\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertBooleans(true)
                .booleanExtraTrueToken("si")
                .booleanExtraFalseToken("nope")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 19. .convertNumbers() - independent number conversion, sub-format toggles
        System.out.println("19. .convertNumbers() / .numberCurrency()");
        input = "{\"price\":\"$45.67\",\"count\":\"1,234.56\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .convertNumbers(true)
                .numberCurrency(false)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println(
                    "   Note: numberCurrency(false) leaves '$45.67' as a string; thousands-separator cleanup is still core\n");
        }

        // 20. .maxArrayIndex() - DoS guard during unflatten
        System.out.println("20. .maxArrayIndex()");
        String okInput = "{\"items.0\":\"a\",\"items.1\":\"b\"}";
        try (JsonToolsHandle tools =
                JsonTools.builder().unflatten().maxArrayIndex(10).build()) {
            System.out.println("   Within limit -> In:  " + okInput);
            System.out.println("                  Out: " + tools.execute(okInput));
        }
        String badInput = "{\"items.9999\":\"x\"}";
        try (JsonToolsHandle tools =
                JsonTools.builder().unflatten().maxArrayIndex(10).build()) {
            tools.execute(badInput);
            System.out.println("   Unexpected success for out-of-range index");
        } catch (JsonToolsException e) {
            System.out.println("   Exceeds limit  -> In:  " + badInput);
            System.out.println("                  Err: " + e.getMessage() + "\n");
        }

        // 21. Parallel processing tuning knobs
        System.out.println("21. .parallelThreshold() / .numThreads() / .nestedParallelThreshold()");
        String[] batch = new String[200];
        for (int i = 0; i < batch.length; i++) {
            batch[i] = "{\"id\":" + i + ",\"data\":{\"value\":" + (i * 10) + "}}";
        }
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .parallelThreshold(50)
                .numThreads(4)
                .nestedParallelThreshold(200)
                .build()) {
            String[] results = tools.executeBatch(batch);
            System.out.println("   Processed " + results.length + " documents with tuned parallelism");
            System.out.println("   Sample: " + results[0] + "\n");
        }

        // 22. Batch processing - a single executeBatch() call over many documents
        System.out.println("22. Batch processing (String[] input -> String[] output)");
        String[] smallBatch = {"{\"a\":{\"b\":1}}", "{\"c\":{\"d\":2}}"};
        try (JsonToolsHandle tools = JsonTools.builder().flatten().build()) {
            String[] results = tools.executeBatch(smallBatch);
            System.out.println("   In:  " + String.join(", ", smallBatch));
            System.out.println("   Out: " + String.join(", ", results) + "\n");
        }

        // 23. .excludeKey() - drop a key and its entire subtree
        System.out.println("23. .excludeKey() - drop a container key's entire subtree");
        String cryptoInput = "{\"user\":{\"name\":\"John\",\"crypto_wallet\":{\"coin\":\"BTC\",\"balance\":100}}}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().excludeKey("crypto").build()) {
            System.out.println("   In:  " + cryptoInput);
            System.out.println("   Out: " + tools.execute(cryptoInput) + "\n");
        }

        // 24. .excludeValue() - drop a key-value pair by value content
        System.out.println("24. .excludeValue() - drop a key-value pair whose value matches");
        String bannedInput = "{\"user\":{\"name\":\"John\",\"status\":\"banned\"}}";
        try (JsonToolsHandle tools = JsonTools.builder().flatten().excludeValue("banned").build()) {
            System.out.println("   In:  " + bannedInput);
            System.out.println("   Out: " + tools.execute(bannedInput) + "\n");
        }
    }
}
