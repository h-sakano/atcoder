use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: [usize; n],
    }

    let mut count = vec![0; 3];
    for i in 0..n {
        if p[i] <= a {
            count[0] += 1;
        } else if a < p[i] && p[i] <= b {
            count[1] += 1;
        } else {
            count[2] += 1;
        }
    }

    println!("{}", count.iter().min().unwrap())
}
