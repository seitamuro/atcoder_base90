use proconio::input;

fn main() {
    input! {
        mut n:i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut coins = vec![a, b, c];
    coins.sort_by(|x, y| y.partial_cmp(&x).unwrap());

    //println!("{:?}", coins);

    let mut ans = 9999999;
    for i in 0..9900 {
        for j in 0..9900 {
            let re = n - coins[0]*i - coins[1]*j;
            if re < 0 {break;}
            let nc = re / coins[2];
            let d = re - coins[2]*nc;
            if d == 0 {ans = ans.min(nc + i + j);}
        }
    }

    println!("{}", ans);
}
