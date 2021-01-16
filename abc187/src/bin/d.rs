use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    // 高橋派と青木派(重み2)の数の合計の降順に並び替える
    ab.sort_by(|x, y| (y.0 * 2 + y.1).cmp(&(x.0 * 2 + x.1)));
    let mut aoki = 0;
    let mut takahashi = 0;

    for i in 0..n {
        aoki += ab[i].0;
    }

    let mut ans = 0;
    for i in 0..n {
        if takahashi <= aoki {
            ans += 1;
            aoki -= ab[i].0;
            takahashi += ab[i].0 + ab[i].1;
        }
    }

    println!("{}", ans);
}
