use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, i64); m],
    }

    let mut num = vec![-1i64; n];
    for i in 0..m {
        if num[sc[i].0 - 1] != -1 && num[sc[i].0 - 1] != sc[i].1 {
            println!("-1");
            return;
        }
        num[sc[i].0 - 1] = sc[i].1;
    }

    if n >= 2 && num[0] == 0 {
        println!("-1");
        return;
    }

    let mut ans = 0;
    for i in 0..n {
        ans *= 10;
        ans += if n >= 2 && i == 0 && num[i] == -1 {
            1
        } else if num[i] == -1 {
            0
        } else {
            num[i]
        };
    }

    println!("{}", ans);
}
