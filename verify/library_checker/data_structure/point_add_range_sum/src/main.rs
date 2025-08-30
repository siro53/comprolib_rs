// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use monoid::Monoid;
use proconio::{fastout, input};
use segment_tree::SegmentTree;

struct M;

impl Monoid for M {
    type ValueType = i64;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        left_value + right_value
    }

    fn unit() -> Self::ValueType {
        0
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut seg = SegmentTree::<M>::new(n);
    for i in 0..n {
        seg.set(i, a[i]);
    }
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
                println!("{}", seg.prod(l, r));
            }
            _ => unreachable!(),
        }
    }
}
