use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let month: usize = s[5..7].parse().unwrap();

    if month >= 5 {
        println!("TBD");
    } else {
        println!("Heisei");
    }
}
