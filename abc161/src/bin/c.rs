use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize);

    let remain = n % k;
    let ans = std::cmp::min(remain, k - remain);
    println!("{}", ans);
}
