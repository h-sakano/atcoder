use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut count = vec![0; m];
    for _ in 0..n {
        input! {
            k: usize,
            a: [Usize1; k],
        }

        for j in 0..k {
            count[a[j]] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..m {
        if count[i] >= n {
            ans += 1;
        }
    }

    println!("{}", ans);
}
