use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    for i in k..n {
        if a[i] > a[i - k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
