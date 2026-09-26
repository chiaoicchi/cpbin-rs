use proconio::input;

fn main() {
    input! {
        x: u8,
    };
    if x < 90 {
        let ans = if x < 40 {
            40 - x
        } else if x < 70 {
            70 - x
        } else {
            90 - x
        };
        println!("{ans}");
    } else {
        println!("expert");
    }
}
