package io.github.amaye15.jsontoolsrs.spark;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.List;
import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.Row;
import org.apache.spark.sql.RowFactory;
import org.apache.spark.sql.SparkSession;
import org.apache.spark.sql.types.DataTypes;
import org.apache.spark.sql.types.StructField;
import org.apache.spark.sql.types.StructType;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

/** Local-SparkSession integration tests for the Tier-1 row UDFs. */
class FlattenUDFSparkTest {

    private static SparkSession spark;

    @BeforeAll
    static void setUp() {
        spark = SparkSession.builder()
                .appName("json-tools-rs-spark-test")
                .master("local[2]")
                .getOrCreate();
    }

    @AfterAll
    static void tearDown() {
        if (spark != null) {
            spark.stop();
        }
    }

    private static Dataset<Row> singleColumnRows(String columnName, String... values) {
        StructType schema = new StructType(
                new StructField[] {DataTypes.createStructField(columnName, DataTypes.StringType, false)});
        List<Row> rows = new java.util.ArrayList<>();
        for (String value : values) {
            rows.add(RowFactory.create(value));
        }
        return spark.createDataFrame(rows, schema);
    }

    @Test
    void registerJavaFunctionAndQueryViaSql() {
        spark.udf()
                .registerJava(
                        "flatten_json",
                        "io.github.amaye15.jsontoolsrs.spark.FlattenUDF",
                        DataTypes.StringType);

        singleColumnRows("json", "{\"user\":{\"name\":\"John\"}}")
                .createOrReplaceTempView("flatten_input_rows");

        List<Row> results = spark.sql("SELECT flatten_json(json) AS flattened FROM flatten_input_rows")
                .collectAsList();

        assertEquals(1, results.size());
        assertEquals("{\"user.name\":\"John\"}", results.get(0).getString(0));
    }

    @Test
    void registerJavaFunctionForUnflatten() {
        spark.udf()
                .registerJava(
                        "unflatten_json",
                        "io.github.amaye15.jsontoolsrs.spark.UnflattenUDF",
                        DataTypes.StringType);

        singleColumnRows("json", "{\"user.name\":\"John\"}")
                .createOrReplaceTempView("unflatten_input_rows");

        List<Row> results = spark
                .sql("SELECT unflatten_json(json) AS unflattened FROM unflatten_input_rows")
                .collectAsList();

        assertEquals(1, results.size());
        assertEquals("{\"user\":{\"name\":\"John\"}}", results.get(0).getString(0));
    }

    @Test
    void customConfigViaDirectInstanceRegistration() {
        // Mirrors the spark._jvm escape hatch documented in FlattenUDF's javadoc:
        // construct with a custom config JSON and register the instance directly,
        // since registerJavaFunction's reflection-based instantiation can't pass
        // constructor arguments.
        FlattenUDF customUdf = new FlattenUDF("{\"mode\":\"flatten\",\"separator\":\"::\"}");
        spark.udf().register("flatten_json_custom", customUdf, DataTypes.StringType);

        singleColumnRows("json", "{\"a\":{\"b\":1}}").createOrReplaceTempView("custom_input_rows");

        List<Row> results = spark
                .sql("SELECT flatten_json_custom(json) AS flattened FROM custom_input_rows")
                .collectAsList();

        assertEquals(1, results.size());
        assertEquals("{\"a::b\":1}", results.get(0).getString(0));
    }
}
