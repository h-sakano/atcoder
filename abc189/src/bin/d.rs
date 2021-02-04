use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = 1;
    for i in 0..n {
        if s[i] == "OR" {
            ans += 2usize.pow((i + 1) as u32);
        }
    }

    println!("{}", ans);
}
