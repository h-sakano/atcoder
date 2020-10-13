use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
        mut a: usize,
        mut b: usize,
        u: String,
    }

    if u == s {
        a -= 1;
    } else if u == t {
        b -= 1;
    }

    println!("{} {}", a, b);
}
