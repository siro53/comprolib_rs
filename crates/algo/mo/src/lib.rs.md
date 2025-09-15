---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
    title: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
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
  code: "pub trait Mo {\n    type Output: Default;\n\n    fn add(&mut self, _idx:\
    \ usize) {\n        unimplemented!();\n    }\n\n    fn del(&mut self, _idx: usize)\
    \ {\n        unimplemented!();\n    }\n\n    fn add_left(&mut self, idx: usize)\
    \ {\n        self.add(idx);\n    }\n\n    fn add_right(&mut self, idx: usize)\
    \ {\n        self.add(idx);\n    }\n\n    fn del_left(&mut self, idx: usize) {\n\
    \        self.del(idx);\n    }\n\n    fn del_right(&mut self, idx: usize) {\n\
    \        self.del(idx);\n    }\n\n    fn query(&self) -> Self::Output;\n\n   \
    \ fn run(&mut self, n: usize, queries: &[(usize, usize)]) -> Vec<Self::Output>\
    \ {\n        let q = queries.len();\n        let bucket_size = (((n as f64) /\
    \ (q as f64).sqrt()) as usize).max(1);\n\n        let mut ord = (0..q).collect::<Vec<usize>>();\n\
    \        ord.sort_by(|&a, &b| {\n            if queries[a].0 / bucket_size !=\
    \ queries[b].0 / bucket_size {\n                return queries[a].0.cmp(&queries[b].0);\n\
    \            }\n            queries[a].1.cmp(&queries[b].1)\n        });\n\n \
    \       let mut l: usize = 0;\n        let mut r: usize = 0;\n        let mut\
    \ res = Vec::new();\n        res.resize_with(q, Self::Output::default);\n    \
    \    ord.into_iter().for_each(|qid| {\n            while l > queries[qid].0 {\n\
    \                l -= 1;\n                self.add_left(l);\n            }\n \
    \           while l < queries[qid].0 {\n                self.del_left(l);\n  \
    \              l += 1;\n            }\n            while r < queries[qid].1 {\n\
    \                self.add_right(r);\n                r += 1;\n            }\n\
    \            while r > queries[qid].1 {\n                r -= 1;\n           \
    \     self.del_right(r);\n            }\n            res[qid] = self.query();\n\
    \        });\n\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algo/mo/src/lib.rs
  requiredBy: []
  timestamp: '2025-09-15 18:36:07+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
documentation_of: crates/algo/mo/src/lib.rs
layout: document
redirect_from:
- /library/crates/algo/mo/src/lib.rs
- /library/crates/algo/mo/src/lib.rs.html
title: crates/algo/mo/src/lib.rs
---
