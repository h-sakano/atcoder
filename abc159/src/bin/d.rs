use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dict = vec![0i64; n + 1];
    for i in 0..n {
        dict[a[i]] += 1;
    }

    let mut sum = 0;
    for i in 1..=n {
        sum += (dict[i]) * (dict[i] - 1) / 2;
    }

    for k in 0..n {
        println!("{}", sum - (dict[a[k]] - 1));
    }
}
