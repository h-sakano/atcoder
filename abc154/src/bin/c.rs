use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut history = HashSet::new();

    for i in 0..n {
        if history.contains(&a[i]) {
            println!("NO");
            return;
        }
        history.insert(a[i]);
    }
    println!("YES");
}
