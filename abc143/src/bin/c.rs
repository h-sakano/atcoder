use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 1;
    for i in 0..n - 1 {
        if s[i] != s[i + 1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
