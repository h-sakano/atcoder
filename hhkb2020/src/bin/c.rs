use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut candidates = vec![];
    for i in 0..=200_000 {
        candidates.push(i);
    }
    for i in 0..n {
        let index = match candidates.binary_search(&p[i]) {
            Ok(x) => x as i64,
            Err(_) => -1,
        };

        if index >= 0 {
            candidates.remove(index as usize);
        }

        println!("{}", candidates[0]);
    }
}
