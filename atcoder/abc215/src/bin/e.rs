use cplib::num::fp::{Fp, fp};
use proconio::input;
use proconio::marker::Bytes;

const P: u32 = 998_244_353;
fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let mut dp = vec![vec![fp!(0, mod P); 1 << 10]; 10];
    for &c in &s {
        let c = (c - b'A') as usize;
        let mut ep = dp.clone();
        ep[c][1 << c] += fp!(1);
        for a in 0..10 {
            if a != c {
                for v in 0..1 << 10 {
                    if (v >> c) & 1 == 0 {
                        ep[c][v | (1 << c)] += dp[a][v];
                    }
                }
            } else {
                for v in 0..1 << 10 {
                    ep[c][v] += dp[a][v];
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp.iter().flatten().sum::<Fp<P>>());
}
