// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use binary_trie::BinaryTrie;
use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut bt = BinaryTrie::<u32>::new();
    input! {
        q: usize
    }
    for _ in 0..q {
        input! {
            t: usize,
            x: u32
        }
        match t {
            0 => {
                if bt.count(x) > 0 {
                    continue;
                }
                bt.insert(x);
            }
            1 => {
                bt.erase(x);
            }
            2 => {
                bt.all_xor(x);
                println!("{}", bt.min_element());
                bt.all_xor(x);
            }
            _ => unreachable!(),
        }
    }
}
