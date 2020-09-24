use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: usize);

    let count_500 = x / 500;
    let remain = x % 500;
    let count_5 = remain / 5;

    println!("{}", 1000 * count_500 + 5 * count_5);
}
