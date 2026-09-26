use cplib::graph::tree::Tree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    };

    let mut e = vec![vec![]; (1 << n) - 1];
    for i in 1..1 << (n - 1) {
        e[i - 1].push(2 * i - 1);
        e[i - 1].push(2 * i);
        e[2 * i - 1].push(i - 1);
        e[2 * i].push(i - 1);
    }
    let tree = Tree::from_adjacency(&e, 0);
    let mut ans = 0usize;
    for i in 0..(1 << n) - 1 {
        for j in 0..(1 << n) - 1 {
            if tree.dist(i, j) == d {
                ans += 1;
                ans %= 998_244_353;
            }
        }
    }
    println!("{}", ans);
}
