use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    if k < a {
        println!("{} {}", a - k, b);
    } else if k < a + b {
        println!("{} {}", 0, a + b - k);
    } else {
        println!("{} {}", 0, 0);
    }
}
