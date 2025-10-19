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
  _extendedRequiredBy:
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
  - icon: ':warning:'
    path: crates/prelude/src/lib.rs
    title: crates/prelude/src/lib.rs
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
  code: "use crate::graph::DirectedGraph;\n\nimpl<Cost> DirectedGraph<Cost>\nwhere\n\
    \    Cost: Clone + Copy,\n{\n    pub fn topological_sort(&self) -> Option<Vec<usize>>\
    \ {\n        let n = self.size();\n        let mut deg = vec![0_usize; n];\n \
    \       for u in 0..n {\n            self[u].iter().for_each(|e| {\n         \
    \       deg[e.to()] += 1;\n            });\n        }\n        let mut stack =\
    \ Vec::<usize>::new();\n        let mut ret = Vec::<usize>::new();\n        deg.iter().enumerate().for_each(|(i,\
    \ &d)| {\n            if d == 0 {\n                stack.push(i);\n          \
    \  }\n        });\n        while let Some(u) = stack.pop() {\n            ret.push(u);\n\
    \            self[u].iter().for_each(|e| {\n                let v = e.to();\n\
    \                deg[v] -= 1;\n                if deg[v] == 0 {\n            \
    \        stack.push(v);\n                }\n            });\n        }\n\n   \
    \     if ret.len() == n { Some(ret) } else { None }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: false
  path: crates/graph/graph/src/directed_graph_util.rs
  requiredBy:
  - crates/prelude/src/lib.rs
  - crates/graph/heavy_light_decomposition/src/lib.rs
  - crates/graph/graph/src/tree_util.rs
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
documentation_of: crates/graph/graph/src/directed_graph_util.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/directed_graph_util.rs
- /library/crates/graph/graph/src/directed_graph_util.rs.html
title: crates/graph/graph/src/directed_graph_util.rs
---
