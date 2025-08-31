---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
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
  code: "use std::ops::{AddAssign, Bound, RangeBounds, Sub, SubAssign};\n\nuse numeric::zero::Zero;\n\
    \npub struct FenwickTree<T>\nwhere\n    T: Clone + AddAssign + Zero + Sub<Output\
    \ = T>,\n{\n    n: usize,\n    data: Vec<T>,\n}\n\nimpl<T> FenwickTree<T>\nwhere\n\
    \    T: Clone + AddAssign + Zero + Sub<Output = T>,\n{\n    pub fn new(n: usize)\
    \ -> Self {\n        Self {\n            n,\n            data: vec![T::zero();\
    \ n + 1],\n        }\n    }\n\n    pub fn add(&mut self, index: usize, value:\
    \ T) {\n        let mut i = index + 1;\n        while i <= self.n {\n        \
    \    self.data[i] += value.clone();\n            i += i & i.wrapping_neg();\n\
    \        }\n    }\n\n    fn accumulate(&self, index: usize) -> T {\n        assert!(index\
    \ <= self.n);\n        let mut res = T::zero();\n        let mut r = index;\n\
    \        while r >= 1 {\n            res += self.data[r].clone();\n          \
    \  r -= r & r.wrapping_neg();\n        }\n        res\n    }\n\n    pub fn sum<R>(&self,\
    \ range: R) -> T\n    where\n        R: RangeBounds<usize>,\n    {\n        let\
    \ l = match range.start_bound() {\n            Bound::Included(l) => *l,\n   \
    \         Bound::Excluded(l) => l + 1,\n            Bound::Unbounded => 0,\n \
    \       };\n        let r = match range.end_bound() {\n            Bound::Included(r)\
    \ => r + 1,\n            Bound::Excluded(r) => *r,\n            Bound::Unbounded\
    \ => self.n,\n        };\n        assert!(l <= r);\n        self.accumulate(r)\
    \ - self.accumulate(l)\n    }\n\n    pub fn lower_bound(&self, value: T) -> usize\n\
    \    where\n        T: SubAssign + PartialOrd,\n    {\n        let mut pos = 0_usize;\n\
    \        let mut k = 1_usize;\n        let mut value = value;\n        while k\
    \ * 2 <= self.n {\n            k <<= 1;\n        }\n        while k >= 1 {\n \
    \           if pos + k <= self.n && self.data[pos + k] < value {\n           \
    \     pos += k;\n                value -= self.data[pos].clone();\n          \
    \  }\n            k >>= 1;\n        }\n        pos + 1\n    }\n}\n"
  dependsOn:
  - crates/ds/fenwick_tree/src/lib.rs
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  isVerificationFile: false
  path: crates/ds/fenwick_tree/src/fenwick_tree.rs
  requiredBy:
  - crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  timestamp: '2025-08-31 22:36:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
documentation_of: crates/ds/fenwick_tree/src/fenwick_tree.rs
layout: document
redirect_from:
- /library/crates/ds/fenwick_tree/src/fenwick_tree.rs
- /library/crates/ds/fenwick_tree/src/fenwick_tree.rs.html
title: crates/ds/fenwick_tree/src/fenwick_tree.rs
---
