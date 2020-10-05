use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(s: Chars);

    for i in 1..3 {
        if s[i] != s[0] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
