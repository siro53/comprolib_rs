---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/yukicoder/yuki789/yuki789_1/src/main.rs
    title: verify/yukicoder/yuki789/yuki789_1/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Bound, RangeBounds};\n\nuse monoid::Monoid;\n\npub struct\
    \ Node<M>\nwhere\n    M: Monoid,\n{\n    value: M::ValueType,\n    children_node:\
    \ [Option<Box<Node<M>>>; 2],\n}\n\nimpl<M> Node<M>\nwhere\n    M: Monoid,\n{\n\
    \    pub fn new(value: M::ValueType) -> Self {\n        Self {\n            value,\n\
    \            children_node: [None, None],\n        }\n    }\n}\n\npub struct DynamicSegmentTree<M,\
    \ const MIN_INDEX: i64, const MAX_INDEX: i64>\nwhere\n    M: Monoid,\n{\n    root:\
    \ Option<Box<Node<M>>>,\n}\n\nimpl<M, const MIN_INDEX: i64, const MAX_INDEX: i64>\
    \ Default\n    for DynamicSegmentTree<M, MIN_INDEX, MAX_INDEX>\nwhere\n    M:\
    \ Monoid,\n{\n    fn default() -> Self {\n        Self { root: None }\n    }\n\
    }\n\nimpl<M, const MIN_INDEX: i64, const MAX_INDEX: i64> DynamicSegmentTree<M,\
    \ MIN_INDEX, MAX_INDEX>\nwhere\n    M: Monoid,\n{\n    pub fn new() -> Self {\n\
    \        Self::default()\n    }\n\n    pub fn set(&mut self, pos: i64, value:\
    \ M::ValueType) {\n        assert!(MIN_INDEX <= pos);\n        assert!(pos < MAX_INDEX);\n\
    \        Self::_set(&mut self.root, MIN_INDEX, MAX_INDEX, pos, value);\n    }\n\
    \n    pub fn get(&self, pos: i64) -> M::ValueType {\n        assert!(MIN_INDEX\
    \ <= pos);\n        assert!(pos < MAX_INDEX);\n        Self::_get(&self.root,\
    \ MIN_INDEX, MAX_INDEX, pos)\n    }\n\n    pub fn prod<R: RangeBounds<i64>>(&self,\
    \ range: R) -> M::ValueType {\n        let l = match range.start_bound() {\n \
    \           Bound::Included(l) => *l,\n            Bound::Excluded(l) => l + 1,\n\
    \            Bound::Unbounded => MIN_INDEX,\n        };\n        let r = match\
    \ range.end_bound() {\n            Bound::Included(r) => r + 1,\n            Bound::Excluded(r)\
    \ => *r,\n            Bound::Unbounded => MAX_INDEX,\n        };\n        assert!(MIN_INDEX\
    \ <= l);\n        assert!(l <= r);\n        assert!(r <= MAX_INDEX);\n       \
    \ Self::_prod(&self.root, MIN_INDEX, MAX_INDEX, l, r)\n    }\n\n    fn _set(node:\
    \ &mut Option<Box<Node<M>>>, l: i64, r: i64, pos: i64, value: M::ValueType) {\n\
    \        if node.is_none() {\n            *node = Some(Box::new(Node::<M>::new(M::unit())));\n\
    \        }\n        let ptr = node.as_mut().unwrap();\n        if r - l == 1 {\n\
    \            ptr.value = value;\n            return;\n        }\n        let mid\
    \ = (l + r) / 2;\n        if pos < mid {\n            Self::_set(&mut ptr.children_node[0],\
    \ l, mid, pos, value);\n        } else {\n            Self::_set(&mut ptr.children_node[1],\
    \ mid, r, pos, value);\n        }\n        let left_value = if ptr.children_node[0].is_none()\
    \ {\n            M::unit()\n        } else {\n            ptr.children_node[0].as_mut().unwrap().value.clone()\n\
    \        };\n        let right_value = if ptr.children_node[0].is_none() {\n \
    \           M::unit()\n        } else {\n            ptr.children_node[1].as_mut().unwrap().value.clone()\n\
    \        };\n        ptr.value = M::op(&left_value, &right_value);\n    }\n\n\
    \    fn _get(node: &Option<Box<Node<M>>>, l: i64, r: i64, pos: i64) -> M::ValueType\
    \ {\n        if node.is_none() {\n            return M::unit();\n        }\n \
    \       let ptr = node.as_ref().unwrap();\n        if r - l == 1 {\n         \
    \   return ptr.value.clone();\n        }\n        let mid = (l + r) / 2;\n   \
    \     if pos < mid {\n            Self::_get(&ptr.children_node[0], l, mid, pos)\n\
    \        } else {\n            Self::_get(&ptr.children_node[1], mid, r, pos)\n\
    \        }\n    }\n\n    fn _prod(node: &Option<Box<Node<M>>>, l: i64, r: i64,\
    \ ql: i64, qr: i64) -> M::ValueType {\n        if node.is_none() || (qr <= l ||\
    \ r <= ql) {\n            return M::unit();\n        }\n        let ptr = node.as_ref().unwrap();\n\
    \        if ql <= l && r <= qr {\n            return ptr.value.clone();\n    \
    \    }\n        let mid = (l + r) / 2;\n        M::op(\n            &Self::_prod(&ptr.children_node[0],\
    \ l, mid, ql, qr),\n            &Self::_prod(&ptr.children_node[1], mid, r, ql,\
    \ qr),\n        )\n    }\n}\n"
  dependsOn:
  - crates/traits/monoid/src/lib.rs
  isVerificationFile: false
  path: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-09-07 18:39:39+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/yukicoder/yuki789/yuki789_1/src/main.rs
documentation_of: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
- /library/crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs.html
title: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
---
