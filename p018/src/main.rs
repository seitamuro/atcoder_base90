use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }

    for i in e.into_iter() {
        let xx = -l / 2. * (i / t * std::f64::consts::PI * 2.).sin();
        let yy = l * (i / t).abs();

        let angle = ((yy - y) / ((x - xx)*(x - xx) + y*y).sqrt()).atan() / std::f64::consts::PI * 180.;
        println!("{}", angle.abs());
    }
}
