use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        mut b: usize,
        mut c: usize,
        mut k: usize,
    }

    while k > 0 && b <= a {
        k -= 1;
        b *= 2;
    }

    while k > 0 && c <= b {
        k -= 1;
        c *= 2;
    }

    println!("{}", if a < b && b < c { "Yes" } else { "No" });
}
