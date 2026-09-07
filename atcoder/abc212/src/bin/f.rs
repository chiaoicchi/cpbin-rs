use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;

use cplib::collections::dsu::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abst: [(Usize1, Usize1, u32, u32); m],
        xyz: [(u32, Usize1, u32); q],
    };
    let mut heap = BinaryHeap::new();
    for (j, &(_, _, s, t)) in abst.iter().enumerate() {
        heap.push((!s, !3, j));
        heap.push((!t, !2, j));
    }
    for (k, &(x, _, z)) in xyz.iter().enumerate() {
        heap.push((!x, !0, k));
        heap.push((!z, !1, k));
    }
    let mut dsu = Dsu::new(q);
    let mut people = vec![n + m; q];
    let mut city = vec![q; n];
    let mut bus = vec![q; m];
    let mut ans = vec![String::new(); q];
    while let Some((_, f, p)) = heap.pop() {
        let f = !f;
        match f {
            0 => {
                let y = xyz[p].1;
                if city[y] == q {
                    city[y] = p;
                }
                dsu.unite(city[y], p);
                people[dsu.root(p)] = y;
                city[y] = dsu.root(p);
            }
            1 => {
                let y = dsu.root(p);
                ans[p] = if people[y] < n {
                    format!("{}", people[y] + 1)
                } else {
                    let (a, b, _, _) = abst[people[y] - n];
                    format!("{} {}", a + 1, b + 1)
                }
            }
            2 => {
                let b = abst[p].1;
                if bus[p] < q {
                    if city[b] == q {
                        city[b] = bus[p];
                    }
                    dsu.unite(bus[p], city[b]);
                    people[dsu.root(bus[p])] = b;
                    city[b] = dsu.root(bus[p]);
                }
            }
            3 => {
                let a = abst[p].0;
                if city[a] < q {
                    bus[p] = city[a];
                    people[bus[p]] = n + p;
                    city[a] = q;
                }
            }
            _ => unreachable!(),
        }
    }
    for ans in ans {
        println!("{ans}");
    }
}
