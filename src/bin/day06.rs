use structopt::StructOpt;
use shared::FileContentOpts;
use std::collections::HashSet;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let any_yes: usize = opts.file
        .split("\n\n")
        .map(to_set)
        .map(|h| h.len())
        .sum();
    println!("Star 1: {}", any_yes);

    let all_yes: usize = opts.file
        .split("\n\n")
        .map(|s| {
            s.lines()
             .map(to_set)
             .fold(all_set(), |a, b| a.intersection(&b).cloned().collect())
             .len()
        })
        .sum();
    println!("Star 2: {}", all_yes);

    Ok(())
}

fn all_set() -> HashSet<u8> {
    (b'a' ..= b'z').collect()
}

fn to_set(s: &str) -> HashSet<u8> {
    s.bytes().filter(|&b| b >= b'a' && b <= b'z').collect()
}