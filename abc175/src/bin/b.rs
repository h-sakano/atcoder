use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    if n < 3 {
        println!("{}", 0);
        return;
    }

    l.sort();

    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                if l[i] + l[j] > l[k] && l[i] != l[j] && l[j] != l[k] && l[k] != l[i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
