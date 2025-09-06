---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/misc/rle/src/lib.rs
    title: crates/misc/rle/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        s: String\n    }\n    let rle = rle::runlength_encoding(&s.chars().collect::<Vec<_>>());\n\
    \    let res = rle.iter().map(|(c, _)| c).collect::<String>();\n    println!(\"\
    {}\", res);\n}\n"
  dependsOn:
  - crates/misc/rle/src/lib.rs
  isVerificationFile: false
  path: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:29:25+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
layout: document
redirect_from:
- /library/verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
- /library/verify/yukicoder/yuki1469/yuki1469_1/src/main.rs.html
title: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
---
