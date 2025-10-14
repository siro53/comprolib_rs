---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/directed_graph_util.rs
    title: crates/graph/graph/src/directed_graph_util.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
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
    path: crates/graph/graph/src/directed_graph_util.rs
    title: crates/graph/graph/src/directed_graph_util.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
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
  code: "use std::{\n    marker::PhantomData,\n    ops::{Index, IndexMut},\n};\n\n\
    use crate::edge::Edge;\n\n#[derive(Clone)]\npub struct GraphBase<Cost: Clone,\
    \ Marker> {\n    n: usize,\n    m: usize,\n    g: Vec<Vec<Edge<Cost>>>,\n    _marker:\
    \ PhantomData<fn() -> Marker>,\n}\n\nimpl<Cost, Marker> GraphBase<Cost, Marker>\n\
    where\n    Cost: Clone + Copy,\n{\n    pub fn new(n: usize) -> Self {\n      \
    \  Self {\n            n,\n            m: 0,\n            g: vec![Vec::<Edge<Cost>>::new();\
    \ n],\n            _marker: PhantomData,\n        }\n    }\n\n    pub fn size(&self)\
    \ -> usize {\n        self.n\n    }\n}\n\nimpl<Cost, Marker> Index<usize> for\
    \ GraphBase<Cost, Marker>\nwhere\n    Cost: Clone,\n{\n    type Output = Vec<Edge<Cost>>;\n\
    \n    fn index(&self, index: usize) -> &Self::Output {\n        assert!(index\
    \ < self.n);\n        &self.g[index]\n    }\n}\n\nimpl<Cost, Marker> IndexMut<usize>\
    \ for GraphBase<Cost, Marker>\nwhere\n    Cost: Clone,\n{\n    fn index_mut(&mut\
    \ self, index: usize) -> &mut Self::Output {\n        assert!(index < self.n);\n\
    \        &mut self.g[index]\n    }\n}\n\n#[derive(Clone)]\npub enum UndirectedGraphMarker\
    \ {}\n\n#[derive(Clone)]\npub enum DirectedGraphMarker {}\n\n#[derive(Clone)]\n\
    pub enum TreeMarker {}\n\npub type UndirectedGraph<Cost> = GraphBase<Cost, UndirectedGraphMarker>;\n\
    pub type DirectedGraph<Cost> = GraphBase<Cost, DirectedGraphMarker>;\npub type\
    \ Tree<Cost> = GraphBase<Cost, TreeMarker>;\n\nimpl<Cost> DirectedGraph<Cost>\n\
    where\n    Cost: Clone + Copy,\n{\n    pub fn add_directed_edge(&mut self, from:\
    \ usize, to: usize, cost: Cost) {\n        self.g[from].push(Edge::new(from, to,\
    \ cost, self.m));\n        self.m += 1;\n    }\n}\n\nimpl<Cost> UndirectedGraph<Cost>\n\
    where\n    Cost: Clone + Copy,\n{\n    pub fn add_undirected_edge(&mut self, from:\
    \ usize, to: usize, cost: Cost) {\n        self.g[from].push(Edge::new(from, to,\
    \ cost, self.m));\n        self.m += 1;\n        self.g[to].push(Edge::new(to,\
    \ from, cost, self.m));\n        self.m += 1;\n    }\n}\n\nimpl<Cost> Tree<Cost>\n\
    where\n    Cost: Clone + Copy,\n{\n    pub fn add_edge(&mut self, from: usize,\
    \ to: usize, cost: Cost) {\n        self.g[from].push(Edge::new(from, to, cost,\
    \ self.m));\n        self.m += 1;\n        self.g[to].push(Edge::new(to, from,\
    \ cost, self.m));\n        self.m += 1;\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: false
  path: crates/graph/graph/src/graph.rs
  requiredBy:
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/tree_util.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/heavy_light_decomposition/src/lib.rs
  timestamp: '2025-10-14 10:00:12+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/tree_diameter/src/main.rs
documentation_of: crates/graph/graph/src/graph.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/graph.rs
- /library/crates/graph/graph/src/graph.rs.html
title: crates/graph/graph/src/graph.rs
---
