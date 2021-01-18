use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: i64,
        x: i64,
    }

    let mut ans = vec![];
    for i in std::cmp::max(-1000000, x - k + 1)..=std::cmp::min(1000000, x + k - 1) {
        ans.push(i);
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
