use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for x in 1..=10usize.pow(5) {
        let mut now = 0;
        let mut max = 0;

        for i in 0..n {
            if x <= a[i] {
                now += x;
            } else {
                now = 0;
            }
            max = std::cmp::max(max, now);
        }

        ans = std::cmp::max(ans, max);
    }

    println!("{}", ans);
}
