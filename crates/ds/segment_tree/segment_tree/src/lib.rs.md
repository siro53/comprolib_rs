---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_range_sum/src/main.rs
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
  code: "use std::ops::{Bound, RangeBounds};\n\nuse monoid::Monoid;\n\npub struct\
    \ SegmentTree<M>\nwhere\n    M: Monoid,\n{\n    n: usize,\n    size: usize,\n\
    \    node: Vec<M::ValueType>,\n}\n\nimpl<M> FromIterator<M::ValueType> for SegmentTree<M>\n\
    where\n    M: Monoid,\n{\n    fn from_iter<T: IntoIterator<Item = M::ValueType>>(iter:\
    \ T) -> Self {\n        let data = iter.into_iter().collect::<Vec<_>>();\n   \
    \     let n = data.len();\n        let size = n.next_power_of_two();\n       \
    \ let mut node = (0..size)\n            .map(|_| M::unit())\n            .chain(data)\n\
    \            .chain((0..size - n).map(|_| M::unit()))\n            .collect::<Vec<_>>();\n\
    \        for i in (1..size).rev() {\n            node[i] = M::op(&node[i << 1],\
    \ &node[i << 1 | 1]);\n        }\n        Self { n, size, node }\n    }\n}\n\n\
    impl<M> SegmentTree<M>\nwhere\n    M: Monoid,\n{\n    pub fn new(n: usize) ->\
    \ Self {\n        let size = n.next_power_of_two();\n        let node = vec![M::unit();\
    \ size * 2];\n        Self { n, size, node }\n    }\n\n    pub fn set(&mut self,\
    \ pos: usize, value: M::ValueType) {\n        assert!(pos < self.n);\n       \
    \ let mut pos = pos + self.size;\n        self.node[pos] = value;\n        while\
    \ pos > 1 {\n            pos >>= 1;\n            self.node[pos] = M::op(&self.node[pos\
    \ << 1], &self.node[pos << 1 | 1]);\n        }\n    }\n\n    pub fn get(&self,\
    \ pos: usize) -> M::ValueType {\n        assert!(pos < self.n);\n        self.node[pos\
    \ + self.size].clone()\n    }\n\n    pub fn apply(&mut self, pos: usize, value:\
    \ M::ValueType) {\n        self.set(pos, M::op(&self.get(pos), &value));\n   \
    \ }\n\n    pub fn prod<R>(&self, range: R) -> M::ValueType\n    where\n      \
    \  R: RangeBounds<usize>,\n    {\n        if range.start_bound() == Bound::Unbounded\
    \ && range.end_bound() == Bound::Unbounded {\n            return self.all_prod();\n\
    \        }\n        let mut l = match range.start_bound() {\n            Bound::Included(l)\
    \ => *l,\n            Bound::Excluded(l) => l + 1,\n            Bound::Unbounded\
    \ => 0,\n        };\n        let mut r = match range.end_bound() {\n         \
    \   Bound::Included(r) => r + 1,\n            Bound::Excluded(r) => *r,\n    \
    \        Bound::Unbounded => self.n,\n        };\n        assert!(l <= r && r\
    \ <= self.n);\n        let mut value_left = M::unit();\n        let mut value_right\
    \ = M::unit();\n        l += self.size;\n        r += self.size;\n        while\
    \ l < r {\n            if (l & 1) == 1 {\n                value_left = M::op(&value_left,\
    \ &self.node[l]);\n                l += 1;\n            }\n            if (r &\
    \ 1) == 1 {\n                r -= 1;\n                value_right = M::op(&self.node[r],\
    \ &value_right);\n            }\n            l >>= 1;\n            r >>= 1;\n\
    \        }\n        M::op(&value_left, &value_right)\n    }\n\n    pub fn all_prod(&self)\
    \ -> M::ValueType {\n        self.node[1].clone()\n    }\n\n    pub fn max_right<F>(&self,\
    \ l: usize, mut f: F) -> usize\n    where\n        F: FnMut(&M::ValueType) ->\
    \ bool,\n    {\n        assert!(l < self.n);\n        assert!(f(&M::unit()));\n\
    \        if l == self.n {\n            return self.n;\n        }\n        let\
    \ mut l = l + self.size;\n        let mut value = M::unit();\n        while {\n\
    \            while (l & 1) == 0 {\n                l >>= 1;\n            }\n \
    \           if !f(&M::op(&value, &self.node[l])) {\n                while l <\
    \ self.size {\n                    l *= 2;\n                    if f(&M::op(&value,\
    \ &self.node[l])) {\n                        value = M::op(&value, &self.node[l]);\n\
    \                        l += 1;\n                    }\n                }\n \
    \               return l - self.size;\n            }\n            value = M::op(&value,\
    \ &self.node[l]);\n            l += 1;\n            (l & l.wrapping_neg()) !=\
    \ l\n        } {}\n        self.n\n    }\n\n    pub fn min_left<F>(&self, r: usize,\
    \ mut f: F) -> usize\n    where\n        F: FnMut(&M::ValueType) -> bool,\n  \
    \  {\n        assert!(r <= self.n);\n        assert!(f(&M::unit()));\n       \
    \ if r == 0 {\n            return 0;\n        }\n        let mut r = r + self.size;\n\
    \        let mut value = M::unit();\n        while {\n            r -= 1;\n  \
    \          while r > 1 && (r & 1) == 1 {\n                r >>= 1;\n         \
    \   }\n            if !f(&M::op(&self.node[r], &value)) {\n                while\
    \ r < self.size {\n                    r = r * 2 + 1;\n                    if\
    \ f(&M::op(&self.node[r], &value)) {\n                        value = M::op(&self.node[r],\
    \ &value);\n                        r -= 1;\n                    }\n         \
    \       }\n                return r + 1 - self.size;\n            }\n        \
    \    value = M::op(&self.node[r], &value);\n            (r & r.wrapping_neg())\
    \ != r\n        } {}\n        0\n    }\n}\n"
  dependsOn:
  - crates/traits/monoid/src/lib.rs
  isVerificationFile: false
  path: crates/ds/segment_tree/segment_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-08-31 02:27:27+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/src/main.rs
documentation_of: crates/ds/segment_tree/segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/ds/segment_tree/segment_tree/src/lib.rs
- /library/crates/ds/segment_tree/segment_tree/src/lib.rs.html
title: crates/ds/segment_tree/segment_tree/src/lib.rs
---
