use rand_gen::*;

fn main() {
    let seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut rng = Rng::new(seed);

    let n = rng.range(3, 4) as usize;
    let d = rng.range(1, 4) as usize;
    println!("{n}");
    println!("{d}");
}
