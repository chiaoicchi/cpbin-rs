use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n],
    };

    let mut map = HashMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut x = vec![];
    for (k, v) in map.iter() {
        x.push((*k, *v));
    }
    x.sort();

    let mut ans = 0;
    while x.len() >= 2 {
        let p = x.pop().unwrap();
        let q = x.pop().unwrap();
        if (p.0 - q.0) * p.1 >= k {
            let u = k / p.1;
            ans += f(p.0 + 1 - u, p.0) * p.1;
            ans += (p.0 - u) * (k % p.1);
            k = 0;
            break;
        }
        ans += f(q.0 + 1, p.0) * p.1;
        x.push((q.0, p.1 + q.1));
        k -= (p.0 - q.0) * p.1;
    }
    if k > 0 {
        let p = x.pop().unwrap();
        let u = k / p.1;
        if p.0 + 1 < u {
            ans += f(1, p.0) * p.1;
        } else {
            ans += f(p.0 + 1 - u, p.0) * p.1;
            ans += (p.0 - u) * (k % p.1);
        }
    }

    println!("{}", ans);
}

/// a + a+1 + ... + b
fn f(a: u64, b: u64) -> u64 {
    (a + b) * (b + 1 - a) / 2
}
