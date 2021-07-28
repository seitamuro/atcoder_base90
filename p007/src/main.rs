use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        q: usize,
        b: [i32; q],
    }

    for &i in b.iter() {
        let mut min = std::i32::MAX;
        for &j in a.iter() {
            min = (i - j).abs().min(min);
        }
        println!("{}", min);
    }
}
