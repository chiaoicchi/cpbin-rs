use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        uv: [(Usize1, Usize1); m],
        x: [Usize1; q],
    };
    let sqrt_m = (m as f64).sqrt() as usize;
    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        if sqrt_m < e[v].len() {
            g[u].push(v);
        }
        if sqrt_m < e[u].len() {
            g[v].push(u);
        }
    }
    let mut ans = vec![(0usize, 0usize); n];
    for i in 0..n {
        ans[i] = (i + 1, 0);
    }
    let mut his = vec![(0usize, 0usize); n];
    for (i, &x) in x.iter().enumerate() {
        let i = i + 1;
        for &y in g[x].iter() {
            if his[y].1 > ans[x].1 {
                ans[x] = his[y];
            }
        }
        let vx = ans[x].0;
        if e[x].len() <= sqrt_m {
            for &y in &e[x] {
                ans[y] = (vx, i);
            }
        } else {
            his[x] = (vx, i);
        }
    }
    for i in 0..n {
        let (val, t) = his[i];
        for &j in &e[i] {
            if ans[j].1 < t {
                ans[j].0 = val;
                ans[j].1 = t;
            }
        }
    }
    println!("{}", ans.iter().map(|x| x.0).join(" "));
}
