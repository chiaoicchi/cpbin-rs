use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };
    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[a].push(b);
        e[b].push(a);
    }
    for e in e.iter_mut() {
        e.sort();
    }
    let mut f = vec![false; n];
    let mut ans = vec![];
    rec(0, 0, &mut f, &mut ans, &e);
    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}

fn rec(p: usize, i: usize, f: &mut [bool], ans: &mut Vec<usize>, e: &[Vec<usize>]) {
    f[i] = true;
    ans.push(i);
    for &j in &e[i] {
        if p != j && !f[j] {
            rec(i, j, f, ans, e);
            ans.push(i);
        }
    }
}
