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
    path: verify/library_checker/data_structure/set_xor_min/src/main.rs
    title: verify/library_checker/data_structure/set_xor_min/src/main.rs
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
  code: "use std::{\n    ops::{AddAssign, BitAnd, BitXorAssign, Shl, Shr},\n    usize,\n\
    };\n\nuse numeric::{one::One, zero::Zero};\n\n#[derive(Clone, Default)]\npub struct\
    \ Node {\n    count: usize,\n    children_node: [Option<Box<Node>>; 2],\n}\n\n\
    impl Node {\n    pub fn new() -> Self {\n        Self::default()\n    }\n}\n\n\
    mod private {\n    pub trait Sealed {}\n    impl Sealed for u32 {}\n    impl Sealed\
    \ for u64 {}\n}\n\npub trait BinaryTrieTrait:\n    private::Sealed\n    + Zero\n\
    \    + One\n    + Copy\n    + PartialEq\n    + Shr<usize, Output = Self>\n   \
    \ + Shl<usize, Output = Self>\n    + BitAnd<Output = Self>\n    + AddAssign\n\
    \    + BitXorAssign\n{\n    const BIT_LENGTH: usize;\n\n    fn get_dir<T: BinaryTrieTrait>(value:\
    \ T, i: usize) -> usize {\n        if (value >> i & T::one()) == T::one() {\n\
    \            1\n        } else {\n            0\n        }\n    }\n}\n\nimpl BinaryTrieTrait\
    \ for u32 {\n    const BIT_LENGTH: usize = 32;\n}\n\nimpl BinaryTrieTrait for\
    \ u64 {\n    const BIT_LENGTH: usize = 64;\n}\n\npub struct BinaryTrie<T: BinaryTrieTrait>\
    \ {\n    root: Option<Box<Node>>,\n    all_xor_value: T,\n}\n\nimpl<T: BinaryTrieTrait>\
    \ Default for BinaryTrie<T> {\n    fn default() -> Self {\n        Self {\n  \
    \          root: None,\n            all_xor_value: T::zero(),\n        }\n   \
    \ }\n}\n\nimpl<T: BinaryTrieTrait> BinaryTrie<T> {\n    pub fn new() -> Self {\n\
    \        Self::default()\n    }\n\n    pub fn count(&self, value: T) -> usize\
    \ {\n        if self.root.is_none() {\n            return 0;\n        }\n    \
    \    let mut now = self.root.as_ref();\n        for i in (0..T::BIT_LENGTH).rev()\
    \ {\n            let dir = T::get_dir(value, i);\n            now = now.unwrap().children_node[dir].as_ref();\n\
    \            if now.is_none() || now.unwrap().count == 0 {\n                return\
    \ 0;\n            }\n        }\n        now.unwrap().count\n    }\n\n    pub fn\
    \ insert(&mut self, value: T) {\n        if self.root.is_none() {\n          \
    \  self.root = Some(Box::new(Node::new()));\n        }\n        let mut now =\
    \ self.root.as_mut().unwrap();\n        now.count += 1;\n        for i in (0..T::BIT_LENGTH).rev()\
    \ {\n            let dir: usize = T::get_dir(value, i);\n            let nxt =\
    \ &mut now.children_node[dir];\n            if nxt.is_none() {\n             \
    \   *nxt = Some(Box::new(Node::new()));\n            }\n            now = nxt.as_mut().unwrap();\n\
    \            now.count += 1;\n        }\n    }\n\n    pub fn erase(&mut self,\
    \ value: T) -> bool {\n        if self.count(value) == 0 {\n            return\
    \ false;\n        }\n        let mut now = self.root.as_mut().unwrap();\n    \
    \    for i in (0..T::BIT_LENGTH).rev() {\n            now.count -= 1;\n      \
    \      let dir = T::get_dir(value, i);\n            now = now.children_node[dir].as_mut().unwrap();\n\
    \            assert!(now.count > 0);\n        }\n        now.count -= 1;\n   \
    \     true\n    }\n\n    pub fn min_element(&self) -> T {\n        assert!(self.root.is_some());\n\
    \        let mut res = T::zero();\n        let mut now = self.root.as_ref().unwrap();\n\
    \        for i in (0..T::BIT_LENGTH).rev() {\n            let dir = T::get_dir(self.all_xor_value,\
    \ i);\n            let mut nxt = now.children_node[dir].as_ref();\n          \
    \  if nxt.is_none() || nxt.unwrap().count == 0 {\n                res += T::one()\
    \ << i;\n                nxt = now.children_node[dir ^ 1].as_ref();\n        \
    \    }\n            now = nxt.unwrap();\n        }\n        res\n    }\n\n   \
    \ pub fn max_element(&self) -> T {\n        assert!(self.root.is_some());\n  \
    \      let mut res = T::zero();\n        let mut now = self.root.as_ref().unwrap();\n\
    \        for i in (0..T::BIT_LENGTH).rev() {\n            let dir = T::get_dir(self.all_xor_value,\
    \ i) ^ 1;\n            let mut nxt = now.children_node[dir].as_ref();\n      \
    \      if nxt.is_none() || nxt.unwrap().count == 0 {\n                nxt = now.children_node[dir\
    \ ^ 1].as_ref();\n            } else {\n                res += T::one() << i;\n\
    \            }\n            now = nxt.unwrap();\n        }\n        res\n    }\n\
    \n    pub fn get_kth_min_element(&self, k: usize) -> T {\n        assert!(k <\
    \ self.size());\n        let mut now = self.root.as_ref().unwrap();\n        let\
    \ mut res = T::zero();\n        let mut k = k;\n        for i in (0..T::BIT_LENGTH).rev()\
    \ {\n            let dir = T::get_dir(self.all_xor_value, i);\n            let\
    \ small_node = now.children_node[dir].as_ref();\n            let big_node = now.children_node[dir\
    \ ^ 1].as_ref();\n            let small_count = if let Some(n) = small_node {\
    \ n.count } else { 0 };\n            let big_count = if let Some(n) = big_node\
    \ { n.count } else { 0 };\n            assert!(0 < small_count + big_count);\n\
    \            assert!(k < small_count + big_count);\n            if k >= small_count\
    \ {\n                res += T::one() << i;\n                now = big_node.unwrap();\n\
    \                k -= small_count;\n            } else {\n                now\
    \ = small_node.unwrap();\n            }\n        }\n        res\n    }\n\n   \
    \ pub fn lower_bound(&self, value: T) -> usize {\n        assert!(self.root.is_some());\n\
    \        let mut res: usize = 0;\n        let mut now = self.root.as_ref().unwrap();\n\
    \        for i in (0..T::BIT_LENGTH).rev() {\n            let dir = T::get_dir(self.all_xor_value,\
    \ i);\n            let small_node = now.children_node[dir].as_ref();\n       \
    \     let big_node = now.children_node[dir ^ 1].as_ref();\n            let small_count\
    \ = if let Some(n) = small_node { n.count } else { 0 };\n            let big_count\
    \ = if let Some(n) = big_node { n.count } else { 0 };\n            if small_count\
    \ == 0 {\n                assert!(big_count > 0);\n                now = big_node.unwrap();\n\
    \            } else if big_count == 0 {\n                assert!(small_count >\
    \ 0);\n                now = small_node.unwrap();\n            } else if dir ==\
    \ T::get_dir(value, i) {\n                now = small_node.unwrap();\n       \
    \     } else {\n                res += small_count;\n                now = big_node.unwrap();\n\
    \            }\n        }\n        res\n    }\n\n    pub fn upper_bound(&self,\
    \ value: T) -> usize {\n        let idx = self.lower_bound(value);\n        let\
    \ kth_min_element = self.get_kth_min_element(idx);\n        if kth_min_element\
    \ != value {\n            return idx;\n        }\n        let count = self.count(kth_min_element);\n\
    \        idx + count\n    }\n\n    pub fn size(&self) -> usize {\n        if self.root.is_none()\
    \ {\n            0\n        } else {\n            self.root.as_ref().unwrap().count\n\
    \        }\n    }\n\n    pub fn is_empty(&self) -> bool {\n        self.root.is_none()\
    \ || self.root.as_ref().unwrap().count == 0\n    }\n\n    pub fn all_xor(&mut\
    \ self, value: T) {\n        self.all_xor_value ^= value;\n    }\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use crate::BinaryTrie;\n    use rand::Rng;\n    use superslice::Ext;\n\
    \n    #[test]\n    fn binary_trie_test() {\n        const TESTCASE_NUM: usize\
    \ = 50;\n        const QUERY_NUM: usize = 10000;\n        for testcase_id in 0..TESTCASE_NUM\
    \ {\n            let mut a = Vec::<u32>::new();\n            let mut bt = BinaryTrie::<u32>::new();\n\
    \            let mut rng = rand::rng();\n            for _ in 0..QUERY_NUM {\n\
    \                let t: usize = if a.is_empty() {\n                    0\n   \
    \             } else {\n                    rng.random_range(0..7)\n         \
    \       };\n                match t {\n                    0 => {\n          \
    \              // insert\n                        let x: u32 = rng.random();\n\
    \                        bt.insert(x);\n                        a.push(x);\n \
    \                   }\n                    1 => {\n                        //\
    \ erase\n                        let a_len = a.len();\n                      \
    \  let idx = rng.random_range(0..a_len);\n                        a.swap(idx,\
    \ a_len - 1);\n                        bt.erase(*a.last().unwrap());\n       \
    \                 a.pop();\n                    }\n                    2 => {\n\
    \                        // min_element\n                        assert!(*a.iter().reduce(std::cmp::min).unwrap()\
    \ == bt.min_element());\n                    }\n                    3 => {\n \
    \                       // max_element\n                        assert!(*a.iter().reduce(std::cmp::max).unwrap()\
    \ == bt.max_element());\n                    }\n                    4 => {\n \
    \                       // kth_element\n                        a.sort();\n  \
    \                      let a_len = a.len();\n                        let k = rng.random_range(0..a_len);\n\
    \                        assert!(a[k] == bt.get_kth_min_element(k));\n       \
    \             }\n                    5 => {\n                        // lower_bound\n\
    \                        a.sort();\n                        let a_len = a.len();\n\
    \                        let k = rng.random_range(0..a_len);\n               \
    \         let index1 = bt.lower_bound(a[k]);\n                        let index2\
    \ = a.lower_bound(&a[k]);\n                        assert!(index1 == index2);\n\
    \                    }\n                    6 => {}\n                    _ =>\
    \ unreachable!(),\n                }\n            }\n            eprintln!(\"\
    passed Test {} / {}\", testcase_id + 1, TESTCASE_NUM);\n        }\n    }\n}\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/ds/binary_trie/src/lib.rs
  requiredBy: []
  timestamp: '2025-10-13 14:17:26+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/set_xor_min/src/main.rs
documentation_of: crates/ds/binary_trie/src/lib.rs
layout: document
redirect_from:
- /library/crates/ds/binary_trie/src/lib.rs
- /library/crates/ds/binary_trie/src/lib.rs.html
title: crates/ds/binary_trie/src/lib.rs
---
