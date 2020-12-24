use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut totals = vec![];
    let mut count: i64 = 0;
    let mut max_cursor: i64 = 0;
    for i in 0..n {
        count += a[i];
        if count > max_cursor {
            max_cursor = count;
        }
        totals.push((count, max_cursor));
    }

    let mut cursor = 0;
    let mut max_cursor = 0;
    for i in 0..n {
        max_cursor = std::cmp::max(cursor + totals[i].1, max_cursor);
        cursor += totals[i].0;
    }

    println!("{}", max_cursor);
}
