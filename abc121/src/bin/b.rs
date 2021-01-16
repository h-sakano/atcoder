use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: i64,
        b: [i64; m],
        a: [[i64; m]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut t = 0;
        for j in 0..m {
            t += a[i][j] * b[j];
        }
        t += c;
        if t > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
