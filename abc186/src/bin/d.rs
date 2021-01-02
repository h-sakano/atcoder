use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();
    let mut ans = 0;
    for i in 1..n {
        ans += a[i] - a[0];
    }
    let mut prev = ans;
    for i in 1..n {
        let diff = a[i] - a[i - 1];
        prev = prev - diff * ((n - i) as i64);
        ans += prev;
    }

    println!("{}", ans);
}
