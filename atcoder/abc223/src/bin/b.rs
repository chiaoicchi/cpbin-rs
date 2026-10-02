use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    };

    let mut mn = s.clone();
    let mut mx = s.clone();
    for _ in 1..s.len() {
        s.rotate_left(1);
        mn = mn.min(s.clone());
        mx = mx.max(s.clone());
    }
    println!("{}", mn.iter().collect::<String>());
    println!("{}", mx.iter().collect::<String>());
}
