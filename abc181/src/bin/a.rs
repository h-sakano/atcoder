use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 0 {
        println!("White")
    } else {
        println!("Black")
    }
}
