// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use fenwick_tree::fenwick_tree::FenwickTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n]
    }
    let mut fwt = FenwickTree::<i64>::new(n);
    for i in 0..n {
        fwt.add(i, a[i]);
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
                fwt.add(p, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize
                }
                println!("{}", fwt.sum(l..r));
            }
            _ => unreachable!(),
        }
    }
}
