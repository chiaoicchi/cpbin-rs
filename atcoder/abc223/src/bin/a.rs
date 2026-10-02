use proconio::input;

fn main() {
    input! {
        x: u16,
    };
    println!("{}", if x > 0 && x % 100 == 0 { "Yes" } else { "No" });
}
