---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/binary_trie/src/lib.rs
    title: crates/ds/binary_trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/set_xor_min
    links:
    - https://judge.yosupo.jp/problem/set_xor_min
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::{fastout, input};\n\n#[fastout]\n\
    fn main() {\n    let mut bt = BinaryTrie::<u32>::new();\n    input! {\n      \
    \  q: usize\n    }\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \            x: u32\n        }\n        match t {\n            0 => {\n      \
    \          if bt.count(x) > 0 {\n                    continue;\n             \
    \   }\n                bt.insert(x);\n            }\n            1 => {\n    \
    \            bt.erase(x);\n            }\n            2 => {\n               \
    \ bt.all_xor(x);\n                println!(\"{}\", bt.min_element());\n      \
    \          bt.all_xor(x);\n            }\n            _ => unreachable!(),\n \
    \       }\n    }\n}\n"
  dependsOn:
  - crates/ds/binary_trie/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/set_xor_min/src/main.rs
  requiredBy: []
  timestamp: '2025-10-13 14:17:26+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/set_xor_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/set_xor_min/src/main.rs
- /verify/verify/library_checker/data_structure/set_xor_min/src/main.rs.html
title: verify/library_checker/data_structure/set_xor_min/src/main.rs
---
