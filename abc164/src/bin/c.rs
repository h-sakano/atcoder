use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut words = HashMap::new();

    for si in s {
        words.insert(si, 1);
    }

    println!("{}", words.len());
}
