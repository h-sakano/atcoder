use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();
    let mut ans = std::i64::MAX;

    for i in 0..n - m + 1 {
        let mut count = 0;
        for j in 0..m {
            if s[i + j] != t[j] {
                count += 1;
            }
        }
        if count < ans {
            ans = count;
        }
    }

    println!("{}", ans)
}
