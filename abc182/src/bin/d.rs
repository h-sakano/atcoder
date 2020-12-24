use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut totals = vec![];
    let mut count: i64 = 0;
    for i in 0..n {
        count += a[i];
        totals.push(count);
    }

    let mut max_cursor = -10i64.pow(9);
    let mut max_index = 0;
    let mut cursor = 0;

    for i in 1..=n {
        cursor += totals[i - 1];
        if cursor >= max_cursor {
            max_cursor = cursor;
            max_index = i;
        }
    }

    cursor = max_cursor;
    for i in 0..max_index {
        cursor += a[i];
        if cursor > max_cursor {
            max_cursor = cursor;
        }
    }

    println!("{}", max_cursor);
}
