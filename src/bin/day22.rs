use structopt::StructOpt;
use shared::{ FileContentOpts };
use std::collections::{ VecDeque, HashSet };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let (p1, p2) = parse_input(&opts.file).unwrap();

    println!("Star 1: {}", combat(p1.clone(),p2.clone()));
    println!("Star 2: {}", recursive_combat(p1,p2));

    Ok(())
}

fn combat(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> u64 {
    while !p1.is_empty() && !p2.is_empty() {
        let (n1,n2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if n1 > n2 {
            p1.push_back(n1);
            p1.push_back(n2);
        } else {
            p2.push_back(n2);
            p2.push_back(n1);
        }
    }
    let winning_deck = if p1.is_empty() { p2 } else { p1 };
    score(&winning_deck)
}

fn recursive_combat(p1: VecDeque<u8>, p2: VecDeque<u8>) -> u64 {
    let (_,winning_deck) = do_recursive_combat(p1, p2);
    score(&winning_deck)
}

fn do_recursive_combat(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> (bool,VecDeque<u8>) {
    let mut seen = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        // Bail if we've seen this position before:
        if !seen.insert((p1.clone(),p2.clone())) {
            return (true, p1);
        }

        // Pluck cards:
        let (n1,n2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        // Who wins? May involve a recursive game.
        let p1_wins = if p1.len() >= n1 as usize && p2.len() >= n2 as usize {
            let (p1_wins, _) = do_recursive_combat(
                p1.iter().copied().take(n1 as usize).collect(),
                p2.iter().copied().take(n2 as usize).collect()
            );
            p1_wins
        } else {
            n1 > n2
        };

        // Winner gets the cards
        if p1_wins {
            p1.push_back(n1);
            p1.push_back(n2);
        } else {
            p2.push_back(n2);
            p2.push_back(n1);
        }
    }

    let p1_wins = !p1.is_empty();
    let winning_deck = if p1.is_empty() { p2 } else { p1 };
    (p1_wins, winning_deck)
}

fn score(deck: &VecDeque<u8>) -> u64 {
    deck.iter().rev().enumerate().map(|(idx,&n)| (idx+1) as u64 * n as u64).sum()
}

fn parse_input(s: &str) -> Option<(VecDeque<u8>, VecDeque<u8>)> {
    let mut decks = s.trim().split("\n\n");
    let p1 = decks.next()?;
    let p2 = decks.next()?;
    Some((parse_values(p1).collect(), parse_values(p2).collect()))
}

fn parse_values(s: &str) -> impl Iterator<Item=u8> + '_ {
    s.lines().filter_map(|l| l.trim().parse().ok())
}
