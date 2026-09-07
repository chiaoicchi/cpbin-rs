use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    };
    let ans = if 0 < a && b == 0 {
        "Gold"
    } else if a == 0 && 0 < b {
        "Silver"
    } else {
        "Alloy"
    };
    println!("{}", ans);
}
