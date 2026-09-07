use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        x: Bytes,
    };
    let ans = x.iter().all(|&xi| x[0] == xi)
        || x.windows(2)
            .all(|v| (v[0] + 1 - b'0') % 10 == (v[1] - b'0') % 10);
    println!("{}", if ans { "Weak" } else { "Strong" });
}
