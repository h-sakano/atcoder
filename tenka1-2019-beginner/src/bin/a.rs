use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if std::cmp::min(a, b) < c && c < std::cmp::max(a, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
