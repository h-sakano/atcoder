use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [f64; n],
    }

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;
    for i in 0..n {
        a += x[i].abs();
        b += x[i].powf(2.0);
        c = c.max(x[i].abs());
    }
    b = b.sqrt();

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
