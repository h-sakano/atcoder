use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n],
    }

    let mut d = vec![0usize; n];
    for i in 1..n {
        d[i] = a[i] - a[i - 1];
    }
    d[0] = k - a[n - 1] + a[0];
    println!("{}", k - d.iter().max().copied().unwrap());
}
