use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for j in 0..h {
        for i in 0..w - 1 {
            if s[j][i] == '.' && s[j][i + 1] == '.' {
                ans += 1;
            }
        }
    }

    for i in 0..w {
        for j in 0..h - 1 {
            if s[j][i] == '.' && s[j + 1][i] == '.' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
