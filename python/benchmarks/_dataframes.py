"""Shared DataFrame builders for the flat-DataFrame fast-path benchmarks
(bench_fastpath_gil.py, bench_fastpath_latency.py) -- factored out since both
scripts need the same eligible/ineligible DataFrame shapes, and duplicating
them risked the two scripts' scenarios silently drifting apart over time.
"""


def make_flat_row(i: int, n_cols: int) -> dict:
    """An all-scalar row -- no nested/struct columns, so a DataFrame built
    from these is flat-DataFrame-fast-path-eligible."""
    row = {"id": i}
    for j in range(n_cols - 1):
        if j % 4 == 0:
            row[f"field_{j}"] = i * j
        elif j % 4 == 1:
            row[f"field_{j}"] = f"value_{i}_{j}"
        elif j % 4 == 2:
            row[f"field_{j}"] = (i + j) % 2 == 0
        else:
            row[f"field_{j}"] = f"{i * 1.5 + j:.2f}"
    return row


def make_ineligible_row(i: int, n_cols: int) -> dict:
    """Same shape as make_flat_row, but with one nested column -- disqualifies
    the flat-DataFrame fast path, forcing the existing (already GIL-detached)
    text-round-trip path. Used as a known-good reference point, not a "bad"
    scenario."""
    row = make_flat_row(i, n_cols)
    row["meta"] = {"nested": i}
    return row


def make_flat_data(n_rows: int, n_cols: int) -> list:
    return [make_flat_row(i, n_cols) for i in range(n_rows)]


def make_ineligible_data(n_rows: int, n_cols: int) -> list:
    return [make_ineligible_row(i, n_cols) for i in range(n_rows)]


def build_dataframe(data: list, backend: str):
    """Build a DataFrame/Table for `backend` ("pandas"/"polars"/"pyarrow")
    from a list of dicts."""
    if backend == "pandas":
        import pandas as pd

        return pd.DataFrame(data)
    if backend == "polars":
        import polars as pl

        return pl.DataFrame(data)
    if backend == "pyarrow":
        import pyarrow as pa

        return pa.Table.from_pylist(data)
    raise ValueError(f"unknown backend: {backend}")
