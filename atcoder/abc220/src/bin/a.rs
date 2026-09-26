use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
    };
    for i in a..=b {
        if i % c == 0 {
            println!("{i}");
            return;
        }
    }
    println!("-1");
}
