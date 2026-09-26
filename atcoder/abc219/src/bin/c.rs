use itertools::Itertools;
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        x: Bytes,
        n: usize,
        s: [Bytes; n],
    };
    let mut ans: Vec<usize> = (0..n).collect();
    let mut ix = vec![0; 26];
    for (i, x) in x.iter().enumerate() {
        let x = (x - b'a') as usize;
        ix[x] = i;
    }
    ans.sort_unstable_by_key(|i| {
        s[*i]
            .iter()
            .map(|c| ix[(c - b'a') as usize])
            .collect::<Vec<_>>()
    });
    println!(
        "{}",
        ans.iter()
            .map(|i| s[*i].iter().map(|c| *c as char).collect::<String>())
            .join("\n")
    );
}
