use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(x: isize);

    for a in -118..=119 {
        let a: isize = a;
        for b in -119..=118 {
            let b: isize = b;
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
