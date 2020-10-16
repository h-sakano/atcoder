use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut ans = vec![];
    for _ in 0..std::cmp::max(a, b) {
        ans.push(std::char::from_digit(std::cmp::min(a, b), 10).unwrap());
    }

    println!("{}", ans.into_iter().collect::<String>());
}
