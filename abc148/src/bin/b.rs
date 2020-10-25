use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let mut ans: String = "".to_string();
    for i in 0..n {
        ans = format!("{}{}{}", ans, s[i], t[i]);
    }

    println!("{}", ans);
}
