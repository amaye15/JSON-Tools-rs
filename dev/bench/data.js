window.BENCHMARK_DATA = {
  "lastUpdate": 1783923870683,
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
          "id": "ad7acba31f8888d1c251e7f476ce2993e41ff64d",
          "message": "Fix clippy::useless_conversion in python.rs\n\nRemove redundant .into_iter() calls in .zip() arguments; clippy 1.95\nflags these as errors under -D warnings.\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T06:51:23+10:00",
          "tree_id": "ae6d7fc40167b7589ff5074144f3dee82ae73b74",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/ad7acba31f8888d1c251e7f476ce2993e41ff64d"
        },
        "date": 1779658839074,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1806,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7293,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 10362,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 49414,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 25779,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 175689,
            "range": "± 808",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2706803,
            "range": "± 10359",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23217472,
            "range": "± 119331",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10501,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 10769,
            "range": "± 893",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 10547,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 10768,
            "range": "± 902",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3132,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20310,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 53145,
            "range": "± 154",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6988458,
            "range": "± 28478",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5533,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6067,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30927,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36130,
            "range": "± 528",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 72326,
            "range": "± 894",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78789,
            "range": "± 791",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10380529,
            "range": "± 48713",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12092001,
            "range": "± 105950",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3214,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3284,
            "range": "± 223",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 20137,
            "range": "± 856",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21303,
            "range": "± 367",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37259,
            "range": "± 2632",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40770,
            "range": "± 1759",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5371449,
            "range": "± 15302",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5947427,
            "range": "± 13049",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1528,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1808,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1797,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1807,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 10546,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 10420,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 10410,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8690,
            "range": "± 868",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25773,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 25764,
            "range": "± 231",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 25801,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25799,
            "range": "± 1143",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1801604,
            "range": "± 16597",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1798980,
            "range": "± 17796",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1801346,
            "range": "± 22606",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1799664,
            "range": "± 5807",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1781,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 10596,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 25834,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1813903,
            "range": "± 26763",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2341,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2338,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1555,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2379,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8941,
            "range": "± 927",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21938,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19671,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 43913,
            "range": "± 352",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1799078,
            "range": "± 14402",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4602666,
            "range": "± 32186",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5704,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33834,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78324,
            "range": "± 292",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11533062,
            "range": "± 44271",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3963,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25826,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48485,
            "range": "± 1294",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7415028,
            "range": "± 48626",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7086,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 63260,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 117556,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19042549,
            "range": "± 185232",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 9076,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10828,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 58490,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 93713,
            "range": "± 1688",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 198565,
            "range": "± 932",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274577,
            "range": "± 1155",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25518041,
            "range": "± 130104",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36719981,
            "range": "± 188218",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1809,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 90377,
            "range": "± 1034",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8702,
            "range": "± 649",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 151146,
            "range": "± 1052",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 17726,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 185915,
            "range": "± 2581",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 75454,
            "range": "± 3757",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 219703,
            "range": "± 2419",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 180197,
            "range": "± 824",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 245504,
            "range": "± 1791",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1833015,
            "range": "± 6548",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1829678,
            "range": "± 4731",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1831065,
            "range": "± 6882",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1834540,
            "range": "± 10830",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1975,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3645,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3652,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3774,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 2994,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3722,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3601,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3728,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1977,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3477,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3646,
            "range": "± 330",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6550,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3669,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8501,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11913,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9613,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13680,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3683,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5621,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8421,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 6247,
            "range": "± 200",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9389,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1983,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1977,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 2035,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1974,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1989,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1980,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2518,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3698,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 7523,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 907,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 920,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3220,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5207,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3133,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1654,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 4165,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3507,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6619,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6883,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3377,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6626,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9847,
            "range": "± 207",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 9867,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3391,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8847,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5642,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10941,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3410,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3380,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7287,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7226,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3402,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3362,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3397,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3353,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3425,
            "range": "± 401",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 4331,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6530,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8666,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7787,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9133,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8821,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9345,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1349,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1038,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1803,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1351,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1473,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3374,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8789,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7268,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3340,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12182,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3392,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19582,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 5108,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13728,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 14252,
            "range": "± 412",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4020,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17406,
            "range": "± 222",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6722,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7125,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13132,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16038,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3535,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11036,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5709,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4309,
            "range": "± 472",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11371,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13030,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3736,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13394,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10192,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 607,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1200,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1088,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2390,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2173,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4547,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4210,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8218,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4536,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 37393,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 21092,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 301111,
            "range": "± 2891",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 41762,
            "range": "± 455",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 421184,
            "range": "± 3699",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 208176,
            "range": "± 3025",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1401668,
            "range": "± 44317",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 18386,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 18983,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 94343,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 94543,
            "range": "± 1482",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 188843,
            "range": "± 541",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 194519,
            "range": "± 752",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1009272,
            "range": "± 4155",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1040457,
            "range": "± 3492",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1981,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3831,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4784,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 140776,
            "range": "± 1045",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 142437,
            "range": "± 1314",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 721896,
            "range": "± 41440",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 719875,
            "range": "± 2885",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1449086,
            "range": "± 14983",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1451022,
            "range": "± 17033",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2453,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2383,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17404,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 886,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 19279,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1790,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1791,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1712,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1721,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1625,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13468,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 269108,
            "range": "± 3857",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 271839,
            "range": "± 1877",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 270258,
            "range": "± 1250",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 478547,
            "range": "± 1811",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 480824,
            "range": "± 4187",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 479916,
            "range": "± 1561",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 479981,
            "range": "± 6811",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 480614,
            "range": "± 1757",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "ad7acba31f8888d1c251e7f476ce2993e41ff64d",
          "message": "Fix clippy::useless_conversion in python.rs\n\nRemove redundant .into_iter() calls in .zip() arguments; clippy 1.95\nflags these as errors under -D warnings.\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-24T20:51:23Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/ad7acba31f8888d1c251e7f476ce2993e41ff64d"
        },
        "date": 1779699894378,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1519,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7311,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8449,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48095,
            "range": "± 701",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19586,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 174991,
            "range": "± 631",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2737441,
            "range": "± 58752",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23270515,
            "range": "± 672909",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10308,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 10694,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8498,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8852,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3105,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20082,
            "range": "± 597",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 53916,
            "range": "± 767",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7030170,
            "range": "± 62709",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5462,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6082,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30689,
            "range": "± 4802",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36407,
            "range": "± 1022",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 72751,
            "range": "± 7739",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78090,
            "range": "± 1049",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10305750,
            "range": "± 44420",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 11973088,
            "range": "± 59326",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3245,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3265,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19656,
            "range": "± 867",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21546,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37738,
            "range": "± 649",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40848,
            "range": "± 615",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5368501,
            "range": "± 34761",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6035417,
            "range": "± 26130",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1791,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1518,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1501,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1526,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8463,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8440,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8442,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 10267,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19599,
            "range": "± 1686",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 20260,
            "range": "± 2836",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 20078,
            "range": "± 2260",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 20072,
            "range": "± 2603",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1838707,
            "range": "± 11010",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1840587,
            "range": "± 19442",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1837544,
            "range": "± 18070",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1837831,
            "range": "± 19312",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1505,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8468,
            "range": "± 461",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 25521,
            "range": "± 241",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1838206,
            "range": "± 20716",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2329,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2348,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1528,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2046,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8606,
            "range": "± 319",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21817,
            "range": "± 864",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19680,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 37837,
            "range": "± 1212",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1836808,
            "range": "± 13345",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4595696,
            "range": "± 34126",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 6014,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33794,
            "range": "± 996",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78868,
            "range": "± 770",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11565255,
            "range": "± 256429",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4013,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 26530,
            "range": "± 788",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48375,
            "range": "± 590",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7420184,
            "range": "± 44630",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7091,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 64072,
            "range": "± 717",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 119944,
            "range": "± 1471",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19175156,
            "range": "± 192259",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8988,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10393,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 56549,
            "range": "± 835",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 92626,
            "range": "± 935",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 197799,
            "range": "± 3493",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 273105,
            "range": "± 838",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25885714,
            "range": "± 635392",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36822409,
            "range": "± 225966",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1543,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 92772,
            "range": "± 1528",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8775,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 159310,
            "range": "± 1860",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14799,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 198229,
            "range": "± 3019",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 75392,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 235468,
            "range": "± 5717",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 150955,
            "range": "± 678",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 251071,
            "range": "± 4163",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1857762,
            "range": "± 22907",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1855678,
            "range": "± 23052",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1857626,
            "range": "± 21921",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1858638,
            "range": "± 20306",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1990,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3611,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3613,
            "range": "± 260",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3797,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3636,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3747,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3673,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3741,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1981,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3472,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3684,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6435,
            "range": "± 267",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3670,
            "range": "± 348",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8379,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11723,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9333,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13836,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3668,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5659,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8785,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5783,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 10066,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1989,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1976,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 2013,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1973,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1983,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 2006,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2483,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3678,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 7517,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 947,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 956,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3296,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5146,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3312,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1784,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3353,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3468,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6624,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6929,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3366,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6483,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9797,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 9683,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3374,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8707,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5594,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 11028,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3341,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3369,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7254,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7254,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3391,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3336,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3363,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4143,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3369,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3440,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6536,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 9461,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6993,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9232,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8833,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9387,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1366,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1043,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1809,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1357,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1489,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3379,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9568,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7355,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3339,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12359,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3371,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19801,
            "range": "± 154",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 5121,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13655,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 14120,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4694,
            "range": "± 348",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17263,
            "range": "± 584",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6687,
            "range": "± 483",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7083,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13763,
            "range": "± 780",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16175,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2936,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10866,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5651,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4282,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11245,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 12852,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3943,
            "range": "± 579",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13441,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10305,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 608,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1198,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1065,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2368,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2159,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4393,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4102,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8168,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4397,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36792,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 22166,
            "range": "± 489",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 304653,
            "range": "± 2620",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 42745,
            "range": "± 1004",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 425975,
            "range": "± 14437",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 207471,
            "range": "± 6505",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1408192,
            "range": "± 36944",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 18360,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19118,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 93931,
            "range": "± 2180",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 96827,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 189390,
            "range": "± 2088",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 196366,
            "range": "± 1225",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1009579,
            "range": "± 3174",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1042759,
            "range": "± 2048",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1958,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3759,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4760,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 140818,
            "range": "± 1016",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 140758,
            "range": "± 874",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 722950,
            "range": "± 3822",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 722654,
            "range": "± 2749",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1446538,
            "range": "± 18117",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1446247,
            "range": "± 17069",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2414,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2363,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17454,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 880,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 17691,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1776,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1744,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1715,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1703,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1628,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13659,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 269719,
            "range": "± 3244",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 275901,
            "range": "± 2373",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 273259,
            "range": "± 1798",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 504087,
            "range": "± 2222",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 504580,
            "range": "± 7512",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 504418,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 503552,
            "range": "± 3339",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 503879,
            "range": "± 2945",
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
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T20:41:57+10:00",
          "tree_id": "f94dcf6b05f8dfc27b85a089ded899f0c6371a7d",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1779708642202,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1552,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7572,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8533,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48884,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19907,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 175588,
            "range": "± 820",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2696521,
            "range": "± 12837",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23672205,
            "range": "± 298254",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10356,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8843,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8694,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8853,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3111,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20144,
            "range": "± 508",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 53317,
            "range": "± 406",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7010114,
            "range": "± 34369",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5446,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5994,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30803,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36016,
            "range": "± 516",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 72199,
            "range": "± 621",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78366,
            "range": "± 579",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10342359,
            "range": "± 52529",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12132408,
            "range": "± 94834",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3237,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3284,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19881,
            "range": "± 171",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21875,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37437,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 41432,
            "range": "± 621",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5452637,
            "range": "± 58949",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6066571,
            "range": "± 53974",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1537,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1569,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1783,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1538,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8479,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8483,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 10486,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8471,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19931,
            "range": "± 576",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19983,
            "range": "± 451",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19705,
            "range": "± 478",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25671,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1824439,
            "range": "± 5755",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1823980,
            "range": "± 14994",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1824645,
            "range": "± 18103",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1830447,
            "range": "± 18508",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1514,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8509,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 25694,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1833259,
            "range": "± 23275",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2321,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2357,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1834,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2300,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 10486,
            "range": "± 846",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21869,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19969,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 37274,
            "range": "± 1221",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1818775,
            "range": "± 16472",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4580587,
            "range": "± 21272",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5668,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33512,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78079,
            "range": "± 466",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11520483,
            "range": "± 86493",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3981,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 26439,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48316,
            "range": "± 764",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7446514,
            "range": "± 36488",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7093,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 62451,
            "range": "± 917",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 117127,
            "range": "± 1440",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19119371,
            "range": "± 252695",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8940,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10621,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 57421,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 91961,
            "range": "± 461",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 195371,
            "range": "± 1697",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274708,
            "range": "± 1696",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25785093,
            "range": "± 272123",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 34354674,
            "range": "± 707079",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1540,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 91865,
            "range": "± 1118",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7385,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 155866,
            "range": "± 1658",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 15238,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 192093,
            "range": "± 3082",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 78779,
            "range": "± 7198",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 224595,
            "range": "± 2539",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 150872,
            "range": "± 1658",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 249388,
            "range": "± 6656",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1827182,
            "range": "± 14392",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1825085,
            "range": "± 5021",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1834865,
            "range": "± 11781",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1826565,
            "range": "± 5049",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 2032,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3031,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 2959,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3772,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3623,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3749,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3705,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3781,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1992,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3495,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3689,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6590,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3665,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8584,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11876,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9883,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 14084,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3689,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5650,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8532,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5786,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9390,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1678,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1960,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1988,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1979,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1964,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1993,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2498,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3656,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 7507,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 905,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 929,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3278,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5263,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3142,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1655,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3356,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3470,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6652,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6842,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3360,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 7387,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10226,
            "range": "± 342",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10190,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 4216,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8810,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5478,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10797,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3385,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 4210,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7241,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7164,
            "range": "± 388",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3548,
            "range": "± 276",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3335,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3510,
            "range": "± 291",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4140,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 4219,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 4279,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 7351,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 9371,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7707,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9200,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8891,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9262,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1357,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1033,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1801,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1353,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1472,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 4160,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9214,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7225,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3331,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 11957,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3372,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19389,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6413,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13799,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 14148,
            "range": "± 216",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3918,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17066,
            "range": "± 214",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6762,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5431,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13247,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16052,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3499,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10938,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6298,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4275,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11299,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 12801,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3717,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13425,
            "range": "± 215",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10187,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 605,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1192,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1063,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2382,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2129,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4359,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4144,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8308,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4582,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36900,
            "range": "± 960",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 21953,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 305247,
            "range": "± 1880",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 42806,
            "range": "± 647",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 422628,
            "range": "± 3908",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 214153,
            "range": "± 2975",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1410420,
            "range": "± 9695",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 18691,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19367,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 95556,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 99115,
            "range": "± 198",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 192177,
            "range": "± 1436",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 198242,
            "range": "± 2921",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1019547,
            "range": "± 4918",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1054227,
            "range": "± 4538",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1975,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3748,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4818,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 140446,
            "range": "± 1158",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 138369,
            "range": "± 2413",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 733949,
            "range": "± 5049",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 733532,
            "range": "± 5376",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1484550,
            "range": "± 24354",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1463612,
            "range": "± 15131",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2450,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2355,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17732,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 1470,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 17721,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 2351,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1760,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1713,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1721,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1627,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13502,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 267786,
            "range": "± 3768",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 270335,
            "range": "± 2024",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 266842,
            "range": "± 2916",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 480725,
            "range": "± 7801",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 480891,
            "range": "± 8728",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 481481,
            "range": "± 3602",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 480855,
            "range": "± 4183",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 481134,
            "range": "± 2968",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1780308616348,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1463,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7154,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8380,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 46589,
            "range": "± 306",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19371,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 171936,
            "range": "± 714",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2704974,
            "range": "± 21911",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 21531658,
            "range": "± 908202",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8307,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8708,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8320,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8674,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3075,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20100,
            "range": "± 241",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 53734,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7254897,
            "range": "± 31929",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5638,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6334,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 31205,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36892,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 72328,
            "range": "± 2868",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 80043,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10991402,
            "range": "± 98095",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12902026,
            "range": "± 127815",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3163,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3363,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19249,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21443,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37636,
            "range": "± 416",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40496,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5251754,
            "range": "± 16619",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6024312,
            "range": "± 26273",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1494,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1465,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1466,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1469,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8274,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8342,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8289,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8265,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25872,
            "range": "± 223",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19422,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19353,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 19470,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1793752,
            "range": "± 12936",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1797150,
            "range": "± 7827",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1798328,
            "range": "± 19800",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1798574,
            "range": "± 12393",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1425,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8347,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19261,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1797482,
            "range": "± 29103",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2337,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2350,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1459,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1888,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8327,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 20174,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19594,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 34861,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1797257,
            "range": "± 14686",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4166190,
            "range": "± 18582",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5877,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33799,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78310,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 12336514,
            "range": "± 84534",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4005,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25493,
            "range": "± 161",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48668,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7588121,
            "range": "± 43024",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7115,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 62410,
            "range": "± 958",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 114964,
            "range": "± 508",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19319113,
            "range": "± 114940",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8687,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10325,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 55081,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 89551,
            "range": "± 845",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 193456,
            "range": "± 601",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274128,
            "range": "± 1149",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 23434686,
            "range": "± 658980",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 35509993,
            "range": "± 559413",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1464,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 82237,
            "range": "± 3321",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7055,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 139280,
            "range": "± 1983",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 13979,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 183938,
            "range": "± 3888",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 85512,
            "range": "± 803",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 203624,
            "range": "± 2461",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 142564,
            "range": "± 2690",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 237121,
            "range": "± 2792",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1799551,
            "range": "± 9740",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1799355,
            "range": "± 6328",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1801645,
            "range": "± 16267",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1799335,
            "range": "± 12176",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1678,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3065,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3022,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3150,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3034,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3120,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3035,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3241,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1696,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3211,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3101,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5782,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3888,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8061,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 10991,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8999,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13192,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3101,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 4973,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7928,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5210,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8845,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1698,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1677,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1689,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1670,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1685,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1685,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2138,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3050,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6582,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 876,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 892,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2725,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4501,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3030,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1624,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3372,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3385,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6599,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6938,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3396,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6479,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9957,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10202,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3396,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8739,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5312,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10877,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3401,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3456,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 6813,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6776,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3412,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3365,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3389,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3363,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3301,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3459,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6403,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8740,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7075,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9243,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8941,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9497,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1327,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1008,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1760,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1352,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1408,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3387,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8946,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7180,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3350,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12207,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3302,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 18659,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 4779,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13273,
            "range": "± 370",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 13927,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3559,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16629,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6175,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5207,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13413,
            "range": "± 1031",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16449,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2762,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10675,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5551,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4168,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11006,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13027,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3638,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 14321,
            "range": "± 557",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 11330,
            "range": "± 626",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 614,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1203,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1132,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2503,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2259,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4631,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4218,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8591,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4792,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36417,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 22577,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 287357,
            "range": "± 4749",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 44684,
            "range": "± 4526",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 410785,
            "range": "± 3548",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 225322,
            "range": "± 8370",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1390150,
            "range": "± 72280",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 19113,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19912,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 97640,
            "range": "± 630",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 101042,
            "range": "± 371",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 194318,
            "range": "± 841",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 201518,
            "range": "± 905",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1053949,
            "range": "± 3866",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1077488,
            "range": "± 2928",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1761,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3494,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4540,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 145299,
            "range": "± 901",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 145184,
            "range": "± 601",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 752507,
            "range": "± 1855",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 752456,
            "range": "± 4193",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1497015,
            "range": "± 17751",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1497394,
            "range": "± 8519",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2583,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2502,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 16561,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 901,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 14111,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1683,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1505,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1467,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1509,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1407,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 15019,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 248587,
            "range": "± 3576",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 247938,
            "range": "± 3032",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 245082,
            "range": "± 8048",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 492031,
            "range": "± 1221",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 491428,
            "range": "± 1524",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 491092,
            "range": "± 1035",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 491196,
            "range": "± 2385",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 491130,
            "range": "± 8269",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1780911452548,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1513,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7061,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8386,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 46534,
            "range": "± 882",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19531,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 173792,
            "range": "± 549",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2716984,
            "range": "± 16423",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23171521,
            "range": "± 805293",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8309,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8705,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8323,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8800,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3154,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 20127,
            "range": "± 653",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 54020,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7164748,
            "range": "± 31691",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5557,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6164,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30553,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 35547,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 72842,
            "range": "± 878",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78922,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10781905,
            "range": "± 54491",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12547394,
            "range": "± 53839",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3068,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3217,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 18981,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21376,
            "range": "± 472",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 36139,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40221,
            "range": "± 474",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5266289,
            "range": "± 26312",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6134689,
            "range": "± 18705",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1483,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1480,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1481,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1484,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8366,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8323,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8465,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8404,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19713,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19380,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19328,
            "range": "± 368",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 19277,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1800762,
            "range": "± 16966",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1801924,
            "range": "± 83262",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1801158,
            "range": "± 19158",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1799742,
            "range": "± 13495",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1470,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8423,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19349,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1804134,
            "range": "± 23110",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2277,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2335,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1518,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1968,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8355,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 20144,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19352,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 35300,
            "range": "± 631",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1801574,
            "range": "± 23857",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4183434,
            "range": "± 132843",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5703,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33040,
            "range": "± 614",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78071,
            "range": "± 876",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 12094033,
            "range": "± 232361",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3949,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25537,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 47339,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7555292,
            "range": "± 159334",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7054,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 61553,
            "range": "± 535",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 112332,
            "range": "± 1738",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 18895385,
            "range": "± 323617",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8862,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10392,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 55952,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 92769,
            "range": "± 430",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 193897,
            "range": "± 3059",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 281822,
            "range": "± 823",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 26092230,
            "range": "± 147048",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 34345711,
            "range": "± 193631",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1525,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 81477,
            "range": "± 691",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8730,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 143883,
            "range": "± 1923",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14469,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 185965,
            "range": "± 7768",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 71828,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 212190,
            "range": "± 1806",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 150074,
            "range": "± 2022",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 243239,
            "range": "± 3186",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1796000,
            "range": "± 10274",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1797123,
            "range": "± 17359",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1794359,
            "range": "± 17201",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1797302,
            "range": "± 17420",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 2060,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3051,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3046,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3090,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3046,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3115,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3047,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3191,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1679,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3180,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3040,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5920,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3041,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7777,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11162,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8865,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13179,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3103,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5027,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7940,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5172,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8842,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1708,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1689,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1701,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 2038,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1717,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1714,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2156,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3111,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6497,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 871,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 886,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2708,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4536,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3056,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1600,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3306,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3507,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6619,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7160,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3412,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6464,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9992,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10011,
            "range": "± 533",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3440,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 9011,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5339,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10846,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3390,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3295,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 6952,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6807,
            "range": "± 185",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3403,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3276,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3331,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3279,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3293,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3443,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6521,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8778,
            "range": "± 207",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6921,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9355,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 9065,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9517,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1348,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1013,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1822,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1367,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1482,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3294,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8973,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7163,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3413,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12182,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3383,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 18938,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6254,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13158,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 14276,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3647,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16565,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6212,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5342,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 12914,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16358,
            "range": "± 348",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2880,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11224,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6216,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4110,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 10890,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13074,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3654,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13999,
            "range": "± 391",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10525,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 623,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1215,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1128,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2446,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2128,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4661,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4223,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8642,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4848,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 35894,
            "range": "± 730",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 23136,
            "range": "± 491",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 288811,
            "range": "± 3432",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 45795,
            "range": "± 586",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 406907,
            "range": "± 3119",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 225483,
            "range": "± 39695",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1388381,
            "range": "± 39433",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 19443,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19950,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 96742,
            "range": "± 695",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 100212,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 194871,
            "range": "± 699",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 202195,
            "range": "± 1355",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1051735,
            "range": "± 3609",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1082123,
            "range": "± 9540",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1764,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3507,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4521,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 144907,
            "range": "± 1281",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 145332,
            "range": "± 2143",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 741855,
            "range": "± 2527",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 743906,
            "range": "± 7977",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1493370,
            "range": "± 5582",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1494000,
            "range": "± 17716",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2165,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2156,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 16863,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 831,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 14002,
            "range": "± 654",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1718,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1514,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1497,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1507,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1454,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14512,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 250033,
            "range": "± 3814",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 248067,
            "range": "± 3019",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 244572,
            "range": "± 9833",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 489879,
            "range": "± 2010",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 490630,
            "range": "± 9137",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 490840,
            "range": "± 2434",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 490872,
            "range": "± 4247",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 490058,
            "range": "± 7603",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1781520350721,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1509,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7052,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8443,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48477,
            "range": "± 2825",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19830,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 176701,
            "range": "± 9815",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2603267,
            "range": "± 16042",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23249873,
            "range": "± 106237",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8355,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8707,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8345,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8709,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3091,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 19672,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 52044,
            "range": "± 332",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6803257,
            "range": "± 38773",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5337,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5977,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30576,
            "range": "± 1037",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36043,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71021,
            "range": "± 901",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 77568,
            "range": "± 454",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10361310,
            "range": "± 56540",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12056326,
            "range": "± 221303",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3170,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3219,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19090,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 20992,
            "range": "± 171",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 36968,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 39897,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5336838,
            "range": "± 29736",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5946553,
            "range": "± 18954",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1520,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1509,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1527,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1515,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8341,
            "range": "± 664",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8334,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8356,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 10163,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19474,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19776,
            "range": "± 166",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19816,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25501,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1747010,
            "range": "± 28413",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1746550,
            "range": "± 18564",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1747144,
            "range": "± 11137",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1746749,
            "range": "± 15547",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1483,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8422,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19897,
            "range": "± 372",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1745346,
            "range": "± 5767",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2293,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2335,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1550,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2003,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8389,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21852,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25148,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 37144,
            "range": "± 601",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1746536,
            "range": "± 12877",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4612843,
            "range": "± 25896",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5530,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 32882,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 76626,
            "range": "± 683",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11375082,
            "range": "± 38715",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3952,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25645,
            "range": "± 669",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 53715,
            "range": "± 2488",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7297335,
            "range": "± 26847",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7007,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 62804,
            "range": "± 937",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 117857,
            "range": "± 738",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19052094,
            "range": "± 537996",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8799,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10578,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 57707,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 92735,
            "range": "± 397",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 198511,
            "range": "± 1164",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 273891,
            "range": "± 868",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25229953,
            "range": "± 543969",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36698825,
            "range": "± 854902",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1519,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 90637,
            "range": "± 1542",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7147,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 148212,
            "range": "± 1263",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14473,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 187704,
            "range": "± 2367",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 73189,
            "range": "± 816",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 217287,
            "range": "± 1928",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 147323,
            "range": "± 2471",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 248542,
            "range": "± 1797",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1712918,
            "range": "± 17587",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1714345,
            "range": "± 13867",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1714377,
            "range": "± 12518",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1714294,
            "range": "± 9149",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1748,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3829,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3096,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3161,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3085,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3158,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3081,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3164,
            "range": "± 360",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1767,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3243,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3092,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5795,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3099,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7738,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11209,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8732,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 12880,
            "range": "± 153",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3109,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5732,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7845,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5211,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8958,
            "range": "± 481",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 2105,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1787,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1755,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1758,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1761,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1806,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2215,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3066,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6840,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 884,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 893,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3455,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4594,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3109,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1614,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3308,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3474,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6637,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6954,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 4218,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6485,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 9723,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10111,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3362,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8707,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5461,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10943,
            "range": "± 409",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 4195,
            "range": "± 390",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 4167,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7082,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7015,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3385,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3323,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3347,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3352,
            "range": "± 372",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3355,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3457,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6475,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8708,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6931,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9198,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8857,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9331,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1358,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1045,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1780,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1371,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1461,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 4161,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8883,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7281,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3298,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12211,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3358,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19329,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6354,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 14703,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 15183,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4435,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17517,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 7189,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 6862,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 14512,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17517,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3376,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11449,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6145,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5310,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 12368,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13713,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 4629,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 14131,
            "range": "± 133",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 11041,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 666,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1250,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1197,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2487,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2398,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4624,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4491,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8645,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4818,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36959,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 22870,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 288860,
            "range": "± 2333",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 45525,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 403293,
            "range": "± 12698",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 226270,
            "range": "± 894",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1377084,
            "range": "± 34330",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 20482,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 21119,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 105298,
            "range": "± 401",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 108659,
            "range": "± 768",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 211176,
            "range": "± 4399",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 217832,
            "range": "± 889",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1136995,
            "range": "± 6931",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1170097,
            "range": "± 3816",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1837,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3564,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4689,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 162970,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 162569,
            "range": "± 584",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 819702,
            "range": "± 6752",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 824002,
            "range": "± 1770",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1645415,
            "range": "± 26341",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1647350,
            "range": "± 19862",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2218,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2127,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17321,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 848,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 13137,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1714,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1556,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1486,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1495,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1447,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14414,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 250806,
            "range": "± 3348",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 248866,
            "range": "± 2447",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 246538,
            "range": "± 3885",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 485975,
            "range": "± 6111",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 484758,
            "range": "± 6057",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 484426,
            "range": "± 5225",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 484651,
            "range": "± 6662",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 483130,
            "range": "± 5710",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1782124392213,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1465,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7199,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8252,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 48057,
            "range": "± 4212",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19382,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 173135,
            "range": "± 530",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2742720,
            "range": "± 17612",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23451751,
            "range": "± 106910",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8314,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8651,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8337,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8568,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3082,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 19971,
            "range": "± 575",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 54343,
            "range": "± 1120",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7309326,
            "range": "± 21661",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5697,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6165,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 31085,
            "range": "± 720",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 35914,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71903,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 79230,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10979326,
            "range": "± 56564",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12731122,
            "range": "± 76117",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3079,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3199,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 18973,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21189,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 36323,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40280,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5240131,
            "range": "± 69801",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6060935,
            "range": "± 129785",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1443,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1748,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1445,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1452,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8370,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8362,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8317,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8468,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19353,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 25677,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19547,
            "range": "± 364",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 19313,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1787654,
            "range": "± 17640",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1785489,
            "range": "± 13423",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1786254,
            "range": "± 17303",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1784389,
            "range": "± 8336",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1449,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8302,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19813,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1783686,
            "range": "± 9144",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2364,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2381,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1479,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1902,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8343,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 20237,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19338,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 34959,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1791308,
            "range": "± 26028",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4154419,
            "range": "± 21799",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5755,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33283,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 78023,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 12102797,
            "range": "± 69573",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4053,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25756,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 47374,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7513390,
            "range": "± 20435",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7167,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 59831,
            "range": "± 651",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 113766,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 18936436,
            "range": "± 88745",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8696,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10301,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 54405,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 89996,
            "range": "± 1258",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 195564,
            "range": "± 3059",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 277497,
            "range": "± 3171",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25884972,
            "range": "± 148478",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36685317,
            "range": "± 214129",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1473,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 82366,
            "range": "± 707",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7044,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 141627,
            "range": "± 1535",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14036,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 186419,
            "range": "± 4215",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 70675,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 211854,
            "range": "± 1875",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 147652,
            "range": "± 2731",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 243744,
            "range": "± 2388",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1786366,
            "range": "± 11097",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1786428,
            "range": "± 13186",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1789557,
            "range": "± 8049",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1788892,
            "range": "± 7325",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1689,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3068,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3114,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3177,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3107,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3113,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3052,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3200,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1696,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3213,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3014,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5799,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3092,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7709,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11171,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8950,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13992,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3101,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5116,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8064,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5229,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8859,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1702,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1698,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1706,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1702,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1699,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1698,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2139,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3092,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6495,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 914,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 898,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2739,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4517,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3059,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1621,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3426,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3395,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6568,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6913,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3382,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6440,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10272,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10042,
            "range": "± 610",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3287,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8758,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5424,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10878,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3283,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3281,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 6945,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6700,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3339,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3260,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3379,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3383,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3296,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3486,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6453,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8775,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6949,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9240,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8919,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9444,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1356,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1019,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1786,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1370,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1431,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3310,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9037,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7213,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3397,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12071,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3310,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 18576,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 4851,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 14495,
            "range": "± 408",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 13954,
            "range": "± 662",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3652,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16949,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6261,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 5233,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 12844,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17033,
            "range": "± 454",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2882,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10800,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5773,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4162,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11165,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13255,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3599,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13550,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 9966,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 606,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1202,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1113,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2490,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2107,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4594,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4303,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8802,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4833,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36479,
            "range": "± 1201",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 25296,
            "range": "± 155",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 288401,
            "range": "± 3585",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 44711,
            "range": "± 475",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 408526,
            "range": "± 2968",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 226889,
            "range": "± 2563",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1379587,
            "range": "± 41052",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 19061,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 19935,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 96884,
            "range": "± 296",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 100382,
            "range": "± 306",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 186451,
            "range": "± 10986",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 193949,
            "range": "± 1371",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1054157,
            "range": "± 8485",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1082890,
            "range": "± 10301",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1786,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3481,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4581,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 143902,
            "range": "± 2245",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 143979,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 743120,
            "range": "± 15566",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 742014,
            "range": "± 1591",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1489175,
            "range": "± 13574",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1487757,
            "range": "± 17215",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2207,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2086,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 16694,
            "range": "± 216",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 912,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 13973,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1626,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1566,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1513,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1531,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1459,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14808,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 251693,
            "range": "± 3317",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 250274,
            "range": "± 2874",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 247064,
            "range": "± 2131",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 492752,
            "range": "± 3344",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 492125,
            "range": "± 2179",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 492994,
            "range": "± 1078",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 492946,
            "range": "± 5775",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 493635,
            "range": "± 2286",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1782726137937,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1506,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7561,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8371,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 51132,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19831,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 175651,
            "range": "± 804",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2594731,
            "range": "± 7800",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23377721,
            "range": "± 105764",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8315,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8694,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8319,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 10703,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3122,
            "range": "± 488",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 19512,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 52584,
            "range": "± 254",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6989678,
            "range": "± 25640",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5484,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6102,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30585,
            "range": "± 999",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 35690,
            "range": "± 1076",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71455,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 78507,
            "range": "± 544",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10496588,
            "range": "± 35970",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12329523,
            "range": "± 34455",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3134,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3252,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19145,
            "range": "± 739",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21093,
            "range": "± 207",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 36705,
            "range": "± 664",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40698,
            "range": "± 799",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5291434,
            "range": "± 25732",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5934595,
            "range": "± 36546",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1522,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1548,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1494,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1510,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8323,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8328,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8312,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8310,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25625,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19865,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 25523,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25531,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1760054,
            "range": "± 28294",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1755086,
            "range": "± 7967",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1760208,
            "range": "± 12881",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1754447,
            "range": "± 6098",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1492,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8300,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 20097,
            "range": "± 2092",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1753894,
            "range": "± 138339",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2387,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2398,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1524,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2025,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8324,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 23082,
            "range": "± 860",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 19858,
            "range": "± 2574",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 36823,
            "range": "± 965",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1754501,
            "range": "± 14049",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4459223,
            "range": "± 37772",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5818,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33363,
            "range": "± 851",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 77208,
            "range": "± 3642",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11562672,
            "range": "± 26417",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4158,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25657,
            "range": "± 764",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 48047,
            "range": "± 542",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7303258,
            "range": "± 31273",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7237,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 63357,
            "range": "± 571",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 118661,
            "range": "± 1057",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19067279,
            "range": "± 119810",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8935,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10323,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 59389,
            "range": "± 587",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 93474,
            "range": "± 735",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 200823,
            "range": "± 1041",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274964,
            "range": "± 10976",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 24994894,
            "range": "± 156960",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36669263,
            "range": "± 151141",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1557,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 88376,
            "range": "± 1056",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7192,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 145026,
            "range": "± 998",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14509,
            "range": "± 1271",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 187571,
            "range": "± 2360",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 73736,
            "range": "± 561",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 214480,
            "range": "± 2397",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 146357,
            "range": "± 748",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 247406,
            "range": "± 1666",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1757109,
            "range": "± 4358",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1753169,
            "range": "± 6732",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1757892,
            "range": "± 6526",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1751796,
            "range": "± 20031",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 2080,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3062,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3054,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3126,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3069,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3155,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3059,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3149,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1717,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3190,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3068,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5830,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3058,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7780,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11233,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8765,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 12952,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3066,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 4974,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7976,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5087,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8772,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1722,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1703,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 2054,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1706,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 2060,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1715,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2199,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3796,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6766,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 885,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 921,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2734,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4615,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3128,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1583,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3328,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3476,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6956,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7340,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3411,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6825,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10293,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10267,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3392,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 9110,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5544,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 11064,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3382,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3340,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7140,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7084,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 4148,
            "range": "± 352",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3355,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3370,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4157,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 4181,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3484,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6485,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8707,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 6976,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9285,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8903,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9374,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1458,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1068,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1797,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1383,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1466,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 4194,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9295,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7307,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3357,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12326,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3358,
            "range": "± 237",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19608,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6337,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 15332,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 15283,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4417,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 17783,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 7268,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7441,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 14595,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17886,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3661,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11152,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6306,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5537,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 12546,
            "range": "± 200",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13561,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 5026,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 14360,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 11348,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 637,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1246,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1180,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2515,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2457,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4617,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4472,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8559,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4764,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36697,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 22885,
            "range": "± 207",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 299903,
            "range": "± 2105",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 45542,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 413281,
            "range": "± 2841",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 227509,
            "range": "± 23697",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1403515,
            "range": "± 11844",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 19458,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 20182,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 99161,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 100223,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 199204,
            "range": "± 358",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 201273,
            "range": "± 654",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 1063588,
            "range": "± 5846",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1102661,
            "range": "± 3800",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1835,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3584,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4626,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 149012,
            "range": "± 705",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 149016,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 770465,
            "range": "± 2339",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 770823,
            "range": "± 2491",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1544966,
            "range": "± 17006",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1546005,
            "range": "± 15816",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2174,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2120,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 16788,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 849,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 12847,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1684,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1820,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1489,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1498,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1467,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14614,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 252248,
            "range": "± 3829",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 248332,
            "range": "± 1655",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 243121,
            "range": "± 2122",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 484751,
            "range": "± 1595",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 484371,
            "range": "± 1599",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 483939,
            "range": "± 1410",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 484336,
            "range": "± 3938",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 484171,
            "range": "± 1261",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "a05c9edf2c818168e51acd35240c28d4c9cf1c40",
          "message": "Fix cargo fmt violations in python.rs after clippy fix\n\nCo-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-05-25T10:41:57Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/a05c9edf2c818168e51acd35240c28d4c9cf1c40"
        },
        "date": 1783328684701,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1529,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7511,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 9193,
            "range": "± 769",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 50617,
            "range": "± 375",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 24966,
            "range": "± 2566",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 179948,
            "range": "± 812",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2653272,
            "range": "± 14958",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23093799,
            "range": "± 607156",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8434,
            "range": "± 644",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 9330,
            "range": "± 770",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8440,
            "range": "± 594",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 9007,
            "range": "± 773",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3138,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 19789,
            "range": "± 467",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 52697,
            "range": "± 1455",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 7128625,
            "range": "± 80102",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5383,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5975,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 30650,
            "range": "± 519",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 36263,
            "range": "± 461",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 71444,
            "range": "± 1047",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 80313,
            "range": "± 1639",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10857543,
            "range": "± 299802",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12908806,
            "range": "± 183381",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3287,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3301,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 20073,
            "range": "± 465",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 21615,
            "range": "± 496",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37761,
            "range": "± 1846",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40901,
            "range": "± 1379",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5325208,
            "range": "± 37652",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5923084,
            "range": "± 36293",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1510,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1596,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1524,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1525,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8572,
            "range": "± 750",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 9633,
            "range": "± 776",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8485,
            "range": "± 668",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8876,
            "range": "± 783",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 22144,
            "range": "± 2649",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 21606,
            "range": "± 2453",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 22430,
            "range": "± 2531",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25182,
            "range": "± 2393",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1738966,
            "range": "± 17909",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1741447,
            "range": "± 8321",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1742763,
            "range": "± 20778",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1740918,
            "range": "± 16298",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1513,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8557,
            "range": "± 757",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 20088,
            "range": "± 2729",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1740994,
            "range": "± 17060",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2314,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2322,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1523,
            "range": "± 128",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2096,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8544,
            "range": "± 740",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 21981,
            "range": "± 433",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 22590,
            "range": "± 2656",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 38297,
            "range": "± 1366",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1730282,
            "range": "± 18759",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4561372,
            "range": "± 74414",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5700,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 33393,
            "range": "± 1278",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 76774,
            "range": "± 1419",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11529744,
            "range": "± 80752",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3987,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25874,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 47747,
            "range": "± 1438",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7355650,
            "range": "± 152751",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 7117,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 63405,
            "range": "± 298",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 118463,
            "range": "± 516",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 20013373,
            "range": "± 345260",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 9295,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10723,
            "range": "± 425",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 58897,
            "range": "± 363",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 94444,
            "range": "± 671",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 200457,
            "range": "± 3640",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 280262,
            "range": "± 5000",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 23812379,
            "range": "± 530507",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 34847131,
            "range": "± 400049",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1558,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 96763,
            "range": "± 932",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7248,
            "range": "± 559",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 160047,
            "range": "± 1284",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14786,
            "range": "± 1218",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 208685,
            "range": "± 2577",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 75823,
            "range": "± 5360",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 232442,
            "range": "± 2730",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 151470,
            "range": "± 11828",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 258948,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1774698,
            "range": "± 21646",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1768903,
            "range": "± 19211",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1775099,
            "range": "± 33480",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1771264,
            "range": "± 10163",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1891,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3142,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3151,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3210,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3140,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3194,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3105,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3947,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1766,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3299,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3149,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6127,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3125,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7886,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11362,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8905,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13127,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3127,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5296,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8720,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5473,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8987,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1898,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 2241,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1896,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 2106,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1889,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1760,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2261,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3139,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6974,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 911,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 923,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2716,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4934,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3077,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1665,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3303,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 4205,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6595,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 6829,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3364,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6422,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10079,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 9800,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3347,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 8684,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5430,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 10922,
            "range": "± 437",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3384,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3352,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7275,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 7242,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3378,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3338,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3369,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3380,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3376,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3396,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6440,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8640,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7646,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9105,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 8795,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9180,
            "range": "± 344",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1335,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1029,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1783,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1340,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1459,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3368,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 8705,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7191,
            "range": "± 314",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3285,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12138,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3329,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19551,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6441,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 15067,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 15321,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4659,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 18205,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 7502,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7028,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 14902,
            "range": "± 752",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17780,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3500,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 11349,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6083,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5406,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 12133,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13823,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 4919,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 14229,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 11182,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 634,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1250,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1138,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2479,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2266,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4532,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4273,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8473,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4769,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36825,
            "range": "± 205",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 23237,
            "range": "± 617",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 299455,
            "range": "± 3480",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 45101,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 417692,
            "range": "± 3023",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 226788,
            "range": "± 33378",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1412539,
            "range": "± 16705",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 17263,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 17893,
            "range": "± 449",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 87464,
            "range": "± 667",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 90407,
            "range": "± 299",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 176560,
            "range": "± 658",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 184241,
            "range": "± 791",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 925804,
            "range": "± 16299",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 961953,
            "range": "± 29235",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2026,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3899,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4698,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 127901,
            "range": "± 3148",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 127906,
            "range": "± 1062",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 660459,
            "range": "± 16626",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 656708,
            "range": "± 5596",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1315694,
            "range": "± 23002",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1324734,
            "range": "± 12517",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2468,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2441,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17254,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 888,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 17617,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1732,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1809,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1722,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1726,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1654,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14573,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 259027,
            "range": "± 3067",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 263723,
            "range": "± 2903",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 263835,
            "range": "± 3520",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 483234,
            "range": "± 1540",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 484351,
            "range": "± 5331",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 479993,
            "range": "± 7059",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 483340,
            "range": "± 3683",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 480416,
            "range": "± 2369",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "committer": {
            "name": "Andrew Mayes",
            "email": "andrewmayes@Andrews-MacBook-Air.local"
          },
          "id": "bb9a46386f59c312b6a14bb877b14bc442ed4018",
          "message": "Add persistent cross-commit benchmark history (CSV + CI), mirroring flash-attention-cpu's pattern\n\n- examples/bench_quick.rs: hand-timed harness (no Criterion wait) with\n  a --csv mode; 11 scenarios mapped 1:1 onto BENCHMARKS.md's existing\n  \"Performance Targets\" table.\n- examples/bench_compare.rs: diffs two commits' worth of rows from\n  benches/history.csv, joining on everything except timing so it never\n  compares across mismatched targets/thread-counts.\n- benches/history.csv: committed, append-only historical record.\n- rust-ci.yml: cross-platform job now also runs bench_quick natively on\n  each OS leg and uploads a CSV artifact; new bench-history job appends\n  those to benches/history.csv on real pushes to main/master only\n  (idempotent against reruns), committing as github-actions[bot] with\n  [skip ci]. Runs alongside the existing github-action-benchmark job,\n  not replacing it.\n- Documented in BENCHMARKS.md (\"Persistent Cross-Commit History\") and\n  CONTRIBUTING.md (\"Recording a benchmark\").\n\nCo-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>",
          "timestamp": "2026-07-12T21:30:22Z",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/bb9a46386f59c312b6a14bb877b14bc442ed4018"
        },
        "date": 1783894978365,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1787,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7287,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 10104,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 46374,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 24983,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 173342,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2596036,
            "range": "± 15205",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 23082462,
            "range": "± 807211",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10137,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 10458,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 10117,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 10463,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3307,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 21351,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 57339,
            "range": "± 483",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6780174,
            "range": "± 32241",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5750,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6200,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 32238,
            "range": "± 352",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 37196,
            "range": "± 230",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 77246,
            "range": "± 422",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 83721,
            "range": "± 462",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10228001,
            "range": "± 169514",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 12002107,
            "range": "± 36346",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3491,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3487,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 21286,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 22760,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 43099,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 46070,
            "range": "± 408",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5333012,
            "range": "± 20707",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5883169,
            "range": "± 15962",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1757,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1767,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1814,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1805,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 10130,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 10125,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 10133,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 10114,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 24760,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 25372,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 25353,
            "range": "± 216",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25271,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1755308,
            "range": "± 28066",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1741077,
            "range": "± 19802",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1740438,
            "range": "± 14284",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1744682,
            "range": "± 14008",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1781,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 10165,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 24931,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1748263,
            "range": "± 14878",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2320,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2358,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1775,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 2088,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 10082,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 20262,
            "range": "± 337",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25038,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 39725,
            "range": "± 259",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1745902,
            "range": "± 19320",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4906693,
            "range": "± 15285",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5862,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 35604,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 84145,
            "range": "± 439",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11478288,
            "range": "± 44796",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4213,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 27406,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 53653,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7354055,
            "range": "± 67424",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 6741,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 60652,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 118324,
            "range": "± 846",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 19111406,
            "range": "± 80733",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 8917,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10090,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 57074,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 89750,
            "range": "± 265",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 201253,
            "range": "± 1773",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274071,
            "range": "± 1514",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 25222512,
            "range": "± 167381",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 36677332,
            "range": "± 187217",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1804,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 94629,
            "range": "± 1284",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8626,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 156405,
            "range": "± 1664",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 17339,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 206857,
            "range": "± 3026",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 87915,
            "range": "± 499",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 226052,
            "range": "± 1750",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 175289,
            "range": "± 1468",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 260882,
            "range": "± 1815",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1747121,
            "range": "± 8686",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1750283,
            "range": "± 9460",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1749322,
            "range": "± 4772",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1748590,
            "range": "± 18533",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1656,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 2990,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3025,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3151,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3021,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3113,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 2977,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3161,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1693,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3255,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3073,
            "range": "± 363",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5934,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3689,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 7894,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11130,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 8858,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13168,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3028,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 4956,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7972,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 4995,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 8620,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1671,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1652,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1649,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1638,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1668,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1666,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2114,
            "range": "± 153",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3014,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 5780,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 899,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 906,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2673,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 4536,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3009,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1641,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 4110,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 4237,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 7428,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7793,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 4115,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 7323,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10570,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10763,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 4134,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 9820,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 6226,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 12075,
            "range": "± 115",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 4079,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 4103,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 7030,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6971,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 4165,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 4111,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 4146,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4120,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 4164,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 4227,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 7420,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 9589,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7874,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 10093,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 9720,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 10178,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1409,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1087,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1512,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1410,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1190,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 4141,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9693,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 8005,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 4130,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12987,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 4149,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19241,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6475,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 14563,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 15346,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4748,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16741,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6522,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7031,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13853,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17552,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3513,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10552,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6063,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5423,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11438,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13654,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 4812,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13403,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10389,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 609,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1235,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1163,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2490,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2310,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4571,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4464,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8544,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4898,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 37228,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 24586,
            "range": "± 311",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 294083,
            "range": "± 3148",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 48218,
            "range": "± 838",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 413578,
            "range": "± 2583",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 240019,
            "range": "± 9938",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1393625,
            "range": "± 9041",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 17414,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 17942,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 86897,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 90245,
            "range": "± 785",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 174369,
            "range": "± 1016",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 180726,
            "range": "± 1439",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 916684,
            "range": "± 1495",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 950228,
            "range": "± 3264",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2079,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3827,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4647,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 128043,
            "range": "± 3908",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 127841,
            "range": "± 491",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 655161,
            "range": "± 2319",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 654326,
            "range": "± 3795",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1307820,
            "range": "± 31150",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1309863,
            "range": "± 6597",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2597,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2517,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17391,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 889,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 13007,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1720,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1832,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1784,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1803,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1719,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14643,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 261172,
            "range": "± 3447",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 257895,
            "range": "± 2703",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 255248,
            "range": "± 3567",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 494520,
            "range": "± 4676",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 495568,
            "range": "± 1813",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 495466,
            "range": "± 32999",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 494568,
            "range": "± 4946",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 496549,
            "range": "± 1996",
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
          "id": "38cdd56a8ac515ea9c05f92cbad3033078e891be",
          "message": "Fix RustSec advisories: bump pyo3 0.28.2->0.29.0, crossbeam-epoch 0.9.18->0.9.20, rkyv 0.8.15->0.8.17\n\n- pyo3: RUSTSEC-2026-0176 (out-of-bounds read in PyList/PyTuple iterator\n  nth/nth_back) and RUSTSEC-2026-0177 (missing Sync bound on\n  PyCFunction::new_closure). Bumped pythonize in lockstep (0.28.0 ->\n  0.29.0) since it tracks pyo3's version. Checked the 0.28->0.29\n  migration guide against this codebase's actual API usage (no\n  PyObject alias, no new_closure, no tuple-based subclass init, no\n  UTF error From impls) -- confirmed no source changes needed. Verified\n  with cargo build/test/clippy/fmt across --all-features, plus a real\n  maturin develop + pytest run (149 passed, 0 failed) against the\n  compiled 0.29 extension module.\n- crossbeam-epoch (transitive, via criterion -> rayon -> rayon-core ->\n  crossbeam-deque, dev-dependency only): RUSTSEC-2026-0204 (invalid\n  pointer dereference in fmt::Pointer for Atomic/Shared).\n- rkyv (transitive, via sonic-rs -> faststr): RUSTSEC-2026-0122\n  (unsound InlineVec::clear/SerVec::clear, panic-safety use-after-free).\n\ncargo audit and cargo deny check both clean locally after this change.\nPre-existing failures, unrelated to the benchmarking-infra changes in\nthe prior two commits.\n\nCo-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>",
          "timestamp": "2026-07-13T09:17:12+10:00",
          "tree_id": "7535f65ec5c71a1e43732ffb83ac3995e500e0fb",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/38cdd56a8ac515ea9c05f92cbad3033078e891be"
        },
        "date": 1783901242763,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1807,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7003,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 10621,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 49244,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 25676,
            "range": "± 223",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 173426,
            "range": "± 695",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2729973,
            "range": "± 19194",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 21303364,
            "range": "± 297153",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 10323,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 10661,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 10322,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 10658,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3320,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 21830,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 57627,
            "range": "± 2121",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6826886,
            "range": "± 60305",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5881,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 6217,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 32548,
            "range": "± 551",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 37340,
            "range": "± 496",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 76914,
            "range": "± 1362",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 83135,
            "range": "± 627",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10211800,
            "range": "± 43566",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 11978020,
            "range": "± 51262",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3428,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3570,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 21898,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 23087,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 43413,
            "range": "± 538",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 45906,
            "range": "± 408",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5466833,
            "range": "± 37269",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 6011711,
            "range": "± 22548",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1799,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1829,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1804,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1833,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 10333,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 10338,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 10364,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 10548,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 25774,
            "range": "± 425",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 25711,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 25774,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 25693,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1876674,
            "range": "± 16549",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1875331,
            "range": "± 11431",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1874712,
            "range": "± 11189",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1873469,
            "range": "± 15621",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1816,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 10339,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 25660,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1877551,
            "range": "± 18729",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2451,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2401,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1858,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1944,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 10354,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 19518,
            "range": "± 246",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25805,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 39228,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1877853,
            "range": "± 7843",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4825985,
            "range": "± 25439",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5796,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 34895,
            "range": "± 641",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 82557,
            "range": "± 597",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11291399,
            "range": "± 42511",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 4253,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 27434,
            "range": "± 442",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 54425,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7504517,
            "range": "± 29592",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 6825,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 59185,
            "range": "± 2434",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 116387,
            "range": "± 546",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 18821422,
            "range": "± 450994",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 9265,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 10370,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 59356,
            "range": "± 531",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 89097,
            "range": "± 335",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 200894,
            "range": "± 464",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 274708,
            "range": "± 867",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 23106645,
            "range": "± 174801",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 34379219,
            "range": "± 304479",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1857,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 94698,
            "range": "± 2542",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 8950,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 158096,
            "range": "± 2183",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 17745,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 210200,
            "range": "± 5697",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 89989,
            "range": "± 2849",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 229657,
            "range": "± 4659",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 176249,
            "range": "± 5903",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 266922,
            "range": "± 3318",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1877374,
            "range": "± 83636",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1879930,
            "range": "± 14366",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1880481,
            "range": "± 8215",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1879753,
            "range": "± 18513",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 2067,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3826,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3853,
            "range": "± 227",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3945,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3824,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3956,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3819,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3930,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 2029,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3556,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3807,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6537,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3823,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8424,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 11785,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9591,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13569,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3827,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5693,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8920,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5808,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9456,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 2034,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 2028,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 2041,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 2028,
            "range": "± 237",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 2048,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 2056,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2143,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3819,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6373,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 926,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 939,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3463,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5273,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3196,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1635,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 3686,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 3888,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6877,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7203,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 3726,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 6817,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10067,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10043,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 3740,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 9089,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 5881,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 11239,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 3681,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 3723,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 6577,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6584,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 3607,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 3608,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 3722,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 3681,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3722,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 3755,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 6804,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 8949,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 7233,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9523,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 9180,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 9624,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1346,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1083,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1455,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1359,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1139,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 3756,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9260,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7479,
            "range": "± 882",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 3611,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 12525,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 3752,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 18772,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 6559,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 14351,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 15274,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4747,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16736,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 6420,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 7163,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13811,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 17659,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3545,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10642,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 6186,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 5396,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 11208,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13854,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 4827,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 13288,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 10325,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 605,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1222,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1169,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2478,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2340,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4603,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4375,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8507,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4974,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 37229,
            "range": "± 484",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 24099,
            "range": "± 388",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 302580,
            "range": "± 2402",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 49191,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 422793,
            "range": "± 3439",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 246571,
            "range": "± 22304",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1414983,
            "range": "± 9817",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 17475,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 18457,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 87012,
            "range": "± 2246",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 94714,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 174845,
            "range": "± 874",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 190077,
            "range": "± 954",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 926632,
            "range": "± 4026",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 1005310,
            "range": "± 9649",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2042,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 4158,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4753,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 128182,
            "range": "± 736",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 128409,
            "range": "± 1225",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 654928,
            "range": "± 2580",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 655061,
            "range": "± 3490",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1316519,
            "range": "± 8623",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1312485,
            "range": "± 5612",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2564,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2520,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17330,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 900,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 13232,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1839,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1850,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1796,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1802,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1721,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14973,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 269092,
            "range": "± 3316",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 264808,
            "range": "± 4570",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 261421,
            "range": "± 5234",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 476171,
            "range": "± 1624",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 475855,
            "range": "± 4427",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 478461,
            "range": "± 1857",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 477083,
            "range": "± 4371",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 478266,
            "range": "± 1902",
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
          "id": "231931b5b574dfa103bbb447cf68496ce7f138fc",
          "message": "Revert rkyv to 0.8.15: 0.8.16+ pulls hashbrown 0.17, which breaks MSRV 1.80\n\nThe previous commit bumped rkyv 0.8.15 -> 0.8.17 to fix RUSTSEC-2026-0122.\nCI's MSRV Check (1.80) job then failed: hashbrown 0.17.x (pulled in\nstarting at rkyv 0.8.16, the minimum version satisfying that advisory's\npatched range of >=0.8.16) declares rust-version 1.85.0 and needs the\nedition2024 Cargo feature, unsupported by cargo/rustc 1.80.\n\nConfirmed empirically that no rkyv 0.8.16+ patch avoids this (0.8.16 and\n0.8.17 both pull hashbrown 0.17 in this dependency graph, transitively via\nsonic-rs -> faststr -> rkyv). RUSTSEC-2026-0122 is an \"unsound\" advisory,\nnot a \"vulnerability\" -- cargo-audit and cargo-deny both already treat it\nas non-blocking (confirmed: `cargo deny check advisories` -> \"advisories\nok\" and `cargo audit` exits 0 even with this warning present), so\nreverting trades an already-non-blocking informational warning for actual\nMSRV compatibility, which is a hard CI gate. The pyo3/pythonize/\ncrossbeam-epoch fixes from the previous commit are unaffected and stay in\nplace. Bumping the project's MSRV to clear this warning too would be a\nreal policy decision, not made here.\n\nVerified: cargo +1.80.0 check --locked --features test-assets-performance\n(the exact MSRV CI command) passes locally, alongside the full\nbuild/test/clippy/fmt/deny/audit sweep from the previous commit.\n\nCo-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>",
          "timestamp": "2026-07-13T10:12:43+10:00",
          "tree_id": "629b5731eac9bf95a8b2267e048154f30784a697",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/231931b5b574dfa103bbb447cf68496ce7f138fc"
        },
        "date": 1783904507346,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1378,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 5849,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 7526,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 38175,
            "range": "± 582",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 16672,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 133904,
            "range": "± 7222",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2020807,
            "range": "± 22815",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 18776689,
            "range": "± 493468",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 7649,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 7611,
            "range": "± 415",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 7655,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 7800,
            "range": "± 370",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 2827,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 17664,
            "range": "± 435",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 45542,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6171342,
            "range": "± 149761",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 4827,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5301,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 25538,
            "range": "± 537",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 29698,
            "range": "± 642",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 58512,
            "range": "± 1790",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 65598,
            "range": "± 850",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 9009847,
            "range": "± 198891",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 10376854,
            "range": "± 247181",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 2827,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 2909,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 16917,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 18274,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 31283,
            "range": "± 770",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 33931,
            "range": "± 728",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 4588998,
            "range": "± 58729",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 4970588,
            "range": "± 68422",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1389,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1389,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1392,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1389,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 7604,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 7648,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 7613,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 7570,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 16858,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 16812,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 16688,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 16855,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1520765,
            "range": "± 26740",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1522914,
            "range": "± 49798",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1523964,
            "range": "± 16296",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1519486,
            "range": "± 17422",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1361,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 7586,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 16722,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1517387,
            "range": "± 12558",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2098,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2176,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1392,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1469,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 7588,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 15662,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 16966,
            "range": "± 171",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 28004,
            "range": "± 433",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1521111,
            "range": "± 17645",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4026944,
            "range": "± 102221",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5177,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 28270,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 65536,
            "range": "± 1135",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 10112784,
            "range": "± 101241",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3549,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 21960,
            "range": "± 183",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 39763,
            "range": "± 1222",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 6225972,
            "range": "± 60634",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 5995,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 48701,
            "range": "± 900",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 92135,
            "range": "± 2002",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 16244557,
            "range": "± 312188",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 7322,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 8458,
            "range": "± 179",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 44660,
            "range": "± 1069",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 72045,
            "range": "± 2324",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 150642,
            "range": "± 4086",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 220870,
            "range": "± 4730",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 19659698,
            "range": "± 568757",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 29257176,
            "range": "± 522010",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1320,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 55008,
            "range": "± 1544",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 6356,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 92141,
            "range": "± 1217",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 13305,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 108731,
            "range": "± 1489",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 68242,
            "range": "± 1070",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 133124,
            "range": "± 1722",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 135332,
            "range": "± 3748",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 154018,
            "range": "± 1856",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1522607,
            "range": "± 12614",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1514325,
            "range": "± 24128",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1521061,
            "range": "± 17576",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1524003,
            "range": "± 15196",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1506,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 2818,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 2825,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 2881,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 2771,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 2873,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 2772,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 2847,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1510,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 2999,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 2748,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 5264,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 2751,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 6627,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 9347,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 7421,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 10885,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 2781,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 4345,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 7045,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 4563,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 7560,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1522,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1497,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1488,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1456,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1495,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1503,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 1567,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 2744,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 5080,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 764,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 772,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 2409,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 3931,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 2647,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1376,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 2578,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 2687,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 6014,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 5750,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 2637,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 5714,
            "range": "± 154",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 7953,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 8118,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 2645,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 7269,
            "range": "± 185",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 4332,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 9059,
            "range": "± 233",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 2632,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 2628,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 5161,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 5143,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 2638,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 2565,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 2602,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 2550,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 2632,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 2681,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 5607,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 7104,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 5896,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 7597,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 7538,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 7869,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1222,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 969,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1297,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1237,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1011,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 2616,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 7498,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 5884,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 2594,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 10172,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 2610,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 15363,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 4225,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 11188,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 11631,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 3537,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 13543,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 4857,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 4726,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 10339,
            "range": "± 244",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 13316,
            "range": "± 338",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 2468,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 8394,
            "range": "± 490",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 4955,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 3787,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 9081,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 10956,
            "range": "± 330",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 3325,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 10313,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 8413,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 455,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1008,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 942,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2057,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2041,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 3955,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 3769,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 7475,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4459,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 33862,
            "range": "± 510",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 21551,
            "range": "± 424",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 237596,
            "range": "± 2918",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 43881,
            "range": "± 572",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 374763,
            "range": "± 3885",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 218361,
            "range": "± 3020",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1509981,
            "range": "± 30438",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 15503,
            "range": "± 199",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 16392,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 79066,
            "range": "± 838",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 83260,
            "range": "± 1138",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 155076,
            "range": "± 1946",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 165187,
            "range": "± 1890",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 804765,
            "range": "± 13877",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 835782,
            "range": "± 11952",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 1535,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3578,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 3853,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 110393,
            "range": "± 1190",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 110329,
            "range": "± 1761",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 554071,
            "range": "± 6727",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 556486,
            "range": "± 4399",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1125777,
            "range": "± 15507",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1115235,
            "range": "± 16061",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2010,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 1961,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 14416,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 663,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 12430,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1452,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1436,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1392,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1386,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1315,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 13277,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 161119,
            "range": "± 2965",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 175338,
            "range": "± 2150",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 169701,
            "range": "± 2283",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 453935,
            "range": "± 5175",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 455019,
            "range": "± 4125",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 453420,
            "range": "± 3176",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 455044,
            "range": "± 13273",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 453132,
            "range": "± 4050",
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
          "id": "b39938c9035f66ab1d98b15d6efc78ec7e052e6e",
          "message": "Fix real bottlenecks found in benches/history.csv: parallel batch dispatch and unflatten's object tree\n\nCI-collected data (benches/history.csv) showed two consistent, cross-platform\nregressions worth chasing:\n\n1. batch_flatten parallel(100) was SLOWER than sequential(100) on Windows\n   (445us vs 151us) and Linux (259us vs 199us) -- the default\n   parallel_threshold was actively counterproductive. Root cause:\n   std::thread::scope spawns fresh OS threads on every .execute() call, and\n   OS thread creation costs ~40-150us on Linux/macOS, historically up to\n   ~500us on Windows -- consistent with the data. Reintroduced rayon (removed\n   in the May dependency-reduction pass, before this data existed) for its\n   persistent global work-stealing pool: builder.rs::process_batch and\n   flatten.rs::flatten_collecting_parallel now dispatch via par_iter/\n   par_chunks on the shared pool by default, only paying to build a scoped\n   custom pool when a caller explicitly overrides num_threads.\n\n2. unflatten was consistently 4.4-6.3x slower than flatten for equivalent\n   payloads, on every platform. Investigated the two engines side by side:\n   unflatten's UnflatNode::Object was an FxHashMap plus a sort_unstable() of\n   its keys at every single object node during serialization, purely to fake\n   determinism over hash-map iteration order (a leftover from an earlier\n   BTreeMap->FxHashMap migration).\n\n   First attempt replaced it with a plain Vec<(String, UnflatNode)> (linear\n   scan beats hashing for typical narrow objects, and insertion order needs\n   no sort) -- but this has a real O(n) lookup per key, and a JSON object\n   used as a keyed map (many distinct top-level keys, e.g.\n   \"user_<id>.field\" for thousands of users -- a common real-world shape)\n   turns that into O(n^2) overall. Measured before shipping it: 20K such\n   entries went from a few ms to over a second. Fixed by using\n   FxIndexMap (indexmap, fxhash's hasher) instead: O(1) average lookup like\n   the original FxHashMap, insertion order preserved like the Vec attempt,\n   no sort needed either way. Re-verified: 20K entries back down to ~26ms\n   with flat per-entry cost.\n\n   Note: unflatten's output key order is now insertion-order rather than\n   alphabetical. Not a documented contract (no test asserts exact ordering)\n   but a real, deliberate, approved output change.\n\n   Assessed the deeper redesign (a true zero-tree fast path mirroring\n   flatten's zero-allocation DirectWalker, plus caching key-path scan\n   offsets to avoid a second re-scan per entry) and chose not to ship it:\n   the fast path isn't soundly achievable without either assuming\n   pre-grouped input or unsafe lifetime tricks, and the rescan-caching\n   change's expected win (skipping one memchr-based scan of a short key\n   string) didn't justify more surgery on this correctness-critical path\n   right after the tree-structure change above.\n\nindexmap is pinned to >=2.11,<2.12 (Cargo.toml has the full reasoning):\n2.12.0 raised its own rust-version to 1.82 and 2.14.0 requires edition2024\n(~1.85), both above this crate's MSRV of 1.80; 2.11.x's MSRV is 1.63 and\navoids pulling in hashbrown 0.17 (which caused an MSRV break earlier this\nsession when bumping rkyv, for the same underlying reason).\n\nVerified: full test suite (all features), stress benchmark suite\n(cargo test --bench, correctness mode), clippy/fmt clean, cargo deny/audit\nclean, and a real maturin develop + pytest run (149 passed) against the\ncompiled extension.\n\nCo-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>",
          "timestamp": "2026-07-13T15:34:55+10:00",
          "tree_id": "0ccfc13ab67ab0082f14fada19e173607c813118",
          "url": "https://github.com/amaye15/JSON-Tools-rs/commit/b39938c9035f66ab1d98b15d6efc78ec7e052e6e"
        },
        "date": 1783923869836,
        "tool": "cargo",
        "benches": [
          {
            "name": "01_baseline/flatten/small",
            "value": 1506,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/small",
            "value": 7252,
            "range": "± 250",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/medium",
            "value": 8289,
            "range": "± 590",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/medium",
            "value": 47891,
            "range": "± 357",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/large",
            "value": 19700,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/large",
            "value": 172812,
            "range": "± 1948",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/flatten/xlarge",
            "value": 2558529,
            "range": "± 48332",
            "unit": "ns/iter"
          },
          {
            "name": "01_baseline/unflatten/xlarge",
            "value": 21803902,
            "range": "± 357532",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/dot",
            "value": 8324,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_colon",
            "value": 8754,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/underscore",
            "value": 8229,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "02_separator/medium/double_underscore",
            "value": 8581,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/small",
            "value": 3295,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/medium",
            "value": 21350,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/large",
            "value": 51500,
            "range": "± 338",
            "unit": "ns/iter"
          },
          {
            "name": "03_lowercase_keys/enabled/xlarge",
            "value": 6823016,
            "range": "± 43705",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/small",
            "value": 5295,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/small",
            "value": 5905,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/medium",
            "value": 29955,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/medium",
            "value": 34872,
            "range": "± 473",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/large",
            "value": 70230,
            "range": "± 346",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/large",
            "value": 77236,
            "range": "± 6109",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/literal/xlarge",
            "value": 10305751,
            "range": "± 73432",
            "unit": "ns/iter"
          },
          {
            "name": "04_key_replacement/regex/xlarge",
            "value": 11962197,
            "range": "± 104301",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/small",
            "value": 3142,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/small",
            "value": 3202,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/medium",
            "value": 19715,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/medium",
            "value": 20834,
            "range": "± 1140",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/large",
            "value": 37245,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/large",
            "value": 40365,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/literal/xlarge",
            "value": 5401834,
            "range": "± 26327",
            "unit": "ns/iter"
          },
          {
            "name": "05_value_replacement/regex/xlarge",
            "value": 5868656,
            "range": "± 154946",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/small",
            "value": 1510,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/small",
            "value": 1506,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/small",
            "value": 1514,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/small",
            "value": 1555,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/medium",
            "value": 8261,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/medium",
            "value": 8262,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/medium",
            "value": 8250,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/medium",
            "value": 8263,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/large",
            "value": 19675,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/large",
            "value": 19613,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/large",
            "value": 19704,
            "range": "± 361",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/large",
            "value": 19630,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_strings/xlarge",
            "value": 1678637,
            "range": "± 7813",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_nulls/xlarge",
            "value": 1678282,
            "range": "± 19520",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_objects/xlarge",
            "value": 1672910,
            "range": "± 18102",
            "unit": "ns/iter"
          },
          {
            "name": "06_individual_filters/remove_empty_arrays/xlarge",
            "value": 1673163,
            "range": "± 32039",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/small",
            "value": 1510,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/medium",
            "value": 8264,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/large",
            "value": 19620,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "07_all_filters/combined/xlarge",
            "value": 1679197,
            "range": "± 20080",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/without_collision_handling",
            "value": 2271,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "08_key_collision/with_collision_handling",
            "value": 2289,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/small",
            "value": 1573,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/small",
            "value": 1604,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/medium",
            "value": 8238,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/medium",
            "value": 17778,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/large",
            "value": 25383,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/large",
            "value": 34458,
            "range": "± 441",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/disabled/xlarge",
            "value": 1671995,
            "range": "± 6257",
            "unit": "ns/iter"
          },
          {
            "name": "09_auto_type_conversion/enabled/xlarge",
            "value": 4577082,
            "range": "± 25640",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/small",
            "value": 5590,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/medium",
            "value": 32903,
            "range": "± 270",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/large",
            "value": 76623,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "10_all_key_transformations/combined/xlarge",
            "value": 11296208,
            "range": "± 33963",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/small",
            "value": 3913,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/medium",
            "value": 25647,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/large",
            "value": 47452,
            "range": "± 254",
            "unit": "ns/iter"
          },
          {
            "name": "11_all_value_transformations/combined/xlarge",
            "value": 7325057,
            "range": "± 63639",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/small",
            "value": 6538,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/medium",
            "value": 57414,
            "range": "± 2231",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/large",
            "value": 110637,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "12_comprehensive/all_features/xlarge",
            "value": 18848681,
            "range": "± 86800",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/small",
            "value": 9004,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/small",
            "value": 9918,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/medium",
            "value": 56339,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/medium",
            "value": 87885,
            "range": "± 1128",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/large",
            "value": 191972,
            "range": "± 2186",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/large",
            "value": 268604,
            "range": "± 607",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/basic/xlarge",
            "value": 22876085,
            "range": "± 142122",
            "unit": "ns/iter"
          },
          {
            "name": "13_roundtrip/with_transformations/xlarge",
            "value": 34366969,
            "range": "± 219941",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/1",
            "value": 1527,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/1",
            "value": 1944,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/5",
            "value": 7321,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/5",
            "value": 14951,
            "range": "± 573",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/10",
            "value": 14988,
            "range": "± 200",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/10",
            "value": 19087,
            "range": "± 525",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/50",
            "value": 75084,
            "range": "± 3676",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/50",
            "value": 52314,
            "range": "± 861",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/sequential/100",
            "value": 148980,
            "range": "± 1021",
            "unit": "ns/iter"
          },
          {
            "name": "14_batch_processing/parallel/100",
            "value": 90235,
            "range": "± 1884",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/sequential",
            "value": 1712206,
            "range": "± 3893",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_50",
            "value": 1714533,
            "range": "± 8631",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_100",
            "value": 1711124,
            "range": "± 6626",
            "unit": "ns/iter"
          },
          {
            "name": "15_nested_parallelism/xlarge/threshold_500",
            "value": 1712966,
            "range": "± 70301",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/small",
            "value": 1988,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_01_baseline/flatten/medium",
            "value": 3725,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/dot",
            "value": 3730,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_colon",
            "value": 3871,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/underscore",
            "value": 3736,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/double_underscore",
            "value": 3176,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/slash",
            "value": 3734,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_02_separator_only/separator/arrow",
            "value": 3858,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/small",
            "value": 1988,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/small",
            "value": 3713,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/baseline/medium",
            "value": 3742,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "iso_03_lowercase_only/lowercase/medium",
            "value": 6892,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/baseline",
            "value": 3729,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_single",
            "value": 8787,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/literal_multiple",
            "value": 12423,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_single",
            "value": 9938,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "iso_04_key_replacement_only/regex_multiple",
            "value": 13485,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/baseline",
            "value": 3776,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_single",
            "value": 5785,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/literal_multiple",
            "value": 8600,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_single",
            "value": 5796,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "iso_05_value_replacement_only/regex_multiple",
            "value": 9609,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/baseline",
            "value": 1983,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_strings",
            "value": 1950,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_nulls",
            "value": 1984,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_objects",
            "value": 1954,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_06_filters_individual/remove_empty_arrays",
            "value": 1998,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/small",
            "value": 1985,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/small",
            "value": 2109,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/baseline/medium",
            "value": 3750,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "iso_07_auto_type_conversion_only/auto_convert/medium",
            "value": 6509,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/baseline",
            "value": 909,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "iso_08_key_collision_only/collision_handling",
            "value": 922,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/baseline",
            "value": 3380,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "iso_09_normal_mode/with_transformations",
            "value": 5514,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/basic",
            "value": 3099,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "iso_10_unflatten_only/custom_separator",
            "value": 1649,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/baseline",
            "value": 4093,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/separator_only",
            "value": 4314,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/lowercase_only",
            "value": 7459,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_01_separator_lowercase/combined",
            "value": 7777,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/baseline",
            "value": 4181,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/lowercase_only",
            "value": 7349,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/key_replacement_only",
            "value": 10630,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_02_lowercase_key_replacement/combined",
            "value": 10570,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/baseline",
            "value": 4189,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/key_replacement_only",
            "value": 9423,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/value_replacement_only",
            "value": 6223,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_03_key_value_replacement/combined",
            "value": 11873,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/baseline",
            "value": 4161,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/filters_only",
            "value": 4078,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/auto_convert_only",
            "value": 6963,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_04_filters_auto_convert/combined",
            "value": 6883,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/baseline",
            "value": 4206,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_strings_and_nulls",
            "value": 4179,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/empty_objects_and_arrays",
            "value": 4190,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "combo_2f_05_all_filters/all_four_filters",
            "value": 4172,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/baseline",
            "value": 3723,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/separator_only",
            "value": 4484,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lowercase_only",
            "value": 7324,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/key_replacement_only",
            "value": 9337,
            "range": "± 135",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_lower",
            "value": 8190,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/sep_keyrep",
            "value": 9906,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/lower_keyrep",
            "value": 9592,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_01_sep_lower_keyrep/all_three",
            "value": 10078,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/baseline",
            "value": 1380,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/filters_only",
            "value": 1056,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/auto_convert_only",
            "value": 1505,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/collision_only",
            "value": 1386,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_02_filters_convert_collision/all_three",
            "value": 1176,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/baseline",
            "value": 4141,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/key_transform",
            "value": 9654,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/value_transform",
            "value": 7975,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/filters",
            "value": 4155,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "combo_3f_03_key_value_filters/all_three",
            "value": 13143,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/baseline",
            "value": 4194,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "combo_max_all_features/maximum_features",
            "value": 19013,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/flatten",
            "value": 5828,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/typical_processing",
            "value": 13518,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_01_aws_cloudtrail/extract_key_fields",
            "value": 14527,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/flatten",
            "value": 4183,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/snake_to_camel",
            "value": 16016,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_02_github_api/extract_metrics",
            "value": 5970,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/flatten",
            "value": 6144,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/monitoring_format",
            "value": 13018,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_03_kubernetes/extract_metadata",
            "value": 16541,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/flatten",
            "value": 3117,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/log_processing",
            "value": 10018,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_04_elasticsearch/csv_export_prep",
            "value": 5747,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/flatten",
            "value": 4767,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/database_prep",
            "value": 10710,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_05_stripe_api/extract_billing",
            "value": 13014,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/flatten",
            "value": 4246,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/analytics_processing",
            "value": 12747,
            "range": "± 179",
            "unit": "ns/iter"
          },
          {
            "name": "realworld_06_twitter_api/extract_metrics",
            "value": 9849,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/10",
            "value": 623,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/10",
            "value": 1252,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/25",
            "value": 1198,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/25",
            "value": 2502,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/50",
            "value": 2360,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/50",
            "value": 4642,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten/100",
            "value": 4670,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "stress_01_deep_nesting/flatten_transform/100",
            "value": 8709,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/100",
            "value": 4979,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/100",
            "value": 36421,
            "range": "± 852",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/500",
            "value": 24475,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/500",
            "value": 247076,
            "range": "± 6208",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/1000",
            "value": 48333,
            "range": "± 155",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/1000",
            "value": 366445,
            "range": "± 7418",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten/5000",
            "value": 235420,
            "range": "± 21269",
            "unit": "ns/iter"
          },
          {
            "name": "stress_02_wide_objects/flatten_transform/5000",
            "value": 1402624,
            "range": "± 60165",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/100",
            "value": 17530,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/100",
            "value": 18230,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/500",
            "value": 90330,
            "range": "± 1358",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/500",
            "value": 94501,
            "range": "± 485",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/1000",
            "value": 179998,
            "range": "± 677",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/1000",
            "value": 186088,
            "range": "± 2553",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten/5000",
            "value": 943821,
            "range": "± 4398",
            "unit": "ns/iter"
          },
          {
            "name": "stress_03_large_arrays/flatten_auto_convert/5000",
            "value": 982948,
            "range": "± 2926",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten",
            "value": 2023,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_lowercase",
            "value": 3866,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "stress_04_unicode_heavy/flatten_replacements",
            "value": 4802,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/1000",
            "value": 133260,
            "range": "± 878",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/1000",
            "value": 132416,
            "range": "± 581",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/5000",
            "value": 676806,
            "range": "± 1308",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/5000",
            "value": 677330,
            "range": "± 2775",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten/10000",
            "value": 1356820,
            "range": "± 34731",
            "unit": "ns/iter"
          },
          {
            "name": "stress_05_many_small_objects/flatten_parallel/10000",
            "value": 1357047,
            "range": "± 13142",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten",
            "value": 2653,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/flatten_all_filters",
            "value": 2519,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "stress_06_mixed_types/roundtrip",
            "value": 17030,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten",
            "value": 904,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_value_replacement",
            "value": 13207,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "stress_07_long_strings/flatten_regex_replacement",
            "value": 1814,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/no_filters",
            "value": 1911,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_nulls_only",
            "value": 1836,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/remove_empty_strings_only",
            "value": 1817,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_08_nulls_and_empties/all_filters",
            "value": 1727,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/no_regex",
            "value": 14823,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/single_regex",
            "value": 168935,
            "range": "± 5232",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/multiple_regex",
            "value": 185764,
            "range": "± 5266",
            "unit": "ns/iter"
          },
          {
            "name": "stress_09_regex_heavy/regex_with_collision",
            "value": 182803,
            "range": "± 7023",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/sequential",
            "value": 517082,
            "range": "± 1524",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_50",
            "value": 516129,
            "range": "± 69659",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_100",
            "value": 516768,
            "range": "± 1919",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_500",
            "value": 516580,
            "range": "± 2052",
            "unit": "ns/iter"
          },
          {
            "name": "stress_10_parallel_thresholds/nested_parallel/threshold_1000",
            "value": 517544,
            "range": "± 2309",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}