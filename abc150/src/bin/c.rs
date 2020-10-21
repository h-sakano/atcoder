use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut a = 1;
    let mut b = 1;
    for (i, v) in (1..=n as usize).permutations(n).enumerate() {
        if v == p {
            a = i;
        }
        if v == q {
            b = i;
        }
    }

    println!("{}", if a >= b { a - b } else { b - a });
}
