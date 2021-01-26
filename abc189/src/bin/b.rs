use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }

    let mut count = 0;
    for i in 0..n {
        count += vp[i].0 * vp[i].1;
        if count > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
