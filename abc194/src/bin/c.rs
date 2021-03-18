use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dict = vec![0; 401];
    for i in 0..n {
        dict[(a[i] + 200) as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..=399 {
        let mut tmp = 0;
        for j in i + 1..=400 {
            tmp += (j - i) * (j - i) * dict[j];
        }
        ans += tmp * dict[i];
    }

    println!("{}", ans);
}
