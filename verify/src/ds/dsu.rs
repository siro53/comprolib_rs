// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use ds::dsu::Dsu;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }
    let mut uf = Dsu::new(n);
    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize
        };
        if t == 0 {
            uf.merge(u, v);
        } else {
            let res = if uf.same(u, v) { 1 } else { 0 };
            println!("{}", res);
        }
    }
}
