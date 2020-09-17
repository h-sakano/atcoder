use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: [(isize, isize); n],
    }

    let mut ans = 0;
    for (x, y) in p {
        if x.pow(2) + y.pow(2) <= d.pow(2) as isize {
            ans += 1;
        }
    }
    println!("{}", ans);
}
