// いもす法

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        stp: [(usize, usize, i64); n],
    }

    let time = 2 * 10usize.pow(5);
    let mut a = vec![0i64; time + 1];
    for i in 0..n {
        a[stp[i].0] += stp[i].2;
        a[stp[i].1] -= stp[i].2;
    }

    for i in 0..=time {
        if i + 1 <= time {
            a[i + 1] += a[i];
        }
        if a[i] > w as i64 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
