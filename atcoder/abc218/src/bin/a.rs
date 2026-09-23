use proconio::input;
use proconio::marker::{Bytes, Usize1};

fn main() {
    input! {
        n: Usize1,
        s: Bytes,
    };
    println!("{}", if s[n] == b'o' { "Yes" } else { "No" });
}
