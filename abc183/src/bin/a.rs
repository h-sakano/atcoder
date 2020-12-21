use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }

    println!("{}", std::cmp::max(x, 0));
}
