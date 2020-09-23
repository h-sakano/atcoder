use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [[usize; 2]; n],
    }

    let mut count = 0;
    let mut t = false;
    for i in 0..n {
        if d[i][0] == d[i][1] {
            count += 1;
            if count >= 3 {
                t = true;
                break;
            }
        } else {
            count = 0;
        }
    }

    println!("{}", if t { "Yes" } else { "No" });
}
