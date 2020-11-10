use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut i = 1;
    let mut ans = std::usize::MAX;
    while i * i <= n {
        if n % i == 0 {
            ans = std::cmp::min(ans, i - 1 + n / i - 1);
        }
        i += 1;
    }

    println!("{}", ans);
}
