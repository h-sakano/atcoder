use num::integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(k: usize);

    let mut ab = vec![vec![0; k + 1]; k + 1];
    for i in 1..=k {
        for j in 1..=k {
            ab[i][j] = gcd(i, j);
        }
    }

    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += ab[ab[a][b]][c]Z;
            }
        }
    }

    println!("{}", ans);
}
