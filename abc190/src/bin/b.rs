use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n],
    }

    for i in 0..n {
        if xy[i].0 < s && xy[i].1 > d {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
