use proconio::input;

use cplib::algebra::canonical::Canonical;
use cplib::algebra::xor::Xor;
use cplib::convolution::{InverseTransform, Transform};
use cplib::num::fp::fp;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: u64,
        k: usize,
        a: [usize; k],
    };

    let mut f = vec![fp!(0, mod P); 1 << 16];
    for &a in &a {
        f[a] += fp!(1);
    }

    Xor::new().transform(&Canonical::new(), &mut f);
    for f in f.iter_mut() {
        *f = if *f == fp!(1) {
            fp!(n)
        } else {
            *f * (f.pow(n) - fp!(1)) / (*f - fp!(1))
        };
    }
    Xor::new().inverse_transform(&Canonical::new(), &mut f);
    println!(
        "{}",
        (1..=n)
            .map(|i| fp!(k).pow(i))
            .fold(fp!(0), |acc, a| acc + a)
            - f[0]
    );
}
