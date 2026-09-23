use std::collections::{BinaryHeap, VecDeque};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
        };
        match t {
            1 => {
                input! {
                    x: u32,
                };
                queue.push_back(x);
            }
            2 => {
                if let Some(x) = heap.pop() {
                    println!("{}", !x);
                } else {
                    let x = queue.pop_front().unwrap();
                    println!("{}", x);
                }
            }
            3 => {
                while let Some(x) = queue.pop_front() {
                    heap.push(!x);
                }
            }
            _ => unreachable!(),
        }
    }
}
