use cplib::algebra::max::Max;
use cplib::collections::sparse_table::SparseTable;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(u64, u64); n],
    };

    xy.sort();
    let rmq = SparseTable::from_vec(Max::new(), xy.iter().map(|v| v.1).collect());

    let mut ans = 0;
    for i in 0..n {
        let mut l = 0;
        let mut r = 1 << 50;
        while r - l > 1 {
            let o = l + (r - l) / 2;
            let k = xy[i..].partition_point(|v| v.0 < o + xy[i].0);
            if let Some(x) = rmq.fold(i + k..) {
                if x >= o + xy[i].1 {
                    l = o;
                } else {
                    r = o;
                }
            } else {
                r = o;
            }
        }
        ans = ans.max(l);
    }
    xy.reverse();
    let xy: Vec<(u64, u64)> = xy.iter().map(|(x, y)| (!0 / 2 - x, !0 / 2 - y)).collect();
    let rmq = SparseTable::from_vec(Max::new(), xy.iter().map(|v| v.1).collect());

    let mut ans = 0;
    for i in 0..n {
        let mut l = 0;
        let mut r = 1 << 50;
        while r - l > 1 {
            let o = l + (r - l) / 2;
            let k = xy[i..].partition_point(|v| v.0 < o + xy[i].0);
            if let Some(x) = rmq.fold(i + k..) {
                if x >= o + xy[i].1 {
                    l = o;
                } else {
                    r = o;
                }
            } else {
                r = o;
            }
        }
        ans = ans.max(l);
    }

    println!("{}", ans);
}
