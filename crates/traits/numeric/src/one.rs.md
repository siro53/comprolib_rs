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
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/binary_trie/src/lib.rs
    title: crates/ds/binary_trie/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/cumulative_sum_2d/src/lib.rs
    title: crates/misc/cumulative_sum_2d/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/fraction/src/lib.rs
    title: crates/misc/fraction/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':warning:'
    path: crates/prelude/src/lib.rs
    title: crates/prelude/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
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
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
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
  code: "pub trait One {\n    fn one() -> Self;\n}\n\nmacro_rules! impl_one_integer\
    \ {\n    ($($ty:ty),*) => {\n        $(\n            impl One for $ty {\n    \
    \            #[inline]\n                fn one() -> Self {\n                 \
    \   1\n                }\n            }\n        )*\n    };\n}\n\nmacro_rules!\
    \ impl_one_float {\n    ($($ty:ty),*) => {\n        $(\n            impl One for\
    \ $ty {\n                #[inline]\n                fn one() -> Self {\n     \
    \               1.\n                }\n            }\n        )*\n    };\n}\n\n\
    impl_one_integer!(\n    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128,\
    \ isize\n);\n\nimpl_one_float!(f32, f64);\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/traits/numeric/src/one.rs
  requiredBy:
  - crates/prelude/src/lib.rs
  - crates/traits/monoid/src/lib.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/zero.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/ds/binary_trie/src/lib.rs
  - crates/modint/src/lib.rs
  - crates/misc/cumulative_sum_2d/src/lib.rs
  - crates/misc/fraction/src/lib.rs
  - crates/graph/graph/src/lib.rs
  timestamp: '2025-08-31 11:54:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/traits/numeric/src/one.rs
layout: document
redirect_from:
- /library/crates/traits/numeric/src/one.rs
- /library/crates/traits/numeric/src/one.rs.html
title: crates/traits/numeric/src/one.rs
---
