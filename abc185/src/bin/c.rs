use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u128,
    }

    let mut numerator: u128 = 1;
    let mut denominator: u128 = 1;
    for i in 1..=11 {
        numerator *= l - i as u128;
        denominator *= i as u128;
    }

    println!("{}", numerator / denominator)
}
