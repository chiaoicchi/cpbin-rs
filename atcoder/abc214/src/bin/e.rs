use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            mut lr: [(Usize1, usize); n],
        }
        lr.sort_by_key(|x| x.1);
        let mut set = BTreeSet::new();
        for (l, r) in &lr {}
    }
}
