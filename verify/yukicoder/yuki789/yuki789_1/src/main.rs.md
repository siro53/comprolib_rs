---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
    title: crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/add.rs
    title: crates/traits/monoid/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/affine.rs
    title: crates/traits/monoid/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_and.rs
    title: crates/traits/monoid/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_or.rs
    title: crates/traits/monoid/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/bitwise_xor.rs
    title: crates/traits/monoid/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max.rs
    title: crates/traits/monoid/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/max_with_index.rs
    title: crates/traits/monoid/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/min.rs
    title: crates/traits/monoid/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/min_with_index.rs
    title: crates/traits/monoid/src/min_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/mul.rs
    title: crates/traits/monoid/src/mul.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/789
    links:
    - https://yukicoder.me/problems/no/789
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/789\n\n\
    use dynamic_segment_tree::DynamicSegmentTree;\nuse monoid::add::Additive;\nuse\
    \ proconio::{fastout, input};\n\ntype DynamicSegTree = DynamicSegmentTree<Additive<i64>,\
    \ 0, 1_000_000_002>;\n\n#[fastout]\nfn main() {\n    input! {\n        q: usize\n\
    \    };\n    let mut seg = DynamicSegTree::new();\n    let mut ans = 0_i64;\n\
    \    for _ in 0..q {\n        input! {\n            t: usize\n        };\n   \
    \     match t {\n            0 => {\n                input! {\n              \
    \      x: i64,\n                    y: i64\n                };\n             \
    \   seg.set(x, seg.get(x) + y);\n            }\n            1 => {\n         \
    \       input! {\n                    l: i64,\n                    r: i64\n  \
    \              }\n                ans += seg.prod(l..=r);\n            }\n   \
    \         _ => unreachable!(),\n        }\n    }\n    println!(\"{}\", ans);\n\
    }\n"
  dependsOn:
  - crates/ds/segment_tree/dynamic_segment_tree/src/lib.rs
  - crates/traits/monoid/src/add.rs
  - crates/traits/monoid/src/affine.rs
  - crates/traits/monoid/src/bitwise_and.rs
  - crates/traits/monoid/src/bitwise_or.rs
  - crates/traits/monoid/src/bitwise_xor.rs
  - crates/traits/monoid/src/lib.rs
  - crates/traits/monoid/src/max.rs
  - crates/traits/monoid/src/max_with_index.rs
  - crates/traits/monoid/src/min.rs
  - crates/traits/monoid/src/min_with_index.rs
  - crates/traits/monoid/src/mul.rs
  isVerificationFile: true
  path: verify/yukicoder/yuki789/yuki789_1/src/main.rs
  requiredBy: []
  timestamp: '2025-10-18 15:26:53+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/yuki789/yuki789_1/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/yuki789/yuki789_1/src/main.rs
- /verify/verify/yukicoder/yuki789/yuki789_1/src/main.rs.html
title: verify/yukicoder/yuki789/yuki789_1/src/main.rs
---
