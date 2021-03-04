use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
    }

    println!("{}", (n - h + 1) * (n - w + 1));
}
