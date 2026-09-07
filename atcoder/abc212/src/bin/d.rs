use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
    };
    let mut heap = BinaryHeap::new();
    let mut f = 0i64;
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                heap.push(!(x - f));
            }
            2 => {
                input! {
                    x: i64,
                }
                f += x;
            }
            3 => {
                let x = !heap.pop().unwrap();
                println!("{}", x + f);
            }
            _ => unreachable!(),
        }
    }
}
