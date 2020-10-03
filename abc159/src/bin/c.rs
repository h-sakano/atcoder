use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: f64,
    }

    println!("{}", (l / 3f64).powf(3f64));
}
