---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/segment_tree/src/lib.rs
    title: crates/ds/segment_tree/segment_tree/src/lib.rs
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
  code: "pub trait Monoid {\n    type ValueType: Clone;\n    fn op(left_value: &Self::ValueType,\
    \ right_value: &Self::ValueType) -> Self::ValueType;\n    fn unit() -> Self::ValueType;\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/traits/monoid/src/lib.rs
  requiredBy:
  - crates/ds/segment_tree/segment_tree/src/lib.rs
  timestamp: '2025-08-30 21:29:59+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_range_sum/src/main.rs
documentation_of: crates/traits/monoid/src/lib.rs
layout: document
redirect_from:
- /library/crates/traits/monoid/src/lib.rs
- /library/crates/traits/monoid/src/lib.rs.html
title: crates/traits/monoid/src/lib.rs
---
