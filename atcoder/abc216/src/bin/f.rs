use cplib::num::fp::{Fp, fp};
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|i| a[*i]);
    let mut dp = vec![Fp::new(0); 5001];
    dp[0] = fp!(1, mod P);
    let mut ans = fp!(0);
    for i in 0..n {
        let i = idx[i];
        let mut ep = vec![fp!(0); 5001];
        for j in 0..=5000 - b[i] {
            ep[j + b[i]] += dp[j];
        }
        ans += ep[..=a[i]].iter().sum::<Fp<P>>();
        for i in 0..=5000 {
            ep[i] += dp[i];
        }
        dp = ep;
    }
    println!("{ans}");
}
