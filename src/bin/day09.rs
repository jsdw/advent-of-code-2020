use structopt::StructOpt;
use shared::FileContentOpts;
use std::collections::{ HashSet };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let nums: Vec<i64> = opts.file.lines().filter_map(|l| l.parse().ok()).collect();

    let first_invalid = nums
        .windows(26)
        .filter(|w| !is_valid(&w[..25], w[25]))
        .next()
        .unwrap()[25];
    println!("Star 1: {}", first_invalid);

    println!("Star 2: {:?}", part2(&nums, first_invalid));

    Ok(())
}

fn part2(nums: &[i64], val: i64) -> i64 {
    let sum_ups: Vec<i64> = nums
        .iter()
        .scan(0, |sum, n| { *sum += n; Some(*sum) })
        .collect();

    let (i,j) = (0..nums.len()-1)
        .flat_map(|i| (i+1..nums.len()).map(move |j| (i,j)))
        .filter(|&(i,j)| {
            sum_ups[j] - if i == 0 { 0 } else { sum_ups[i-1] } == val
        })
        .next()
        .unwrap();

    let r = &nums[i..=j];
    r.iter().max().unwrap() + r.iter().min().unwrap()
}

fn is_valid(nums: &[i64], val: i64) -> bool {
    let mut s = HashSet::new();
    for &n in nums {
        if val != n && s.contains(&(val - n)) {
            return true
        }
        s.insert(n);
    }
    return false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example_part2() {
        let nums = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];
        assert_eq!(part2(&nums, 127), 62);
    }

}