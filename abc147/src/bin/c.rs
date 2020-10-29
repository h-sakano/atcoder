use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut testimony = vec![];

    for _ in 0..n {
        input! {
            a: usize,
            xy: [(Usize1, usize); a],
        }
        testimony.push((a, xy))
    }

    let mut max_count = 0;
    for i in 0..2usize.pow(n as u32) {
        let mut contradiction = false;
        let mut count = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                count += 1;
                for k in 0..testimony[j].0 {
                    if i & 1 << testimony[j].1[k].0 != 0 {
                        if testimony[j].1[k].1 == 0 {
                            contradiction = true;
                        }
                    } else if i & 1 << testimony[j].1[k].0 == 0 {
                        if testimony[j].1[k].1 == 1 {
                            contradiction = true;
                        }
                    }
                }
            }
        }
        if contradiction {
            continue;
        }

        max_count = std::cmp::max(max_count, count);
    }

    println!("{}", max_count);
}
