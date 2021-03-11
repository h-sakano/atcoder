use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        apx: [(i64, i64, i64); n],
    }

    let mut ans = -1;

    for i in 0..n {
        if apx[i].0 < apx[i].2 {
            if ans < 0 {
                ans = apx[i].1;
            } else {
                ans = std::cmp::min(apx[i].1, ans);
            }
        }
    }

    println!("{}", ans);
}
