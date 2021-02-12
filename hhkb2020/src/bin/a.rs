use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: char,
        t: char,
    }

    if s == 'Y' {
        println!("{}", t.to_uppercase());
    } else {
        println!("{}", t);
    }
}
