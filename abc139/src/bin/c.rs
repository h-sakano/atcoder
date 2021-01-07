use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = 0;
    let mut count = 0;
    for i in 1..n {
        if h[i] <= h[i - 1] {
            count += 1;
        } else {
            count = 0;
        }
        ans = std::cmp::max(ans, count);
    }

    println!("{}", ans);
}
