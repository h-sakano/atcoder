use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(f64, f64); n],
    }

    let mut length = 0.0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            length += ((edges[i].0 - edges[j].0).powf(2f64) + (edges[i].1 - edges[j].1).powf(2f64))
                .sqrt();
        }
    }
    println!("{}", 2f64 * length / n as f64);
}
