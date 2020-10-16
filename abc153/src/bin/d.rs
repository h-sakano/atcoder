use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut h: usize,
    }

    let mut num = 1;
    let mut ans = 0i64;
    while h >= 1 {
        h /= 2;
        ans += num;
        num *= 2;
    }

    println!("{}", ans);
}
