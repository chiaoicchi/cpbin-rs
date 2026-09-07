use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        mut b: [u32; m],
    };
    a.sort();
    b.sort();
    let mut ans = !0;
    let mut idx = 0;
    for &a in &a {
        while idx < m && a > b[idx] {
            ans = ans.min(a - b[idx]);
            idx += 1;
        }
        if idx < m {
            ans = ans.min(b[idx] - a);
        }
    }
    println!("{ans}");
}
