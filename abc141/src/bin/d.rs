use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut bh = BinaryHeap::new();
    for i in 0..n {
        bh.push(a[i]);
    }

    for _ in 0..m {
        let target = bh.pop().unwrap();
        bh.push(target / 2);
    }

    println!("{}", bh.iter().sum::<usize>());
}
