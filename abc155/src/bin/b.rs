use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = "APPROVED";

    for i in 0..n {
        if a[i] % 2 == 0 && a[i] % 3 != 0 && a[i] % 5 != 0 {
            ans = "DENIED";
            break;
        }
    }

    println!("{}", ans);
}
