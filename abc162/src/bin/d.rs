use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut r: usize = 0;
    let mut g: usize = 0;
    let mut b: usize = 0;

    for i in 0..n {
        match s[i] {
            'R' => r += 1,
            'G' => g += 1,
            'B' => b += 1,
            _ => unreachable!(),
        }
    }

    let mut ans = r * g * b;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let k = 2 * j - i;
            if k < n {
                if s[i] != s[j] && s[j] != s[k] && s[k] != s[i] {
                    ans -= 1;
                }
            }
        }
    }

    println!("{}", ans);
}
