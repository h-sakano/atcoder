use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }

    c.sort();
    c.reverse();

    let mut edges = VecDeque::new();
    edges.push_back(0usize);

    let mut ans = 0;
    for i in 0..n {
        ans += match edges.pop_front() {
            Some(t) => t,
            None => 0,
        };
        if i == 0 {
            edges.push_back(c[i])
        } else {
            edges.push_back(c[i]);
            edges.push_back(c[i]);
        }
    }

    println!("{}", ans);
}
