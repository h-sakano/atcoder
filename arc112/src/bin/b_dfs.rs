use proconio::{fastout, input};
use std::collections::HashSet;

fn dfs(set: &mut std::collections::HashSet<i128>, b: i128, c: i128) {
    if set.contains(&b) || c < 0 {
        return;
    }
    set.insert(b);
    dfs(set, -b, c - 1);
    dfs(set, b - 1, c - 2);
}

#[fastout]
fn main() {
    input! {
        b: i128,
        c: i128,
    }

    let mut set = HashSet::<i128>::new();
    dfs(&mut set, b, c);

    println!("{}", set.len());
}
