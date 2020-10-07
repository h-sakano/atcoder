use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }

    let mut board = vec![vec![false; 3]; 3];

    for i in 0..n {
        for y in 0..3 {
            for x in 0..3 {
                if a[y][x] == b[i] {
                    board[y][x] = true;
                }
            }
        }
    }

    // 横
    for y in 0..3 {
        if board[y][0] && board[y][1] && board[y][2] {
            println!("Yes");
            return;
        }
    }

    // 縦
    for x in 0..3 {
        if board[0][x] && board[1][x] && board[2][x] {
            println!("Yes");
            return;
        }
    }

    // 斜め
    if board[0][0] && board[1][1] && board[2][2] {
        println!("Yes");
        return;
    }
    if board[0][2] && board[1][1] && board[2][0] {
        println!("Yes");
        return;
    }

    println!("No");
}
