use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut min = p[0];
    let mut ans = 1;
    for i in 1..n {
        if p[i] <= min {
            min = p[i];
            ans += 1;
        }
    }

    println!("{}", ans);
}
