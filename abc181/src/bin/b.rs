use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;

    for i in 0..n {
        let a = ab[i].0;
        let b = ab[i].1;
        ans += (a + b) * (b - a + 1) / 2;
    }

    println!("{}", ans);
}
