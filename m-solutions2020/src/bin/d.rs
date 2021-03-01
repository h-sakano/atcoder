use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut price = 1000;
    for i in 0..n - 1 {
        if a[i] < a[i + 1] {
            price = a[i + 1] * (price / a[i]) + price % a[i];
        }
    }

    println!("{}", price);
}
