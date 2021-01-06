use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = 0;
    for i in 0..3 {
        if s[i] == t[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
