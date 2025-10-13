---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/fenwick_tree.rs
    title: crates/ds/fenwick_tree/src/fenwick_tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/ds/fenwick_tree/src/lib.rs
    title: crates/ds/fenwick_tree/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
    title: verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
    title: verify/library_checker/tree/vertex_add_path_sum/src/main.rs
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
  code: "use std::ops::{Add, AddAssign, Bound, Mul, Neg, RangeBounds, Sub};\n\nuse\
    \ numeric::zero::Zero;\n\nuse crate::fenwick_tree::FenwickTree;\n\npub struct\
    \ RangeFenwickTree<T>\nwhere\n    T: Clone\n        + From<usize>\n        + AddAssign\n\
    \        + Sub<Output = T>\n        + Neg<Output = T>\n        + Add<Output =\
    \ T>\n        + Mul<Output = T>\n        + Neg\n        + Zero,\n{\n    n: usize,\n\
    \    fwt1: FenwickTree<T>,\n    fwt2: FenwickTree<T>,\n}\n\nimpl<T> RangeFenwickTree<T>\n\
    where\n    T: Clone\n        + From<usize>\n        + AddAssign\n        + Sub<Output\
    \ = T>\n        + Neg<Output = T>\n        + Add<Output = T>\n        + Mul<Output\
    \ = T>\n        + Neg\n        + Zero,\n{\n    pub fn new(n: usize) -> Self {\n\
    \        let fwt1 = FenwickTree::<T>::new(n + 1);\n        let fwt2 = FenwickTree::<T>::new(n\
    \ + 1);\n        Self { n, fwt1, fwt2 }\n    }\n\n    pub fn add<R>(&mut self,\
    \ range: R, value: T)\n    where\n        R: RangeBounds<usize>,\n        T: Neg<Output\
    \ = T> + Mul<Output = T> + From<usize>,\n    {\n        let l = match range.start_bound()\
    \ {\n            Bound::Included(l) => *l,\n            Bound::Excluded(l) =>\
    \ l + 1,\n            Bound::Unbounded => 0,\n        };\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(r) => r + 1,\n            Bound::Excluded(r)\
    \ => *r,\n            Bound::Unbounded => self.n,\n        };\n        assert!(l\
    \ <= r);\n        self.fwt1.add(l, -value.clone() * T::from(l));\n        self.fwt1.add(r,\
    \ value.clone() * T::from(r));\n        self.fwt2.add(l, value.clone());\n   \
    \     self.fwt2.add(r, -value.clone());\n    }\n\n    fn accumulate(&self, index:\
    \ usize) -> T {\n        assert!(index <= self.n);\n        self.fwt1.sum(..index)\
    \ + self.fwt2.sum(..index) * T::from(index)\n    }\n\n    pub fn sum<R>(&self,\
    \ range: R) -> T\n    where\n        R: RangeBounds<usize>,\n    {\n        let\
    \ l = match range.start_bound() {\n            Bound::Included(l) => *l,\n   \
    \         Bound::Excluded(l) => l + 1,\n            Bound::Unbounded => 0,\n \
    \       };\n        let r = match range.end_bound() {\n            Bound::Included(r)\
    \ => r + 1,\n            Bound::Excluded(r) => *r,\n            Bound::Unbounded\
    \ => self.n,\n        };\n        assert!(l <= r);\n        self.accumulate(r)\
    \ - self.accumulate(l)\n    }\n}\n"
  dependsOn:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  isVerificationFile: false
  path: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
  requiredBy:
  - crates/ds/fenwick_tree/src/fenwick_tree.rs
  - crates/ds/fenwick_tree/src/lib.rs
  timestamp: '2025-08-31 22:36:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/verify_fenwick_tree/src/main.rs
  - verify/library_checker/data_structure/static_range_inversions_query/src/main.rs
  - verify/library_checker/tree/vertex_add_path_sum/src/main.rs
documentation_of: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
layout: document
redirect_from:
- /library/crates/ds/fenwick_tree/src/range_fenwick_tree.rs
- /library/crates/ds/fenwick_tree/src/range_fenwick_tree.rs.html
title: crates/ds/fenwick_tree/src/range_fenwick_tree.rs
---
