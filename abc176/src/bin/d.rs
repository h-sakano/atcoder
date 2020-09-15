// 幅優先探索, 01-BFS

use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        mut s: [Chars; h],
    }

    let mut q = VecDeque::new();
    q.push_back((ch, cw));

    let mut costs = vec![vec![-1; w]; h];
    costs[ch][cw] = 0;

    while !q.is_empty() {
        if let Some((y, x)) = q.pop_front() {
            s[y][x] = '#';
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if i64::abs(dx) == i64::abs(dy) {
                        continue;
                    }
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if nx < 0 || nx >= w as i64 || ny < 0 || ny >= h as i64 {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[ny][nx] != '#' && (costs[ny][nx] == -1 || costs[ny][nx] > costs[y][x]) {
                        costs[ny][nx] = costs[y][x];
                        q.push_front((ny, nx));
                    }
                }
            }
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let nx = x as i64 + dx;
                    let ny = y as i64 + dy;
                    if nx < 0 || nx >= w as i64 || ny < 0 || ny >= h as i64 {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[ny][nx] != '#' && (costs[ny][nx] == -1 || costs[ny][nx] > costs[y][x] + 1)
                    {
                        costs[ny][nx] = costs[y][x] + 1;
                        q.push_back((ny, nx));
                    }
                }
            }
        }
    }
    println!("{}", costs[dh][dw]);
}
