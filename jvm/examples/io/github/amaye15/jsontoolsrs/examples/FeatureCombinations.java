package io.github.amaye15.jsontoolsrs.examples;

import io.github.amaye15.jsontoolsrs.JsonTools;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;

/**
 * Curated multi-feature pipelines, building on {@link FeatureByFeature}. Not an
 * exhaustive combinatorial sweep (the builder has ~10 independent toggles, so a literal
 * power-set would be 1000+ cases) -- these are realistic groupings of features commonly
 * used together, plus one "kitchen sink" example exercising nearly everything at once.
 * Mirrors ../../../../../../../../examples/feature_combinations.rs and
 * python/examples/feature_combinations.py.
 *
 * <p>Run with:
 *
 * <pre>
 * mvn -P examples compile exec:java \
 *     -Dexec.mainClass=io.github.amaye15.jsontoolsrs.examples.FeatureCombinations
 * </pre>
 */
public final class FeatureCombinations {

    private FeatureCombinations() {}

    public static void main(String[] args) {
        System.out.println("JSON Tools RS - Feature Combinations");
        System.out.println("=====================================\n");

        // 1. separator + lowercaseKeys + keyReplacement + handleKeyCollision
        System.out.println("1. separator + lowercaseKeys + keyReplacement + handleKeyCollision");
        String input = "{\"User\":{\"Full_Name\":\"John\"},\"Admin\":{\"Full_Name\":\"Jane\"}}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .separator("_")
                .lowercaseKeys(true)
                .keyReplacement("r'^(user|admin)_'", "")
                .handleKeyCollision(true)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 2. keyReplacement + valueReplacement together
        System.out.println("2. keyReplacement + valueReplacement");
        input = "{\"usr_nm\":\"John\",\"usr_eml\":\"john@old.com\"}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .keyReplacement("usr_", "user_")
                .valueReplacement("@old.com", "@new.com")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 3. All four empty-value filters together
        System.out.println(
                "3. removeEmptyStrings + removeNulls + removeEmptyObjects + removeEmptyArrays");
        input = "{\"name\":\"John\",\"bio\":\"\",\"age\":null,\"tags\":[],\"meta\":{}}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .removeEmptyStrings(true)
                .removeNulls(true)
                .removeEmptyObjects(true)
                .removeEmptyArrays(true)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 4. Real-world normalization pipeline: replacement + filtering + collision
        System.out.println("4. lowercaseKeys + keyReplacement + filtering + handleKeyCollision");
        input = "{\"User_Name\":\"John\",\"User_Bio\":\"\",\"Admin_Name\":\"Jane\",\"Admin_Bio\":null}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .lowercaseKeys(true)
                .keyReplacement("r'^(user|admin)_'", "")
                .removeEmptyStrings(true)
                .removeNulls(true)
                .handleKeyCollision(true)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 5. unflatten + keyReplacement + valueReplacement
        System.out.println("5. unflatten + separator + keyReplacement + valueReplacement");
        input = "{\"PREFIX_user_name\":\"john@OLD.com\",\"PREFIX_user_age\":30}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .unflatten()
                .separator("_")
                .keyReplacement("PREFIX_user_", "profile_")
                .valueReplacement("@OLD.com", "@new.com")
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input) + "\n");
        }

        // 6. normal mode + autoConvertTypes + valueReplacement + filtering
        System.out.println("6. normal + autoConvertTypes + valueReplacement + removeEmptyStrings");
        input = "{\"user\":{\"status\":\"ACTIVE\",\"note\":\"\",\"score\":\"95.5\"}}";
        try (JsonToolsHandle tools = JsonTools.builder()
                .normal()
                .autoConvertTypes(true)
                .valueReplacement("r'^ACTIVE$'", "enabled")
                .removeEmptyStrings(true)
                .build()) {
            System.out.println("   In:  " + input);
            System.out.println("   Out: " + tools.execute(input));
            System.out.println(
                    "   Note: nesting preserved, string replaced, empty note dropped, score converted\n");
        }

        // 7. Batch processing + parallel tuning + type conversion together
        System.out.println(
                "7. batch execute + parallelThreshold + numThreads + "
                        + "nestedParallelThreshold + autoConvertTypes");
        String[] batch = new String[150];
        for (int i = 0; i < batch.length; i++) {
            batch[i] = "{\"id\":\"" + i + "\",\"active\":\"true\"}";
        }
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .parallelThreshold(50)
                .numThreads(4)
                .nestedParallelThreshold(100)
                .autoConvertTypes(true)
                .build()) {
            String[] results = tools.executeBatch(batch);
            System.out.println("   Processed " + results.length + " documents");
            System.out.println("   Sample: " + results[0] + "\n");
        }

        // 8. Kitchen sink: (almost) every feature at once, on a realistic messy batch
        System.out.println("8. Kitchen sink - every applicable feature combined");
        String[] apiBatch = {
            "{\"API_Response\":{\"User_Data\":{\"First_Name\":\"John\",\"Email\":\"john@old.com\","
                    + "\"Bio\":\"\",\"Score\":\"88.5\"}}}",
            "{\"API_Response\":{\"User_Data\":{\"First_Name\":\"Jane\",\"Email\":\"jane@old.com\","
                    + "\"Bio\":null,\"Score\":\"91.2\"}}}"
        };
        try (JsonToolsHandle tools = JsonTools.builder()
                .flatten()
                .separator("::")
                .lowercaseKeys(true)
                .keyReplacement("r'^api_response::user_data::'", "")
                .keyReplacement("first_name", "name")
                .valueReplacement("@old.com", "@new.com")
                .removeEmptyStrings(true)
                .removeNulls(true)
                .removeEmptyObjects(true)
                .removeEmptyArrays(true)
                .autoConvertTypes(true)
                .parallelThreshold(50)
                .numThreads(2)
                .nestedParallelThreshold(200)
                .build()) {
            String[] results = tools.executeBatch(apiBatch);
            System.out.println("   Features: separator, lowercase, 2x keyReplacement, valueReplacement,");
            System.out.println("             4x filtering, autoConvertTypes, parallel tuning, batch");
            for (int i = 0; i < results.length; i++) {
                System.out.println("   [" + i + "]: " + results[i]);
            }
        }
    }
}
