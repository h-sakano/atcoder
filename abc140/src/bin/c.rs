use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        b: [usize; n-1],
    }

    let mut ans = b[0] + b[n - 2];
    for i in 1..n - 1 {
        ans += std::cmp::min(b[i - 1], b[i]);
    }

    println!("{}", ans);
}
