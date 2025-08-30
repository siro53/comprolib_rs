---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/segment_tree/src/lib.rs
    title: crates/ds/segment_tree/segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_add_range_sum
    links:
    - https://judge.yosupo.jp/problem/point_add_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum\n\
    \nuse monoid::Monoid;\nuse proconio::{fastout, input};\nuse segment_tree::SegmentTree;\n\
    \nstruct M;\n\nimpl Monoid for M {\n    type ValueType = i64;\n\n    fn op(left_value:\
    \ &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {\n    \
    \    left_value + right_value\n    }\n\n    fn unit() -> Self::ValueType {\n \
    \       0\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [i64; n]\n    }\n    let mut seg = SegmentTree::<M>::new(n);\n\
    \    for i in 0..n {\n        seg.set(i, a[i]);\n    }\n    for _ in 0..q {\n\
    \        input! {\n            t: usize\n        }\n        match t {\n      \
    \      0 => {\n                input! {\n                    p: usize,\n     \
    \               x: i64\n                }\n                seg.apply(p, x);\n\
    \            }\n            1 => {\n                input! {\n               \
    \     l: usize,\n                    r: usize\n                }\n           \
    \     println!(\"{}\", seg.prod(l, r));\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/ds/segment_tree/segment_tree/src/lib.rs
  - crates/traits/monoid/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-08-30 22:12:56+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/point_add_range_sum/src/main.rs
- /verify/verify/library_checker/data_structure/point_add_range_sum/src/main.rs.html
title: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
---
