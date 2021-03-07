use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        abs: [(usize, usize); n],
    }

    let mut ans = 0;
    for i in 0..n {
        if h <= abs[i].0 && w <= abs[i].1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
