use structopt::StructOpt;
use shared::{ FileContentOpts };
use itertools::Itertools;
use cups::Cups;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let input = parse_input(&opts.file);

    let mut cups = Cups::new(input.len(), input.clone());
    for _ in 0..100 {
        cups.step();
    }
    println!("Star 1: {}", cups.next_after(1).take(8).join(""));

    let mut cups = Cups::new(1_000_000, input.clone());
    for _ in 0..10_000_000 {
        cups.step();
    }
    println!("Star 2: {}", cups.next_after(1).take(2).product::<usize>());

    Ok(())
}

fn parse_input(s: &str) -> Vec<usize> {
    s.as_bytes()
        .iter()
        .filter_map(|b| b.checked_sub(48))
        .map(|n| n as usize)
        .collect()
}

mod cups {

    use std::iter::successors;

    pub struct Cups {
        /// Each index corresponds to the cup_number. The
        /// value stored corresponds to the index of the
        /// next cup in the sequence
        vec: Vec<usize>,
        /// This is the index into the vec of the current
        /// cup.
        current_index: usize
    }

    impl Cups {
        /// Given a starting arrangement, give back some Cups. Starting order must
        /// contain every number from 1 to starting_order.len(); the rest will be
        /// filled in order.
        pub fn new(len: usize, starting_order: Vec<usize>) -> Cups {
            let mut v = vec![0; len+1];
            let padding_start = starting_order.iter().copied().max().unwrap_or(0) + 1;
            let order = starting_order.iter().copied().chain(padding_start..len+1);
            for (n, next_n) in order.clone().zip(order.cycle().skip(1)).take(len) {
                v[n] = next_n;
            }
            Cups {
                vec: v,
                current_index: starting_order[0]
            }
        }
        /// Take one turn from the current position:
        pub fn step(&mut self) {
            // Take 3 cups clockwise of current:
            let taken_indexes: Vec<usize> = self.next_after(self.current_index).take(3).collect();
            // Find idx of cup to put them in front of:
            let mut next_idx = self.minus_one_cup(self.current_index);
            while taken_indexes.contains(&next_idx) {
                next_idx = self.minus_one_cup(next_idx);
            }
            // The current index now points to the thing after the last taken cup:
            self.vec[self.current_index] = self.vec[*taken_indexes.last().unwrap()];
            // Last taken index now points to what the next_index used to:
            self.vec[*taken_indexes.last().unwrap()] = self.vec[next_idx];
            // Next index now points to the first taken cup:
            self.vec[next_idx] = taken_indexes[0];
            // Current index is now the next cup around:
            self.current_index = self.vec[self.current_index];
        }
        /// Return an iterator over the next cups in line from the number given:
        pub fn next_after(&self, cup_idx: usize) -> impl Iterator<Item=usize> + '_ {
            successors(Some(cup_idx), move |idx| Some(self.vec[*idx])).skip(1)
        }
        /// Minus one from the cup number to get the previous one.
        fn minus_one_cup(&self, n: usize) -> usize {
            let num_cups = self.vec.len() - 1;
            (n + (num_cups - 1) - 1) % num_cups + 1
        }
    }

}
