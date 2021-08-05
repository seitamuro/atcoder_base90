use proconio::input;
//use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }

    let mut dp = vec![vec![0; 8]; s.len()+1];
    dp[0][0] = 1;
    for (i, c) in s.chars().enumerate() {
        for j in 0..=7 {
            dp[i+1][j] += dp[i][j];
            if c == 'a' && j == 0 { dp[i+1][j+1] += dp[i][j]; }
            if c == 't' && j == 1 { dp[i+1][j+1] += dp[i][j]; }
            if c == 'c' && j == 2 { dp[i+1][j+1] += dp[i][j]; }
            if c == 'o' && j == 3 { dp[i+1][j+1] += dp[i][j]; }
            if c == 'd' && j == 4 { dp[i+1][j+1] += dp[i][j]; }
            if c == 'e' && j == 5 { dp[i+1][j+1] += dp[i][j]; }
            if c == 'r' && j == 6 { dp[i+1][j+1] += dp[i][j]; }
        }
    }

    for i in dp.iter() {
        println!("{:?}", i);
    }

    println!("{}", dp[s.len()][7]);
}
