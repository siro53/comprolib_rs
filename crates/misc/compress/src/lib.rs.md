---
data:
  _extendedDependsOn: []
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
  code: "use std::ops::Index;\n\nuse superslice::Ext;\n\n#[derive(Default)]\npub struct\
    \ Compress<T> {\n    data: Vec<T>,\n    is_built: bool,\n}\n\nimpl<T> Compress<T>\n\
    where\n    T: Clone + PartialOrd + Ord,\n{\n    pub fn new() -> Self {\n     \
    \   Self {\n            data: Vec::<T>::new(),\n            is_built: false,\n\
    \        }\n    }\n\n    pub fn push(&mut self, value: T) {\n        self.data.push(value);\n\
    \        self.is_built = false;\n    }\n\n    pub fn build(&mut self) {\n    \
    \    self.data.sort();\n        self.data.dedup();\n        self.is_built = true;\n\
    \    }\n\n    pub fn get(&self, value: T) -> usize {\n        assert!(self.is_built);\n\
    \        self.data.lower_bound(&value)\n    }\n\n    #[allow(clippy::len_without_is_empty)]\n\
    \    pub fn len(&self) -> usize {\n        self.data.len()\n    }\n}\n\nimpl<T>\
    \ Index<usize> for Compress<T> {\n    type Output = T;\n\n    fn index(&self,\
    \ index: usize) -> &Self::Output {\n        assert!(self.is_built);\n        &self.data[index]\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/compress/src/lib.rs
  requiredBy: []
  timestamp: '2025-08-31 23:58:05+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/aoj/aoj2426/src/main.rs
documentation_of: crates/misc/compress/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/compress/src/lib.rs
- /library/crates/misc/compress/src/lib.rs.html
title: crates/misc/compress/src/lib.rs
---
