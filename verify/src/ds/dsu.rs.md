---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: ds/src/dsu.rs
    title: ds/src/dsu.rs
  - icon: ':heavy_check_mark:'
    path: ds/src/lib.rs
    title: ds/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    \nuse ds::dsu::Dsu;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        q: usize\n    }\n    let mut uf\
    \ = Dsu::new(n);\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \            u: usize,\n            v: usize\n        };\n        match t {\n\
    \            0 => {\n                uf.merge(u, v);\n            }\n        \
    \    1 => println!(\"{}\", uf.same(u, v) as i32),\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - ds/src/dsu.rs
  - ds/src/lib.rs
  isVerificationFile: true
  path: verify/src/ds/dsu.rs
  requiredBy: []
  timestamp: '2025-08-17 14:49:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/src/ds/dsu.rs
layout: document
redirect_from:
- /verify/verify/src/ds/dsu.rs
- /verify/verify/src/ds/dsu.rs.html
title: verify/src/ds/dsu.rs
---
