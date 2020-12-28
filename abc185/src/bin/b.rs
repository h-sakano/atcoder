use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        m: usize,
        t: usize,
        ab: [(usize, usize); m],
    }

    let mut battery = n;
    let mut prev_cafe_out = 0;
    for i in 0..m {
        battery -= (ab[i].0 - prev_cafe_out) as i64;
        if battery <= 0 {
            println!("No");
            return;
        }
        battery = std::cmp::min(n, battery + (ab[i].1 - ab[i].0) as i64);
        prev_cafe_out = ab[i].1;
    }
    battery -= (t - prev_cafe_out) as i64;
    if battery <= 0 {
        println!("No");
        return;
    }
    println!("Yes");
}
