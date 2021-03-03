use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        k: usize,
    }

    let mut ans = "".to_string();
    for i in 0..n {
        if s[i] == s[k - 1] {
            ans += &s[i].to_string();
        } else {
            ans += "*";
        }
    }

    println!("{}", ans);
}
