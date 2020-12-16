use structopt::StructOpt;
use shared::{ FileContentOpts, regex };
use std::{collections::{ HashMap, HashSet }, ops::RangeInclusive};

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let input = Input::from_str(&opts.file).unwrap();

    let all_ranges: Vec<_> = input.rules
        .iter()
        .flat_map(|(_,ranges)| ranges.to_vec())
        .collect();

    let scanning_error_rate: usize = input.nearby_tickets
        .iter()
        .flatten()
        .filter(|n| !all_ranges.iter().any(|r| r.contains(n)))
        .sum();
    println!("Star 1: {}", scanning_error_rate);

    // We assume based on the text that if a ticket's values all fall within
    // _one of_ the ranges, that the ticket is valid and _all of_ its values
    // will, taken together, fall into a separate rule's range.
    let valid_nearby_tickets: Vec<_> = input.nearby_tickets
        .into_iter()
        .filter(|ns| ns.iter().all(|n| all_ranges.iter().any(|r| r.contains(n))))
        .collect();

    // Step 1: find which indexes are valid for each rule based on the nearby tickets.
    let valid_idxs_per_rule: Vec<_> = input.rules.into_iter().map(|(name,ranges)| {
        let len = valid_nearby_tickets[0].len();
        let valid_idxs: HashSet<usize> = (0..len)
            .filter(|&idx| {
                valid_nearby_tickets
                    .iter()
                    .all(|ns| ranges.iter().any(|r| r.contains(&ns[idx])))
            })
            .collect();
        (name, valid_idxs)
    }).collect();

    // Step 2: iteratively reduce the overlapping rules by filtering indexes that are the
    // only valid ones for a given rule. This will solve or come close.
    let mut final_idxs = HashMap::new();
    let mut valid_idxs = valid_idxs_per_rule;
    while let Some((name,idxs)) = valid_idxs.iter().find(|(_,idxs)| idxs.len() == 1) {
        let name = name.clone();
        let idx = *idxs.iter().next().unwrap();
        valid_idxs = valid_idxs
            .into_iter()
            .filter(|(n,_)| n != &name)
            .map(|(n,mut idxs)| { idxs.retain(|&i| i != idx); (n,idxs) })
            .collect();
        final_idxs.insert(name, idx);
    }

    // Step 3: it might be that we'd need to find a valid combination given any remaining overlaps,
    // but the results suggest no remaining overlap, so we go ahead and find the answer.
    let my_ticket = input.my_ticket;
    let departure_values: usize = final_idxs
        .into_iter()
        .filter(|(n,_)| n.starts_with("departure"))
        .map(|(_,idx)| my_ticket[idx])
        .product();
    println!("Star 2: {}", departure_values);

    Ok(())
}

#[derive(Debug,Clone)]
struct Input {
    rules: Vec<(String, [RangeInclusive<usize>; 2])>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>
}

impl Input {
    fn from_str(s: &str) -> Option<Input> {
        let mut sections = s.split("\n\n");

        let rules = regex!(r"([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)")
            .captures_iter(sections.next()?)
            .filter_map(|caps| {
                Some((
                    caps[1].to_owned(),
                    [ RangeInclusive::new(caps[2].parse().ok()?, caps[3].parse().ok()?)
                    , RangeInclusive::new(caps[4].parse().ok()?, caps[5].parse().ok()?) ]
                ))
            })
            .collect();

        let my_ticket = sections.next()?
            .lines()
            .filter_map(parse_ns)
            .next()?;

        let nearby_tickets = sections.next()?
            .lines()
            .filter_map(parse_ns)
            .collect();

        Some(Input { rules, my_ticket, nearby_tickets })
    }
}

fn parse_ns(s: &str) -> Option<Vec<usize>> {
    let ns: Vec<_> = s.split(",").filter_map(|n| n.parse().ok()).collect();
    if ns.len() > 0 { Some(ns) } else { None }
}