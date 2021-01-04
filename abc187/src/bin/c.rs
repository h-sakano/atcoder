use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        words: [String; n],
    }

    let words: HashSet<String> = words.into_iter().collect();
    for word in &words {
        if words.contains(&format!("{}{}", "!".to_string(), word)) {
            println!("{}", word);
            return;
        }
    }

    println!("satisfiable");
}
