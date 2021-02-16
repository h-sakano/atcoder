use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let mut ans = "".to_string();
    for _ in 0..k {
        ans += "ACL";
    }

    println!("{}", ans);
}
