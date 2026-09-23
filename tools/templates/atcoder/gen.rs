use rand_gen::*;

fn main() {
    let seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut rng = Rng::new(seed);

    let n = rng.range(1, 1 << 20) as usize;
}
