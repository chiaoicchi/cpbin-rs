use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n < 126 {
        4
    } else if n < 212 {
        6
    } else {
        8
    };
    println!("{ans}");
}
