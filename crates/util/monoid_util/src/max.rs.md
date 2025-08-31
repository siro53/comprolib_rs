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
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
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
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
  - icon: ':x:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
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
  code: "use std::marker::PhantomData;\n\nuse monoid::Monoid;\nuse numeric::bound::BoundedBelow;\n\
    \npub struct Max<T>(PhantomData<fn() -> T>);\n\nimpl<T> Monoid for Max<T>\nwhere\n\
    \    T: Copy + Ord + BoundedBelow,\n{\n    type ValueType = T;\n\n    fn op(left_value:\
    \ &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {\n    \
    \    std::cmp::max(*left_value, *right_value)\n    }\n\n    fn unit() -> Self::ValueType\
    \ {\n        T::min_value()\n    }\n}\n"
  dependsOn:
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: false
  path: crates/util/monoid_util/src/max.rs
  requiredBy:
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_add_range_sum/src/main.rs
documentation_of: crates/util/monoid_util/src/max.rs
layout: document
redirect_from:
- /library/crates/util/monoid_util/src/max.rs
- /library/crates/util/monoid_util/src/max.rs.html
title: crates/util/monoid_util/src/max.rs
---
