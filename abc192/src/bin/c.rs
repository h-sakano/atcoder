use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        k: usize,
    }

    let mut a = n;

    for _ in 0..k {
        let g1: i64 = a
            .to_string()
            .chars()
            .sorted()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        let g2: i64 = a
            .to_string()
            .chars()
            .sorted()
            .collect::<String>()
            .parse()
            .unwrap();
        a = g1 - g2;
    }

    println!("{}", a);
}
