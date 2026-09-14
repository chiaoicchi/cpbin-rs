use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u64; n],
        t: [u64; n],
    };
    let mut heap = BinaryHeap::new();
    let mut ans = vec![!0; n];
    for (i, &t) in t.iter().enumerate() {
        heap.push((!t, i));
        ans[i] = t;
    }
    while let Some((t, i)) = heap.pop() {
        let t = !t;
        if ans[i] < t {
            continue;
        }
        let next = (i + 1) % n;
        if t + s[i] < ans[next] {
            ans[next] = t + s[i];
            heap.push((!(t + s[i]), next));
        }
    }
    println!("{}", ans.iter().join("\n"));
}
