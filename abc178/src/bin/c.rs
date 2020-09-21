use proconio::{fastout, input};
const MOD: i128 = 1_000_000_000 + 7;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut pow10 = 1;
    let mut pow9 = 1;
    let mut pow8 = 1;
    for _ in 1..=n {
        pow10 = pow10 * 10 % MOD;
        pow9 = pow9 * 9 % MOD;
        pow8 = pow8 * 8 % MOD;
    }

    println!("{}", (pow10 + 2 * (MOD - pow9) + pow8) % MOD);
}
