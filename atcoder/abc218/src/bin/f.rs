use itertools::Itertools;
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(Usize1, Usize1); m],
    };
    let mut e = vec![vec![]; n];
    for (i, &(s, t)) in st.iter().enumerate() {
        e[s].push((t, i));
    }

    let mut queue = VecDeque::new();
    let mut a = vec![!0; n];
    let mut d = vec![!0; n];
    queue.push_back(0);
    a[0] = m;
    d[0] = 0usize;
    while let Some(u) = queue.pop_front() {
        for (v, i) in e[u].iter() {
            if d[*v] == !0 {
                d[*v] = d[u] + 1;
                a[*v] = *i;
                queue.push_back(*v);
            }
        }
    }
    if d[n - 1] == !0 {
        for _ in 0..m {
            println!("-1");
        }
        return;
    }
    let mut used = vec![];
    let mut ans = vec![d[n - 1]; m];
    let mut k = n - 1;
    while k > 0 {
        let i = a[k];
        used.push(i);
        ans[i] = !0;
        k = st[i].0;
    }

    for p in 0..m {
        if ans[p] < !0 {
            continue;
        }
        let mut e = vec![vec![]; n];
        for (i, &(s, t)) in st.iter().enumerate() {
            if i != p {
                e[s].push(t);
            }
        }
        let mut queue = VecDeque::new();
        let mut d = vec![!0; n];
        queue.push_back(0);
        d[0] = 0usize;
        while let Some(u) = queue.pop_front() {
            for &v in &e[u] {
                if d[v] == !0 {
                    d[v] = d[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        ans[p] = d[n - 1];
    }
    println!("{}", ans.iter().map(|x| *x as isize).join("\n"));
}
