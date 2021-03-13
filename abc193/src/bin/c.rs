use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut set = HashSet::new();
    for i in 2..=((n as f64).sqrt().floor() as usize) {
        let mut now = i * i;
        while now <= n {
            set.insert(now);
            now *= i;
        }
    }

    println!("{}", n - set.len());
}
