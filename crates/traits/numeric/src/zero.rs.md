---
data:
  _extendedDependsOn:
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
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
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
  code: "pub trait Zero {\n    fn zero() -> Self;\n}\n\nmacro_rules! impl_zero_integer\
    \ {\n    ($($ty:ty),*) => {\n        $(\n            impl Zero for $ty {\n   \
    \             #[inline]\n                fn zero() -> Self {\n               \
    \     0\n                }\n            }\n        )*\n    };\n}\n\nmacro_rules!\
    \ impl_zero_float {\n    ($($ty:ty),*) => {\n        $(\n            impl Zero\
    \ for $ty {\n                #[inline]\n                fn zero() -> Self {\n\
    \                    0.\n                }\n            }\n        )*\n    };\n\
    }\n\nimpl_zero_integer!(\n    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64,\
    \ i128, isize\n);\n\nimpl_zero_float!(f32, f64);\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  isVerificationFile: false
  path: crates/traits/numeric/src/zero.rs
  requiredBy:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/one.rs
  - crates/modint/src/lib.rs
  - crates/util/monoid_util/src/lib.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/traits/numeric/src/zero.rs
layout: document
redirect_from:
- /library/crates/traits/numeric/src/zero.rs
- /library/crates/traits/numeric/src/zero.rs.html
title: crates/traits/numeric/src/zero.rs
---
