use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort_by_key(|&x| std::cmp::Reverse(x));
    let mut ans = 0;
    for i in k..n {
        ans += h[i];
    }

    println!("{}", ans);
}
