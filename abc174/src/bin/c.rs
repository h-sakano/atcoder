use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let mut a = vec![0; k];
    a[0] = 7 % k;
    for i in 1..k {
        a[i] = (a[i - 1] * 10 + 7) % k;
    }

    for i in 0..k {
        if a[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", -1);
}
