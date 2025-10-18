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
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/tree_util.rs
    title: crates/graph/graph/src/tree_util.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
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
  code: "use std::{mem::swap, ops::Index};\n\nuse graph::graph::Tree;\n\npub struct\
    \ HeavyLightDecomposition<Cost>\nwhere\n    Cost: Clone,\n{\n    tree: Tree<Cost>,\n\
    \    input: Vec<usize>,\n    output: Vec<usize>,\n    size: Vec<usize>,\n    head:\
    \ Vec<usize>,\n    parent: Vec<Option<usize>>,\n    depth: Vec<usize>,\n    rev_input:\
    \ Vec<usize>,\n    is_built: bool,\n}\n\nimpl<Cost> HeavyLightDecomposition<Cost>\n\
    where\n    Cost: Clone + Copy,\n{\n    fn _dfs1(&mut self, u: usize, p: Option<usize>,\
    \ d: usize) {\n        self.depth[u] = d;\n        self.size[u] = 1;\n       \
    \ if !self.tree[u].is_empty() && Some(self.tree[u].first().unwrap().to()) == p\
    \ {\n            let sz = self.tree[u].len();\n            self.tree[u].swap(0,\
    \ sz - 1);\n        }\n        for i in 0..self.tree[u].len() {\n            let\
    \ v = self.tree[u][i].to();\n            if Some(v) == p {\n                continue;\n\
    \            }\n            self._dfs1(v, Some(u), d + 1);\n            self.size[u]\
    \ += self.size[v];\n            let h = self.tree[u][0].to();\n            if\
    \ self.size[v] > self.size[h] {\n                self.tree[u].swap(0, i);\n  \
    \          }\n        }\n    }\n\n    fn _dfs2(&mut self, u: usize, p: Option<usize>,\
    \ index: &mut usize) {\n        self.parent[u] = p;\n        self.input[u] = *index;\n\
    \        *index += 1;\n        self.rev_input[self.input[u]] = u;\n        for\
    \ i in 0..self.tree[u].len() {\n            let v = self.tree[u][i].to();\n  \
    \          if Some(v) == p {\n                continue;\n            }\n     \
    \       let h = self.tree[u][0].to();\n            self.head[v] = if v == h {\
    \ self.head[u] } else { v };\n            self._dfs2(v, Some(u), index);\n   \
    \     }\n        self.output[u] = *index;\n    }\n\n    pub fn new(tree: &Tree<Cost>)\
    \ -> Self {\n        Self {\n            tree: tree.clone(),\n            input:\
    \ vec![0; tree.size()],\n            output: vec![0; tree.size()],\n         \
    \   size: vec![0; tree.size()],\n            head: vec![0; tree.size()],\n   \
    \         parent: vec![None; tree.size()],\n            depth: vec![0; tree.size()],\n\
    \            rev_input: vec![0; tree.size()],\n            is_built: false,\n\
    \        }\n    }\n\n    pub fn build(&mut self, root: usize) {\n        self._dfs1(root,\
    \ None, 0);\n        self.head[root] = root;\n        let mut index = 0;\n   \
    \     self._dfs2(root, None, &mut index);\n        self.is_built = true;\n   \
    \ }\n\n    pub fn lca(&self, u: usize, v: usize) -> usize {\n        assert!(self.is_built);\n\
    \        assert!(u < self.tree.size());\n        assert!(v < self.tree.size());\n\
    \        let mut u = u;\n        let mut v = v;\n        loop {\n            if\
    \ self.input[u] > self.input[v] {\n                swap(&mut u, &mut v);\n   \
    \         }\n            if self.head[u] == self.head[v] {\n                return\
    \ u;\n            }\n            v = self.parent[self.head[v]].unwrap();\n   \
    \     }\n    }\n\n    pub fn dist(&self, u: usize, v: usize) -> usize {\n    \
    \    assert!(self.is_built);\n        assert!(u < self.tree.size());\n       \
    \ assert!(v < self.tree.size());\n        self.depth[u] + self.depth[v] - self.depth[self.lca(u,\
    \ v)] * 2\n    }\n\n    pub fn la(&self, u: usize, k: usize) -> Option<usize>\
    \ {\n        assert!(self.is_built);\n        assert!(u < self.tree.size());\n\
    \        if self.depth[u] < k {\n            return None;\n        }\n       \
    \ let mut u = u;\n        let mut k = k;\n        loop {\n            if self.input[u]\
    \ - k >= self.input[self.head[u]] {\n                return Some(self.rev_input[self.input[u]\
    \ - k]);\n            }\n            k -= self.input[u] - self.input[self.head[u]]\
    \ + 1;\n            u = self.parent[self.head[u]].unwrap();\n        }\n    }\n\
    \n    fn _path_query<F>(&self, u: usize, v: usize, f: &mut F, is_edge: bool)\n\
    \    where\n        F: FnMut(usize, usize),\n    {\n        assert!(self.is_built);\n\
    \        assert!(u < self.tree.size());\n        assert!(v < self.tree.size());\n\
    \        let mut u = u;\n        let mut v = v;\n        loop {\n            if\
    \ self.input[u] > self.input[v] {\n                swap(&mut u, &mut v);\n   \
    \         }\n            if self.head[u] == self.head[v] {\n                f(self.input[u]\
    \ + (is_edge as usize), self.input[v] + 1);\n                break;\n        \
    \    } else {\n                f(self.input[self.head[v]], self.input[v] + 1);\n\
    \            }\n            v = self.parent[self.head[v]].unwrap();\n        }\n\
    \    }\n\n    pub fn path_query_commutative<F>(&self, u: usize, v: usize, f: &mut\
    \ F, is_edge: bool)\n    where\n        F: FnMut(usize, usize),\n    {\n     \
    \   assert!(self.is_built);\n        assert!(u < self.tree.size());\n        assert!(v\
    \ < self.tree.size());\n        self._path_query(u, v, f, is_edge);\n    }\n\n\
    \    pub fn path_query_not_commutative<F, InverseF>(\n        &self,\n       \
    \ u: usize,\n        v: usize,\n        f: &mut F,\n        inv_f: &mut InverseF,\n\
    \        is_edge: bool,\n    ) where\n        F: FnMut(usize, usize),\n      \
    \  InverseF: FnMut(usize, usize),\n    {\n        assert!(self.is_built);\n  \
    \      assert!(u < self.tree.size());\n        assert!(v < self.tree.size());\n\
    \        let lca = self.lca(u, v);\n        self._path_query(u, lca, f, is_edge);\n\
    \        self._path_query(lca, v, inv_f, true);\n    }\n\n    pub fn subtree_query<F>(&self,\
    \ u: usize, f: &mut F)\n    where\n        F: FnMut(usize, usize),\n    {\n  \
    \      assert!(self.is_built);\n        assert!(u < self.tree.size());\n     \
    \   f(self.input[u], self.output[u]);\n    }\n}\n\nimpl<Cost> Index<usize> for\
    \ HeavyLightDecomposition<Cost>\nwhere\n    Cost: Clone + Copy,\n{\n    type Output\
    \ = usize;\n\n    fn index(&self, u: usize) -> &Self::Output {\n        assert!(self.is_built);\n\
    \        assert!(u < self.tree.size());\n        &self.input[u]\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  isVerificationFile: false
  path: crates/graph/heavy_light_decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2025-10-18 15:13:35+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/tree/vertex_add_subtree_sum/src/main.rs
documentation_of: crates/graph/heavy_light_decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/heavy_light_decomposition/src/lib.rs
- /library/crates/graph/heavy_light_decomposition/src/lib.rs.html
title: crates/graph/heavy_light_decomposition/src/lib.rs
---
