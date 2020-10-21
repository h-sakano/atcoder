use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        m: i64,
        a: [i64; n - 1],
    }

    let sum: i64 = a.iter().sum();
    let target = m * n as i64;
    let score = target - sum;

    if score > k {
        println!("{}", -1);
    } else {
        println!("{}", std::cmp::max(score, 0));
    }
}
