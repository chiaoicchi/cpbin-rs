use proconio::input;
use proconio::marker::Bytes;

const P: u64 = 1_000_000_007;

fn main() {
    input! {
        s: Bytes,
    };
    let n = s.len();
    let mut dp = vec![0, 1];
    let mut prev = vec![];
    let mut tmp = [0; 26];
    for (i, &c) in s.iter().enumerate() {
        prev.push(tmp.clone());
        tmp[(c - b'a') as usize] = i + 1;
    }

    for (i, c) in s.iter().enumerate().skip(1) {
        dp.push(
            (dp[dp.len() - 1] + dp[dp.len() - 2] + P
                - if i == 0 {
                    0
                } else {
                    dp[prev[i][(c - b'a') as usize]]
                }
                + 1)
                % P,
        );
    }
    println!("{}", dp[n]);
}
