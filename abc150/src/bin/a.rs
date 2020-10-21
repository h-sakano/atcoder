use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: i64,
        x: i64,
    }

    if 500 * k - x >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
