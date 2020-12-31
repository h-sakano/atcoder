use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut minimum = std::usize::MAX;
    for i in 0..h {
        for j in 0..w {
            minimum = std::cmp::min(minimum, a[i][j]);
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += a[i][j] - minimum;
        }
    }

    println!("{}", ans);
}
