---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
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
  isVerificationFile: false
  path: crates/graph/graph/src/edge.rs
  requiredBy:
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/graph.rs
  timestamp: '2025-10-13 16:28:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/graph/src/edge.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/edge.rs
- /library/crates/graph/graph/src/edge.rs.html
title: crates/graph/graph/src/edge.rs
---
