use proconio::{fastout, input, marker::Chars};

fn char_to_u32(c: char) -> u32 {
    return c as u32 - '0' as u32;
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut numbers = vec![0i64; 10];
    for i in 0..s.len() {
        numbers[char_to_u32(s[i]) as usize] += 1;
    }

    if s.len() == 1 {
        let num = char_to_u32(s[0]);
        if num % 8 == 0 {
            println!("Yes");
            return;
        }
    } else if s.len() == 2 {
        let num = char_to_u32(s[1]) * 10 + char_to_u32(s[0]);
        let num2 = char_to_u32(s[0]) * 10 + char_to_u32(s[1]);

        if num % 8 == 0 || num2 % 8 == 0 {
            println!("Yes");
            return;
        }
    } else {
        for i in (0..999).step_by(8) {
            let a = i / 100;
            let b = (i % 100) / 10;
            let c = i % 10;

            let mut usage = vec![0i64; 10];
            usage[a] += 1;
            usage[b] += 1;
            usage[c] += 1;

            let yes = usage.into_iter().enumerate().all(|(i, n)| numbers[i] >= n);

            if yes {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
