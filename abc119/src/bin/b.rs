use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n],
    }

    let mut ans = 0.0;
    for i in 0..n {
        ans += if xu[i].1 == "BTC" {
            xu[i].0 * 380000.0
        } else {
            xu[i].0
        };
    }

    println!("{}", ans);
}
