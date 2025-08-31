---
data:
  _extendedDependsOn:
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
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/aoj/aoj2426/src/main.rs
    title: verify/aoj/aoj2426/src/main.rs
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
  code: "use std::ops::{Add, AddAssign, Bound, RangeBounds, Sub};\n\nuse numeric::zero::Zero;\n\
    \npub struct CumulativeSum2D<T> {\n    sum: Vec<Vec<T>>,\n    is_built: bool,\n\
    }\n\nimpl<T> CumulativeSum2D<T>\nwhere\n    T: Clone + Copy,\n{\n    pub fn new(height:\
    \ usize, width: usize) -> Self\n    where\n        T: Zero,\n    {\n        Self\
    \ {\n            sum: vec![vec![T::zero(); width + 1]; height + 1],\n        \
    \    is_built: false,\n        }\n    }\n\n    pub fn add(&mut self, row: usize,\
    \ column: usize, value: T)\n    where\n        T: AddAssign,\n    {\n        assert!(row\
    \ + 1 < self.sum.len());\n        assert!(column + 1 < self.sum[0].len());\n \
    \       self.sum[row + 1][column + 1] += value;\n    }\n\n    pub fn build(&mut\
    \ self)\n    where\n        T: AddAssign + Add<Output = T> + Sub<Output = T>,\n\
    \    {\n        for i in 1..self.sum.len() {\n            for j in 1..self.sum[i].len()\
    \ {\n                self.sum[i][j] = self.sum[i][j] + self.sum[i - 1][j] + self.sum[i][j\
    \ - 1]\n                    - self.sum[i - 1][j - 1];\n            }\n       \
    \ }\n        self.is_built = true;\n    }\n\n    pub fn sum<R>(&self, range_row:\
    \ R, range_column: R) -> T\n    where\n        R: RangeBounds<usize>,\n      \
    \  T: Add<Output = T> + Sub<Output = T>,\n    {\n        let row_l = match range_row.start_bound()\
    \ {\n            Bound::Included(l) => *l,\n            Bound::Excluded(l) =>\
    \ l + 1,\n            Bound::Unbounded => 0,\n        };\n        let row_r =\
    \ match range_row.end_bound() {\n            Bound::Included(r) => r + 1,\n  \
    \          Bound::Excluded(r) => *r,\n            Bound::Unbounded => self.sum.len(),\n\
    \        };\n        let column_l = match range_column.start_bound() {\n     \
    \       Bound::Included(l) => *l,\n            Bound::Excluded(l) => l + 1,\n\
    \            Bound::Unbounded => 0,\n        };\n        let column_r = match\
    \ range_column.end_bound() {\n            Bound::Included(r) => r + 1,\n     \
    \       Bound::Excluded(r) => *r,\n            Bound::Unbounded => self.sum[0].len(),\n\
    \        };\n        assert!(row_l <= row_r && row_r < self.sum.len());\n    \
    \    assert!(column_l <= column_r && column_r < self.sum[0].len());\n        assert!(self.is_built);\n\
    \        self.sum[row_r][column_r] - self.sum[row_l][column_r] - self.sum[row_r][column_l]\n\
    \            + self.sum[row_l][column_l]\n    }\n}\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/misc/cumulative_sum_2d/src/lib.rs
  requiredBy: []
  timestamp: '2025-08-31 23:58:05+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/aoj/aoj2426/src/main.rs
documentation_of: crates/misc/cumulative_sum_2d/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/cumulative_sum_2d/src/lib.rs
- /library/crates/misc/cumulative_sum_2d/src/lib.rs.html
title: crates/misc/cumulative_sum_2d/src/lib.rs
---
