use rand_gen::*;

fn main() {
    let seed: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut rng = Rng::new(seed);

    let n = rng.range(1, 6) as usize;
    let a = values(&mut rng, n, 0, 10);
    println!("{n}");
    println!(
        "{}",
        a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
    );
}
