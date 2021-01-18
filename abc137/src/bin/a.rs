use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    }

    println!("{}", std::cmp::max(std::cmp::max(a + b, a - b), a * b))
}
