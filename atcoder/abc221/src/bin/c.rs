use itertools::Itertools;
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: Bytes,
    };
    let n: Vec<u64> = n.iter().map(|i| (i - b'0') as u64).collect();
    let len = n.len();
    let mut ans = 0;
    for p in n.into_iter().permutations(len) {
        for k in 1..len {
            let a = f(&p[..k]);
            let b = f(&p[k..]);
            ans = ans.max(a * b);
        }
    }
    println!("{ans}");
}

fn f(x: &[u64]) -> u64 {
    let mut ans = 0;
    for x in x.iter() {
        ans *= 10;
        ans += x;
    }
    ans
}
