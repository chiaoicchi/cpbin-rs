use cplib::algebra::canonical::{Additive, Canonical};
use cplib::algebra::closures::FnAction;
use cplib::algebra::min_max::Min;
use cplib::collections::lazy_segment_tree::LazySegmentTree;
use proconio::input;
use proconio::marker::{Bytes, Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Bytes,
        queries: [(u8, Usize1, Usize1); q],
    };
    let mut acc = vec![0i64];
    for &c in &s {
        if c == b'(' {
            acc.push(acc.last().unwrap() + 1);
        } else {
            acc.push(acc.last().unwrap() - 1);
        }
    }
    let mut segtree = LazySegmentTree::from_vec(
        Min::new(),
        Additive(Canonical::new()),
        FnAction {
            act: |f: &i64, x: &i64| f + x,
        },
        acc,
    );

    for &(t, l, r) in &queries {
        if t == 1 {
            match (s[l], s[r]) {
                (b'(', b')') => {
                    segtree.range_apply(l + 1..r + 1, &-2);
                    s.swap(l, r);
                }
                (b')', b'(') => {
                    segtree.range_apply(l + 1..r + 1, &2);
                    s.swap(l, r);
                }
                _ => {}
            }
        } else {
            let s = segtree.get(l).unwrap();
            let t = segtree.get(r + 1).unwrap();
            let mn = segtree.fold(l..r + 2);
            if s <= mn && s == t {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
