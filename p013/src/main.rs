use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, usize); m],
    }

    let abc: Vec<(usize, usize, usize)> = abc.into_iter().map(|(a, b, c)| (a-1, b-1, c)).collect();
    let mut path = vec![vec![]; n];
    for (a, b, c) in abc.into_iter() {
        path[a].push((b, c));
        path[b].push((a, c));
    }

    for i in 0..n { 
        println!("{}", solve(0, i, &path) + solve(i, n-1, &path));
    }
}

fn solve(from: usize, to: usize, path: &[Vec<(usize, usize)>]) -> usize {
    let mut todo = VecDeque::new();
    let mut checked = vec![false; path.len()];
    let mut cost = vec![usize::MAX; path.len()];
    cost[from] = 0;
    checked[from] = true;
    todo.push_back(from);

    while !todo.is_empty() {
        let f = todo.remove(0).unwrap();
        for &(t, c) in path[f].iter() {
            if cost[t] > cost[f] + c {
                cost[t] = cost[f] + c;
                todo.push_back(t);
            }
        }
    }

    cost[to]
}
