use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    }

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let (x3, y3) = points[k];

                if (x1 - x3) * (y2 - y3) == (y1 - y3) * (x2 - x3) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
