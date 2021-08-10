use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    a.sort();
    b.sort();

    let mut ans = 0;
    for (i, j) in a.into_iter().zip(b.into_iter()) {
        ans += (j - i).abs();
    }

    println!("{}", ans);
}