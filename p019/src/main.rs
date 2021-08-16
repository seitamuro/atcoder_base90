use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 2*n],
    }
    
    let mut dp = vec![vec![9999999999; 2*n];2*n];
    for i in 0..2*n {
        dp[i][i] = 0;

        if i+1 < 2*n {
            dp[i][i+1] = (a[i] - a[i+1]).abs();
        }
    }

    for i in (1..2*n).step_by(2) {
        for j in 0..2*n-i {
            let l = j;
            let r = j+i;
            for k in j+1..j+i {
                dp[l][r] = dp[l][r].min(dp[l][k] + dp[k+1][r]);
            }
            dp[l][r] = dp[l][r].min(dp[l+1][r-1] + (a[l] - a[r]).abs());
        }
    }

    println!("{}", dp[0][2*n-1]);
}
