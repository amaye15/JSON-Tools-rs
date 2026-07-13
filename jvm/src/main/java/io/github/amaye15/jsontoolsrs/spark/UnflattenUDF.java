package io.github.amaye15.jsontoolsrs.spark;

import io.github.amaye15.jsontoolsrs.JsonToolsException;
import io.github.amaye15.jsontoolsrs.NativeHandleCache;
import org.apache.spark.sql.api.java.UDF1;

/**
 * Row UDF: unflattens a single JSON string per call. See {@link FlattenUDF} for the
 * two registration patterns (default-config via {@code registerJavaFunction}, custom
 * config via the {@code spark._jvm} escape hatch) and the malformed-input policy
 * (throws {@link JsonToolsException}, does not swallow to {@code null}) -- both apply
 * identically here.
 */
public class UnflattenUDF implements UDF1<String, String> {

    private static final long serialVersionUID = 1L;
    private static final String DEFAULT_CONFIG = "{\"mode\":\"unflatten\"}";

    private final String configJson;

    public UnflattenUDF() {
        this(DEFAULT_CONFIG);
    }

    public UnflattenUDF(String configJson) {
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
