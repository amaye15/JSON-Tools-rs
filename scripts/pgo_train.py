#!/usr/bin/env python3
"""PGO training driver for the Python wheel's native extension.

Exercises flatten/unflatten/normal-mode, both batch and single-item, across
literal and regex key/value replacement, all four filter options, type
conversion, and key-collision handling -- the same feature surface
`examples/bench_quick.rs` exercises for the pure-Rust side. Used to generate
`-Cprofile-generate` .profraw data before an `-Cprofile-use` optimized rebuild.
See CONTRIBUTING.md's "Profile-Guided Optimization" section for the full recipe.

Deliberately not pytest/unittest: this only needs to run realistic code paths
enough times for LLVM to see stable branch/call frequencies, not assert anything.
"""

import json
import random

import json_tools_rs

random.seed(1234)


def rand_json(depth=3, width=4):
    if depth == 0:
        return random.choice(
            [1, "x", True, None, 3.14, "$1,234.56", "true", "", "42"]
        )
    return {f"Key_{i}_{random.choice(['A', 'B'])}": rand_json(depth - 1, width) for i in range(width)}


def main():
    payloads = [rand_json() for _ in range(300)]
    strs = [json.dumps(p) for p in payloads]

    pipelines = [
        json_tools_rs.JSONTools().flatten(),
        json_tools_rs.JSONTools()
        .flatten()
        .lowercase_keys(True)
        .remove_nulls(True)
        .remove_empty_strings(True)
        .remove_empty_objects(True)
        .remove_empty_arrays(True)
        .auto_convert_types(True)
        .key_replacement("r'Key_'", "k_")
        .value_replacement("r'\\$([0-9,]+\\.[0-9]{2})'", "$1")
        .handle_key_collision(True),
        json_tools_rs.JSONTools().unflatten(),
        json_tools_rs.JSONTools().normal().lowercase_keys(True).auto_convert_types(True),
    ]

    for _ in range(15):
        flat_batch = pipelines[1].execute(strs)
        pipelines[0].execute(strs)
        pipelines[2].execute(flat_batch if isinstance(flat_batch, list) else [flat_batch])
        pipelines[3].execute(strs)
        for s in strs[:80]:
            pipelines[1].execute(s)
            pipelines[0].execute(s)

    print("PGO training run complete")


if __name__ == "__main__":
    main()
