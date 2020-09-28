use num::integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(k: usize);

    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd(a, gcd(b, c));
            }
        }
    }

    println!("{}", ans);
}
