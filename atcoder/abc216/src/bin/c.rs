use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: u64,
    };
    let mut ans = vec![];
    while n > 0 {
        if n & 1 == 1 {
            ans.push('A');
        }
        ans.push('B');
        n /= 2;
    }
    ans.pop();
    ans.reverse();
    println!("{}", ans.iter().join(""));
}
