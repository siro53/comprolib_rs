---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/misc/compress/src/lib.rs
    title: crates/misc/compress/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/cumulative_sum_2d/src/lib.rs
    title: crates/misc/cumulative_sum_2d/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://onlinejudge.u-aizu.ac.jp/problems/2426
    links:
    - https://onlinejudge.u-aizu.ac.jp/problems/2426
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2426\n\
    \nuse compress::Compress;\nuse cumulative_sum_2d::CumulativeSum2D;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  m: usize\n    }\n    let mut cx = Compress::<i32>::new();\n    let mut cy =\
    \ Compress::<i32>::new();\n    let mut treasure = vec![(0_i32, 0_i32); n];\n \
    \   for i in 0..n {\n        input! {\n            x: i32,\n            y: i32\n\
    \        }\n        cx.push(x);\n        cy.push(y);\n        treasure[i] = (x,\
    \ y);\n    }\n    cx.build();\n    cy.build();\n    let len_x = cx.len();\n  \
    \  let len_y = cy.len();\n    let mut sum_2d = CumulativeSum2D::<i32>::new(len_x,\
    \ len_y);\n    for i in 0..n {\n        let (x, y) = treasure[i];\n        //\
    \ dbg!(cx.get(x), cy.get(y));\n        sum_2d.add(cx.get(x), cy.get(y), 1);\n\
    \    }\n    sum_2d.build();\n    for _ in 0..m {\n        input! {\n         \
    \   x1: i32,\n            y1: i32,\n            x2: i32,\n            y2: i32\n\
    \        }\n        let sx = cx.get(x1);\n        let mut gx = cx.get(x2);\n \
    \       let sy = cy.get(y1);\n        let mut gy = cy.get(y2);\n        if gx\
    \ < len_x && cx[gx] == x2 {\n            gx += 1;\n        }\n        if gy <\
    \ len_y && cy[gy] == y2 {\n            gy += 1;\n        }\n        // dbg!(sx,\
    \ gx, sy, gy);\n        println!(\"{}\", sum_2d.sum(sx..gx, sy..gy));\n    }\n\
    }\n"
  dependsOn:
  - crates/misc/compress/src/lib.rs
  - crates/misc/cumulative_sum_2d/src/lib.rs
  isVerificationFile: true
  path: verify/aoj/aoj2426/src/main.rs
  requiredBy: []
  timestamp: '2025-09-15 18:36:07+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/aoj/aoj2426/src/main.rs
layout: document
redirect_from:
- /verify/verify/aoj/aoj2426/src/main.rs
- /verify/verify/aoj/aoj2426/src/main.rs.html
title: verify/aoj/aoj2426/src/main.rs
---
