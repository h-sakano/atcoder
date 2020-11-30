use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();

    let upper_bound = |s, x| {
        let (mut ok, mut ng) = (s, n);
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if l[mid] < x {
                ok = mid
            } else {
                ng = mid;
            }
        }
        ok - s
    };

    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            ans += upper_bound(j, l[i] + l[j]);
        }
    }

    println!("{}", ans);
}
