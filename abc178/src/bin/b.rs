use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    let mut ans = a * c;
    ans = std::cmp::max(ans, a * d);
    ans = std::cmp::max(ans, b * c);
    ans = std::cmp::max(ans, b * d);

    println!("{}", ans);
}
