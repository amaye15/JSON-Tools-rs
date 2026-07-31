"""Quick, hand-timed benchmark for the DataFrame reconstruction path --
execute(..., normalise=True, target=...) and PySpark execute(spark_df) --
i.e. the Arrow-native build_normalise_table()/reconstruct_*_normalise()
functions in src/python.rs (github.com/amaye15/JSON-Tools-rs/issues/35;
replaced the older union_and_columnarize()-based implementation this script
originally measured).

This code lives entirely behind the `python` feature (PyO3 layer) and can't
be reached by the pure-Rust `benches/*.rs` (Criterion) suite or
`examples/bench_quick.rs`, so it had zero benchmark coverage before this
script -- found while investigating performance after issues #31-#33 added
this reconstruction machinery. Deliberately informational, not statistically
rigorous, mirroring bench_quick.rs's scope/style for the Rust core.

Usage:
    python python/benchmarks/bench_normalise.py
    python python/benchmarks/bench_normalise.py --csv
    python python/benchmarks/bench_normalise.py --with-pyspark
    python python/benchmarks/bench_normalise.py --repeats 5
"""

import argparse
import statistics
import time

import json_tools_rs as jt

# ("small"/"medium"/"large", rows, cols). "large" mirrors the scale issue
# #31/#33 actually reported (754 rows, ~4,042 flattened columns).
SCENARIOS = [
    ("small", 50, 50),
    ("medium", 200, 500),
    ("large", 754, 4042),
]

# Date-typed scenarios (issue #35's Date32/Timestamp support, gated on
# .convert_dates(True)) -- separate from SCENARIOS above since these need
# convert_dates enabled and exercise date-specific columns/mixes, not just
# scalar/collision shapes. "date_worst_case" is deliberately adversarial:
# convert_dates(True) is on but *nothing* is actually a date, measuring the
# wasted chrono-parse-attempt cost on every string cell in the case where
# date detection never pays off.
DATE_SCENARIOS = [
    ("date_medium", 200, 500),
    ("date_large", 754, 4042),
    ("date_worst_case", 200, 500),
]


# Heterogeneous: only every 3rd row carries the "extra_i" key, so the union
# has more columns than any single row -- exercises the sparse/union case
# issues #32/#33 are actually about, not just a uniform dense grid.
def make_row(i: int, n_cols: int) -> dict:
    row = {
        f"field_{j}": (i * j if j % 3 == 0 else f"value_{i}_{j}") for j in range(n_cols)
    }
    if i % 3 == 0:
        row["extra_marker"] = i
    return row


def make_data(n_rows: int, n_cols: int) -> list:
    return [make_row(i, n_cols) for i in range(n_rows)]


# Same shape as make_row, but every 5th column is a date-shaped string and
# every 7th is a full datetime -- a realistic mix of temporal and ordinary
# columns, not an all-date table (which would be unrealistically favorable).
def make_row_with_dates(i: int, n_cols: int) -> dict:
    row = {}
    for j in range(n_cols):
        if j % 5 == 0:
            day = 1 + (i + j) % 28
            row[f"field_{j}"] = f"2024-{1 + (j % 12):02d}-{day:02d}"
        elif j % 7 == 0:
            hour = i % 24
            row[f"field_{j}"] = f"2024-01-{1 + (i % 28):02d}T{hour:02d}:30:00Z"
        else:
            row[f"field_{j}"] = i * j if j % 3 == 0 else f"value_{i}_{j}"
    if i % 3 == 0:
        row["extra_marker"] = i
    return row


def make_date_data(n_rows: int, n_cols: int) -> list:
    return [make_row_with_dates(i, n_cols) for i in range(n_rows)]


def time_once(data: list, target: str, convert_dates: bool = False) -> float:
    tools = jt.JSONTools().flatten()
    if convert_dates:
        tools = tools.convert_dates(True)
    start = time.perf_counter()
    tools.execute(data, normalise=True, target=target)
    return time.perf_counter() - start


def time_once_pyspark(data: list, spark) -> float:
    tools = jt.JSONTools().flatten()
    sdf = spark.createDataFrame(
        [{"json": __import__("json").dumps(row)} for row in data]
    )
    start = time.perf_counter()
    tools.execute(sdf)
    return time.perf_counter() - start


def bench(data: list, target: str, repeats: int, convert_dates: bool = False) -> float:
    times = [time_once(data, target, convert_dates) for _ in range(repeats)]
    return statistics.median(times)


def bench_pyspark(data: list, spark, repeats: int) -> float:
    times = [time_once_pyspark(data, spark) for _ in range(repeats)]
    return statistics.median(times)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--csv", action="store_true", help="Print CSV instead of a table"
    )
    parser.add_argument(
        "--repeats", type=int, default=3, help="Median of this many runs per scenario"
    )
    parser.add_argument(
        "--with-pyspark",
        action="store_true",
        help="Also benchmark the PySpark path (requires pyspark; starts a local SparkSession)",
    )
    args = parser.parse_args()

    targets = ["pandas", "polars", "pyarrow"]
    available_targets = []
    for target in targets:
        try:
            __import__(target)
            available_targets.append(target)
        except ImportError:
            pass

    spark = None
    if args.with_pyspark:
        try:
            from pyspark.sql import SparkSession

            spark = (
                SparkSession.builder.master("local[2]")
                .appName("json_tools_rs_bench_normalise")
                .getOrCreate()
            )
            spark.sparkContext.setLogLevel("ERROR")
        except ImportError:
            print("--with-pyspark given but pyspark is not installed; skipping")

    rows_out = []
    for name, n_rows, n_cols in SCENARIOS:
        data = make_data(n_rows, n_cols)
        for target in available_targets:
            t = bench(data, target, args.repeats)
            rows_out.append((name, n_rows, n_cols, target, t))
        if spark is not None:
            t = bench_pyspark(data, spark, args.repeats)
            rows_out.append((name, n_rows, n_cols, "pyspark", t))

    for name, n_rows, n_cols in DATE_SCENARIOS:
        # date_worst_case reuses the plain (dateless) data generator, still
        # with convert_dates(True) on -- the adversarial "pays the detection
        # cost, gets nothing for it" case; the other two use real date/
        # datetime-bearing rows.
        data = (
            make_data(n_rows, n_cols)
            if name == "date_worst_case"
            else make_date_data(n_rows, n_cols)
        )
        for target in available_targets:
            t = bench(data, target, args.repeats, convert_dates=True)
            rows_out.append((name, n_rows, n_cols, target, t))

    if args.csv:
        print("scenario,rows,cols,target,median_seconds")
        for name, n_rows, n_cols, target, t in rows_out:
            print(f"{name},{n_rows},{n_cols},{target},{t:.6f}")
    else:
        header = (
            f"{'scenario':<10} {'rows':>6} {'cols':>6} {'target':<10} {'median_s':>10}"
        )
        print(header)
        print("-" * len(header))
        for name, n_rows, n_cols, target, t in rows_out:
            print(f"{name:<10} {n_rows:>6} {n_cols:>6} {target:<10} {t:>10.4f}")


if __name__ == "__main__":
    main()
