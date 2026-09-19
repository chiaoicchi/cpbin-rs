use cplib::algebra::closures::{FnAction, FnMonoid};
use cplib::collections::lazy_segment_tree::LazySegmentTree;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lrx: [(Usize1, usize, usize); m],
    };
    lrx.sort_by_key(|p| p.1);

    let mut lst = LazySegmentTree::from_vec(
        FnMonoid {
            id: (0, 0),
            op: |a: &(usize, usize), b: &(usize, usize)| -> (usize, usize) {
                (a.0 + b.0, a.1 + b.1)
            },
        },
        FnMonoid {
            id: None,
            op: |a: &Option<usize>, b: &Option<usize>| -> Option<usize> {
                if b.is_some() { *b } else { *a }
            },
        },
        FnAction {
            act: |f: &Option<usize>, x: &(usize, usize)| -> (usize, usize) {
                let s = if f.is_none() { x.0 } else { f.unwrap() * x.1 };
                (s, x.1)
            },
        },
        vec![(0, 1); n],
    );
    for &(l, r, x) in &lrx {
        if lst.fold(l..r).0 >= x {
            continue;
        }
        let mut lo = l;
        let mut hi = r;
        while hi - lo > 1 {
            let o = (hi + lo) / 2;
            if lst.fold(l..o).0 + r - o < x {
                hi = o;
            } else {
                lo = o;
            }
        }
        lst.range_apply(lo..r, &Some(1));
    }
    println!("{}", (0..n).map(|i| lst.get(i).unwrap().0).join(" "));
}
