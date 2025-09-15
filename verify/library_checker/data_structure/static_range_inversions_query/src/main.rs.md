---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algo/mo/src/lib.rs
    title: crates/algo/mo/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/compress/src/lib.rs
    title: crates/misc/compress/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_inversions_query
    links:
    - https://judge.yosupo.jp/problem/static_range_inversions_query
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query\n\
    \nuse compress::Compress;\nuse fenwick_tree::fenwick_tree::FenwickTree;\nuse mo::Mo;\n\
    use proconio::{fastout, input};\n\nstruct Solver {\n    a: Vec<usize>,\n    fwt:\
    \ FenwickTree<i64>,\n    now_ans: i64,\n    sz: usize,\n}\n\nimpl Mo for Solver\
    \ {\n    type Output = i64;\n\n    fn add_left(&mut self, idx: usize) {\n    \
    \    self.now_ans += self.fwt.sum(0..self.a[idx]);\n        self.fwt.add(self.a[idx],\
    \ 1);\n    }\n\n    fn add_right(&mut self, idx: usize) {\n        self.now_ans\
    \ += self.fwt.sum(self.a[idx] + 1..self.sz);\n        self.fwt.add(self.a[idx],\
    \ 1);\n    }\n\n    fn del_left(&mut self, idx: usize) {\n        self.now_ans\
    \ -= self.fwt.sum(..self.a[idx]);\n        self.fwt.add(self.a[idx], -1);\n  \
    \  }\n\n    fn del_right(&mut self, idx: usize) {\n        self.now_ans -= self.fwt.sum(self.a[idx]\
    \ + 1..self.sz);\n        self.fwt.add(self.a[idx], -1);\n    }\n\n    fn query(&self)\
    \ -> Self::Output {\n        self.now_ans\n    }\n}\n\n#[fastout]\nfn main() {\n\
    \    input! {\n        n: usize,\n        q: usize,\n        mut a: [usize; n],\n\
    \        queries: [(usize, usize); q]\n    }\n    let comp = a.clone().into_iter().collect::<Compress<usize>>();\n\
    \    for i in 0..n {\n        a[i] = comp.get(a[i]);\n    }\n    let sz = comp.len();\n\
    \    let mut solver = Solver {\n        a,\n        fwt: FenwickTree::<i64>::new(sz),\n\
    \        now_ans: 0,\n        sz,\n    };\n    let ans = solver.run(n, &queries);\n\
    \    for i in 0..q {\n        println!(\"{}\", ans[i]);\n    }\n}\n"
  dependsOn:
  - crates/algo/mo/src/lib.rs
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - crates/misc/compress/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
  requiredBy: []
  timestamp: '2025-09-15 18:36:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
- /verify/verify/library_checker/data_structure/static_range_inversions_query/src/main.rs.html
title: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
---
