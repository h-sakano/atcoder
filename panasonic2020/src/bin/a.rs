use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let sequence = vec![
        1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1, 4,
        1, 51,
    ];

    println!("{}", sequence[k - 1]);
}
