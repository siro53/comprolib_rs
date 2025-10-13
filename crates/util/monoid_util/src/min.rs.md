---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/affine.rs
    title: crates/util/monoid_util/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_and.rs
    title: crates/util/monoid_util/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_or.rs
    title: crates/util/monoid_util/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_xor.rs
    title: crates/util/monoid_util/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max.rs
    title: crates/util/monoid_util/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/affine.rs
    title: crates/util/monoid_util/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_and.rs
    title: crates/util/monoid_util/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_or.rs
    title: crates/util/monoid_util/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_xor.rs
    title: crates/util/monoid_util/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max.rs
    title: crates/util/monoid_util/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/yuki789/yuki789_1/src/main.rs
    title: verify/yukicoder/yuki789/yuki789_1/src/main.rs
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
  code: "use std::marker::PhantomData;\n\nuse monoid::Monoid;\nuse numeric::bound::BoundedAbove;\n\
    \npub struct Min<T>(PhantomData<fn() -> T>);\n\nimpl<T> Monoid for Min<T>\nwhere\n\
    \    T: Copy + Ord + BoundedAbove,\n{\n    type ValueType = T;\n\n    fn op(left_value:\
    \ &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {\n    \
    \    std::cmp::min(*left_value, *right_value)\n    }\n\n    fn unit() -> Self::ValueType\
    \ {\n        T::max_value()\n    }\n}\n"
  dependsOn:
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: false
  path: crates/util/monoid_util/src/min.rs
  requiredBy:
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/max.rs
  timestamp: '2025-08-31 18:12:56+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/yukicoder/yuki789/yuki789_1/src/main.rs
documentation_of: crates/util/monoid_util/src/min.rs
layout: document
redirect_from:
- /library/crates/util/monoid_util/src/min.rs
- /library/crates/util/monoid_util/src/min.rs.html
title: crates/util/monoid_util/src/min.rs
---
