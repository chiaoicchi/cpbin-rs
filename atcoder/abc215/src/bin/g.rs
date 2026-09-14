use std::collections::HashMap;

use cplib::collections::compression::Compression;
use cplib::num::factorial::Factorial;
use cplib::num::fp::{Fp, fp};
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        c: [u64; n],
    };
    let comp = Compression::from_vec(c.clone());
    let mut v = vec![0usize; comp.len()];
    for c in c.iter() {
        v[comp.compress(c).unwrap()] += 1;
    }
    v.sort();
    let mut w = HashMap::new();
    for &v in &v {
        *w.entry(v).or_insert(0) += 1usize;
    }
    let factorial = Factorial::<Fp<P>>::new(1 << 20);
    for k in 1..=n {
        let p = factorial.binomial(n, k).inv();
        let ans = w
            .iter()
            .map(|(v, w)| fp!(*w) * (fp!(1) - factorial.binomial(n - v, k) * p))
            .sum::<Fp<P>>();
        println!("{ans}");
    }
}
