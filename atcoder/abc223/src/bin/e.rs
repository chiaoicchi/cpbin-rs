use proconio::input;

fn main() {
    input! {
        mut x: u64,
        mut y: u64,
        mut a: u64,
        mut b: u64,
        mut c: u64,
    };
    let mut ans = false;
    for _ in 0..2 {
        // a b c
        if a.div_ceil(x) + b.div_ceil(x) + c.div_ceil(x) <= y {
            ans = true;
        }

        for _ in 0..3 {
            // a b
            // c
            if c.div_ceil(y) < x {
                let x = x - c.div_ceil(y);
                if a.div_ceil(x) + b.div_ceil(x) <= y {
                    ans = true;
                }
            }
            (a, b, c) = (b, c, a);
        }
        (x, y) = (y, x);
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
