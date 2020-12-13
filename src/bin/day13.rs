use structopt::StructOpt;
use shared::{ FileContentOpts };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let (n, ids) = parse_input(&opts.file).unwrap();

    println!("Star 1: {}", part1(n, &ids));
    println!("Star 2: {}", part2(ids));

    Ok(())
}

fn part1(n: i64, ids: &[BusTime]) -> i64 {
    let (offset,id) = ids
        .iter()
        .map(|t| ((t.step - (n % t.step)) % t.step, t.step))
        .min_by_key(|&(t,_)| t)
        .unwrap();
    offset * id
}

fn part2(nums: Vec<BusTime>) -> i64 {
    // Iteratively work out a common offset and common step that
    // represents each pair of bus times. Minus the offset (id) from
    // the final step to get t=0.
    let fst = nums[0];
    let combined = nums
        .iter()
        .skip(1)
        .fold(fst, |a, &b| combine_bus_times(a,b));

    combined.step - combined.idx
}

// Take 2 bus times and output a bus time that will overlap
// in the same places as the inputs.
fn combine_bus_times(a: BusTime, b: BusTime) -> BusTime {
    let new_step = lcm(a.step, b.step);
    let t = find_t(a,b);
    BusTime { idx: new_step - t, step: new_step }
}

// Given two pairs of BusTimes, find the first
// number that they align on, if we assume that busses
// first leave at -idx.
fn find_t(a: BusTime, b: BusTime) -> i64 {
    let (a, b) = (a.max(b), a.min(b));
    std::iter::successors(Some(a.step-a.idx), |n| Some(n + a.step))
        .filter(|&n| n >= 0)
        .find(|a_val| (a_val + b.idx) % b.step == 0)
        .unwrap()
}

// Least common multiple. Find the smallest
// number that both a and b divide into.
fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 { return 0 }
    (a * b).abs() / gcd(a,b)
}

// Greatest common divisor. Find the largest
// number that you can divide a and b by.
fn gcd(a: i64, b: i64) -> i64 {
    // the euclidean algorithm
    if a == 0 { return b }
    if b == 0 { return a }
    let (a, b) = (a.max(b), a.min(b));
    let r = a % b;
    gcd(b, r)
}

fn parse_input(s: &str) -> Option<(i64, Vec<BusTime>)> {
    let mut lines = s.lines();
    let n = lines.next()?.parse().ok()?;
    let ids = parse_times(lines.next()?);
    Some((n, ids))
}

fn parse_times(s: &str) -> Vec<BusTime> {
    s.trim()
     .split(',')
     .map(|n| n.trim().parse().ok())
     .enumerate()
     .filter_map(|(idx,id)| id.map(|id| BusTime { idx: idx as i64, step: id }))
     .collect()
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct BusTime {
    step: i64,
    idx: i64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_lcm() {
        let cases = vec![
            (21,6,42)
        ];
        for (a,b,res) in cases {
            assert_eq!(lcm(a,b), res, "lcd({},{}) should eq {}", a, b, res);
        }
    }

    #[test]
    fn test_gcd() {
        let cases = vec![
            (21,6,3),
            (270,192,6)
        ];
        for (a,b,res) in cases {
            assert_eq!(gcd(a,b), res, "gcd({},{}) should eq {}", a, b, res);
        }
    }

    #[test]
    fn test_finding_t() {
        let cases = vec![
            (BusTime { step: 20, idx: 0 }, BusTime { step: 6, idx: 4 }, 20),
            (BusTime { step: 30, idx: 10 }, BusTime { step: 6, idx: 4 }, 20),
            (BusTime { step: 7, idx: 5 }, BusTime { step: 10, idx: 35 }, 65),
            (BusTime { step: 6, idx: 3 }, BusTime { step: 10, idx: 35 }, 15),
        ];
        for (a,b,res) in cases {
            assert_eq!(find_t(a,b), res, "first overlap of {:?} and {:?}", a, b);
        }
    }

    #[test]
    fn test_combine_bus_times() {
        let cases = vec![
            (BusTime { step: 20, idx: 0 }, BusTime { step: 6, idx: 4 }, BusTime { step: 60, idx: 40 }),
        ];
        for (a,b,res) in cases {
            assert_eq!(combine_bus_times(a,b), res, "combination of {:?} and {:?}", a, b);
        }
    }

    #[test]
    fn test_part2_examples() {
        let examples = vec![
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("7,13,x,x,59,x,31,19", 1068781),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ];
        for (i,res) in examples {
            let ids = parse_times(i);
            assert_eq!(part2(ids), res, "input was {}", i);
        }
    }

}