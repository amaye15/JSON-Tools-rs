#!/usr/bin/env python3
"""
JSON Tools RS - Feature Combinations (Python)

Curated multi-feature pipelines, building on feature_by_feature.py. Not an
exhaustive combinatorial sweep (the builder has ~10 independent toggles, so a
literal power-set would be 1000+ cases) -- these are realistic groupings of
features commonly used together, plus one "kitchen sink" example exercising
nearly everything at once. Mirrors examples/feature_combinations.rs and the
Java equivalent under jvm/examples/.
"""

import json_tools_rs


def main() -> None:
    print("JSON Tools RS - Feature Combinations")
    print("=====================================\n")

    # 1. separator + lowercase_keys + key_replacement + handle_key_collision
    print("1. separator + lowercase_keys + key_replacement + handle_key_collision")
    data = {"User": {"Full_Name": "John"}, "Admin": {"Full_Name": "Jane"}}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .separator("_")
        .lowercase_keys(True)
        .key_replacement("r'^(user|admin)_'", "")
        .handle_key_collision(True)
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 2. key_replacement + value_replacement together
    print("2. key_replacement + value_replacement")
    data = {"usr_nm": "John", "usr_eml": "john@old.com"}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .key_replacement("usr_", "user_")
        .value_replacement("@old.com", "@new.com")
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 3. All four empty-value filters together
    print(
        "3. remove_empty_strings + remove_nulls + remove_empty_objects + remove_empty_arrays"
    )
    data = {"name": "John", "bio": "", "age": None, "tags": [], "meta": {}}
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .remove_empty_strings(True)
        .remove_nulls(True)
        .remove_empty_objects(True)
        .remove_empty_arrays(True)
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 4. Real-world normalization pipeline: replacement + filtering + collision
    print("4. lowercase_keys + key_replacement + filtering + handle_key_collision")
    data = {
        "User_Name": "John",
        "User_Bio": "",
        "Admin_Name": "Jane",
        "Admin_Bio": None,
    }
    out = (
        json_tools_rs.JSONTools()
        .flatten()
        .lowercase_keys(True)
        .key_replacement("r'^(user|admin)_'", "")
        .remove_empty_strings(True)
        .remove_nulls(True)
        .handle_key_collision(True)
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 5. unflatten + key_replacement + value_replacement
    print("5. unflatten + separator + key_replacement + value_replacement")
    data = {"PREFIX_user_name": "john@OLD.com", "PREFIX_user_age": 30}
    out = (
        json_tools_rs.JSONTools()
        .unflatten()
        .separator("_")
        .key_replacement("PREFIX_user_", "profile_")
        .value_replacement("@OLD.com", "@new.com")
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}\n")

    # 6. normal mode + auto_convert_types + value_replacement + filtering
    print("6. normal + auto_convert_types + value_replacement + remove_empty_strings")
    data = {"user": {"status": "ACTIVE", "note": "", "score": "95.5"}}
    out = (
        json_tools_rs.JSONTools()
        .normal()
        .auto_convert_types(True)
        .value_replacement("r'^ACTIVE$'", "enabled")
        .remove_empty_strings(True)
        .execute(data)
    )
    print(f"   In:  {data}\n   Out: {out}")
    print(
        "   Note: nesting preserved, string replaced, empty note dropped, score converted\n"
    )

    # 7. Batch processing + parallel tuning + type conversion together
    print(
        "7. batch execute + parallel_threshold + num_threads + "
        "nested_parallel_threshold + auto_convert_types"
    )
    batch = [{"id": str(i), "active": "true"} for i in range(150)]
    tools = (
        json_tools_rs.JSONTools()
        .flatten()
        .parallel_threshold(50)
        .num_threads(4)
        .nested_parallel_threshold(100)
        .auto_convert_types(True)
    )
    results = tools.execute(batch)
    print(f"   Processed {len(results)} documents")
    print(f"   Sample: {results[0]}\n")

    # 8. Kitchen sink: (almost) every feature at once, on a realistic messy batch
    print("8. Kitchen sink - every applicable feature combined")
    api_batch = [
        {
            "API_Response": {
                "User_Data": {
                    "First_Name": "John",
                    "Email": "john@old.com",
                    "Bio": "",
                    "Score": "88.5",
                }
            }
        },
        {
            "API_Response": {
                "User_Data": {
                    "First_Name": "Jane",
                    "Email": "jane@old.com",
                    "Bio": None,
                    "Score": "91.2",
                }
            }
        },
    ]
    tools = (
        json_tools_rs.JSONTools()
        .flatten()
        .separator("::")
        .lowercase_keys(True)
        .key_replacement("r'^api_response::user_data::'", "")
        .key_replacement("first_name", "name")
        .value_replacement("@old.com", "@new.com")
        .remove_empty_strings(True)
        .remove_nulls(True)
        .remove_empty_objects(True)
        .remove_empty_arrays(True)
        .auto_convert_types(True)
        .parallel_threshold(50)
        .num_threads(2)
        .nested_parallel_threshold(200)
    )
    results = tools.execute(api_batch)
    print("   Features: separator, lowercase, 2x key_replacement, value_replacement,")
    print("             4x filtering, auto_convert_types, parallel tuning, batch")
    for i, r in enumerate(results):
        print(f"   [{i}]: {r}")


if __name__ == "__main__":
    main()
