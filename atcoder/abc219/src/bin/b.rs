use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: [String; 3],
        t: Bytes,
    };
    let mut ans = "".to_string();
    for &t in &t {
        let t = (t - b'1') as usize;
        ans += &s[t];
    }
    println!("{ans}");
}
