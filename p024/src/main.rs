use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let diff = a.into_iter().zip(b.into_iter()).fold(0, |acc, (a, b)| acc + (a - b).abs());
    if diff <= k && (diff - k) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
