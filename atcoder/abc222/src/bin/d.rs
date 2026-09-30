use cplib::num::fp::{Fp, fp};
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let mut dp = vec![fp!(0, mod P); 3001];
    dp[0] = fp!(1);
    for (a, b) in a.iter().zip(b.iter()) {
        let mut s = vec![fp!(0)];
        for i in 0..=3000 {
            s.push(s.last().unwrap() + dp[i]);
        }
        let mut ep = vec![fp!(0); 3001];
        for k in *a..=*b {
            ep[k] += s[k + 1];
        }
        dp = ep;
    }
    println!("{}", dp.iter().sum::<Fp<P>>());
}
