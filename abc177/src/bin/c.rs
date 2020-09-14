use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    let mut sum: u64 = a.iter().sum();

    for i in a {
        sum -= i;
        ans += (sum % MOD) * (i % MOD) % MOD;
        ans %= MOD;
    }

    println!("{}", ans)
}
