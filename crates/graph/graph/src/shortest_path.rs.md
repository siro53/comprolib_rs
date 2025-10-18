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
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
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
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
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
  code: "use std::{\n    cmp::Reverse,\n    collections::{BinaryHeap, VecDeque},\n\
    \    ops::Add,\n};\n\nuse numeric::one::One;\n\nuse crate::graph::GraphBase;\n\
    \nimpl<Cost, Marker> GraphBase<Cost, Marker>\nwhere\n    Cost: Clone + Copy,\n\
    {\n    pub fn bfs(&self, start: usize, iv: Cost, inf: Cost) -> (Vec<Cost>, Vec<Option<usize>>)\n\
    \    where\n        Cost: One + Add<Output = Cost> + PartialOrd,\n    {\n    \
    \    let mut dist = vec![inf; self.size()];\n        let mut prev: Vec<Option<usize>>\
    \ = vec![None; self.size()];\n        let mut que = VecDeque::<usize>::new();\n\
    \        que.push_back(start);\n        dist[start] = iv;\n        while let Some(u)\
    \ = que.pop_front() {\n            self[u].iter().for_each(|e| {\n           \
    \     if dist[e.to()] > dist[u] + Cost::one() {\n                    dist[e.to()]\
    \ = dist[u] + Cost::one();\n                    prev[e.to()] = Some(u);\n    \
    \                que.push_back(e.to());\n                }\n            });\n\
    \        }\n        (dist, prev)\n    }\n\n    pub fn dijkstra(&self, start: usize,\
    \ iv: Cost, inf: Cost) -> (Vec<Cost>, Vec<Option<usize>>)\n    where\n       \
    \ Cost: Add<Output = Cost> + PartialOrd + Ord,\n    {\n        let mut dist =\
    \ vec![inf; self.size()];\n        let mut prev: Vec<Option<usize>> = vec![None;\
    \ self.size()];\n        let mut que = BinaryHeap::new();\n        que.push(Reverse((iv,\
    \ start)));\n        dist[start] = iv;\n        while let Some(Reverse((d, u)))\
    \ = que.pop() {\n            if d > dist[u] {\n                continue;\n   \
    \         }\n            self[u].iter().for_each(|e| {\n                if dist[e.to()]\
    \ > dist[u] + e.cost() {\n                    dist[e.to()] = dist[u] + e.cost();\n\
    \                    prev[e.to()] = Some(u);\n                    que.push(Reverse((dist[e.to()],\
    \ e.to())));\n                }\n            });\n        }\n        (dist, prev)\n\
    \    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: false
  path: crates/graph/graph/src/shortest_path.rs
  requiredBy:
  - crates/graph/heavy_light_decomposition/src/lib.rs
  - crates/graph/graph/src/tree_util.rs
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  timestamp: '2025-10-18 15:13:35+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/tree_diameter/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/graph/graph/src/shortest_path.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/shortest_path.rs
- /library/crates/graph/graph/src/shortest_path.rs.html
title: crates/graph/graph/src/shortest_path.rs
---
