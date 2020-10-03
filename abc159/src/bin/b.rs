use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    for i in 0..n {
        if i >= n - 1 - i {
            break;
        }
        if s[i] != s[n - 1 - i] {
            println!("No");
            return;
        }
    }

    for i in 0..(n - 1) / 2 {
        if i >= (n - 1) / 2 - 1 - i {
            break;
        }
        if s[i] != s[(n - 1) / 2 - 1 - i] {
            println!("No");
            return;
        }
    }

    for i in 0..(n - 1) / 2 {
        if (n + 3) / 2 - 1 + i >= n - 1 - i {
            break;
        }
        if s[(n + 3) / 2 - 1 + i] != s[n - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
