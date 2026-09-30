use cplib::algebra::min_max::Max;
use cplib::graph::rerooting::Rerooting;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, u64); n - 1],
        d: [u64; n],
    };
    let mut edges = vec![];
    for i in 0..n - 1 {
        edges.push((abc[i].0, abc[i].1));
    }

    let rerooting = Rerooting::from_edges(
        n,
        &edges,
        &Max::new(),
        |x: &u64, _: usize, _: usize, i: usize| x + abc[i].2,
        |y: &u64, v: usize| (*y).max(d[v]),
    );
    let mut ans = vec![0; n];
    for &(u, v, c) in &abc {
        ans[u] = ans[u].max(rerooting.fold(v, u) + c);
        ans[v] = ans[v].max(rerooting.fold(u, v) + c);
    }
    println!("{}", ans.iter().join("\n"));
}
