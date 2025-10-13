---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/tree_util.rs
    title: crates/graph/graph/src/tree_util.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/tree_diameter
    links:
    - https://judge.yosupo.jp/problem/tree_diameter
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter\n\
    \nuse graph::graph::Tree;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n    }\n    let mut tree = Tree::<i64>::new(n);\n\
    \    for _ in 0..n - 1 {\n        input! {\n            a: usize,\n          \
    \  b: usize,\n            c: i64\n        }\n        tree.add_edge(a, b, c);\n\
    \    }\n    let (diam, path) = tree.diameter();\n    println!(\"{} {}\", diam,\
    \ path.len());\n    for i in 0..path.len() {\n        if i + 1 == path.len() {\n\
    \            println!(\"{}\", path[i]);\n        } else {\n            print!(\"\
    {} \", path[i]);\n        }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: true
  path: verify/library_checker/tree/tree_diameter/src/main.rs
  requiredBy: []
  timestamp: '2025-10-14 00:04:46+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/tree_diameter/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/tree_diameter/src/main.rs
- /verify/verify/library_checker/tree/tree_diameter/src/main.rs.html
title: verify/library_checker/tree/tree_diameter/src/main.rs
---
