// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use dsu::Dsu;
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
        match t {
            0 => {
                uf.merge(u, v);
            }
            1 => println!("{}", uf.same(u, v) as i32),
            _ => unreachable!(),
        }
    }
}
