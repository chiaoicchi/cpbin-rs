use cplib::collections::range_map::RangeMap;
use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(u8, usize); q],
    };
    let mut map = RangeMap::new();
    map.assign(0, l, (l, 0));
    for &(c, x) in &cx {
        if c == 1 {
            let Some((lo, hi, val)) = map.get(x) else {
                panic!();
            };
            let lo = lo.clone();
            let hi = hi.clone();
            map.assign(lo, x, (x - lo, lo));
            map.assign(x, hi, (hi - x, x));
        } else {
            println!("{}", map.get(x).unwrap().2.0);
        }
    }
}
