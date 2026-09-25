use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = vec![vec![!0usize; y + 1]; x + 1];
    dp[0][0] = 0usize;
    for &(a, b) in &ab {
        let mut ep = dp.clone();
        for i in 0..=x {
            for j in 0..=y {
                if dp[i][j] < !0 {
                    let ni = x.min(a + i);
                    let nj = y.min(b + j);
                    ep[ni][nj] = ep[ni][nj].min(dp[i][j] + 1);
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp[x][y] as isize);
}
