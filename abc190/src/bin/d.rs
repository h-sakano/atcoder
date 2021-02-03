use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }

    while n % 2 == 0 {
        n /= 2;
    }
    let mut ans = 0;
    let sq = (n as f64).sqrt() as usize;
    for i in 1..=sq {
        if n % i == 0 {
            ans += 2;
        }
    }

    if sq * sq == n {
        ans -= 1;
    }

    println!("{}", ans * 2);
}
