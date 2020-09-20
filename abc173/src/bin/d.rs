use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }

    c.sort();

    let mut ans = 0;
    for i in 1..n {
        ans += c[n - 1 - i / 2]
    }

    println!("{}", ans);
}
