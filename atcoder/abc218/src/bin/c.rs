use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        s: [Bytes; n],
        mut t: [Bytes; n],
    };
}

fn f(t: &mut Vec<Vec<u8>>) {
    let n = t.len();
    let mut u = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            u[j][n - 1 - i] = t[i][j];
        }
    }
    *t = u;
}
