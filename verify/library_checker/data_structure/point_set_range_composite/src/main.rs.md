---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/segment_tree/src/lib.rs
    title: crates/ds/segment_tree/segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
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
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':question:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_set_range_composite
    links:
    - https://judge.yosupo.jp/problem/point_set_range_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite\n\
    \nuse modint::ModInt998244353;\nuse monoid_util::affine::{Affine, AffineOperator};\n\
    use proconio::{fastout, input};\nuse segment_tree::SegmentTree;\n\ntype Mint =\
    \ ModInt998244353;\ntype SegTree = SegmentTree<AffineOperator<Mint, true>>;\n\n\
    #[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize\n \
    \   }\n    let mut f = Vec::<Affine<Mint>>::new();\n    for _ in 0..n {\n    \
    \    input! {\n            a: u32,\n            b: u32\n        }\n        f.push(Affine(Mint::raw(a),\
    \ Mint::raw(b)));\n    }\n    let mut seg = f.into_iter().collect::<SegTree>();\n\
    \    for _ in 0..q {\n        input! {\n            t: usize\n        }\n    \
    \    match t {\n            0 => {\n                input! {\n               \
    \     p: usize,\n                    c: u32,\n                    d: u32\n   \
    \             }\n                seg.set(p, Affine(Mint::raw(c), Mint::raw(d)));\n\
    \            }\n            1 => {\n                input! {\n               \
    \     l: usize,\n                    r: usize,\n                    x: u32\n \
    \               }\n                let folded_f = seg.prod(l..r);\n          \
    \      println!(\"{}\", folded_f.eval(Mint::raw(x)));\n            }\n       \
    \     _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/ds/segment_tree/segment_tree/src/lib.rs
  - crates/modint/src/lib.rs
  - crates/modint/src/modulus.rs
  - crates/modint/src/static_modint.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-08-31 18:19:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/point_set_range_composite/src/main.rs
- /verify/verify/library_checker/data_structure/point_set_range_composite/src/main.rs.html
title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
---
