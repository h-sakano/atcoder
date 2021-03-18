use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abs: [(usize, usize); n],
    }

    let mut ans = 1_000_000;
    for i in 0..n {
        let a = abs[i].0;
        for j in 0..n {
            let b = abs[j].1;
            let mut time = std::cmp::max(a, b);
            if i == j {
                time = a + b;
            }
            ans = std::cmp::min(ans, time);
        }
    }

    println!("{}", ans);
}
