use proconio::input;

fn main() {
    input! {
        n: usize,
        pos: [(usize, usize, usize, usize); n],
    }

    let mut map = vec![vec![0; 1009]; 1009];
    for (lx, ly, rx, ry) in pos.into_iter() {
        map[lx][ly] += 1;
        map[rx][ly] -= 1;
        map[lx][ry] -= 1;
        map[rx][ry] += 1;
    }

    for i in 0..1000 {
        let mut t = 0;
        for j in 0..1000 {
            t += map[i][j];
            map[i][j] = t;
        }
    }

    let mut ans = vec![0usize; 100001];
    for i in 0..1000 {
        let mut t = 0;
        for j in 0..1000 {
            t += map[j][i];
            map[j][i] = t;
            ans[t as usize] += 1;
        }
    }

    for i in ans.into_iter().take(n+1).skip(1) {
        println!("{}", i);
    }
}
