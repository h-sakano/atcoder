use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            let diff = a[i] - a[i + 1];
            ans += diff;
            a[i + 1] += diff;
        }
    }
    println!("{}", ans)
}
