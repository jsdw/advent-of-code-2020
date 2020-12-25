use structopt::StructOpt;
use shared::{ FileContentOpts };

const DIVISOR: usize = 20201227;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let (pk1,pk2) = parse_input(&opts.file).expect("two numbers");

    let k1 = private_key(pk1,7);
    println!("Star 1: {}", public_key(k1, pk2));

    Ok(())
}

fn public_key(loop_size: usize, subject_number: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val = (val * subject_number) % DIVISOR
    }
    val
}

fn private_key(public_key: usize, subject_number: usize) -> usize {
    let mut loop_size = 0;
    let mut value = public_key;
    while value != 1 {
        loop_size += 1;
        value = step_back_one_iter(value, subject_number);
    }
    loop_size
}

fn step_back_one_iter(public_key: usize, subject_number: usize) -> usize {
    // We could find the first value such that:
    //   value * subject_number % 20201227 == public_key
    // But it's much quicker to find the first X such that:
    //   x * 20201227 + public_key % subject_number == 0
    // And given that x, we can see what the value would be.
    let n_divisors = (0..).find(|x| (DIVISOR * x + public_key) % subject_number == 0).unwrap();
    (DIVISOR * n_divisors + public_key) / subject_number
}

fn parse_input(s: &str) -> Option<(usize,usize)> {
    let mut keys = s.trim().lines().filter_map(|l| l.trim().parse().ok());
    Some((keys.next()?, keys.next()?))
}

