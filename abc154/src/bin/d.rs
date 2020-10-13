use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [f64; n],
    }
    let mut exp = 0f64;
    for i in 0..k {
        exp += (p[i] + 1f64) / 2f64;
    }

    let mut max_exp = exp;
    for i in 0..n - k {
        exp = exp - (p[i] + 1f64) / 2f64 + (p[i + k] + 1f64) / 2f64;
        max_exp = if exp > max_exp { exp } else { max_exp };
    }

    println!("{}", max_exp);
}
