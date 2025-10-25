---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
  - icon: ':warning:'
    path: crates/prelude/src/lib.rs
    title: crates/prelude/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[allow(dead_code)]\npub trait Modulus: 'static + Copy + Eq {\n    const\
    \ MOD: u32;\n}\n"
  dependsOn:
  - crates/modint/src/lib.rs
  - crates/modint/src/static_modint.rs
  isVerificationFile: false
  path: crates/modint/src/modulus.rs
  requiredBy:
  - crates/prelude/src/lib.rs
  - crates/modint/src/lib.rs
  - crates/modint/src/static_modint.rs
  timestamp: '2025-08-31 18:19:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
documentation_of: crates/modint/src/modulus.rs
layout: document
redirect_from:
- /library/crates/modint/src/modulus.rs
- /library/crates/modint/src/modulus.rs.html
title: crates/modint/src/modulus.rs
---
