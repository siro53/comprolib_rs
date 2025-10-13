---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/bound.rs
    title: crates/traits/numeric/src/bound.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/infinity.rs
    title: crates/traits/numeric/src/infinity.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/lib.rs
    title: crates/traits/numeric/src/lib.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/one.rs
    title: crates/traits/numeric/src/one.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':warning:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
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
  code: 'pub mod edge;

    pub mod graph;

    pub mod shortest_path;

    '
  dependsOn:
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/graph/graph/src/lib.rs
  requiredBy:
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  timestamp: '2025-10-13 16:28:18+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/graph/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/graph/src/lib.rs
- /library/crates/graph/graph/src/lib.rs.html
title: crates/graph/graph/src/lib.rs
---
