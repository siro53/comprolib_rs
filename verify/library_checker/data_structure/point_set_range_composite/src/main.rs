// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use modint::ModInt998244353;
use monoid_util::affine::{Affine, AffineOperator};
use proconio::{fastout, input};
use segment_tree::SegmentTree;

type Mint = ModInt998244353;
type SegTree = SegmentTree<AffineOperator<Mint, true>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }
    let mut f = Vec::<Affine<Mint>>::new();
    for _ in 0..n {
        input! {
            a: u32,
            b: u32
        }
        f.push(Affine(Mint::raw(a), Mint::raw(b)));
    }
    let mut seg = f.into_iter().collect::<SegTree>();
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            0 => {
                input! {
                    p: usize,
                    c: u32,
                    d: u32
                }
                seg.set(p, Affine(Mint::raw(c), Mint::raw(d)));
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                    x: u32
                }
                let folded_f = seg.prod(l..r);
                println!("{}", folded_f.eval(Mint::raw(x)));
            }
            _ => unreachable!(),
        }
    }
}
