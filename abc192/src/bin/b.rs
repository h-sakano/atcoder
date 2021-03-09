use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i].is_uppercase() {
                println!("No");
                return;
            }
        } else {
            if s[i].is_lowercase() {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
