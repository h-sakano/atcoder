use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    println!("{}", 100 - x % 100);
}
