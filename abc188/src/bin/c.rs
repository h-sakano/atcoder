use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: u32,
        a: [usize; 2usize.pow(n)],
    }

    let mut queue = VecDeque::new();
    for i in 0..2usize.pow(n) {
        queue.push_back(i);
    }

    // 決勝まで進める
    for _ in 0..n - 1 {
        let mut new_queue = VecDeque::new();
        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            let j = queue.pop_front().unwrap();
            new_queue.push_back(if a[i] > a[j] { i } else { j });
        }
        queue = new_queue;
    }

    let i = queue.pop_front().unwrap();
    let j = queue.pop_front().unwrap();

    println!("{}", if a[i] < a[j] { i } else { j } + 1);
}
