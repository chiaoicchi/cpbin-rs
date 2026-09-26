use cplib::graph::tree::Tree;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };
    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }
    let tree = Tree::from_adjacency(&e, 0);
    let mut ans = vec![!0; n];
    ans[0] = (0..n).map(|i| tree.dist(0, i)).sum();
    for i in 1..n {
        let v = tree.vertex(i);
        let pv = tree.parent(v);
        let a = tree.subtree_size(v);
        ans[v] = ans[pv] + (n - a) - a;
    }
    println!("{}", ans.iter().join("\n"));
}
