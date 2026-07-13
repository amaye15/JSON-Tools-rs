package io.github.amaye15.jsontoolsrs.spark;

import io.github.amaye15.jsontoolsrs.JsonToolsException;
import io.github.amaye15.jsontoolsrs.JsonToolsHandle;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;
import org.apache.spark.TaskContext;
import org.apache.spark.api.java.function.MapFunction;
import org.apache.spark.api.java.function.MapPartitionsFunction;
import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.Encoders;
import org.apache.spark.sql.Row;
import org.apache.spark.util.TaskCompletionListener;

/**
 * Tier-2: higher-throughput batched transform, using {@code Dataset.mapPartitions} to
 * amortize JNI-crossing overhead across many rows per native call instead of Spark's
 * row-at-a-time UDF invocation (see {@code FlattenUDF}/{@code UnflattenUDF} for that
 * simpler, SQL-native alternative).
 *
 * <p>Called from a Python Lakeflow Declarative Pipeline via the {@code df._jdf}
 * escape hatch, since a {@code mapPartitions} transform on a Java-backed {@code
 * Dataset} is not directly reachable from plain PySpark DataFrame calls -- see
 * {@code jvm/README.md} for the full worked example.
 *
 * <p><b>Batch-failure isolation:</b> the native batch call ({@link
 * JsonToolsHandle#executeBatch}) fails the whole chunk if any single row in it is
 * malformed (the underlying Rust batch processor short-circuits on the first error),
 * reported as an opaque per-chunk index that doesn't map back to anything meaningful
 * on the Spark side. A chunk that fails as a batch is retried row by row so the
 * resulting exception identifies the actual malformed row's own JSON error instead --
 * this does not change Spark's normal task-failure semantics (the task still fails
 * and is retried/aborted the same way any exception from a {@code mapPartitions}
 * function would), it only makes the failure diagnosable.
 *
 * <p><b>Tuning:</b> the default {@code batchSize} (64) is deliberately kept below the
 * core library's own {@code parallel_threshold} default (100), so that out of the box
 * this stays on the sequential per-batch path inside Rust, leaving Spark's own
 * task-level parallelism (multiple partitions/tasks running concurrently per
 * executor) as the only parallelism axis in play. Raising {@code batchSize} together
 * with {@code parallel_threshold}/{@code num_threads} in the config only pays off for
 * workloads with few partitions and low task concurrency; otherwise it stacks
 * rayon's intra-batch fan-out on top of Spark's task parallelism on the same cores.
 */
public final class BatchTransform {

    public static final int DEFAULT_BATCH_SIZE = 64;

    private BatchTransform() {}

    public static Dataset<String> flattenPartitioned(Dataset<String> input, String configJson) {
        return flattenPartitioned(input, configJson, DEFAULT_BATCH_SIZE);
    }

    public static Dataset<String> flattenPartitioned(
            Dataset<String> input, String configJson, int batchSize) {
        return input.mapPartitions(
                (MapPartitionsFunction<String, String>)
                        rows -> new BatchingIterator(rows, configJson, batchSize),
                Encoders.STRING());
    }

    /**
     * Same as {@link #flattenPartitioned(Dataset, String, int)}, but takes a {@code
     * Dataset<Row>} plus the name of its (string-typed) JSON column instead of a
     * {@code Dataset<String>} -- this is the entry point meant to be called from
     * PySpark via the {@code spark._jvm} escape hatch, since {@code df.select(col)._jdf}
     * is a much simpler thing to hand across py4j than assembling a JVM {@code
     * Dataset<String>} from Python.
     */
    public static Dataset<String> flattenPartitioned(
            Dataset<Row> input, String columnName, String configJson) {
        return flattenPartitioned(input, columnName, configJson, DEFAULT_BATCH_SIZE);
    }

    public static Dataset<String> flattenPartitioned(
            Dataset<Row> input, String columnName, String configJson, int batchSize) {
        return flattenPartitioned(
                input.select(columnName)
                        .map((MapFunction<Row, String>) row -> row.getString(0), Encoders.STRING()),
                configJson,
                batchSize);
    }

    /**
     * Lazily pulls {@code batchSize} rows at a time from the partition's row
     * iterator, processes each chunk as a single native batch call, and yields
     * results one at a time -- avoids materializing an entire partition's output in
     * memory up front.
     *
     * <p>The native handle is closed via a {@link TaskContext} completion listener,
     * not merely on iterator exhaustion: a downstream operator (e.g. {@code
     * .limit()}) can stop pulling before this iterator is drained, and completion
     * listeners are the reliable, Spark-blessed way to guarantee per-partition
     * resource cleanup regardless of how the task actually ends.
     */
    private static final class BatchingIterator implements Iterator<String> {
        private final Iterator<String> rows;
        private final JsonToolsHandle handle;
        private final int batchSize;
        private Iterator<String> currentChunkResults = Collections.emptyIterator();

        BatchingIterator(Iterator<String> rows, String configJson, int batchSize) {
            this.rows = rows;
            this.handle = new JsonToolsHandle(configJson);
            this.batchSize = batchSize;
            TaskContext context = TaskContext.get();
            if (context != null) {
                context.addTaskCompletionListener((TaskCompletionListener) ctx -> handle.close());
            }
        }

        @Override
        public boolean hasNext() {
            if (currentChunkResults.hasNext()) {
                return true;
            }
            if (!rows.hasNext()) {
                return false;
            }
            List<String> chunk = new ArrayList<>(batchSize);
            while (rows.hasNext() && chunk.size() < batchSize) {
                chunk.add(rows.next());
            }
            currentChunkResults = processChunk(handle, chunk).iterator();
            return currentChunkResults.hasNext();
        }

        @Override
        public String next() {
            if (!hasNext()) {
                throw new NoSuchElementException();
            }
            return currentChunkResults.next();
        }
    }

    private static List<String> processChunk(JsonToolsHandle handle, List<String> chunk) {
        try {
            return Arrays.asList(handle.executeBatch(chunk.toArray(new String[0])));
        } catch (JsonToolsException batchFailure) {
            List<String> results = new ArrayList<>(chunk.size());
            for (String row : chunk) {
                results.add(handle.execute(row));
            }
            return results;
        }
    }
}
