use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u8,
        a: [u8; n],
    };
    println!("{}", a.iter().filter(|a| **a < p).count());
}
