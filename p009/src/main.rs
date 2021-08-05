use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [(f64, f64); n],
    }

    let mut ans = 0.;
    for i in 0..x.len() {
        let s = solve(i, &x);
        if ans < s {
            ans = s;
        }
    }

    println!("{}", ans);
}

fn solve(pos: usize, x: &[(f64, f64)]) -> f64 {
    let mut angles = Vec::new();
    for (ind, i) in x.iter().enumerate() {
        if ind == pos {continue;}
        angles.push(getangle(*i, x[pos]));
    }
    //dbg!(angles.clone());
    angles.sort_by(|a, b| {
        //println!("{:?} {:?}", a, b);
        a.partial_cmp(b).unwrap()
    });
    //dbg!(angles.clone());

    let mut ans = 0.;
    for &i in angles.iter() {
        let target = if i+180. >= 360. {i-180.} else {i+180.};
        let pos = higher_bound(&angles, target);
        //println!("target: {}", target);
        //println!("pos:  {:?}", pos.unwrap_or(0));
        let pos = pos.unwrap_or(0);
        let ind1 = pos%angles.len();
        let ind2 = (pos + angles.len() - 1) % angles.len();
        //println!("cand: {} {}", ind1, angles[ind1]);
        //println!("cand: {} {}", ind2, angles[ind2]);
        if ans < getangle2(i, angles[ind1]) {
            ans = getangle2(i, angles[ind1]);
        }
        if ans < getangle2(i, angles[ind2]) {
            ans = getangle2(i, angles[ind2]);
        }
    }

    //println!("{}", ans);
    ans
}

fn getangle(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let p = (p2.0 - p1.0, p2.1 - p1.1);

    if p.1 >= 0. {
        let mycos = p.0 / (p.0*p.0 + p.1*p.1).sqrt();
        mycos.acos() / (2.*std::f64::consts::PI) * 360.
    } else {
        let mycos = p.0 / (p.0*p.0 + p.1*p.1).sqrt();
        360.-mycos.acos() / (2.*std::f64::consts::PI) * 360.
    }
}

fn getangle2(angle1: f64, angle2: f64) -> f64 {
    let ans = (angle1 - angle2).abs();
    if ans > 180. {
        360. - ans
    } else {
        ans
    }
}

fn higher_bound<T: PartialOrd>(list: &[T], value: T) -> Option<usize> {
    let mut lower = -1i32;
    let mut higher = list.len() as i32;
    let h = list.len() as i32 - 1;

    fn w(v: i32, low: i32, high: i32) -> usize {
        if v < low {
            return low as usize;
        } else if v > high {
            return high as usize;
        }
        v as usize
    }

    while lower != higher && (higher - lower) > 1 {
        let m = (higher - lower) / 2 + lower;
        if list[w(m, 0, h)] <= value {
            lower = m as i32;
        } else if list[w(m, 0, h)] > value {
            higher = m as i32;
        }
    }

    if lower < 0 {
        None
    } else {
        Some(lower as usize)
    }
}
