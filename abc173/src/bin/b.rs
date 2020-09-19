use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let c = ["AC", "WA", "TLE", "RE"];
    let mut ans = vec![0; 4];
    for i in 0..n {
        for j in 0..4 {
            if s[i] == c[j] {
                ans[j] += 1;
            }
        }
    }

    for i in 0..4 {
        println!("{} x {}", c[i], ans[i]);
    }
}
