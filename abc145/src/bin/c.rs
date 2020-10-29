use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(f64, f64); n],
    }

    let mut fact = 1;
    for i in 2..=n {
        fact *= i;
    }
    let mut length = 0.0;
    for (_, v) in (1..=n as usize).permutations(n).enumerate() {
        for j in 1..n {
            length += ((edges[v[j] - 1].0 - edges[v[j - 1] - 1].0).powf(2f64)
                + (edges[v[j] - 1].1 - edges[v[j - 1] - 1].1).powf(2f64))
            .sqrt();
        }
    }
    println!("{}", length / fact as f64);
}
