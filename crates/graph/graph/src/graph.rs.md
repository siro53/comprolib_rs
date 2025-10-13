---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
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
  code: "use std::{\n    marker::PhantomData,\n    ops::{Index, IndexMut},\n};\n\n\
    use crate::edge::Edge;\n\npub struct GraphBase<Cost: Clone, Marker> {\n    n:\
    \ usize,\n    m: usize,\n    g: Vec<Vec<Edge<Cost>>>,\n    _marker: PhantomData<fn()\
    \ -> Marker>,\n}\n\nimpl<Cost, Marker> GraphBase<Cost, Marker>\nwhere\n    Cost:\
    \ Clone + Copy,\n{\n    pub fn new(n: usize) -> Self {\n        Self {\n     \
    \       n,\n            m: 0,\n            g: vec![Vec::<Edge<Cost>>::new(); n],\n\
    \            _marker: PhantomData,\n        }\n    }\n\n    pub fn size(&self)\
    \ -> usize {\n        self.n\n    }\n}\n\nimpl<Cost, Marker> Index<usize> for\
    \ GraphBase<Cost, Marker>\nwhere\n    Cost: Clone,\n{\n    type Output = Vec<Edge<Cost>>;\n\
    \n    fn index(&self, index: usize) -> &Self::Output {\n        assert!(index\
    \ < self.n);\n        &self.g[index]\n    }\n}\n\nimpl<Cost, Marker> IndexMut<usize>\
    \ for GraphBase<Cost, Marker>\nwhere\n    Cost: Clone,\n{\n    fn index_mut(&mut\
    \ self, index: usize) -> &mut Self::Output {\n        assert!(index < self.n);\n\
    \        &mut self.g[index]\n    }\n}\n\npub enum UndirectedGraphMarker {}\npub\
    \ enum DirectedGraphMarker {}\npub enum TreeMarker {}\n\npub type UndirectedGraph<Cost>\
    \ = GraphBase<Cost, UndirectedGraphMarker>;\npub type DirectedGraph<Cost> = GraphBase<Cost,\
    \ DirectedGraphMarker>;\npub type Tree<Cost> = GraphBase<Cost, TreeMarker>;\n\n\
    impl<Cost> DirectedGraph<Cost>\nwhere\n    Cost: Clone + Copy,\n{\n    pub fn\
    \ add_directed_edge(&mut self, from: usize, to: usize, cost: Cost) {\n       \
    \ self.g[from].push(Edge::new(from, to, cost, self.m));\n        self.m += 1;\n\
    \    }\n}\n\nimpl<Cost> UndirectedGraph<Cost>\nwhere\n    Cost: Clone + Copy,\n\
    {\n    pub fn add_undirected_edge(&mut self, from: usize, to: usize, cost: Cost)\
    \ {\n        self.g[from].push(Edge::new(from, to, cost, self.m));\n        self.m\
    \ += 1;\n        self.g[to].push(Edge::new(to, from, cost, self.m));\n       \
    \ self.m += 1;\n    }\n}\n\nimpl<Cost> Tree<Cost>\nwhere\n    Cost: Clone + Copy,\n\
    {\n    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {\n    \
    \    self.g[from].push(Edge::new(from, to, cost, self.m));\n        self.m +=\
    \ 1;\n        self.g[to].push(Edge::new(to, from, cost, self.m));\n        self.m\
    \ += 1;\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  isVerificationFile: false
  path: crates/graph/graph/src/graph.rs
  requiredBy:
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/edge.rs
  timestamp: '2025-10-13 16:28:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/graph/src/graph.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/graph.rs
- /library/crates/graph/graph/src/graph.rs.html
title: crates/graph/graph/src/graph.rs
---
