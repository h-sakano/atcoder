use num::integer::gcd;
use proconio::{fastout, input};

fn make_divisors(n: usize) -> Vec<usize> {
    let mut lower_divisors = vec![];
    let mut upper_divisors = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            lower_divisors.push(i);
            if i != n / i {
                upper_divisors.push(n / i);
            }
        }
        i += 1;
    }
    lower_divisors.append(&mut upper_divisors);
    return lower_divisors;
}

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let da = make_divisors(a);
    let mut cd = vec![];
    for d in da {
        if b % d == 0 {
            cd.push(d);
        }
    }

    cd.sort();
    let n = cd.len();
    let mut ans = 0;
    for i in (0..n).rev() {
        let mut ok = true;
        for j in 0..i {
            if gcd(cd[i], cd[j]) != 1 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
