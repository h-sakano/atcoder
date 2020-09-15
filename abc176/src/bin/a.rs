use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    }

    println!("{}", ((n - 1) / x + 1) * t);
}
