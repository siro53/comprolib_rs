// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use monoid_util::add::Additive;
use proconio::{fastout, input};
use segment_tree::SegmentTree;

type SegTree = SegmentTree<Additive<i64>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut seg = a.into_iter().collect::<SegTree>();
    for _ in 0..q {
        input! {
            t: usize
        }
        match t {
            0 => {
                input! {
                    p: usize,
                    x: i64
                }
                seg.apply(p, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize
                }
                println!("{}", seg.prod(l..r));
            }
            _ => unreachable!(),
        }
    }
}
