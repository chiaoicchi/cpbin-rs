use cplib::grid::GridShape;
use proconio::input;

fn main() {
    input! {
        a: [[u8; 4]; 4],
    };
    let grid = GridShape::new(4, 4);
    let mut ans = 0usize;
    for v in 0..1 << 16 {
        if f(v, &grid) && g(v, &grid, &a) {
            ans += 1;
        }
    }
    println!("{ans}");
}

fn f(v: u64, grid: &GridShape) -> bool {
    for i in 0..16 {
        if (v >> i) & 1 == 1 {
            let mut stack = vec![grid.coord(i)];
            let mut flag = vec![false; 16];
            flag[i] = true;
            while let Some(u) = stack.pop() {
                for p in grid.neighbors4(u.0, u.1) {
                    let j = grid.index(p.0, p.1);
                    if (v >> j) & 1 == 1 && !flag[j] {
                        stack.push(p);
                        flag[j] = true;
                    }
                }
            }
            for p in 0..16 {
                if (v >> p) & 1 == 1 {
                    if !flag[p] {
                        return false;
                    }
                }
            }
            return true;
        }
    }
    false
}

fn g(v: u64, grid: &GridShape, a: &[Vec<u8>]) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] == 1 {
                let p = grid.index(i, j);
                if (v >> p) & 1 == 0 {
                    return false;
                }
            }
        }
    }
    true
}
