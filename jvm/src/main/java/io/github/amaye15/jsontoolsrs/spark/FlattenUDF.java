package io.github.amaye15.jsontoolsrs.spark;

import io.github.amaye15.jsontoolsrs.JsonToolsException;
import io.github.amaye15.jsontoolsrs.NativeHandleCache;
import org.apache.spark.sql.api.java.UDF1;

/**
 * Row UDF: flattens a single JSON string per call.
 *
 * <p>Databricks Lakeflow Declarative Pipelines cannot attach JVM libraries to pipeline
 * compute at all (serverless or classic-backed) -- this UDF is for Databricks Jobs and
 * notebooks running on classic compute, or other Spark workloads. For transforming JSON
 * genuinely inside a Lakeflow pipeline, wrap the Python bindings in a {@code pandas_udf}
 * instead -- see {@code docs/src/guide/databricks-setup.md}.
 *
 * <p>Two ways to use this from a Job/notebook (Python):
 *
 * <ul>
 *   <li>Default configuration ({@code "."} separator, no replacements/filters), via
 *       the standard {@code registerJavaFunction} path, which instantiates this class
 *       through its no-arg constructor:
 *       <pre>{@code
 * spark.udf.registerJavaFunction(
 *     "flatten_json",
 *     "io.github.amaye15.jsontoolsrs.spark.FlattenUDF",
 *     StringType())
 *       }</pre>
 *   <li>Custom configuration, via the {@code spark._jvm} escape hatch (Spark's
 *       reflection-based {@code registerJavaFunction} has no way to pass constructor
 *       arguments from Python):
 *       <pre>{@code
 * config_json = '{"mode":"flatten","separator":"::","remove_nulls":true}'
 * jvm_udf = spark._jvm.io.github.amaye15.jsontoolsrs.spark.FlattenUDF(config_json)
 * spark._jsparkSession.udf().register(
 *     "flatten_json_custom", jvm_udf,
 *     spark._jvm.org.apache.spark.sql.types.DataTypes.StringType())
 *       }</pre>
 * </ul>
 *
 * <p>Malformed input throws {@link JsonToolsException}, consistent with how both the
 * core Rust library and its Python bindings already behave -- it is not silently
 * swallowed to {@code null}. Wrap in SQL {@code TRY(...)} if you want {@code
 * from_json}-style null-on-error semantics instead.
 */
public class FlattenUDF implements UDF1<String, String> {

    private static final long serialVersionUID = 1L;
    private static final String DEFAULT_CONFIG = "{\"mode\":\"flatten\"}";

    private final String configJson;

    public FlattenUDF() {
        this(DEFAULT_CONFIG);
    }

    public FlattenUDF(String configJson) {
        this.configJson = configJson;
    }

    @Override
    public String call(String json) {
        if (json == null) {
            return null;
        }
        return NativeHandleCache.get(configJson).execute(json);
    }
}
