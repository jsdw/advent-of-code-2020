use structopt::StructOpt;
use shared::FileContentOpts;
use std::collections::HashSet;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let nums: Vec<i64> = opts.file.lines().filter_map(|s| s.parse().ok()).collect();

    // Find two numbers that sum to 2020:
    let mut seen = HashSet::new();
    for &n in nums.iter() {
        let other = 2020 - n;
        if seen.contains(&other) {
            println!("Star 1: {}", n * other);
            break
        } else {
            seen.insert(n);
        }
    }

    // No cleverness here
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i]+nums[j]+nums[k] == 2020 {
                    println!("Star 2: {}", nums[i]*nums[j]*nums[k]);
                    break
                }
            }
        }
    }

    Ok(())
}
