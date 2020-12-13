use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; q],
    }

    let mut answers = vec![0; n];

    for i in 0..q {
        answers[a[i]] += 1;
    }

    for i in 0..n {
        if (k as i64) - (q as i64) + (answers[i] as i64) > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
