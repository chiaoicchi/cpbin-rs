use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        t: Chars,
    };
    let x = &t[..t.len() - 2];
    let y = t[t.len() - 1] as usize - '0' as usize;
    println!(
        "{}{}",
        x.iter().join(""),
        if y < 3 {
            "-"
        } else if y < 7 {
            ""
        } else {
            "+"
        },
    );
}
