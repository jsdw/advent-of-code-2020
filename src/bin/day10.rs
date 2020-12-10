use structopt::StructOpt;
use shared::FileContentOpts;
use std::collections::HashMap;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let mut nums: Vec<i64> = opts.file.lines().filter_map(|l| l.parse().ok()).collect();
    let device_joltage = nums.iter().max().unwrap() + 3;
    nums.push(device_joltage);
    nums.sort();

    let mut diff1 = 0;
    let mut diff3 = 0;
    for (a,b) in std::iter::once(&0).chain(nums.iter()).zip(nums.iter()) {
        if b-a == 1 {
            diff1 += 1;
        } else if b-a == 3 {
            diff3 += 1;
        }
    }
    println!("Star 1: {}", diff1 * diff3);

    let mut seen_counts: HashMap<i64, usize> = HashMap::new();
    seen_counts.insert(0, 1);
    for n in nums {
        let c
            = seen_counts.get(&(n-1)).unwrap_or(&0)
            + seen_counts.get(&(n-2)).unwrap_or(&0)
            + seen_counts.get(&(n-3)).unwrap_or(&0);
        seen_counts.insert(n, c);
    }
    println!("Star 2: {}", seen_counts.get(&device_joltage).unwrap());

    Ok(())
}