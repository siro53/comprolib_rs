---
data:
  _extendedDependsOn:
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
  _extendedRequiredBy:
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
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/tree_diameter/src/main.rs
    title: verify/library_checker/tree/tree_diameter/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
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
  code: "#[derive(Clone)]\npub struct Edge<Cost: Clone> {\n    from: usize,\n    to:\
    \ usize,\n    cost: Cost,\n    id: usize,\n}\n\nimpl<Cost: Clone + Copy> Edge<Cost>\
    \ {\n    pub fn new(from: usize, to: usize, cost: Cost, id: usize) -> Self {\n\
    \        Self { from, to, cost, id }\n    }\n\n    pub fn to(&self) -> usize {\n\
    \        self.to\n    }\n\n    pub fn cost(&self) -> Cost {\n        self.cost\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: false
  path: crates/graph/graph/src/edge.rs
  requiredBy:
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/tree_util.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/heavy_light_decomposition/src/lib.rs
  timestamp: '2025-10-14 00:04:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/tree_diameter/src/main.rs
documentation_of: crates/graph/graph/src/edge.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/edge.rs
- /library/crates/graph/graph/src/edge.rs.html
title: crates/graph/graph/src/edge.rs
---
