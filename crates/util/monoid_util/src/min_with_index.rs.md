---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/affine.rs
    title: crates/util/monoid_util/src/affine.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_and.rs
    title: crates/util/monoid_util/src/bitwise_and.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_or.rs
    title: crates/util/monoid_util/src/bitwise_or.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_xor.rs
    title: crates/util/monoid_util/src/bitwise_xor.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/max.rs
    title: crates/util/monoid_util/src/max.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedRequiredBy:
  - icon: ':question:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/affine.rs
    title: crates/util/monoid_util/src/affine.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_and.rs
    title: crates/util/monoid_util/src/bitwise_and.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_or.rs
    title: crates/util/monoid_util/src/bitwise_or.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/bitwise_xor.rs
    title: crates/util/monoid_util/src/bitwise_xor.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/max.rs
    title: crates/util/monoid_util/src/max.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - icon: ':x:'
    path: verify/yukicoder/yuki789/yuki789_1/src/main.rs
    title: verify/yukicoder/yuki789/yuki789_1/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::marker::PhantomData;\n\nuse monoid::Monoid;\nuse numeric::bound::BoundedAbove;\n\
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
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: false
  path: crates/util/monoid_util/src/min_with_index.rs
  requiredBy:
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  timestamp: '2025-08-31 18:12:56+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/yukicoder/yuki789/yuki789_1/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_add_range_sum/verify_segment_tree/src/main.rs
documentation_of: crates/util/monoid_util/src/min_with_index.rs
layout: document
redirect_from:
- /library/crates/util/monoid_util/src/min_with_index.rs
- /library/crates/util/monoid_util/src/min_with_index.rs.html
title: crates/util/monoid_util/src/min_with_index.rs
---
