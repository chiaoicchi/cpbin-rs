use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

use cplib::num::fp::fp;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut dp = vec![fp!(0, mod P); 1 << n];
    for i in 0..n {
        dp[1 << i] = fp!(1);
    }
    for &(a, b) in &ab {
        let mut ep = dp.clone();
        for v in 0..1 << n {
            if (v >> a) & 1 == 1 && (v >> b) & 1 == 1 {
                ep[v] += dp[v];
            }
            let mut x = v;
            loop {
                let y = v - x;
                if (x >> a) & 1 == 1 && (y >> a) & 1 == 0 && (x >> b) & 1 == 0 && (y >> b) & 1 == 1
                {
                    ep[v] += dp[x] * dp[y];
                }
                if x == 0 {
                    break;
                }
                x = (x - 1) & v;
            }
        }
        dp = ep;
    }
    let mut ans = vec![fp!(0); n];
    for v in 0..1 << n {
        for i in 1..n {
            if v & 1 == 1 && (v >> i) & 1 == 1 {
                ans[i] += dp[v];
            }
        }
    }
    println!("{}", ans[1..].iter().join("\n"));
}
