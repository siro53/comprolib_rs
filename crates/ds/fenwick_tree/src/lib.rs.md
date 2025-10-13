---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/bound.rs
    title: crates/traits/numeric/src/bound.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/infinity.rs
    title: crates/traits/numeric/src/infinity.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/lib.rs
    title: crates/traits/numeric/src/lib.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/one.rs
    title: crates/traits/numeric/src/one.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
    title: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
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
  code: 'pub mod fenwick_tree;

    pub mod range_fenwick_tree;

    '
  dependsOn:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/ds/fenwick_tree/src/lib.rs
  requiredBy:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  timestamp: '2025-08-31 22:36:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
  - verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
documentation_of: crates/ds/fenwick_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/ds/fenwick_tree/src/lib.rs
- /library/crates/ds/fenwick_tree/src/lib.rs.html
title: crates/ds/fenwick_tree/src/lib.rs
---
