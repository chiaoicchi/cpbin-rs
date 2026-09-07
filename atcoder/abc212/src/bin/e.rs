use proconio::input;
use proconio::marker::Usize1;

use cplib::num::fp::fp;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m],
    };
    let mut dp = vec![fp!(0, mod P); n];
    dp[0] = fp!(1);
    for _ in 0..k {
        let t = dp.iter().fold(fp!(0), |acc, a| acc + a);
        let mut ep: Vec<_> = dp.iter().map(|dp| t - dp).collect();
        for &(u, v) in &uv {
            ep[u] -= dp[v];
            ep[v] -= dp[u];
        }
        dp = ep;
    }
    println!("{}", dp[0]);
}
