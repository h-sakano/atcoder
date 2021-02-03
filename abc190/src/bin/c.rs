use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    }

    let mut ans = 0;
    for i in 0..2usize.pow(k as u32) {
        let mut count = vec![0; n];
        for j in 0..k {
            count[if i & (1 << j) == 0 { cd[j].0 } else { cd[j].1 }] += 1;
        }
        let mut t = 0;
        for j in 0..m {
            if count[ab[j].0] > 0 && count[ab[j].1] > 0 {
                t += 1;
            }
        }
        ans = std::cmp::max(ans, t);
    }

    println!("{}", ans);
}
