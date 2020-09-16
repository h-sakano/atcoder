use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut chain = 0;
    for c in s {
        if c == 'R' {
            chain += 1;
        } else {
            chain = 0;
        }
        ans = std::cmp::max(chain, ans);
    }

    println!("{}", ans);
}
