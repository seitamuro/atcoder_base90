use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    let mut ans = 0;
    for (i, (l1, r1)) in lr.iter().enumerate() {
        for (j, (l2, r2)) in lr.iter().enumerate() {
            if i < j {continue;}

            let (l1, r1) = if l1 > r1 {(l1, r1)} else {(r1, l1)};
            let (l2, r2) = if l2 > r2 {(l2, r2)} else {(r2, l2)};

            if (r1 < r2) && (r2 < l1) && (l1 < l2) {
                ans += 1;
            } else if (r2 < r1) && (r1 < l2) && (l2 < l1) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}