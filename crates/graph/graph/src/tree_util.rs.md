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
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
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
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
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
  code: "use std::ops::Add;\n\nuse numeric::zero::Zero;\n\nuse crate::graph::Tree;\n\
    \nimpl<Cost> Tree<Cost>\nwhere\n    Cost: Clone + Copy + Zero + PartialOrd + Add<Output\
    \ = Cost>,\n{\n    fn _diameter_dfs(\n        &self,\n        u: usize,\n    \
    \    p: Option<usize>,\n        d: Cost,\n        depth: &mut Vec<Cost>,\n   \
    \     parent: &mut Vec<Option<usize>>,\n    ) {\n        depth[u] = d;\n     \
    \   parent[u] = p;\n        self[u].iter().for_each(|e| {\n            if p.is_some()\
    \ && e.to() == p.unwrap() {\n                return;\n            }\n        \
    \    self._diameter_dfs(e.to(), Some(u), d + e.cost(), depth, parent);\n     \
    \   });\n    }\n\n    pub fn diameter(&self) -> (Cost, Vec<usize>) {\n       \
    \ let mut depth: Vec<Cost> = vec![Cost::zero(); self.size()];\n        let mut\
    \ parent: Vec<Option<usize>> = vec![None; self.size()];\n        self._diameter_dfs(0,\
    \ None, Cost::zero(), &mut depth, &mut parent);\n        let from = depth\n  \
    \          .iter()\n            .enumerate()\n            .reduce(|acc, elem|\
    \ {\n                if acc.1 < elem.1 {\n                    return elem;\n \
    \               }\n                acc\n            })\n            .unwrap()\n\
    \            .0;\n        self._diameter_dfs(from, None, Cost::zero(), &mut depth,\
    \ &mut parent);\n        let to = depth\n            .iter()\n            .enumerate()\n\
    \            .reduce(|acc, elem| {\n                if acc.1 < elem.1 {\n    \
    \                return elem;\n                }\n                acc\n      \
    \      })\n            .unwrap()\n            .0;\n        let mut path = vec![to];\n\
    \        loop {\n            let nxt = parent[*path.last().unwrap()];\n      \
    \      if nxt.is_none() {\n                break;\n            }\n           \
    \ path.push(nxt.unwrap());\n        }\n        (depth[to], path)\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  isVerificationFile: false
  path: crates/graph/graph/src/tree_util.rs
  requiredBy:
  - crates/graph/heavy_light_decomposition/src/lib.rs
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  timestamp: '2025-10-18 15:13:35+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/tree_diameter/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/graph/graph/src/tree_util.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/tree_util.rs
- /library/crates/graph/graph/src/tree_util.rs.html
title: crates/graph/graph/src/tree_util.rs
---
