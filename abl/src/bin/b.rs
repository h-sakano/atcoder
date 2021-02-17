use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    println!(
        "{}",
        if std::cmp::min(b, d) - std::cmp::max(a, c) >= 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
