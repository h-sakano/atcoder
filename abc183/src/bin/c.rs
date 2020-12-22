// 順列の問題

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut ans = 0;
    for (_, v) in (1..=n - 1 as usize).permutations(n - 1).enumerate() {
        let mut dist = 0;
        let mut prev = 0;
        for i in 0..n - 1 {
            dist += t[prev][v[i]];
            prev = v[i];
        }
        dist += t[prev][0];

        if dist == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
