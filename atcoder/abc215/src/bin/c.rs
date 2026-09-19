use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Usize1};
fn main() {
    input! {
        s: Bytes,
        k: Usize1,
    };
    let mut x = vec![];
    for v in s.iter().permutations(s.len()) {
        x.push(v);
    }
    x.sort();
    x.dedup();
    println!("{}", x[k].iter().map(|c| **c as char).join(""));
}
