use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![0; 100_000];
    let n_sqrt = (n as f64).sqrt().ceil() as usize;

    for x in 1..=n_sqrt {
        for y in 1..=n_sqrt {
            for z in 1..=n_sqrt {
                ans[x * x + y * y + z * z + x * y + y * z + z * x] += 1;
            }
        }
    }

    for i in 0..n {
        println!("{}", ans[i + 1]);
    }
}
