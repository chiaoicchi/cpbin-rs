use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    };
    let mut ans = false;
    if s == t {
        ans = true;
    }
    for i in 0..s.len() - 1 {
        let mut ss = s.clone();
        ss.swap(i, i + 1);
        if ss == t {
            ans = true;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
