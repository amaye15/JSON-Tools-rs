window.BENCHMARK_DATA = {
  "lastUpdate": 1779655152515,
  "repoUrl": "https://github.com/amaye15/JSON-Tools-rs",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "andrewmayes@Andrews-MacBook-Air.local",
            "name": "Andrew Mayes"
          },
          "committer": {
            "email": "andrewmayes@Andrews-MacBook-Air.local",
            "name": "Andrew Mayes"
          },
          "distinct": true,
          "id": "d7e7b85fc64a9695e15d1913c1dc73fe9af78228",
          "message": "Remove 6 direct deps, replace with inline impls; add benchmark tracking\n\nDependency removals (14 → 7 direct deps):\n- rustc-hash: inline FxHasher in src/fxhash.rs (~60 lines)\n- phf: replace BOOL_MAP static with match in convert.rs\n- itoa: inline IntBuf stack formatter in flatten.rs; to_string() elsewhere\n- thiserror: manual Display + Error impls in error.rs\n- crossbeam: replaced with std::thread::scope (stable since 1.63)\n- dashmap: replaced with RwLock<FxHashMap> in cache.rs\n- fast-float2: replaced with inline parse_f64 wrapping str::parse\n\nCI: wire up github-action-benchmark for all 5 benchmark suites with\nhistorical tracking via gh-pages (auto-push on main, fail-on-alert on PRs)\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-24T22:32:51+10:00",
          "tree_id": "1818f00651e86f7e9ed793554750073573acfb5f",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/d7e7b85fc64a9695e15d1913c1dc73fe9af78228"
        },
        "date": 1779628920364,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1782,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7139,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 10312,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48106,
            "range": "± 1472",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19520,
            "range": "± 811",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 174555,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2673316,
            "range": "± 12022",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 21414031,
            "range": "± 326455",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10481,
            "range": "± 604",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 10662,
            "range": "± 902",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 10381,
            "range": "± 674",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8852,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3140,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20059,
            "range": "± 665",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 53221,
            "range": "± 1591",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7071960,
            "range": "± 29043",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5802,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6166,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30834,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36266,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71732,
            "range": "± 1555",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78325,
            "range": "± 1463",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10397704,
            "range": "± 31891",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12292579,
            "range": "± 49222",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3110,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3265,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19676,
            "range": "± 710",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21225,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 36772,
            "range": "± 2457",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40371,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5417093,
            "range": "± 32607",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6006126,
            "range": "± 41489",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1788,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1783,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1754,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1527,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 10284,
            "range": "± 895",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 10318,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 10293,
            "range": "± 968",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8468,
            "range": "± 691",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25487,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 25299,
            "range": "± 1686",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 25452,
            "range": "± 1621",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 19840,
            "range": "± 2817",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1829144,
            "range": "± 12071",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1814468,
            "range": "± 90431",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1815691,
            "range": "± 66888",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1812686,
            "range": "± 13547",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1765,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 10304,
            "range": "± 699",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 25342,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1811461,
            "range": "± 16799",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2322,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2339,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1779,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2344,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 10327,
            "range": "± 320",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21958,
            "range": "± 633",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25860,
            "range": "± 359",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 37802,
            "range": "± 885",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1815177,
            "range": "± 9375",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4534239,
            "range": "± 71394",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5762,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33903,
            "range": "± 455",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 77888,
            "range": "± 761",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11709335,
            "range": "± 65867",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4097,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 26264,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48275,
            "range": "± 796",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7481562,
            "range": "± 61827",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7156,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 65003,
            "range": "± 1094",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 117383,
            "range": "± 627",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19184555,
            "range": "± 133212",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8848,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10584,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 58218,
            "range": "± 731",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 93835,
            "range": "± 844",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 198302,
            "range": "± 2219",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 273829,
            "range": "± 2611",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 24951638,
            "range": "± 292464",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36460232,
            "range": "± 301004",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1522,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 90352,
            "range": "± 1382",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8637,
            "range": "± 476",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 154503,
            "range": "± 1566",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14707,
            "range": "± 1403",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 190380,
            "range": "± 3093",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 89656,
            "range": "± 2913",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 220058,
            "range": "± 2700",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 174431,
            "range": "± 13006",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 244939,
            "range": "± 3798",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1818329,
            "range": "± 35555",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1817993,
            "range": "± 23935",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1813676,
            "range": "± 8730",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1817497,
            "range": "± 23653",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 2009,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3682,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3711,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3780,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3719,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3772,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3725,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3766,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1998,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3462,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3041,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6385,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3724,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8406,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11728,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9621,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13746,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3736,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5631,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8677,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5761,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9317,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 2006,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1984,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 2040,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1987,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 2003,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 2002,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2514,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3716,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 7444,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 916,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 922,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3268,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5258,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3006,
            "range": "± 224",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1591,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 4143,
            "range": "± 368",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3456,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6645,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6912,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 4178,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6539,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9864,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 9857,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3376,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8849,
            "range": "± 355",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5501,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10938,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3392,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 4147,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7172,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7157,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 4167,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3338,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3374,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3340,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3367,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3471,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6507,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8738,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6882,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9218,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8920,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9323,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1337,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1041,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1772,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1344,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1433,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3360,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8816,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7227,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3298,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12093,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3624,
            "range": "± 335",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19335,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 5110,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13745,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 13983,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3985,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17126,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6688,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5439,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13295,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 15985,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3519,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10968,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5672,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4336,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11321,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 12917,
            "range": "± 524",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3752,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13152,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10108,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 638,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1195,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1119,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2472,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2222,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4542,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4199,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8216,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4535,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 37502,
            "range": "± 161",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 21049,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 291668,
            "range": "± 10766",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 43367,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 411528,
            "range": "± 4166",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 212195,
            "range": "± 3083",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1397347,
            "range": "± 10015",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 18560,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19238,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 94204,
            "range": "± 1858",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 96817,
            "range": "± 397",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 188846,
            "range": "± 754",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 194026,
            "range": "± 800",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1019457,
            "range": "± 7573",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1043773,
            "range": "± 2984",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2006,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3759,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4780,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 143790,
            "range": "± 1181",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 144033,
            "range": "± 443",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 746055,
            "range": "± 3461",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 745919,
            "range": "± 4463",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1507122,
            "range": "± 17231",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1485042,
            "range": "± 9671",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2456,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2390,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17587,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 930,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 17768,
            "range": "± 524",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1788,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1749,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1488,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1719,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1633,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13593,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 262522,
            "range": "± 3978",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 263867,
            "range": "± 3799",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 263340,
            "range": "± 2579",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 479213,
            "range": "± 3069",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 478453,
            "range": "± 2693",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 478203,
            "range": "± 13215",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 478923,
            "range": "± 1311",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 478345,
            "range": "± 2195",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "andrewmayes@Andrews-MacBook-Air.local",
            "name": "Andrew Mayes"
          },
          "committer": {
            "email": "andrewmayes@Andrews-MacBook-Air.local",
            "name": "Andrew Mayes"
          },
          "distinct": true,
          "id": "a6bcfe994160572cf9accbb886c76b7f158b36a1",
          "message": "Fix cargo fmt formatting violations\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T05:50:12+10:00",
          "tree_id": "1d47eb619dfe12803e000dd4d515d05c8a4305b0",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a6bcfe994160572cf9accbb886c76b7f158b36a1"
        },
        "date": 1779655152231,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1568,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7237,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8482,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48869,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 25698,
            "range": "± 2831",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 177658,
            "range": "± 2800",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2707104,
            "range": "± 18923",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23921529,
            "range": "± 620015",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8500,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8860,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 10310,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8845,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3292,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20098,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 52563,
            "range": "± 244",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6941748,
            "range": "± 64404",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5522,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5987,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30612,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 35379,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71183,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 77643,
            "range": "± 352",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10531925,
            "range": "± 89666",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12201809,
            "range": "± 69776",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3225,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3234,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19965,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21454,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37182,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40715,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5464271,
            "range": "± 82614",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6077977,
            "range": "± 54696",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1531,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1510,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1524,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1524,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 10419,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8456,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8457,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8478,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25888,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 20056,
            "range": "± 590",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 20071,
            "range": "± 239",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25853,
            "range": "± 2872",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1866535,
            "range": "± 26654",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1866010,
            "range": "± 17203",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1865694,
            "range": "± 19824",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1866871,
            "range": "± 41252",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1502,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8481,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19991,
            "range": "± 481",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1873795,
            "range": "± 13902",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2324,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2342,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1555,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2064,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8470,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21719,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25936,
            "range": "± 1234",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 37222,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1873111,
            "range": "± 15883",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4569775,
            "range": "± 14787",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5720,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33234,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 77545,
            "range": "± 528",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11702710,
            "range": "± 90863",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4006,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25861,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 49802,
            "range": "± 229",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7456886,
            "range": "± 313825",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7042,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 62731,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 116648,
            "range": "± 849",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19249641,
            "range": "± 220106",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8973,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10699,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 58554,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 93682,
            "range": "± 535",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 198920,
            "range": "± 5478",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 273888,
            "range": "± 9860",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25637411,
            "range": "± 679590",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36750716,
            "range": "± 1102943",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1556,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 88033,
            "range": "± 1324",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7406,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 144994,
            "range": "± 2817",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 15008,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 182481,
            "range": "± 3439",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 75368,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 212994,
            "range": "± 2523",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 150255,
            "range": "± 859",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 240716,
            "range": "± 4501",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1879399,
            "range": "± 12093",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1876974,
            "range": "± 19767",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1880899,
            "range": "± 17681",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1884932,
            "range": "± 13051",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1998,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3650,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3664,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3747,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3674,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3770,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3649,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3748,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1996,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3497,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3665,
            "range": "± 161",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6531,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3670,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8538,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 12012,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9560,
            "range": "± 254",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 14051,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3722,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5713,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8615,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5845,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9472,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1998,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1980,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1996,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1968,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1979,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1691,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2443,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3652,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 7424,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 940,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 913,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3307,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5496,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3064,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1751,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3361,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3539,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6635,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7744,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3385,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6640,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9838,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 9751,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3382,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8780,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5529,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 11111,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3383,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3357,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7260,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7294,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 4197,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 4167,
            "range": "± 383",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3406,
            "range": "± 418",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4187,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3399,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3470,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6627,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8750,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6985,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9117,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8888,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9333,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1366,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1085,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1806,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1375,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1498,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3398,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8863,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7336,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3326,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12405,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3367,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 20267,
            "range": "± 266",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6522,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13615,
            "range": "± 216",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 13928,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3972,
            "range": "± 335",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17032,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6691,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5572,
            "range": "± 786",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13111,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16021,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2933,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11037,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5755,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5362,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11355,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 12893,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3791,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13259,
            "range": "± 519",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10165,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 605,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1177,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1070,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2359,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2151,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4349,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4157,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8135,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4457,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36878,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 21103,
            "range": "± 280",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 289660,
            "range": "± 3131",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 41884,
            "range": "± 662",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 405028,
            "range": "± 3399",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 209524,
            "range": "± 5585",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1394863,
            "range": "± 66878",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 18299,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 18933,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 93807,
            "range": "± 609",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 96673,
            "range": "± 493",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 188351,
            "range": "± 3958",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 194167,
            "range": "± 1051",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1009090,
            "range": "± 3824",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1040814,
            "range": "± 19443",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2026,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3730,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4839,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 142329,
            "range": "± 1140",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 142225,
            "range": "± 1330",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 721266,
            "range": "± 2000",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 723927,
            "range": "± 1958",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1468821,
            "range": "± 31184",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1454709,
            "range": "± 19443",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2435,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2369,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17440,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 888,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 17634,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1723,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1742,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1719,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1714,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1619,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13364,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 256679,
            "range": "± 3219",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 254472,
            "range": "± 3028",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 253352,
            "range": "± 2485",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 481905,
            "range": "± 2411",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 481779,
            "range": "± 10389",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 482875,
            "range": "± 883",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 478401,
            "range": "± 1238",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 482079,
            "range": "± 14256",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}