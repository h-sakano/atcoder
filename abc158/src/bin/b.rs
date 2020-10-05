use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut count = a * (n / (a + b));
    count += std::cmp::min(n % (a + b), a);

    println!("{}", count);
}
