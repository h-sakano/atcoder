use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut ans = 0;
    let mut history = vec![false; 200_001];
    for i in 0..n {
        history[p[i]] = true;
        if ans == p[i] {
            for j in ans + 1..=200_000 {
                if !history[j] {
                    ans = j;
                    break;
                }
            }
        }
        println!("{}", ans);
    }
}
