use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        p: [Usize1; 26],
    };
    println!(
        "{}",
        p.iter().map(|i| ('a' as u8 + *i as u8) as char).join("")
    );
}
