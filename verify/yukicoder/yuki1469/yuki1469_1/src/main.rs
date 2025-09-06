use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let rle = rle::runlength_encoding(&s.chars().collect::<Vec<_>>());
    let res = rle.iter().map(|(c, _)| c).collect::<String>();
    println!("{}", res);
}
