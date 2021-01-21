use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(usize, usize, i64); n],
    }

    let mut events = vec![];
    for i in 0..n {
        events.push((abc[i].0 - 1, abc[i].2));
        events.push((abc[i].1, -abc[i].2));
    }
    events.sort();

    let mut ans = 0;
    let mut fee = 0i64;
    let mut t = 0;
    for (x, y) in events {
        if x != t {
            ans += std::cmp::min(c, fee) * (x - t) as i64;
            t = x;
        }
        fee += y;
    }

    println!("{}", ans);
}
