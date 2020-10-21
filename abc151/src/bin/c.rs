use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        submits: [(Usize1, String); m],
    }

    let mut wa = vec![0; n];
    let mut ac = vec![false; n];
    let mut count = 0;
    let mut penalty = 0;
    for s in submits {
        if !ac[s.0] && s.1 == "AC" {
            count += 1;
            ac[s.0] = true;
            penalty += wa[s.0];
        } else {
            wa[s.0] += 1;
        }
    }

    println!("{} {}", count, penalty);
}
