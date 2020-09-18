use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut ans = 0;

    let mut l = 0;
    let mut r = n - 1;
    while r as i64 - l as i64 > 0 {
        if c[l] == 'R' {
            l += 1;
        } else if c[r] == 'W' {
            r -= 1;
        } else if c[l] == 'W' && c[r] == 'R' {
            ans += 1;
            l += 1;
            r -= 1;
        }
    }

    println!("{}", ans)
}
