use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: u128,
        w: u128,
    }

    if h == 1 || w == 1 {
        println!("1")
    } else {
        println!("{}", (h * w + 1) / 2)
    }
}
