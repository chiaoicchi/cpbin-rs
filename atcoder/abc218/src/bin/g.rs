use cplib::graph::tree::Tree;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        uv: [(Usize1, Usize1); n - 1],
    };
    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let mut hi = 1 << 30;
    let mut lo = 0;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        let cnt = f(mid);
        if cnt * 2 < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("{}", hi);
}

fn f(x: u32) -> usize {
    todo!();
}
