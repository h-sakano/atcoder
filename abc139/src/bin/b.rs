use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = 0;
    let mut count = 1;
    while count < b {
        count += a - 1;
        ans += 1;
    }

    println!("{}", ans);
}
