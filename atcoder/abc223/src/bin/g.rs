use cplib::algebra::assert::AssertCommutative;
use cplib::algebra::closures::FnMonoid;
use cplib::graph::rerooting::{self, Rerooting};
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    };

    let rerooting = Rerooting::from_edges(
        n,
        &uv,
        &AssertCommutative(FnMonoid {
            id: (0, 0, -1 << 30),
            op: |x: &(isize, isize, isize), y: &(isize, isize, isize)| {
                (
                    x.0 + y.0,
                    (x.0 + y.1).max(x.1 + y.0),
                    (x.0 + y.2).max(x.2 + y.0),
                )
            },
        }),
        |x: &(isize, isize, isize), _: usize, _: usize, _: usize| {
            (x.0.max(x.2), x.0.max(x.2), x.1 + 1)
        },
        |y: &(isize, isize, isize), _: usize| *y,
    );

    let mut ans = 0usize;
    let mut cnt = vec![0; n];
    let x = rerooting.fold(0, 0);
    for &(u, v) in &uv {
        let (x0, x1, x2) = rerooting.fold(v, u);
        cnt[u] += x0.max(x1).max(x2);
        let (x0, x1, x2) = rerooting.fold(u, v);
        cnt[v] += x0.max(x1).max(x2);
    }
    for cnt in cnt {
        if cnt == x.0.max(x.1).max(x.2) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
