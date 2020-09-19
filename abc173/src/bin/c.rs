use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut ans = 0;
    for i in 0..2usize.pow((h + w) as u32) {
        let mut num = 0;
        for row in 0..h {
            if i >> row & 1 != 0 {
                continue;
            }
            for col in 0..w {
                if i >> (h + col) & 1 != 0 {
                    continue;
                }
                if c[row][col] == '#' {
                    num += 1;
                }
            }
        }

        if num == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
