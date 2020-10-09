use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f64,
        k: f64,
    }
    println!("{}", (n.log(k).floor() + 1f64).floor());
}
