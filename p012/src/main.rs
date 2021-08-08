//use proconio::input;
use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut h = 0usize;
    let mut w = 0usize;
    //let mut q = 0usize;
    let mut query = Vec::new();

    for (i, v) in buf.split('\n').enumerate() {
        if i == 0 {
            let j: Vec<String> = v.split(' ').map(|x| x.to_string()).collect();
            h = j[0].parse().unwrap();
            w = j[1].parse().unwrap();
        } else if i == 1 {
            //q = v.parse().unwrap();
        } else if !v.is_empty() {
            let j: Vec<usize> = v.split(' ').map(|x| x.to_string().parse().unwrap()).collect();
            let v3 = *j.get(3).unwrap_or(&1) - 1;
            let v4 = *j.get(4).unwrap_or(&1) - 1;
            query.push((j[0], j[1] - 1, j[2] - 1, v3, v4));
        }
    }

    //println!("{:?}", query);

    let mut map = vec![vec![0;h];w];

    for &i in query.iter() {
        if i.0 == 1 {
            map[i.1][i.2] = 1;
        } else if i.0 == 2 {
            if is_reachable(i.1, i.2, i.3, i.4, (h, w), &map) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }

}

fn is_reachable(x1: usize, y1: usize, x2: usize, y2: usize, hw: (usize, usize), map: &[Vec<usize>]) -> bool {
    let mut todo = VecDeque::new();
    todo.push_back((x1, y1));
    let mut checked = vec![vec![false;hw.0];hw.1];
    let mut reachable = vec![vec![false;hw.0];hw.1];
    checked[x1][y1] = true;
    reachable[x1][y1] = true;

    if map[x1][y1] != 1 || map[x2][y2] != 1 {
        return false;
    }

    while !todo.is_empty() {
        let pos = todo.remove(0).unwrap();
        if pos.0 < hw.1 - 1 && !checked[pos.0+1][pos.1] {
            checked[pos.0+1][pos.1] = true;
            if map[pos.0+1][pos.1] == 1 {
                reachable[pos.0+1][pos.1] = true;
                todo.push_back((pos.0+1, pos.1));
            }
        }
        if pos.0 > 0 && !checked[pos.0-1][pos.1] {
            checked[pos.0-1][pos.1] = true;
            if map[pos.0-1][pos.1] == 1 {
                reachable[pos.0-1][pos.1] = true;
                todo.push_back((pos.0-1, pos.1));
            }
        }
        if pos.1 > 0 && !checked[pos.0][pos.1-1] {
            checked[pos.0][pos.1-1] = true;
            if map[pos.0][pos.1-1] == 1 {
                reachable[pos.0][pos.1-1] = true;
                todo.push_back((pos.0, pos.1-1));
            }
        }
        if pos.1 < hw.0 - 1 && !checked[pos.0][pos.1+1] {
            checked[pos.0][pos.1+1] = true;
            if map[pos.0][pos.1+1] == 1 {
                reachable[pos.0][pos.1+1] = true;
                todo.push_back((pos.0, pos.1+1));
            }
        }
    }

    reachable[x2][y2]
}
