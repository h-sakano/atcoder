use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();
    let sum: usize = l[0..n - 1].iter().sum();
    if l[n - 1] < sum {
        println!("Yes");
    } else {
        println!("No");
    }
}
