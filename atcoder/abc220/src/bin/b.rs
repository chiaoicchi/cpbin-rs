use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        k: u64,
        a: Bytes,
        b: Bytes,
    };
    println!("{}", f(&a, k) * f(&b, k));
}

fn f(v: &[u8], k: u64) -> u64 {
    let mut ans = 0;
    let mut base = 1;
    for &c in v.iter().rev() {
        let c = (c - b'0') as u64;
        ans += c * base;
        base *= k;
    }
    ans
}
