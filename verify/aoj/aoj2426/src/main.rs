// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/2426

use compress::Compress;
use cumulative_sum_2d::CumulativeSum2D;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut cx = Compress::<i32>::new();
    let mut cy = Compress::<i32>::new();
    let mut treasure = vec![(0_i32, 0_i32); n];
    for i in 0..n {
        input! {
            x: i32,
            y: i32
        }
        cx.push(x);
        cy.push(y);
        treasure[i] = (x, y);
    }
    cx.build();
    cy.build();
    let len_x = cx.len();
    let len_y = cy.len();
    let mut sum_2d = CumulativeSum2D::<i32>::new(len_x, len_y);
    for i in 0..n {
        let (x, y) = treasure[i];
        // dbg!(cx.get(x), cy.get(y));
        sum_2d.add(cx.get(x), cy.get(y), 1);
    }
    sum_2d.build();
    for _ in 0..m {
        input! {
            x1: i32,
            y1: i32,
            x2: i32,
            y2: i32
        }
        let sx = cx.get(x1);
        let mut gx = cx.get(x2);
        let sy = cy.get(y1);
        let mut gy = cy.get(y2);
        if gx < len_x && cx[gx] == x2 {
            gx += 1;
        }
        if gy < len_y && cy[gy] == y2 {
            gy += 1;
        }
        // dbg!(sx, gx, sy, gy);
        println!("{}", sum_2d.sum(sx..gx, sy..gy));
    }
}
