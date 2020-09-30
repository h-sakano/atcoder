use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }

    let x = std::cmp::min(b - 1, n);
    println!("{}", a * x / b - a * (x / b));
}
