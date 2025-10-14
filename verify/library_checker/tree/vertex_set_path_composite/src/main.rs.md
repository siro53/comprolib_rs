---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/ds/segment_tree/segment_tree/src/lib.rs
    title: crates/ds/segment_tree/segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/directed_graph_util.rs
    title: crates/graph/graph/src/directed_graph_util.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/graph.rs
    title: crates/graph/graph/src/graph.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/shortest_path.rs
    title: crates/graph/graph/src/shortest_path.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/tree_util.rs
    title: crates/graph/graph/src/tree_util.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/heavy_light_decomposition/src/lib.rs
    title: crates/graph/heavy_light_decomposition/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  - icon: ':heavy_check_mark:'
    path: crates/modint/src/static_modint.rs
    title: crates/modint/src/static_modint.rs
  - icon: ':heavy_check_mark:'
    path: crates/traits/monoid/src/lib.rs
    title: crates/traits/monoid/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/add.rs
    title: crates/util/monoid_util/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/affine.rs
    title: crates/util/monoid_util/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_and.rs
    title: crates/util/monoid_util/src/bitwise_and.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_or.rs
    title: crates/util/monoid_util/src/bitwise_or.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/bitwise_xor.rs
    title: crates/util/monoid_util/src/bitwise_xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/lib.rs
    title: crates/util/monoid_util/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max.rs
    title: crates/util/monoid_util/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/max_with_index.rs
    title: crates/util/monoid_util/src/max_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/min.rs
    title: crates/util/monoid_util/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/min_with_index.rs
    title: crates/util/monoid_util/src/min_with_index.rs
  - icon: ':heavy_check_mark:'
    path: crates/util/monoid_util/src/mul.rs
    title: crates/util/monoid_util/src/mul.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite\n\
    \nuse graph::graph::Tree;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    use modint::ModInt998244353;\nuse monoid::Monoid;\nuse monoid_util::affine::{Affine,\
    \ AffineOperator};\nuse proconio::{fastout, input};\nuse segment_tree::SegmentTree;\n\
    \ntype mint = ModInt998244353;\ntype AL = AffineOperator<mint, false>;\ntype AR\
    \ = AffineOperator<mint, true>;\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     n: usize,\n        q: usize\n    }\n    let mut af = Vec::<Affine<mint>>::new();\n\
    \    for _ in 0..n {\n        input! {\n            a: u32,\n            b: u32\n\
    \        }\n        af.push(Affine(mint::raw(a), mint::raw(b)));\n    }\n    let\
    \ mut tree = Tree::<usize>::new(n);\n    for _ in 0..n - 1 {\n        input! {\n\
    \            u: usize,\n            v: usize\n        }\n        tree.add_edge(u,\
    \ v, 1);\n    }\n    let mut hld = HeavyLightDecomposition::<usize>::new(&tree);\n\
    \    hld.build(0);\n    let mut seg = af.clone().into_iter().collect::<SegmentTree<AL>>();\n\
    \    let mut seg_rev = af.clone().into_iter().collect::<SegmentTree<AR>>();\n\
    \    for i in 0..n {\n        seg.set(hld[i], af[i].clone());\n        seg_rev.set(hld[i],\
    \ af[i].clone());\n    }\n\n    for _ in 0..q {\n        input! {\n          \
    \  t: usize\n        }\n        match t {\n            0 => {\n              \
    \  input! {\n                    p: usize,\n                    c: u32,\n    \
    \                d: u32\n                }\n                seg.set(hld[p], Affine(mint::raw(c),\
    \ mint::raw(d)));\n                seg_rev.set(hld[p], Affine(mint::raw(c), mint::raw(d)));\n\
    \            }\n            1 => {\n                input! {\n               \
    \     u: usize,\n                    v: usize,\n                    x: u32\n \
    \               }\n                let mut res_l = AL::unit();\n             \
    \   let mut res_r = AR::unit();\n                let mut f = |l: usize, r: usize|\
    \ {\n                    res_l = AL::op(&seg.prod(l..r), &res_l);\n          \
    \      };\n                let mut inv_f = |l: usize, r: usize| {\n          \
    \          res_r = AR::op(&seg_rev.prod(l..r), &res_r);\n                };\n\
    \                hld.path_query_not_commutative(u, v, &mut f, &mut inv_f, false);\n\
    \                println!(\"{}\", AL::op(&res_r, &res_l).eval(mint::raw(x)));\n\
    \            }\n            _ => unreachable!(),\n        }\n    }\n}\n"
  dependsOn:
  - crates/ds/segment_tree/segment_tree/src/lib.rs
  - crates/graph/graph/src/directed_graph_util.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/graph.rs
  - crates/graph/graph/src/lib.rs
  - crates/graph/graph/src/shortest_path.rs
  - crates/graph/graph/src/tree_util.rs
  - crates/graph/heavy_light_decomposition/src/lib.rs
  - crates/modint/src/lib.rs
  - crates/modint/src/modulus.rs
  - crates/modint/src/static_modint.rs
  - crates/traits/monoid/src/lib.rs
  - crates/util/monoid_util/src/add.rs
  - crates/util/monoid_util/src/affine.rs
  - crates/util/monoid_util/src/bitwise_and.rs
  - crates/util/monoid_util/src/bitwise_or.rs
  - crates/util/monoid_util/src/bitwise_xor.rs
  - crates/util/monoid_util/src/lib.rs
  - crates/util/monoid_util/src/max.rs
  - crates/util/monoid_util/src/max_with_index.rs
  - crates/util/monoid_util/src/min.rs
  - crates/util/monoid_util/src/min_with_index.rs
  - crates/util/monoid_util/src/mul.rs
  isVerificationFile: true
  path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-10-14 10:00:12+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/vertex_set_path_composite/src/main.rs
- /verify/verify/library_checker/tree/vertex_set_path_composite/src/main.rs.html
title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
---
