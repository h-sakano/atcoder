use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    println!("{}", 8 - (x - 400) / 200);
}
