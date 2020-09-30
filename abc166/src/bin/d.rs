use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: isize);

    for a in -118..=119isize {
        for b in -119..=118isize {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
