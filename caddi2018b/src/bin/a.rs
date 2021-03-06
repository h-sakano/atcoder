use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut ans = 0;
    for i in 0..4 {
        if n[i] == '2' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
