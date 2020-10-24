use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    }

    let mut ans = 0;
    let mut history = vec![];
    for i in 0..n {
        if t[i] == 'r' {
            if history.len() >= k && history[history.len() - k] == 'p' {
                history.push('o');
            } else {
                history.push('p');
                ans += p;
            }
        } else if t[i] == 's' {
            if history.len() >= k && history[history.len() - k] == 'r' {
                history.push('o');
            } else {
                history.push('r');
                ans += r;
            }
        } else {
            if history.len() >= k && history[history.len() - k] == 's' {
                history.push('o');
            } else {
                history.push('s');
                ans += s;
            }
        }
    }

    println!("{}", ans);
}
