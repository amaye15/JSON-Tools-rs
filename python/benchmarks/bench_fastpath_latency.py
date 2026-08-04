"""Quick, hand-timed single-threaded latency benchmark for the flat-DataFrame
fast path -- plain .flatten().execute(df) on pandas/Polars/PyArrow, i.e.
execute_arrow_fastpath()/execute_pandas_fastpath() in src/python.rs (added in
v0.9.24; NOT the normalise=True path, see bench_normalise.py for that).

This round wrapped both functions' per-column work in py.detach() to release
the GIL during large-DataFrame calls (see bench_fastpath_gil.py for what that
buys). detach()/reacquire is a real but tiny (a single SuspendAttach guard)
cost, so this script exists to confirm -- not assume -- that it doesn't
regress the fast path's own single-threaded latency. Run once on the parent
commit and once after the GIL-release change, and compare medians directly.

Deliberately informational, not statistically rigorous, mirroring
bench_normalise.py's scope/style.

Usage:
    python python/benchmarks/bench_fastpath_latency.py
    python python/benchmarks/bench_fastpath_latency.py --csv
    python python/benchmarks/bench_fastpath_latency.py --repeats 5
"""

import argparse
import statistics
import time

import json_tools_rs as jt
from _dataframes import build_dataframe, make_flat_data

# ("name", rows, cols) -- "large" matches the v0.9.24 CHANGELOG's own
# reference scale (20K rows x 20 cols) for this fast path.
SCENARIOS = [
    ("small", 500, 10),
    ("medium", 5_000, 20),
    ("large", 20_000, 20),
]


def time_once(df) -> float:
    tools = jt.JSONTools().flatten()
    start = time.perf_counter()
    tools.execute(df)
    return time.perf_counter() - start


def bench(df, repeats: int) -> float:
    times = [time_once(df) for _ in range(repeats)]
    return statistics.median(times)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--csv", action="store_true", help="Print CSV instead of a table"
    )
    parser.add_argument(
        "--repeats", type=int, default=5, help="Median of this many runs per scenario"
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

    rows_out = []
    for name, n_rows, n_cols in SCENARIOS:
        data = make_flat_data(n_rows, n_cols)
        for target in available_targets:
            df = build_dataframe(data, target)
            t = bench(df, args.repeats)
            rows_out.append((name, n_rows, n_cols, target, t))

    if args.csv:
        print("scenario,rows,cols,backend,median_seconds")
        for name, n_rows, n_cols, target, t in rows_out:
            print(f"{name},{n_rows},{n_cols},{target},{t:.6f}")
    else:
        header = (
            f"{'scenario':<10} {'rows':>7} {'cols':>6} {'backend':<10} {'median_s':>10}"
        )
        print(header)
        print("-" * len(header))
        for name, n_rows, n_cols, target, t in rows_out:
            print(f"{name:<10} {n_rows:>7} {n_cols:>6} {target:<10} {t:>10.4f}")


if __name__ == "__main__":
    main()
