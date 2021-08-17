use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }

    if a < c.pow(b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
