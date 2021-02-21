use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for i in (0..n).step_by(2) {
        if a[i] % 2 == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
