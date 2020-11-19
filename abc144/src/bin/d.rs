use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let s = x / a;
    let ans = if s == a * b {
        0f64
    } else if s > a * b / 2f64 {
        // 台形
        let bu = 2f64 * s / a - b;
        let s2 = s - a * bu;
        let b2 = b - bu;
        let a2 = 2f64 * s2 / b2;
        f64::atan((b - bu) / a2)
    } else {
        // 三角形
        f64::atan(b / (2f64 * s / b))
    } * (180f64 / std::f64::consts::PI);

    println!("{}", ans);
}
