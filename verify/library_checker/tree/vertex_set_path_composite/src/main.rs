// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use graph::graph::Tree;
use heavy_light_decomposition::HeavyLightDecomposition;
use modint::ModInt998244353;
use monoid::Monoid;
use monoid::affine::{Affine, AffineOperator};
use proconio::{fastout, input};
use segment_tree::SegmentTree;

type mint = ModInt998244353;
type AL = AffineOperator<mint, false>;
type AR = AffineOperator<mint, true>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }
    let mut af = Vec::<Affine<mint>>::new();
    for _ in 0..n {
        input! {
            a: u32,
            b: u32
        }
        af.push(Affine(mint::raw(a), mint::raw(b)));
    }
    let mut tree = Tree::<usize>::new(n);
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize
        }
        tree.add_edge(u, v, 1);
    }
    let mut hld = HeavyLightDecomposition::<usize>::new(&tree);
    hld.build(0);
    let mut seg = af.clone().into_iter().collect::<SegmentTree<AL>>();
    let mut seg_rev = af.clone().into_iter().collect::<SegmentTree<AR>>();
    for i in 0..n {
        seg.set(hld[i], af[i].clone());
        seg_rev.set(hld[i], af[i].clone());
    }

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
                seg.set(hld[p], Affine(mint::raw(c), mint::raw(d)));
                seg_rev.set(hld[p], Affine(mint::raw(c), mint::raw(d)));
            }
            1 => {
                input! {
                    u: usize,
                    v: usize,
                    x: u32
                }
                let mut res_l = AL::unit();
                let mut res_r = AR::unit();
                let mut f = |l: usize, r: usize| {
                    res_l = AL::op(&seg.prod(l..r), &res_l);
                };
                let mut inv_f = |l: usize, r: usize| {
                    res_r = AR::op(&seg_rev.prod(l..r), &res_r);
                };
                hld.path_query_not_commutative(u, v, &mut f, &mut inv_f, false);
                println!("{}", AL::op(&res_r, &res_l).eval(mint::raw(x)));
            }
            _ => unreachable!(),
        }
    }
}
