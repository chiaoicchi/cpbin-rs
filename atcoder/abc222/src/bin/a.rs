use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        mut n: Bytes,
    };
    while n.len() < 4 {
        n.insert(0, b'0');
    }
    println!("{}", std::str::from_utf8(&n).unwrap());
}
