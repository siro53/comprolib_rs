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
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/directed_graph_util.rs
    title: crates/graph/graph/src/directed_graph_util.rs
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
  - icon: ':heavy_check_mark:'
    path: crates/graph/heavy_light_decomposition/src/lib.rs
    title: crates/graph/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_add_path_sum
    links:
    - https://judge.yosupo.jp/problem/vertex_add_path_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum\n\
    \nuse fenwick_tree::fenwick_tree::FenwickTree;\nuse graph::graph::Tree;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     n: usize,\n        q: usize,\n        a: [i64; n]\n    }\n    let mut tree\
    \ = Tree::<usize>::new(n);\n    for _ in 0..n - 1 {\n        input! {\n      \
    \      u: usize,\n            v: usize\n        }\n        tree.add_edge(u, v,\
    \ 1);\n    }\n    let mut hld = HeavyLightDecomposition::<usize>::new(&tree);\n\
    \    hld.build(0);\n    let mut fwt = FenwickTree::<i64>::new(n);\n    for i in\
    \ 0..n {\n        fwt.add(hld[i], a[i]);\n    }\n\n    for _ in 0..q {\n     \
    \   input! {\n            t: usize\n        }\n        match t {\n           \
    \ 0 => {\n                input! {\n                    u: usize,\n          \
    \          x: i64\n                }\n                fwt.add(hld[u], x);\n  \
    \          }\n            1 => {\n                input! {\n                 \
    \   u: usize,\n                    v: usize\n                }\n             \
    \   let mut ans = 0_i64;\n                let mut f = |l: usize, r: usize| {\n\
    \                    ans += fwt.sum(l..r);\n                };\n             \
    \   hld.path_query_commutative(u, v, &mut f, false);\n                println!(\"\
    {}\", ans);\n            }\n            _ => unreachable!(),\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  - crates/graph/heavy_light_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-10-14 10:00:12+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/vertex_add_path_sum/src/main.rs
- /verify/verify/library_checker/tree/vertex_add_path_sum/src/main.rs.html
title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
---
