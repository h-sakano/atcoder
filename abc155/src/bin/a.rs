use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if (a == b && b == c) || (a != b && b != c && c != a) {
        println!("No");
    } else {
        println!("Yes");
    }
}
