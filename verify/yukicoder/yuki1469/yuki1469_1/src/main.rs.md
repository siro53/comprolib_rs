---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/misc/rle/src/lib.rs
    title: crates/misc/rle/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1469
    links:
    - https://yukicoder.me/problems/no/1469
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1469\n\n\
    use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     s: String\n    }\n    let rle = rle::runlength_encoding(&s.chars().collect::<Vec<_>>());\n\
    \    let res = rle.iter().map(|(c, _)| c).collect::<String>();\n    println!(\"\
    {}\", res);\n}\n"
  dependsOn:
  - crates/misc/rle/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:34:11+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
- /verify/verify/yukicoder/yuki1469/yuki1469_1/src/main.rs.html
title: verify/yukicoder/yuki1469/yuki1469_1/src/main.rs
---
