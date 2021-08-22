use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut path = vec![vec![]; n];
    for (a, b) in ab.into_iter() {
        let a = a - 1;
        let b = b - 1;
        path[a].push(b);
        path[b].push(a);
    }

    for i in solve(&path) {
        print!("{} ", i);
    }
}

fn solve(path: &[Vec<usize>]) -> Vec<usize> {
    for i in 0..path.len() {
        let mut ret = bfs(path, i);
        if ret.len() != 0 {
            ret.sort_by(|a, b| a.cmp(b));
            return ret.into_iter().map(|x| x + 1).collect();
        }
    }

    panic!("answer is nothing.");
}

fn bfs(path: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut todo = VecDeque::new();
    let mut visited = vec![false; path.len()];
    let mut map = vec![None; path.len()];
    let mut ans = vec![];
    todo.push_back(start);
    visited[start] = true;
    ans.push(start);
    map[start] = Some(true);

    while !todo.is_empty() {
        let from = todo.remove(0).unwrap();
        let f = !map[from].unwrap();
        for &to in path[from].iter() {
            if !visited[to] {
                map[to] = Some(f);
                todo.push_back(to);
                visited[to] = true;
                for &around in path[to].iter() {
                    if map[around] == Some(f) {
                        return vec![];
                    }
                }

                if f {
                    ans.push(to);
                }

                if ans.len() == path.len() / 2 {
                    return ans;
                }
            }
        }
    }

    vec![]
}
