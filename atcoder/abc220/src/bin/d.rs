use itertools::Itertools;
use proconio::input;

use cplib::num::fp::fp;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut dp = vec![fp!(0, mod P); 10];
    dp[a[0]] += fp!(1);
    for a in a[1..].iter() {
        let mut ep = vec![fp!(0); 10];
        for i in 0..10 {
            let ni = (i + a) % 10;
            let mi = (i * a) % 10;
            ep[ni] += dp[i];
            ep[mi] += dp[i];
        }
        dp = ep;
    }
    println!("{}", dp.iter().join("\n"));
}
