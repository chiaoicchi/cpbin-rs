use cplib::num::fp::{Fp, fp};
use proconio::input;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: u64,
        d: u64,
    };
    let mut ans = fp!(0, mod P);
    if d <= n - 1 {
        ans += fp!(2).pow(d);
    }
    for k in 1..n {
        let mut x = fp!(0);
        if k < d {
            if d >= 2 {
                x += f(d - k - 1, (d - 2).min(n - 1) + 1);
            }
            if k + d < n {
                x += fp!(2).pow(d);
            }
            ans += fp!(2).pow(k) * x;
        } else {
            if d >= 2 {
                x += f(0, (k - d).min((n - 1 - (k - d)) / 2) + 1);
            }
            x += fp!(1);
            if k + d < n {
                x += fp!(2).pow(d);
            }
            ans += fp!(2).pow(k) * x;
        }
        println!("k:{}, x:{}", k, x);
    }
    println!("{ans}");
}

// sum_[l, r) 2^i
fn f(l: u64, r: u64) -> Fp<P> {
    if l >= r {
        return fp!(0);
    }
    fp!(2).pow(l) * (fp!(2).pow(r - l) - fp!(1))
}
