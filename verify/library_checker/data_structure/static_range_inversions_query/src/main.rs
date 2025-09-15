// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query

use compress::Compress;
use fenwick_tree::fenwick_tree::FenwickTree;
use mo::Mo;
use proconio::{fastout, input};

struct Solver {
    a: Vec<usize>,
    fwt: FenwickTree<i64>,
    now_ans: i64,
    sz: usize,
}

impl Mo for Solver {
    type Output = i64;

    fn add_left(&mut self, idx: usize) {
        self.now_ans += self.fwt.sum(0..self.a[idx]);
        self.fwt.add(self.a[idx], 1);
    }

    fn add_right(&mut self, idx: usize) {
        self.now_ans += self.fwt.sum(self.a[idx] + 1..self.sz);
        self.fwt.add(self.a[idx], 1);
    }

    fn del_left(&mut self, idx: usize) {
        self.now_ans -= self.fwt.sum(..self.a[idx]);
        self.fwt.add(self.a[idx], -1);
    }

    fn del_right(&mut self, idx: usize) {
        self.now_ans -= self.fwt.sum(self.a[idx] + 1..self.sz);
        self.fwt.add(self.a[idx], -1);
    }

    fn query(&self) -> Self::Output {
        self.now_ans
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize, usize); q]
    }
    let comp = a.clone().into_iter().collect::<Compress<usize>>();
    for i in 0..n {
        a[i] = comp.get(a[i]);
    }
    let sz = comp.len();
    let mut solver = Solver {
        a,
        fwt: FenwickTree::<i64>::new(sz),
        now_ans: 0,
        sz,
    };
    let ans = solver.run(n, &queries);
    for i in 0..q {
        println!("{}", ans[i]);
    }
}
