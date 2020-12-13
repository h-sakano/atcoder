use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if (i + 1) % 2 == 0 {
            if s[i] == 'R' {
                println!("No");
                return;
            }
        } else {
            if s[i] == 'L' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
