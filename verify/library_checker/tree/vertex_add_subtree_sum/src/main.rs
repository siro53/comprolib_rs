// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use fenwick_tree::fenwick_tree::FenwickTree;
use graph::graph::Tree;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut tree = Tree::<usize>::new(n);
    for i in 1..n {
        input! {
            p: usize
        }
        tree.add_edge(p, i, 1);
    }
    let mut hld = HeavyLightDecomposition::<usize>::new(&tree);
    hld.build(0);
    let mut fwt = FenwickTree::<i64>::new(n);
    for i in 0..n {
        fwt.add(hld[i], a[i]);
    }

    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            0 => {
                input! {
                    u: usize,
                    x: i64
                }
                fwt.add(hld[u], x);
            }
            1 => {
                input! {
                    u: usize
                }
                let mut ans = 0;
                let mut f = |l: usize, r: usize| {
                    ans += fwt.sum(l..r);
                };
                hld.subtree_query(u, &mut f);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
