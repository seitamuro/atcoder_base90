use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w];h],
    }

    let mut sum_row = vec![0; h];
    let mut sum_col = vec![0; w];

    for i in 0..w {
        for j in 0..h {
            sum_col[i] += a[j][i];
            sum_row[j] += a[j][i];
        }
    }

    for (j, vj) in sum_row.iter().enumerate() {
        for (i, vi) in sum_col.iter().enumerate() {
            print!("{} ", vi+vj-a[j][i]);
        }
        println!("");
    }
}
