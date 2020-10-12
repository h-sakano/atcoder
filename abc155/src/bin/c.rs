use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut dict = HashMap::new();
    let mut max = 0;
    for word in s {
        let count = dict.entry(word).or_insert(0);
        *count += 1;
        max = std::cmp::max(max, *count);
    }

    let mut max_list = vec![];
    for (k, v) in &dict {
        if *v == max {
            max_list.push(k);
        }
    }

    max_list.sort();
    for v in max_list {
        println!("{}", v);
    }
}
