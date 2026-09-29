use cplib::collections::compression::Compression;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    };
    let mut x = vec![];
    for &(a, b) in ab.iter() {
        x.push(a);
        x.push(a + b);
    }
    x.push(0);
    let comp = Compression::from_vec(x);
    let mut s = vec![0i64; 2 * n + 1];
    for (a, b) in &ab {
        let ca = comp.compress(a).unwrap();
        let cb = comp.compress(&(a + b)).unwrap();
        s[ca] += 1;
        s[cb] -= 1;
    }
    for i in 0..2 * n {
        s[i + 1] += s[i];
    }
    let mut prev = comp[0];
    let mut ans = vec![0; n + 1];
    for i in 0..comp.len() - 1 {
        let a = comp[i + 1];
        ans[s[i] as usize] += a - prev;
        prev = a;
    }
    println!("{}", ans[1..].iter().join(" "));
}
