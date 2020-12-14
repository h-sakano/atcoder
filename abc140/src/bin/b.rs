use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [usize; n],
        c: [usize; n - 1],
    }

    let mut ans = 0;
    for i in 0..n {
        ans += b[a[i]];
        if i >= 1 && a[i] >= 1 && a[i] <= n - 1 && a[i] == a[i - 1] + 1 {
            ans += c[a[i] - 1];
        }
    }

    println!("{}", ans);
}
