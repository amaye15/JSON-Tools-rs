#!/usr/bin/env python3
"""
JSON Tools RS - Feature by Feature (Python)

One isolated example per JSONTools builder feature. Companion to
feature_combinations.py, which shows curated multi-feature pipelines.
Mirrors examples/feature_by_feature.rs and the Java equivalent under jvm/examples/.
"""

import json_tools_rs


def main() -> None:
    print("JSON Tools RS - Feature by Feature")
    print("===================================\n")

    # 1. Mode: flatten
    print("1. Mode: .flatten()")
    data = {"user": {"name": "John", "address": {"city": "NYC", "zip": "10001"}}}
    out = json_tools_rs.JSONTools().flatten().execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 2. Mode: unflatten
    print("2. Mode: .unflatten()")
    data = {"user.name": "John", "user.address.city": "NYC"}
    out = json_tools_rs.JSONTools().unflatten().execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 3. Mode: normal (transform in place, no restructuring)
    print("3. Mode: .normal()")
    data = {"user": {"name": "John", "age": None}}
    out = json_tools_rs.JSONTools().normal().remove_nulls(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}")
    print("   Note: nulls removed but nesting preserved (no dot notation)\n")

    # 4. .separator()
    print("4. .separator()")
    data = {"user": {"profile": {"city": "NYC"}}}
    out = json_tools_rs.JSONTools().flatten().separator("::").execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 5. .lowercase_keys()
    print("5. .lowercase_keys()")
    data = {"User": {"Name": "John"}}
    out = json_tools_rs.JSONTools().flatten().lowercase_keys(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 6. .key_replacement() - literal
    print("6. .key_replacement() - literal match")
    data = {"user_name": "John"}
    out = json_tools_rs.JSONTools().flatten().key_replacement("user_", "").execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 7. .key_replacement() - regex (wrap pattern in r'...')
    print("7. .key_replacement() - regex match")
    data = {"user_id": 1, "account_id": 2}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .key_replacement("r'_id$'", "_key")
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 8. .value_replacement() - literal
    print("8. .value_replacement() - literal match")
    data = {"email": "john@example.com"}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .value_replacement("@example.com", "@company.org")
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 9. .value_replacement() - regex
    print("9. .value_replacement() - regex match")
    data = {"phone": "555-1234", "fax": "555-5678"}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .value_replacement("r'^555-'", "10-555-")
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 10. .remove_empty_strings()
    print("10. .remove_empty_strings()")
    data = {"name": "John", "bio": ""}
    out = json_tools_rs.JSONTools().flatten().remove_empty_strings(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 11. .remove_nulls()
    print("11. .remove_nulls()")
    data = {"name": "John", "age": None}
    out = json_tools_rs.JSONTools().flatten().remove_nulls(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 12. .remove_empty_objects()
    print("12. .remove_empty_objects()")
    data = {"name": "John", "meta": {}}
    out = json_tools_rs.JSONTools().flatten().remove_empty_objects(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 13. .remove_empty_arrays()
    print("13. .remove_empty_arrays()")
    data = {"name": "John", "tags": []}
    out = json_tools_rs.JSONTools().flatten().remove_empty_arrays(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 14. .handle_key_collision()
    print("14. .handle_key_collision()")
    data = {"user_name": "John", "admin_name": "Jane"}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .key_replacement("r'^(user|admin)_'", "")
        .handle_key_collision(True)
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}")
    print("   Note: colliding keys are collected into an array\n")

    # 15. .auto_convert_types()
    print("15. .auto_convert_types()")
    data = {"id": "123", "price": "$19.99", "active": "true"}
    out = json_tools_rs.JSONTools().flatten().auto_convert_types(True).execute(data)
    print(f"   In:  {data}\n   Out: {out}\n")

    # 16. .max_array_index() - DoS guard during unflatten
    print("16. .max_array_index()")
    ok_data = {"items.0": "a", "items.1": "b"}
    ok_out = json_tools_rs.JSONTools().unflatten().max_array_index(10).execute(ok_data)
    print(f"   Within limit -> In:  {ok_data}\n                  Out: {ok_out}")
    bad_data = {"items.9999": "x"}
    try:
        json_tools_rs.JSONTools().unflatten().max_array_index(10).execute(bad_data)
        print("   Unexpected success for out-of-range index")
    except json_tools_rs.JsonToolsError as e:
        print(f"   Exceeds limit  -> In:  {bad_data}\n                  Err: {e}\n")

    # 17. Parallel processing tuning knobs
    print("17. .parallel_threshold() / .num_threads() / .nested_parallel_threshold()")
    batch = [{"id": i, "data": {"value": i * 10}} for i in range(200)]
    tools = (
        json_tools_rs.JSONTools()
        .flatten()
        .parallel_threshold(50)
        .num_threads(4)
        .nested_parallel_threshold(200)
    )
    results = tools.execute(batch)
    print(f"   Processed {len(results)} documents with tuned parallelism")
    print(f"   Sample: {results[0]}\n")

    # 18. Batch processing - a single execute() call over many documents
    print("18. Batch processing (list input -> list output, type preserved)")
    batch = [{"a": {"b": 1}}, {"c": {"d": 2}}]
    results = json_tools_rs.JSONTools().flatten().execute(batch)
    print(f"   In:  {batch}\n   Out: {results}\n")


if __name__ == "__main__":
    main()
