use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
        k: usize,
    }

    // preprocessing
    let mut next = HashMap::new();
    for i in s.chars() {
        let ent = next.entry(i).or_insert(vec![0;s.len()+1]);
        ent[s.len()] = s.len();
    }

    let mut keys: Vec<char> = next.keys().copied().collect();
    for (i, &v) in s.chars().collect::<Vec<char>>().iter().enumerate().rev() {
        for &j in keys.iter() {
           let ent = next.get_mut(&j).unwrap();
           if v == j {
               ent[i] = i;
           } else {
               ent[i] = ent[i + 1];
           }
        }
    }

    // 実際に処理を行う
    let mut ans = Vec::new();
    let mut possiblepos = 0;
    let mut nowpos;
    keys.sort_unstable();
    while ans.len() != k {
        for key in keys.iter() {
            if s.len() + ans.len() - next[key][possiblepos] >= k {
                ans.push(*key);
                nowpos = next[key][possiblepos];
                possiblepos = nowpos;
                break;
            }
        }
        possiblepos += 1;
    }

    println!("{}", ans.iter().collect::<String>());
}
