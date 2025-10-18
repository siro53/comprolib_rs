---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/add.rs
    title: crates/traits/monoid/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/affine.rs
    title: crates/traits/monoid/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_and.rs
    title: crates/traits/monoid/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_or.rs
    title: crates/traits/monoid/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_xor.rs
    title: crates/traits/monoid/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max.rs
    title: crates/traits/monoid/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max_with_index.rs
    title: crates/traits/monoid/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/min.rs
    title: crates/traits/monoid/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/mul.rs
    title: crates/traits/monoid/src/mul.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
    title: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/segment_tree/src/lib.rs
    title: crates/ds/segment_tree/segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/add.rs
    title: crates/traits/monoid/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/affine.rs
    title: crates/traits/monoid/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_and.rs
    title: crates/traits/monoid/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_or.rs
    title: crates/traits/monoid/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_xor.rs
    title: crates/traits/monoid/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max.rs
    title: crates/traits/monoid/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max_with_index.rs
    title: crates/traits/monoid/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/min.rs
    title: crates/traits/monoid/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/mul.rs
    title: crates/traits/monoid/src/mul.rs
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
  code: "use std::marker::PhantomData;\n\nuse crate::Monoid;\nuse numeric::bound::BoundedAbove;\n\
    \n#[derive(Clone, Copy)]\n#[allow(unused)]\npub struct MinWithIndex<T> {\n   \
    \ min_value: T,\n    min_index: usize,\n}\n\npub struct MinWithIndexOperator<T>(PhantomData<fn()\
    \ -> T>);\n\nimpl<T> Monoid for MinWithIndexOperator<T>\nwhere\n    T: Copy +\
    \ Ord + BoundedAbove,\n{\n    type ValueType = MinWithIndex<T>;\n\n    fn op(left_value:\
    \ &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {\n    \
    \    if left_value.min_value > right_value.min_value {\n            *right_value\n\
    \        } else {\n            *left_value\n        }\n    }\n\n    fn unit()\
    \ -> Self::ValueType {\n        MinWithIndex {\n            min_value: T::max_value(),\n\
    \            min_index: 0,\n        }\n    }\n}\n"
  dependsOn:
  - crates/traits/monoid/src/add.rs
  - crates/traits/monoid/src/affine.rs
  - crates/traits/monoid/src/bitwise_and.rs
  - crates/traits/monoid/src/bitwise_or.rs
  - crates/traits/monoid/src/bitwise_xor.rs
  - crates/traits/monoid/src/lib.rs
  - crates/traits/monoid/src/max.rs
  - crates/traits/monoid/src/max_with_index.rs
  - crates/traits/monoid/src/min.rs
  - crates/traits/monoid/src/mul.rs
  isVerificationFile: false
  path: crates/traits/monoid/src/min_with_index.rs
  requiredBy:
  - crates/ds/segment_tree/segment_tree/src/lib.rs
  - crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
  - crates/traits/monoid/src/affine.rs
  - crates/traits/monoid/src/mul.rs
  - crates/traits/monoid/src/add.rs
  - crates/traits/monoid/src/bitwise_and.rs
  - crates/traits/monoid/src/max_with_index.rs
  - crates/traits/monoid/src/bitwise_xor.rs
  - crates/traits/monoid/src/min.rs
  - crates/traits/monoid/src/max.rs
  - crates/traits/monoid/src/lib.rs
  - crates/traits/monoid/src/bitwise_or.rs
  timestamp: '2025-10-18 15:26:53+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - verify/yukicoder/yuki789/yuki789_1/src/main.rs
documentation_of: crates/traits/monoid/src/min_with_index.rs
layout: document
redirect_from:
- /library/crates/traits/monoid/src/min_with_index.rs
- /library/crates/traits/monoid/src/min_with_index.rs.html
title: crates/traits/monoid/src/min_with_index.rs
---
