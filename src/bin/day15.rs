use structopt::StructOpt;
use shared::{ FileContentOpts };
use std::collections::HashMap;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let initial_ns: Vec<usize> = opts.file
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("Star 1: {}", get_seen_n(&initial_ns, 2020));
    // This is rather slow (takes a couple of seconds), but what the hey:
    println!("Star 2: {}", get_seen_n(&initial_ns, 30_000_000));

    Ok(())
}

fn get_seen_n(initial_ns: &[usize], at_index: usize) -> usize {
    let mut last_n = *initial_ns.last().unwrap();
    let mut seen: HashMap<_,_> = initial_ns[..initial_ns.len()-1]
        .iter()
        .enumerate()
        .map(|(idx,n)| (*n,idx))
        .collect();
    for idx in seen.len()..at_index-1 {
        let next_n = if let Some(last_idx) = seen.get(&last_n) {
            idx - last_idx
        } else {
            0
        };
        seen.insert(last_n, idx);
        last_n = next_n;
    }
    last_n
}