use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        s: String,
    }

    let mut ans = 0;
    for (i, v) in s.chars().enumerate() {
        if v == 'a' {
            let mut memo = VecDeque::new();
            let mut next_memo = VecDeque::new();
            memo.push_back(i);
            for k in "tcoder".chars() {
                while !memo.is_empty() {
                    let mut checked = vec![false; s.len()];
                    let bound = memo.remove(0).unwrap();
                    for (ind, j) in s.chars().enumerate().skip(i) {
                        if j == k && !checked[ind] && ind > bound {
                            next_memo.push_back(ind);
                            checked[ind] = true;

                            if k == 'r' {
                                ans += 1;
                            }
                        }
                    }
                }
                memo = next_memo;
                next_memo = VecDeque::new();
            }
        }
    }

    println!("{}", ans);
}
