use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|i| !a[*i]);
    println!("{}", idx[1] + 1);
}
