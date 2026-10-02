use itertools::Itertools;
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Bytes; 2 * n],
    };

    let mut dp: Vec<usize> = (0..2 * n).collect();
    let mut cnt = vec![0u32; 2 * n];
    for i in 0..m {
        for k in 0..n {
            let x = dp[2 * k];
            let y = dp[2 * k + 1];
            match (a[x][i], a[y][i]) {
                (b'G', b'C') => cnt[x] += 1,
                (b'C', b'P') => cnt[x] += 1,
                (b'P', b'G') => cnt[x] += 1,
                (b'C', b'G') => cnt[y] += 1,
                (b'P', b'C') => cnt[y] += 1,
                (b'G', b'P') => cnt[y] += 1,
                _ => {}
            }
        }
        let mut ep: Vec<usize> = (0..2 * n).collect();
        ep.sort_unstable_by_key(|i| (!cnt[*i], *i));
        dp = ep;
    }
    println!("{}", dp.iter().map(|i| i + 1).join("\n"));
}
