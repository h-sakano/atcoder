use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        m: isize,
    }

    println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2)
}
