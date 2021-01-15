use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        h: usize,
        w: usize,
    }

    println!("{}", (H - h) * (W - w));
}
