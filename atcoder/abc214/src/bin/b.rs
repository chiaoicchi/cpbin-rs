use proconio::input;

fn main() {
    input! {
        s: u64,
        t: u64,
    };

    let mut ans = 0usize;
    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    ans += 1;
                }
            }
        }
    }
    println!("{ans}");
}
