use modint::{modulus::Modulus, static_modint::StaticModInt};
use monoid_util::affine::{Affine, AffineOperator};
use proconio::{fastout, input};
use segment_tree::SegmentTree;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const MOD: u32 = 998_244_353;
}

type Mint = StaticModInt<Mod998244353>;
type SegTree = SegmentTree<AffineOperator<Mint>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }
    let mut f = Vec::<Affine<Mint>>::new();
    for i in 0..n {
        input! {
            a: u32,
            b: u32
        }
        f[i] = Affine(Mint::raw(a), Mint::raw(b));
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
