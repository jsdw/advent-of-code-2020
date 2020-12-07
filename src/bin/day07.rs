use structopt::StructOpt;
use shared::FileContentOpts;
use shared::regex;
use std::collections::{ HashMap, HashSet };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let rules: Vec<_> = opts.file.lines().filter_map(parse_rule).collect();

    star1(&rules);

    Ok(())
}

fn star1(rules: &[Rule<'_>]) {
    let mut contained_by: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rule in rules {
        for (_,contained_bag) in &rule.contains {
            contained_by
                .entry(contained_bag)
                .or_insert(HashSet::new())
                .insert(rule.bag);
        }
    }

    let mut search_list = vec!["shiny gold"];
    let mut found_colours = HashSet::new();
    while let Some(bag) = search_list.pop() {
        for &item in contained_by.get(bag).unwrap_or(&HashSet::new()).iter() {
            found_colours.insert(item);
            search_list.push(item);
        }
    }

    println!("Star 1: {}", found_colours.len());
}

#[derive(Debug,Clone)]
struct Rule<'a> {
    bag: &'a str,
    contains: Vec<(usize, &'a str)>
}

fn parse_rule(line: &str) -> Option<Rule<'_>> {
    let (bag, contained_str) = {
        let mut s = line.split(" bags contain ");
        (s.next()?, s.next()?)
    };
    let contains = regex!(r"([0-9]+) ([a-z]+ [a-z]+) bag(s)?")
        .captures_iter(contained_str)
        .filter_map(|cap| Some((cap.get(1)?.as_str().parse().ok()?, cap.get(2)?.as_str())))
        .collect();
    Some(Rule { bag, contains })
}
