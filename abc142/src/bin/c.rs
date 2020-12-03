use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut ans = vec![0usize; n];
    for i in 0..n {
        ans[a[i]] = i + 1;
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
