use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }

    let mut ans = std::isize::MAX;
    for p in 0..100isize {
        let mut sum = 0;
        for i in 0..n {
            sum += (x[i] - p).pow(2);
        }
        ans = std::cmp::min(ans, sum);
    }

    println!("{}", ans);
}
