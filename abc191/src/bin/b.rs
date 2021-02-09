use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.iter()
            .filter(|&&t| t != x)
            .map(|&t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
