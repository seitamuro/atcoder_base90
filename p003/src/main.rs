use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    };

    let mut a = Vec::new();
    let mut b = Vec::new();
    for (av, bv) in ab.into_iter() {
        a.push(av);
        b.push(bv);
    }

    let mut path = vec![Vec::new();n];
    for i in 0..n-1 {
        path[a[i] - 1].push(b[i] - 1);
        path[b[i] - 1].push(a[i] - 1);
    }

    let dist = bfs(0, n, &path);
    let mut max = 0;
    let mut maxi = 0;
    for (i, &v) in dist.iter().enumerate() {
        if v > max {
            max = v;
            maxi = i;
        }
    }

    let dist = bfs(maxi, n, &path);
    let max = dist.into_iter().fold(0, |x, a| if x < a {a} else {x});

    println!("{}", max + 1);
}

fn bfs(start: usize, n: usize, path: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ans = vec![0; n];
    let mut todo = Vec::<usize>::new();

    for &i in path[start].iter() {
        todo.push(i);
        ans[i] = 1;
    }

    while todo.len() != 0 {
        let node = todo.remove(0);

        for &i in path[node].iter() {
            if ans[i] == 0 {
                todo.push(i);
                ans[i] = ans[node] + 1;
            }
        }
    }

    ans
}
