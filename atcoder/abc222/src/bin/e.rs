use std::collections::HashMap;

use cplib::graph::tree::Tree;
use cplib::num::fp::fp;
use proconio::input;
use proconio::marker::Usize1;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        a: [Usize1; m],
        uv: [(Usize1, Usize1); n - 1],
    };

    let mut e = vec![vec![]; n];
    let mut e_map = HashMap::new();
    for (i, &(u, v)) in uv.iter().enumerate() {
        e[u].push(v);
        e[v].push(u);
        e_map.insert((u, v), i);
        e_map.insert((v, u), i);
    }

    let mut cnt = vec![0isize; n - 1];
    for v in a.windows(2) {
        f(v[0], v[1], &e, &e_map, &mut cnt);
    }

    let mut dp = HashMap::new();
    dp.insert(0, fp!(1, mod P));
    for i in 0..n - 1 {
        let mut ep = HashMap::new();
        for (key, value) in dp.iter() {
            *ep.entry(key + cnt[i]).or_insert(fp!(0)) += *value;
            *ep.entry(key - cnt[i]).or_insert(fp!(0)) += *value;
        }
        dp = ep;
    }
    println!("{}", dp.get(&k).unwrap_or(&fp!(0)));
}

fn f(
    s: usize,
    mut t: usize,
    e: &[Vec<usize>],
    e_map: &HashMap<(usize, usize), usize>,
    cnt: &mut [isize],
) {
    let tree = Tree::from_adjacency(&e, s);
    while t != s {
        let p = tree.parent(t);
        cnt[*e_map.get(&(p, t)).unwrap()] += 1;
        t = p;
    }
}
