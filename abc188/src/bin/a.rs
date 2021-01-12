use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }

    println!(
        "{}",
        if std::cmp::max(x, y) - std::cmp::min(x, y) <= 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
