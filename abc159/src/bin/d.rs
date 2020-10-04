use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dict = HashMap::new();
    for i in 0..n {
        let count = dict.entry(a[i]).or_insert(0i64);
        *count += 1;
    }

    let mut counts = 0;
    for (_, &value) in &dict {
        counts += (value) * (value - 1) / 2;
    }

    for k in 0..n {
        let count = dict.entry(a[k]).or_insert(0i64);
        println!("{}", counts - *count + 1);
    }
}
