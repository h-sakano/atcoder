use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Usize1; m],
    }

    if m <= 0 {
        println!("1");
        return;
    } else if n == m {
        println!("0");
        return;
    }

    a.sort();
    let mut blank = vec![];
    for i in 0..m {
        let b = if i == 0 { a[i] } else { a[i] - a[i - 1] - 1 };
        if b > 0 {
            blank.push(b);
        }
        if i == m - 1 {
            let b = n - a[i] - 1;
            if b > 0 {
                blank.push(b);
            }
        }
    }

    let k = blank.iter().min().unwrap();
    let mut ans = 0;
    for i in 0..blank.len() {
        ans += (blank[i] - 1) / k + 1;
    }

    println!("{}", ans);
}
