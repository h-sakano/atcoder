use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut sa = 0;
    for _ in 0..3 {
        sa += a % 10;
        a /= 10;
    }
    let mut sb = 0;
    for _ in 0..3 {
        sb += b % 10;
        b /= 10;
    }

    println!("{}", std::cmp::max(sa, sb));
}
