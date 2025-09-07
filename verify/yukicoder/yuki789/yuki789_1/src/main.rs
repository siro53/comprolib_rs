// verification-helper: PROBLEM https://yukicoder.me/problems/no/789

use dynamic_segment_tree::DynamicSegmentTree;
use monoid_util::add::Additive;
use proconio::{fastout, input};

type DynamicSegTree = DynamicSegmentTree<Additive<i64>, 0, 1_000_000_002>;

#[fastout]
fn main() {
    input! {
        q: usize
    };
    let mut seg = DynamicSegTree::new();
    let mut ans = 0_i64;
    for _ in 0..q {
        input! {
            t: usize
        };
        match t {
            0 => {
                input! {
                    x: i64,
                    y: i64
                };
                seg.set(x, seg.get(x) + y);
            }
            1 => {
                input! {
                    l: i64,
                    r: i64
                }
                ans += seg.prod(l..=r);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
