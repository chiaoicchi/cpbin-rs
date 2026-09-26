use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        mut x: u64,
    };
    let s: u64 = a.iter().sum();
    let mut ans = x / s * n as u64;
    x %= s;
    for &a in &a {
        if x >= a {
            x -= a;
            ans += 1;
        } else {
            ans += 1;
            break;
        }
    }
    println!("{ans}");
}
