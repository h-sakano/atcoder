use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut valid = true;

        let log10 = (n as f64).log(10.0).floor() as usize;
        let mut n10 = i;
        for _ in 0..=log10 {
            if n10 % 10 == 7 {
                valid = false;
                break;
            }
            n10 /= 10;
        }

        let log8 = (n as f64).log(8.0).floor() as usize;
        let mut n8 = i;
        for _ in 0..=log8 {
            if n8 % 8 == 7 {
                valid = false;
                break;
            }
            n8 /= 8;
        }

        if valid {
            ans += 1;
        }
    }

    println!("{}", ans);
}
