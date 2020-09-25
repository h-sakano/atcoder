// DFS(深さ優先探索)

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut pairs: [(Usize1, Usize1, usize, usize); q],
    }

    let mut s = vec![1];
    let ans = dfs(&mut s, n, m, &pairs);

    println!("{}", ans);
}

fn dfs(s: &mut Vec<usize>, n: usize, m: usize, pairs: &Vec<(usize, usize, usize, usize)>) -> usize {
    if s.len() == n {
        let mut point = 0;
        for pair in pairs {
            if s[pair.1] - s[pair.0] == pair.2 {
                point += pair.3;
            }
        }
        return point;
    }
    let mut max = 0;
    for i in s[s.len() - 1]..=m {
        s.push(i);
        max = std::cmp::max(max, dfs(s, n, m, pairs));
        s.pop();
    }
    max
}
