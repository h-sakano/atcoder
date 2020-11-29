use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", std::cmp::max(0, a - b * 2));
}
