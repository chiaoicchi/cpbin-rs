use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    };
    let s: f64 = ab.iter().map(|(a, b)| *a as f64 / *b as f64).sum();

    let mut ok = 0.0f64;
    let mut ng = 1e10;
    while ng - ok > 1e-7 {
        let mid = (ok + ng) / 2.0;
        let mut len = 0.0;
        let mut mid_time = 0.0;
        for &(a, b) in &ab {
            let a = a as f64;
            let b = b as f64;
            if len + a >= mid {
                mid_time += (mid - len) / b;
                break;
            }
            len += a;
            mid_time += a / b;
        }
        if mid_time <= s / 2.0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{ok}");
}
