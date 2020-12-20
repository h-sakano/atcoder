use proconio::{fastout, input, marker::Chars};

fn char_to_uint(c: char) -> usize {
    return c as usize - '0' as usize;
}

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let length = n.len();

    let mut count = vec![0; 3];
    for i in 0..length {
        count[char_to_uint(n[i]) % 3] += 1;
    }

    let remain = (count[1] + 2 * count[2]) % 3;
    if remain == 0 {
        println!("0");
    } else if remain == 1 {
        if count[1] > 0 {
            if length >= 2 {
                println!("1");
            } else {
                println!("-1");
            }
        } else if length >= 3 {
            println!("2");
        } else {
            println!("-1");
        }
    } else {
        if count[2] > 0 {
            if length >= 2 {
                println!("1");
            } else {
                println!("-1");
            }
        } else if length >= 3 {
            println!("2");
        } else {
            println!("-1");
        }
    }
}
