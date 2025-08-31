---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/traits/numeric/src/bound.rs
    title: crates/traits/numeric/src/bound.rs
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
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/bound.rs
    title: crates/traits/numeric/src/bound.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/lib.rs
    title: crates/traits/numeric/src/lib.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/one.rs
    title: crates/traits/numeric/src/one.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Infinity {\n    fn infinity() -> Self;\n}\n\npub trait NegInfinity\
    \ {\n    fn negative_infinity() -> Self;\n}\n\nmacro_rules! impl_infinity {\n\
    \    ($($t:ty, $infinity_value:expr,)*) => {\n        $(\n            impl Infinity\
    \ for $t {\n                fn infinity() -> Self {\n                    $infinity_value\n\
    \                }\n            }\n        )*\n    }\n}\n\nmacro_rules! impl_negative_infinity\
    \ {\n    ($($t:ty, $neg_infinity_value:expr,)*) => {\n        $(\n           \
    \ impl NegInfinity for $t {\n                fn negative_infinity() -> Self {\n\
    \                    $neg_infinity_value\n                }\n            }\n \
    \       )*\n    }\n}\n\nimpl_infinity! {\n    i32, 1_i32 << 30,\n    u32, 1_u32\
    \ << 30,\n    f32, 1e10,\n    i64, 1_i64 << 60,\n    isize, 1_isize << 60,\n \
    \   u64, 1_u64 << 60,\n    usize, 1_usize << 60,\n    f64, 1e20,\n    i128, 1_i128\
    \ << 120,\n    u128, 1_u128 << 120,\n}\n\nimpl_negative_infinity! {\n    i32,\
    \ -1_i32 << 30,\n    u32, 0,\n    f32, -1e10,\n    i64, -1_i64 << 60,\n    isize,\
    \ -1_isize << 60,\n    u64, 0,\n    usize, 0,\n    f64, 1e20,\n    i128, -1_i128\
    \ << 120,\n    u128, 0,\n}\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/traits/numeric/src/infinity.rs
  requiredBy:
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/zero.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/modint/src/lib.rs
  - crates/util/monoid_util/src/lib.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/traits/numeric/src/infinity.rs
layout: document
redirect_from:
- /library/crates/traits/numeric/src/infinity.rs
- /library/crates/traits/numeric/src/infinity.rs.html
title: crates/traits/numeric/src/infinity.rs
---
