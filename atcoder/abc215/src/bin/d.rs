use cplib::num::prime::{factorize, primes};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
    };

    let mut x = vec![true; m + 1];
    x[0] = false;
    for &a in &a {
        for &(p, _) in &factorize(a) {
            if p as usize <= m {
                x[p as usize] = false;
            }
        }
    }
    for prime in primes(m) {
        if !x[prime] {
            for k in 2.. {
                if prime * k > m {
                    break;
                }
                x[prime * k] = false;
            }
        }
    }
    let ans: Vec<usize> = (0..=m).filter(|i| x[*i]).collect();
    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
}
