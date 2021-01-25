use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        c: Chars,
    }

    if c[0] == c[1] && c[1] == c[2] {
        println!("Won");
    } else {
        println!("Lost");
    }
}
