---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
    title: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
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
  code: "pub fn runlength_encoding<T>(v: &Vec<T>) -> Vec<(T, usize)>\nwhere\n    T:\
    \ Copy + PartialEq,\n{\n    if v.is_empty() {\n        return Vec::<(T, usize)>::new();\n\
    \    }\n    let mut res = vec![(v[0], 1_usize)];\n    for e in v {\n        if\
    \ res.last().unwrap().0 == *e {\n            res.last_mut().unwrap().1 += 1;\n\
    \        } else {\n            res.push((*e, 1));\n        }\n    }\n    res\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/rle/src/lib.rs
  requiredBy: []
  timestamp: '2025-09-06 15:29:25+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
documentation_of: crates/misc/rle/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/rle/src/lib.rs
- /library/crates/misc/rle/src/lib.rs.html
title: crates/misc/rle/src/lib.rs
---
