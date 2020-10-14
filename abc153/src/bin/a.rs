use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: i64,
        a: i64,
    }

    println!("{}", (h - 1) / a + 1);
}
