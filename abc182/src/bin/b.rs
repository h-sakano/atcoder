use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max_num = 0;
    let mut max_index = 0;
    for i in 2..1000 {
        let mut num = 0;
        for j in 0..n {
            if a[j] % i == 0 {
                num += 1;
            }
        }
        if num > max_num {
            max_num = num;
            max_index = i;
        }
    }
    println!("{}", max_index);
}
