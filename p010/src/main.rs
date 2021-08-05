use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut sum1 = vec![0; n + 2];
    let mut sum2 = vec![0; n + 2];

    for (i, &(c, p)) in cp.iter().enumerate() {
        let i = i;
        sum1[i + 1] += if c == 1 { sum1[i] + p } else { sum1[i] };
        sum2[i + 1] += if c == 2 { sum2[i] + p } else { sum2[i] };
    }

    let lr: Vec<(usize, usize)> = lr
        .iter()
        .map(|&(x, y)| (x.saturating_sub(1), y))
        .collect();
    for &(l, r) in lr.iter() {
        println!("{} {}", sum1[r] - sum1[l], sum2[r] - sum2[l]);
    }
}
