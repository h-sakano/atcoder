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
        mut x: usize,
    }

    let ans;
    loop {
        let divisors = make_divisors(x);
        if divisors.len() <= 2 {
            ans = x;
            break;
        }
        x += 1;
    }
    println!("{}", ans);
}
