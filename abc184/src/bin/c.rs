use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }

    if r1 == r2 && c1 == c2 {
        println!("0");
    } else if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        println!("1");
    } else if (r1 + c1 + r2 + c2) % 2 == 0
        || (r1 - r2).abs() + (c1 - c2).abs() <= 6
        || ((r1 + c1) - (r2 + c2)).abs() <= 3
        || ((r1 - c1) - (r2 - c2)).abs() <= 3
    {
        println!("2");
    } else {
        println!("3");
    }
}
