use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut path = vec![vec![]; n];
    let mut revpath = vec![vec![]; n];
    for (a, b) in ab.into_iter() {
        let a = a - 1;
        let b = b - 1;
        path[a].push(b);
        revpath[b].push(a);
    }

    println!("{}", solve(&path, &revpath));
}

fn solve(path: &[Vec<usize>], revpath: &[Vec<usize>]) -> usize {
    // first
    let mut visited = vec![false; path.len()];
    let mut num = vec![0; path.len()];
    let mut cnt = 0;
    let mut hist = Vec::new();
    for i in 0..path.len() {
        if !visited[i] {
            dfs(&path, &mut visited, &mut num, &mut cnt, &mut hist, i);
        }
    }

    // seconds
    let mut visited = vec![false; path.len()];
    let hist: Vec<usize> = hist.into_iter().rev().collect();
    let mut ans = 0;
    for i in hist.into_iter() {
        let rev = dfs2(&revpath, &mut visited, i);
        ans += rev*(rev-1)/2;
    }
    ans
}

fn dfs2(path: &[Vec<usize>], visited: &mut Vec<bool>, ind: usize) -> usize {
    let mut group = 0;
    visited[ind] = true;
    group += 1;
    for &i in path[ind].iter() {
        if !visited[i] {
            group += dfs2(path, visited, i);
        }
    }
    group
}

fn dfs(path: &[Vec<usize>], visited: &mut Vec<bool>, num: &mut Vec<usize>, cnt: &mut usize, hist: &mut Vec<usize>, ind: usize) {
    visited[ind] = true;
    for &i in path[ind].iter() {
        if !visited[i] {dfs(path, visited, num, cnt, hist, i);}
    }

    hist.push(ind);
    *cnt += 1;
    num[ind] = *cnt;
}