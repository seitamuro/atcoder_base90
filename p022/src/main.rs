use proconio::input;

fn main() {
    input! {
        a: u64, 
        b: u64, 
        c: u64, 
    }

    let d = gcd(a, gcd(b, c));
    println!("{}", a/d + b/d + c/d - 3);
}

fn gcd(mut n1: u64, mut n2: u64) -> u64 {
    if n1 < n2 {
        let tmp = n1;
        n1 = n2;
        n2 = tmp;
    }

    let n1 = n1 % n2;
    if n1 == 0 {
        n2
    } else {
        gcd(n1, n2)
    }
}
