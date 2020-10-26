use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut k = 0;
    let mut now = 1;
    for i in 0..n {
        if a[i] == now {
            now += 1;
        } else {
            k += 1;
        }
    }

    if k == n {
        println!("-1");
    } else {
        println!("{}", k);
    }
}
