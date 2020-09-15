use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut sum = 0;
    for c in n {
        let num: usize = c as usize - '0' as usize;
        sum += num;
    }

    println!("{}", if sum % 9 == 0 { "Yes" } else { "No" })
}
