use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize,
    }

    println!("{}", r / d - (l - 1) / d);
}
