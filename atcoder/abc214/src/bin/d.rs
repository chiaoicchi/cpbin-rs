use cplib::collections::dsu::Dsu;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, u64); n - 1],
    };

    uvw.sort_by_key(|x| x.2);
    let mut dsu = Dsu::new(n);
    let mut ans = 0;
    for &(u, v, w) in &uvw {
        let p = dsu.set_size(u);
        let q = dsu.set_size(v);
        ans += w * p as u64 * q as u64;
        dsu.unite(u, v);
    }
    println!("{ans}");
}
