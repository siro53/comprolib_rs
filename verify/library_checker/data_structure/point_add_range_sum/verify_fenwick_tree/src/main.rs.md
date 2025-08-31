---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
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
    \nuse fenwick_tree::fenwick_tree::FenwickTree;\nuse proconio::{fastout, input};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [i64; n]\n    }\n    let mut fwt = FenwickTree::<i64>::new(n);\n \
    \   for i in 0..n {\n        fwt.add(i, a[i]);\n    }\n    for _ in 0..q {\n \
    \       input! {\n            t: usize\n        }\n        match t {\n       \
    \     0 => {\n                input! {\n                    p: usize,\n      \
    \              x: i64\n                }\n                fwt.add(p, x);\n   \
    \         }\n            1 => {\n                input! {\n                  \
    \  l: usize,\n                    r: usize\n                }\n              \
    \  println!(\"{}\", fwt.sum(l..r));\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-08-31 22:36:44+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
- /verify/verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs.html
title: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
---
