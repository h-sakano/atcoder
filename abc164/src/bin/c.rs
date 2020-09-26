use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let s: HashSet<String> = s.into_iter().collect();

    println!("{}", s.len());
}
