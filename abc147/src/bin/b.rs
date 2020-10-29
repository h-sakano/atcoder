use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
