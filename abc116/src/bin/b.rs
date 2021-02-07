use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: usize,
    }

    let mut memory = vec![false; 1_000_001];
    let mut m = 1;
    let mut t = s;
    memory[t] = true;
    loop {
        if t % 2 == 0 {
            t /= 2;
        } else {
            t = 3 * t + 1;
        }
        m += 1;
        if memory[t] {
            break;
        }
        memory[t] = true;
    }

    println!("{}", m);
}
