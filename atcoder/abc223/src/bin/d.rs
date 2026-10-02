use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut e = vec![vec![]; n];
    let mut ord = vec![0usize; n];
    for &(a, b) in &ab {
        ord[b] += 1;
        e[a].push(b);
    }
    let mut heap = BinaryHeap::new();
    let mut ans = vec![];
    for i in 0..n {
        if ord[i] == 0 {
            heap.push(!i);
        }
    }
    while let Some(u) = heap.pop() {
        let u = !u;
        ans.push(u);
        for &v in &e[u] {
            ord[v] -= 1;
            if ord[v] == 0 {
                heap.push(!v);
            }
        }
    }
    if ans.len() == n {
        println!("{}", ans.iter().map(|i| i + 1).join(" "));
    } else {
        println!("-1");
    }
}
