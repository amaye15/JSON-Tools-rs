"""GIL-release *effectiveness* benchmark for the flat-DataFrame fast path --
NOT a latency benchmark (see bench_fastpath_latency.py for that).

Every other execution path in src/python.rs (single string, dict, batch
list, the old DataFrame text-round-trip path) already wraps its Rust
computation in py.detach() to release the GIL, so other Python threads in
the process can keep running. Before this round, execute_arrow_fastpath()/
execute_pandas_fastpath() (the flat-DataFrame fast path added in v0.9.24)
were the one remaining gap: they held the GIL for their entire duration,
which for a large DataFrame is tens of milliseconds of pure-Rust computation
that would stall every other Python thread (a multi-threaded web server, a
ThreadPoolExecutor) for no reason.

This script measures that directly: a pure-Python counting loop runs on a
background thread while the main thread repeatedly calls execute(df), and we
compare the background thread's throughput against an uncontended baseline.

Three conditions, each run fresh:
  solo     - background thread alone (uncontended baseline).
  control  - background thread concurrent with execute(df) on a
             deliberately fast-path-*ineligible* DataFrame (one nested
             column) -- forces the existing, already-detached text-round-trip
             path. A known-good reference measured in this same harness.
  fastpath - background thread concurrent with execute(df) on an eligible
             DataFrame at the shipped fast path's own reference scale
             (20K rows x 20 cols).

Reports ratio = concurrent_iters / solo_iters for control and fastpath.
Success: fastpath's ratio lands close to control's (not dramatically lower,
which would mean the background thread was starved). To see the actual
before/after difference concretely rather than infer it, run this script
once against the parent commit (before the py.detach() wrap) and once after,
as a manual A/B during development.

Usage:
    python python/benchmarks/bench_fastpath_gil.py
    python python/benchmarks/bench_fastpath_gil.py --csv
    python python/benchmarks/bench_fastpath_gil.py --duration 3
"""

import argparse
import threading
import time

import json_tools_rs as jt
from _dataframes import build_dataframe, make_flat_data, make_ineligible_data

LARGE_ROWS = 20_000
LARGE_COLS = 20


def _counter_thread(stop_event: threading.Event, counter: list) -> None:
    n = 0
    while not stop_event.is_set():
        n += 1
    counter[0] = n


def run_background_counter(duration: float, workload) -> int:
    """Run a pure-Python counting loop on a background thread for `duration`
    seconds, optionally alongside `workload` (a zero-arg callable invoked
    repeatedly on the *main* thread for the same duration; None for the
    uncontended baseline). Returns the counter's final value."""
    stop_event = threading.Event()
    counter = [0]
    t = threading.Thread(target=_counter_thread, args=(stop_event, counter))
    t.start()

    if workload is None:
        time.sleep(duration)
    else:
        deadline = time.perf_counter() + duration
        while time.perf_counter() < deadline:
            workload()

    stop_event.set()
    t.join()
    return counter[0]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--csv", action="store_true", help="Print CSV instead of a table"
    )
    parser.add_argument(
        "--duration", type=float, default=2.0, help="Seconds per condition (default: 2.0)"
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

    flat_data = make_flat_data(LARGE_ROWS, LARGE_COLS)
    ineligible_data = make_ineligible_data(LARGE_ROWS, LARGE_COLS)

    rows_out = []

    solo_iters = run_background_counter(args.duration, None)
    rows_out.append(("solo", "-", solo_iters, 1.0))

    for target in available_targets:
        tools = jt.JSONTools().flatten()

        control_df = build_dataframe(ineligible_data, target)

        def control_workload(df=control_df, tools=tools):
            tools.execute(df)

        control_iters = run_background_counter(args.duration, control_workload)
        control_ratio = control_iters / solo_iters if solo_iters else 0.0
        rows_out.append((f"control_{target}", target, control_iters, control_ratio))

        fast_df = build_dataframe(flat_data, target)

        def fastpath_workload(df=fast_df, tools=tools):
            tools.execute(df)

        fast_iters = run_background_counter(args.duration, fastpath_workload)
        fast_ratio = fast_iters / solo_iters if solo_iters else 0.0
        rows_out.append((f"fastpath_{target}", target, fast_iters, fast_ratio))

    if args.csv:
        print("condition,backend,background_iters,ratio_to_solo")
        for name, target, iters, ratio in rows_out:
            print(f"{name},{target},{iters},{ratio:.4f}")
    else:
        header = f"{'condition':<20} {'backend':<10} {'bg_iters':>14} {'ratio':>8}"
        print(header)
        print("-" * len(header))
        for name, target, iters, ratio in rows_out:
            print(f"{name:<20} {target:<10} {iters:>14,} {ratio:>8.4f}")


if __name__ == "__main__":
    main()
