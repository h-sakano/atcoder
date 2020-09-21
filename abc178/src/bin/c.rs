use proconio::{fastout, input};
const MOD: u64 = 1_000_000_000 + 7;

pub fn mpow(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut result = 1;

    while e > 0 {
        if e & 1 == 1 {
            result = result * b % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }

    result
}

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    println!(
        "{}",
        (mpow(10, n, MOD) + 2 * (MOD - mpow(9, n, MOD)) + mpow(8, n, MOD)) % MOD
    );
}
