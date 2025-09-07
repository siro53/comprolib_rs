---
data:
  _extendedDependsOn:
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
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/cumulative_sum_2d/src/lib.rs
    title: crates/misc/cumulative_sum_2d/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
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
  code: "pub trait BoundedBelow {\n    fn min_value() -> Self;\n}\n\npub trait BoundedAbove\
    \ {\n    fn max_value() -> Self;\n}\n\nmacro_rules! impl_bound_integer {\n   \
    \ ($($ty:ty),*) => {\n        $(\n            impl BoundedBelow for $ty {\n  \
    \              #[inline]\n                fn min_value() -> Self {\n         \
    \           Self::MIN\n                }\n            }\n\n            impl BoundedAbove\
    \ for $ty {\n                #[inline]\n                fn max_value() -> Self\
    \ {\n                    Self::MAX\n                }\n            }\n       \
    \ )*\n    };\n}\n\nimpl_bound_integer!(\n    i8, i16, i32, i64, i128, isize, u8,\
    \ u16, u32, u64, u128, usize\n);\n"
  dependsOn:
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/traits/numeric/src/bound.rs
  requiredBy:
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/misc/cumulative_sum_2d/src/lib.rs
  - crates/traits/numeric/src/zero.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/one.rs
  - crates/modint/src/lib.rs
  - crates/util/monoid_util/src/lib.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/traits/numeric/src/bound.rs
layout: document
redirect_from:
- /library/crates/traits/numeric/src/bound.rs
- /library/crates/traits/numeric/src/bound.rs.html
title: crates/traits/numeric/src/bound.rs
---
