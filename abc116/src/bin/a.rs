use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        ab: usize,
        bc: usize,
    }

    println!("{}", ab * bc / 2);
}
