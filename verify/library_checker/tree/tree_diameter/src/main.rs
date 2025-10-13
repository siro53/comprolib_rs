// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter

use graph::graph::Tree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut tree = Tree::<i64>::new(n);
    for _ in 0..n - 1 {
        input! {
            a: usize,
            b: usize,
            c: i64
        }
        tree.add_edge(a, b, c);
    }
    let (diam, path) = tree.diameter();
    println!("{} {}", diam, path.len());
    for i in 0..path.len() {
        if i + 1 == path.len() {
            println!("{}", path[i]);
        } else {
            print!("{} ", path[i]);
        }
    }
}
