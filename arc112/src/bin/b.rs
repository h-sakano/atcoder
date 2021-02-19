use proconio::{fastout, input};

fn exact(b: i64, c: i64) -> (i64, i64) {
    let n = c / 2;
    if c % 2 != 0 {
        return (-b - n, -b + n);
    } else {
        if c == 0 {
            return (b, b);
        } else {
            return (b - n, b + n - 1);
        }
    }
}

#[fastout]
fn main() {
    input! {
        mut b: i64,
        mut c: i64,
    }

    let (w, x) = exact(b, c);
    let (y, z) = exact(b, c - 1);

    println!(
        "{}",
        x - w + 1 + z - y + 1 - std::cmp::max(0, std::cmp::min(x, z) - std::cmp::max(w, y) + 1)
    );
}
