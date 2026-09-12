use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut ab: [(Usize1, Usize1); n],
    };
    let mut a: Vec<usize> = ab.iter().map(|x| x.0).collect();
    let mut b: Vec<usize> = ab.iter().map(|x| x.1).collect();
    a.sort();
    b.sort();
    a.dedup();
    b.dedup();
    for i in 0..n {
        let x = a.partition_point(|k| *k < ab[i].0);
        let y = b.partition_point(|k| *k < ab[i].1);
        println!("{} {}", x + 1, y + 1);
    }
}
