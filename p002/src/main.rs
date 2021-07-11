use proconio::input;

fn main() {
    input! {
        n: usize
    }

    for i in 0..=((1 << n) - 1) {
        let mut s = String::new();
        let mut r = i;
        for _ in 0..n {
            if r % 2 == 0 {
                s.push('(');
            } else {
                s.push(')');
            }
            r /= 2;
        }

        if ok(&s) {
            println!("{}", s);
        }
    }
}

fn ok(s: &str) -> bool {
    if s.len() % 2 == 1 {
        return false;
    } else {
        let mut memo = 0i32;
        for i in s.chars() {
            if i == '(' {
                memo += 1;
            } else {
                memo -= 1;
            }
            if memo < 0 {
                return false;
            }
        }

        if memo != 0 {
            return false;
        }
    }

    true
}
