use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let max = p.iter().max().copied().unwrap();
    let total: usize = p.iter().sum();

    println!("{}", total - max / 2);
}
