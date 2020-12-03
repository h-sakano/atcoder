use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let odd = (n + 1) / 2;
    let even = n / 2;

    println!("{}", odd as f64 / (odd as f64 + even as f64));
}
