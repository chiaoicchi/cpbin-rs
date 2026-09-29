use cplib::algebra::assert::AssertCommutative;
use cplib::algebra::closures::FnMonoid;
use cplib::graph::rerooting::Rerooting;
use cplib::num::fp::{Fp, fp};
use proconio::input;
use proconio::marker::Usize1;

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };
}
