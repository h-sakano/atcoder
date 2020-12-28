use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
        s: Chars,
    }

    let mut point: i64 = x;
    for i in 0..n {
        if s[i] == 'o' {
            point += 1;
        } else {
            point = std::cmp::max(point - 1, 0);
        }
    }

    println!("{}", point);
}
