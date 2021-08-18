use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut path = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        path[a-1].push(b-1);
    }

    println!("{}", solve(&path));
}

fn solve(path: &[Vec<usize>]) -> usize {
    let mut visited = vec![false; path.len()];
    let mut num = vec![0; path.len()];
    let mut cnt = 0;

    // first
    for i in 0..path.len() {
        dfs(path, &mut visited, &mut num, &mut cnt, i);
    }

    // reverse
    let mut newpath = vec![vec![];path.len()];
    for i in 0..path.len() {
        for &j in path[i].iter() {
            newpath[j].push(i);
        }
    }

    let mut visited = vec![false; path.len()];
    let mut group = Vec::new();
    let mut sum = 0;

    while true {
        let mut max = 0;
        let mut max_i = 0;
        for i in 0..path.len() {
            if !visited[i] && num[i] > max {
                max = num[i];
                max_i = i;
            }
        }

        let res = dfs2(&newpath, &mut visited, max_i);
        sum += res;
        group.push(res);

        if sum == path.len()-1 {
            break;
        }
    }

    let mut ans = 0;
    for i in group.into_iter() {
        ans += i*(i-1)/2;
    }

    ans
}

fn dfs(path: &[Vec<usize>], visited: &mut Vec<bool>, num: &mut Vec<usize>, cnt: & mut usize, ind: usize) {
    let inc = !visited[ind];
    visited[ind] = true;
    for &i in path[ind].iter() {
        if !visited[i] {
            dfs(path, visited, num, cnt, i);
        }
    }

    if inc {
        *cnt += 1;
        num[ind] = *cnt;
    }
}

fn dfs2(path: &[Vec<usize>], visited: &mut Vec<bool>, ind: usize) -> usize {
    visited[ind] = true;
    let mut cnt = 0;
    for &i in path[ind].iter() {
        if !visited[i] {
            cnt += dfs2(path, visited, i);
        }
    }

    cnt + 1
}