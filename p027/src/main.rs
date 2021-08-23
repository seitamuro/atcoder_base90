use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut memo = HashMap::new();

    for (cnt, i) in s.iter().enumerate() {
        if memo.get(i) == None {
            println!("{}", cnt + 1);
            memo.insert(i, true);
        }
    }
}
