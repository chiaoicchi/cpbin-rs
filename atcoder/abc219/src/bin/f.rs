use std::collections::HashSet;

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
        k: usize,
    };

    let (mut x, mut y) = (0i64, 0i64);
    let mut set = HashSet::new();
    set.insert((x, y));
    let mut prev = 0;
    for i in 0..k {
        let mut a = 0;
        for c in s.iter() {
            match *c {
                b'L' => x -= 1,
                b'R' => x += 1,
                b'U' => y -= 1,
                b'D' => y += 1,
                _ => unreachable!(),
            };
            if set.insert((x, y)) {
                a += 1;
            }
        }
        if prev == a {
            let ans = set.len() + (k - 1 - i) * a;
            println!("{ans}");
            return;
        } else {
            prev = a;
        }
    }
    println!("{}", set.len());
}
