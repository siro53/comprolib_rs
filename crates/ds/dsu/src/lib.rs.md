---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind/src/main.rs
    title: verify/library_checker/data_structure/unionfind/src/main.rs
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
  code: "use std::collections::{BTreeMap, btree_map};\nuse std::mem::swap;\n\npub\
    \ struct Dsu {\n    size: usize,\n    parent_or_size: Vec<i32>,\n}\n\nimpl Dsu\
    \ {\n    pub fn new(size: usize) -> Self {\n        Self {\n            size,\n\
    \            parent_or_size: vec![-1; size],\n        }\n    }\n\n    pub fn leader(&mut\
    \ self, u: usize) -> usize {\n        assert!(u < self.size);\n        if self.parent_or_size[u]\
    \ < 0 {\n            return u;\n        }\n        self.parent_or_size[u] = self.leader(self.parent_or_size[u]\
    \ as usize) as i32;\n        self.parent_or_size[u] as usize\n    }\n\n    pub\
    \ fn same(&mut self, u: usize, v: usize) -> bool {\n        assert!(u < self.size);\n\
    \        assert!(v < self.size);\n        self.leader(u) == self.leader(v)\n \
    \   }\n\n    pub fn merge(&mut self, u: usize, v: usize) -> usize {\n        assert!(u\
    \ < self.size);\n        assert!(v < self.size);\n        let mut lu = self.leader(u);\n\
    \        let mut lv = self.leader(v);\n        if lu == lv {\n            return\
    \ lu;\n        }\n        if -self.parent_or_size[lu] < -self.parent_or_size[lv]\
    \ {\n            swap(&mut lu, &mut lv);\n        }\n        self.parent_or_size[lu]\
    \ += self.parent_or_size[lv];\n        self.parent_or_size[lv] = lu as i32;\n\
    \        lu\n    }\n\n    pub fn size(&mut self, u: usize) -> usize {\n      \
    \  let lu = self.leader(u);\n        -self.parent_or_size[lu] as usize\n    }\n\
    \n    pub fn groups(&mut self) -> Vec<Vec<usize>> {\n        let mut leaders:\
    \ Vec<usize> = vec![0; self.size];\n        let mut group_size: Vec<usize> = vec![0;\
    \ self.size];\n        for i in 0..self.size {\n            leaders[i] = self.leader(i);\n\
    \            group_size[i] += 1;\n        }\n        let mut groups: Vec<Vec<usize>>\
    \ = vec![Vec::new(); self.size];\n        for i in 0..self.size {\n          \
    \  groups[i].reserve(group_size[i]);\n        }\n        for i in 0..self.size\
    \ {\n            groups[leaders[i]].push(i);\n        }\n        groups\n    \
    \        .into_iter()\n            .filter(|group| !group.is_empty())\n      \
    \      .collect()\n    }\n\n    pub fn groups_with_btreemap(&mut self) -> BTreeMap<usize,\
    \ Vec<usize>> {\n        let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();\n\
    \        for i in 0..self.size {\n            let l = self.leader(i);\n      \
    \      if let btree_map::Entry::Vacant(e) = groups.entry(l) {\n              \
    \  e.insert(vec![i]);\n            } else {\n                let group = groups.get_mut(&l).unwrap();\n\
    \                group.push(i);\n            }\n        }\n        groups\n  \
    \  }\n}\n\n#[cfg(test)]\nmod tests {\n    use crate::Dsu;\n\n    #[test]\n   \
    \ fn dsu_test() {\n        let mut d = Dsu::new(4);\n        d.merge(0, 1);\n\
    \        assert!(d.same(0, 1));\n        d.merge(1, 2);\n        assert!(d.same(0,\
    \ 2));\n        assert_eq!(d.size(0), 3);\n        assert!(!d.same(0, 3));\n \
    \       assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);\n        assert_eq!(*d.groups_with_btreemap().get(&0).unwrap(),\
    \ vec![0, 1, 2]);\n        assert_eq!(*d.groups_with_btreemap().get(&3).unwrap(),\
    \ vec![3]);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/ds/dsu/src/lib.rs
  requiredBy: []
  timestamp: '2025-10-14 09:57:23+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/unionfind/src/main.rs
documentation_of: crates/ds/dsu/src/lib.rs
layout: document
redirect_from:
- /library/crates/ds/dsu/src/lib.rs
- /library/crates/ds/dsu/src/lib.rs.html
title: crates/ds/dsu/src/lib.rs
---
