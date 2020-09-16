use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [i128; n],
    }

    let mut max_score = std::i128::MIN;
    for i in 0..n {
        let mut point = i;
        let mut score = 0;
        let mut scorelist = vec![];
        let mut visited = vec![-1i128; n];
        let mut loop_start = -1;
        let mut loop_end = -1;
        for count in 0..=std::cmp::min(n, k - 1) {
            point = p[point];
            score += c[point];
            scorelist.push(score);
            if visited[point] < 0 {
                visited[point] = count as i128;
            } else {
                loop_start = visited[point];
                loop_end = (count - 1) as i128;
                break;
            }
        }

        max_score = std::cmp::max(
            max_score,
            if loop_start >= 0 {
                let loop_start: usize = loop_start as usize;
                let loop_end: usize = loop_end as usize;
                let loop_length = loop_end - loop_start + 1;
                let loop_num = (k - loop_start) / loop_length;
                let before_loop_score = if loop_start > 0 {
                    scorelist[loop_start - 1]
                } else {
                    0
                };
                let loop_score = scorelist[loop_end] - before_loop_score;
                let loop_remain = (k - loop_start) % loop_length;

                let remain_score = match scorelist[loop_start..(loop_start + loop_remain)]
                    .iter()
                    .max()
                    .copied()
                {
                    Some(s) => s,
                    None => 0,
                };

                if loop_score > 0 {
                    before_loop_score
                        + loop_score * (loop_num as i128 - 1)
                        + std::cmp::max(
                            scorelist[loop_start..loop_end]
                                .iter()
                                .max()
                                .copied()
                                .unwrap(),
                            loop_score + remain_score,
                        )
                } else {
                    scorelist.iter().max().copied().unwrap()
                }
            } else {
                scorelist.iter().max().copied().unwrap()
            },
        );
    }

    println!("{}", max_score);
}
