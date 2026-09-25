use cplib::collections::dsu::Dsu;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(Usize1, Usize1, i64); m],
    };

    let s: i64 = abc.iter().map(|x| x.2).sum();
    abc.sort_by_key(|x| x.2);
    let mut dsu = Dsu::new(n);
    let mut k = 0;
    for &(a, b, c) in &abc {
        if dsu.unite(a, b) {
            k += c;
        } else if c < 0 {
            k += c;
        }
    }
    println!("{}", s - k);
}
