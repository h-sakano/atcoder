use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let price_a_min = (a as f64 / 0.08).ceil();
    let price_a_max = ((a + 1) as f64 / 0.08).ceil();
    let price_b_min = (b as f64 / 0.1).ceil();
    let price_b_max = ((b + 1) as f64 / 0.1).ceil();

    if price_b_min <= price_a_min && price_a_min < price_b_max {
        println!("{}", price_a_min);
    } else if price_a_min <= price_b_min && price_b_min < price_a_max {
        println!("{}", price_b_min);
    } else {
        println!("-1");
    }
}
