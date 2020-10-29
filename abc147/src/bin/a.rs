use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3],
    }

    println!(
        "{}",
        if a[0] + a[1] + a[2] >= 22 {
            "bust"
        } else {
            "win"
        }
    );
}
