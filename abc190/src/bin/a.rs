use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if c == 0 {
        if a > b {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        if a >= b {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
