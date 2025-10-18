// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

use dynamic_segment_tree::DynamicSegmentTree;
use modint::ModInt998244353;
use monoid::affine::{Affine, AffineOperator};
use proconio::{fastout, input};

type Mint = ModInt998244353;
type DynamicSegTree = DynamicSegmentTree<AffineOperator<Mint, true>, 0, 1_000_000_000>;

#[fastout]
fn main() {
    input! {
        _n: usize,
        q: usize
    };
    let mut seg = DynamicSegTree::new();
    for _ in 0..q {
        input! {
            t: usize
        };
        match t {
            0 => {
                input! {
                    p: i64,
                    c: u32,
                    d: u32,
                };
                seg.set(p, Affine(Mint::raw(c), Mint::raw(d)));
            }
            1 => {
                input! {
                    l: i64,
                    r: i64,
                    x: u32
                };
                let folded_f = seg.prod(l..r);
                println!("{}", folded_f.eval(Mint::raw(x)));
            }
            _ => unreachable!(),
        }
    }
}
