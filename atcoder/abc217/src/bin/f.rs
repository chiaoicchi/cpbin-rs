use std::collections::{HashMap, HashSet};

use cplib::num::factorial::Factorial;
use cplib::num::fp::{Fp, fp};
use proconio::input;
use proconio::marker::Usize1;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let factorial = Factorial::new(1 << 20);
    let mut e = vec![HashSet::new(); 2 * n];
    for &(a, b) in &ab {
        e[a].insert(b);
    }
    let mut memo = HashMap::new();
    let ans = rec(0, 2 * n, &e, &mut memo, &factorial);
    println!("{ans}");
}

fn rec(
    l: usize,
    r: usize,
    e: &[HashSet<usize>],
    memo: &mut HashMap<(usize, usize), Fp<P>>,
    factorial: &Factorial<Fp<P>>,
) -> Fp<P> {
    if l == r {
        return fp!(1);
    }
    if memo.contains_key(&(l, r)) {
        return *memo.get(&(l, r)).unwrap();
    }
    let mut ans = fp!(0);
    if e[l].contains(&(r - 1)) {
        ans += rec(l + 1, r - 1, e, memo, factorial);
    }
    for t in (l + 2..r).step_by(2) {
        let x = rec(l, t, e, memo, factorial);
        let y = rec(t, r, e, memo, factorial);
        ans += factorial.binomial((r - l) / 2, (t - l) / 2) * x * y;
    }
    memo.insert((l, r), ans);
    ans
}
