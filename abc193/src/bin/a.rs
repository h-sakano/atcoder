use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }

    println!("{}", (1.0 - b / a) * 100.0);
}
