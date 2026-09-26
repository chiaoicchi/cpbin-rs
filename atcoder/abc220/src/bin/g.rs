use std::collections::HashMap;

use cplib::arithmetic::gcd::gcd;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xyc: [(i64, i64, i64); n],
    };

    let mut ans = 0;
    let mut map: HashMap<(i64, i64), [(i64, i64); 2]> = HashMap::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            let (xi, yi, ci) = xyc[i];
            let (xj, yj, cj) = xyc[j];
            let mut dx = xi - xj;
            let mut dy = yi - yj;
            if dx < 0 {
                dx = -dx;
                dy = -dy;
            }
            let (ddx, ddy, rep) = if dx == 0 {
                (0, 1, xi)
            } else {
                let d = gcd(dx, dy.abs());
                (dx / d, dy / d, yi - (dy / d) * (dx / d))
            };
            if map.contains_key(&(ddx, ddy)) {
                let mut v = map.remove(&(ddx, ddy)).unwrap().to_vec();
                if v[0].0 != rep {
                    ans = ans.max(ci + cj + v[0].1);
                } else if v[1].1 <= !0 {
                    ans = ans.max(ci + cj + v[1].1);
                }
                if v[0].0 == rep {
                    v[0].1 = v[0].1.max(ci + cj);
                } else if v[1].0 == rep {
                    v[1].1 = v[1].1.max(ci + cj);
                }
                v.push((rep, ci + cj));
                v.sort_by_key(|x| x.1);
                let nv = [v[0], v[1]];
                map.insert((ddx, ddy), nv);
            } else {
                map.insert((ddx, ddy), [(rep, ci + cj), (0, !0)]);
            }
        }
    }
    println!("{}", ans);
}
