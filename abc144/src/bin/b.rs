use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
