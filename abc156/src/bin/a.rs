use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
    }

    let mut ans = r;
    if n < 10 {
        ans = r + 100 * (10 - n);
    }

    println!("{}", ans);
}
