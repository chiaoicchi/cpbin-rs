use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    };
    let mut set = HashSet::new();
    set.insert("ABC".to_string());
    set.insert("ARC".to_string());
    set.insert("AGC".to_string());
    set.insert("AHC".to_string());
    for s in s.iter() {
        set.remove(s);
    }
    println!("{}", set.iter().next().unwrap());
}
