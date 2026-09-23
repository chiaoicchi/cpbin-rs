use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(u64, u64); n],
    };

    let set: std::collections::HashSet<(u64, u64)> = xy.iter().copied().collect();

    let mut ans = 0usize;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[j];
            if x0 != x1 && y0 != y1 && set.contains(&(x0, y1)) && set.contains(&(x1, y0)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans / 2);
}
