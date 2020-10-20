use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
    }

    let mut ans = 0;
    let mut strength = x;
    loop {
        if strength * a >= y {
            break;
        }
        if strength * (a - 1) > b {
            break;
        }
        strength *= a;
        ans += 1;
    }

    if y > strength {
        ans += (y - strength - 1) / b;
    }

    println!("{}", ans);
}
