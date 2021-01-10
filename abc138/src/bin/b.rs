use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }

    let mut ans = 0.0;
    for i in 0..n {
        ans += 1.0 / a[i];
    }
    ans = 1.0 / ans;
    println!("{}", ans);
}
