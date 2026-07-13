package io.github.amaye15.jsontoolsrs.spark;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.Encoders;
import org.apache.spark.sql.Row;
import org.apache.spark.sql.RowFactory;
import org.apache.spark.sql.SparkSession;
import org.apache.spark.sql.types.DataTypes;
import org.apache.spark.sql.types.StructField;
import org.apache.spark.sql.types.StructType;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

/** Local-SparkSession integration tests for the Tier-2 {@code mapPartitions} batch transform. */
class BatchTransformTest {

    private static SparkSession spark;

    @BeforeAll
    static void setUp() {
        spark = SparkSession.builder()
                .appName("json-tools-rs-batch-transform-test")
                .master("local[2]")
                .getOrCreate();
    }

    @AfterAll
    static void tearDown() {
        if (spark != null) {
            spark.stop();
        }
    }

    @Test
    void flattenPartitionedProcessesAllRows() {
        Dataset<String> input = spark.createDataset(
                Arrays.asList("{\"a\":{\"b\":1}}", "{\"c\":{\"d\":2}}", "{\"e\":{\"f\":3}}"),
                Encoders.STRING());

        Dataset<String> output = BatchTransform.flattenPartitioned(input, "{\"mode\":\"flatten\"}", 2);

        List<String> results = output.collectAsList();
        results.sort(null);
        assertEquals(Arrays.asList("{\"a.b\":1}", "{\"c.d\":2}", "{\"e.f\":3}"), results);
    }

    @Test
    void defaultBatchSizeOverloadWorks() {
        Dataset<String> input =
                spark.createDataset(Arrays.asList("{\"a\":{\"b\":1}}"), Encoders.STRING());

        List<String> results =
                BatchTransform.flattenPartitioned(input, "{\"mode\":\"flatten\"}").collectAsList();

        assertEquals(1, results.size());
        assertEquals("{\"a.b\":1}", results.get(0));
    }

    @Test
    void flattenPartitionedFromRowDatasetColumn() {
        // Mirrors the spark._jvm escape hatch documented in BatchTransform's javadoc:
        // a Dataset<Row> with a named string column, as you'd get from df.select(col)._jdf
        // on the PySpark side, rather than a Dataset<String>.
        StructType schema = new StructType(
                new StructField[] {DataTypes.createStructField("json_column", DataTypes.StringType, false)});
        Dataset<Row> input = spark.createDataFrame(
                List.of(RowFactory.create("{\"a\":{\"b\":1}}"), RowFactory.create("{\"c\":{\"d\":2}}")),
                schema);

        List<String> results = BatchTransform
                .flattenPartitioned(input, "json_column", "{\"mode\":\"flatten\"}")
                .collectAsList();
        results.sort(null);

        assertEquals(Arrays.asList("{\"a.b\":1}", "{\"c.d\":2}"), results);
    }

    @Test
    void flattenPartitionedHandlesLargePartitionAcrossManyChunks() {
        List<String> rows = new ArrayList<>();
        for (int i = 0; i < 500; i++) {
            rows.add("{\"a\":{\"b\":" + i + "}}");
        }
        Dataset<String> input = spark.createDataset(rows, Encoders.STRING()).coalesce(1);

        Dataset<String> output = BatchTransform.flattenPartitioned(input, "{\"mode\":\"flatten\"}", 32);

        assertEquals(500, output.count());
    }

    @Test
    void malformedRowFailsTheTaskWithoutCorruptingOutput() {
        // A malformed row still fails the Spark task (matching the row-UDF tier's
        // fail-loud policy -- this transform never silently drops bad rows), and the
        // batch-failure-isolation retry must not throw something confusing like an
        // NPE or a mismatched-length result in its place.
        Dataset<String> input = spark.createDataset(
                        Arrays.asList("{\"a\":1}", "not valid json", "{\"c\":2}"), Encoders.STRING())
                .coalesce(1);

        Dataset<String> output = BatchTransform.flattenPartitioned(input, "{\"mode\":\"flatten\"}", 10);

        Exception thrown = assertThrows(Exception.class, output::collectAsList);
        assertTrue(thrown.getMessage() != null && !thrown.getMessage().isBlank());
    }
}
