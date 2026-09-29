use cplib::algebra::canonical::{Additive, Canonical};
use cplib::collections::fenwick_tree::FenwickTree;
use cplib::num::fp::{Fp, fp};
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let x: Vec<Fp<P>> = (0..n).map(|i| fp!(2).pow(i as u64)).collect();
    let mut ft = FenwickTree::from_vec(Additive(Canonical::new()), x);
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| (a[i], i));
    let mut ans = fp!(0);
    for i in idx.iter() {
        let s = ft.fold(i + 1..);
        ans += s * fp!(2).inv().pow(*i as u64 + 1);
        ft.set(*i, &fp!(0));
    }
    println!("{ans}");
}
