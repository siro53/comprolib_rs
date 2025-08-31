---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
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
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
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
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
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
  code: "use std::{\n    marker::PhantomData,\n    ops::{Add, Mul},\n};\n\nuse monoid::Monoid;\n\
    use numeric::{one::One, zero::Zero};\n\n#[derive(Clone)]\npub struct Affine<T>(pub\
    \ T, pub T);\n\nimpl<T> Affine<T>\nwhere\n    T: Clone + Add<Output = T> + Mul<Output\
    \ = T>,\n{\n    pub fn eval(self, x: T) -> T {\n        self.0 * x + self.1\n\
    \    }\n}\n\npub struct AffineOperator<T>(PhantomData<fn() -> T>);\n\nimpl<T>\
    \ Monoid for AffineOperator<T>\nwhere\n    T: Copy + Add<Output = T> + Mul<Output\
    \ = T> + Zero + One,\n{\n    type ValueType = Affine<T>;\n\n    fn op(left_value:\
    \ &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {\n    \
    \    Affine(\n            left_value.0 * right_value.0,\n            left_value.0\
    \ * right_value.1 + left_value.1,\n        )\n    }\n\n    fn unit() -> Self::ValueType\
    \ {\n        Affine(T::one(), T::zero())\n    }\n}\n"
  dependsOn:
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: false
  path: crates/util/monoid_util/src/affine.rs
  requiredBy:
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/src/main.rs
documentation_of: crates/util/monoid_util/src/affine.rs
layout: document
redirect_from:
- /library/crates/util/monoid_util/src/affine.rs
- /library/crates/util/monoid_util/src/affine.rs.html
title: crates/util/monoid_util/src/affine.rs
---
