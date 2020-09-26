use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    }

    let mut ans = vec![0; n];
    for ai in a {
        ans[ai] += 1;
    }

    for i in ans {
        println!("{}", i);
    }
}
