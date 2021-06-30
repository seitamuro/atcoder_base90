use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    let inputs: Vec<Vec<String>> = reader
        .lines()
        .take(3)
        .map(|x| x.unwrap())
        .map(|x| x.split(" ").map(|x| x.to_string()).collect())
        .collect();

    let _n: usize = inputs[0][0].parse().unwrap();
    let l: usize = inputs[0][1].parse().unwrap();
    let k: usize = inputs[1][0].parse().unwrap();
    let mut a: Vec<usize> = inputs[2].iter().map(|x| x.parse().unwrap()).collect();
    a.push(l);

    for i in 0..(l / 2 + 1) {
        if need_k(i, &a) < k {
            println!("{}", i-1);
            return;
        }
    }

    println!("{}", l);
}

fn need_k(length: usize, a: &Vec<usize>) -> usize {
    let mut pre = 0;
    let mut need = 0;
    let mut a = a.clone();
    a.insert(0, 0);

    let lens: Vec<usize> = a
        .iter()
        .take(a.len() - 1)
        .zip(a.iter().skip(1))
        .map(|x| x.1 - x.0)
        .collect();
    for i in lens {
        if pre + i >= length {
            pre = 0;
            need += 1;
        } else {
            pre += i;
        }
    }

    if pre != 0 && pre < length {
        need - 1
    } else {
        need
    }
}

#[test]
mod tests {
    use super::*;

    fn test_need_k() {
        let a = vec![7, 11, 16, 20, 28, 34, 38];
    }
}
