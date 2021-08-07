use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }

    dcs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //dbg!(dcs.clone());

    let mut dp = vec![vec![0; 5001];n+1];
    for i in 1..=n { 
        let (d, c, s) = dcs[i-1];
        //println!("{} {} {}", d, c, s);
        for j in 0..5001 {
            if d >= j && j >= c  {
                dp[i][j] = dp[i-1][j].max(dp[i-1][j.saturating_sub(c)] + s);
            } else if j >= d {
                dp[i][j] = dp[i][j-1].max(dp[i][j-1]);
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
        //println!("");
    }

    //dbg!(dp);
    println!("{}", dp[n][5000]);
}
