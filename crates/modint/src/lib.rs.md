---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
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
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
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
  code: "use crate::{modulus::Modulus, static_modint::StaticModInt};\n\npub mod modulus;\n\
    pub mod static_modint;\n\n#[derive(Clone, Copy, PartialEq, Eq)]\npub enum Mod998244353\
    \ {}\n\nimpl Modulus for Mod998244353 {\n    const MOD: u32 = 998_244_353;\n}\n\
    \n#[derive(Clone, Copy, PartialEq, Eq)]\npub enum Mod1000000007 {}\n\nimpl Modulus\
    \ for Mod1000000007 {\n    const MOD: u32 = 1_000_000_007;\n}\n\npub type ModInt998244353\
    \ = StaticModInt<Mod998244353>;\npub type ModInt1000000007 = StaticModInt<Mod1000000007>;\n"
  dependsOn:
  - crates/modint/src/modulus.rs
  - crates/modint/src/static_modint.rs
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/modint/src/lib.rs
  requiredBy:
  - crates/modint/src/static_modint.rs
  - crates/modint/src/modulus.rs
  timestamp: '2025-08-31 18:19:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - verify/library_checker/data_structure/point_set_range_composite_large_array/verify_dynamic_segment_tree/src/main.rs
documentation_of: crates/modint/src/lib.rs
layout: document
redirect_from:
- /library/crates/modint/src/lib.rs
- /library/crates/modint/src/lib.rs.html
title: crates/modint/src/lib.rs
---
