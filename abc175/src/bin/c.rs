use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: isize,
        mut k: isize,
        d: isize,
    }

    x = isize::abs(x);
    let num = std::cmp::min(k, x / d);
    k -= num;
    x -= d * num;

    println!("{}", if k % 2 == 0 { x } else { d - x });
}
