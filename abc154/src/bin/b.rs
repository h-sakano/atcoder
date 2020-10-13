use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut ans = "".to_string();
    for _ in 0..s.len() {
        ans += "x";
    }

    println!("{}", ans);
}
