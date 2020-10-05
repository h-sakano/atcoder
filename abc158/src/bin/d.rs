use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
    }

    let mut queue = VecDeque::from(s);
    let mut reverse = false;
    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            reverse = !reverse;
        } else {
            input! {
                f: usize,
                c: char,
            }

            if f == 1 {
                if reverse {
                    queue.push_back(c);
                } else {
                    queue.push_front(c);
                }
            } else {
                if reverse {
                    queue.push_front(c);
                } else {
                    queue.push_back(c);
                }
            }
        }
    }

    if reverse {
        println!("{}", queue.iter().rev().collect::<String>())
    } else {
        println!("{}", queue.iter().collect::<String>())
    }
}
