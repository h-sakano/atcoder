use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort();
    let mut ans = std::usize::MAX;
    for i in 0..n - k + 1 {
        ans = std::cmp::min(ans, h[i + k - 1] - h[i]);
    }

    println!("{}", ans);
}
