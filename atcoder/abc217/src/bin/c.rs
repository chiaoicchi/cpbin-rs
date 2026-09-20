use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    };
    let mut ans = vec![0; n];
    for (i, p) in p.iter().enumerate() {
        ans[*p] = i;
    }
    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}
