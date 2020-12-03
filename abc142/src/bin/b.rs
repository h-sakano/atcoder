use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        if h[i] >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
