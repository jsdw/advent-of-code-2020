use structopt::StructOpt;
use shared::FileContentOpts;
use shared::regex;
use std::collections::{ HashMap, HashSet };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let rules: Vec<_> = opts.file.lines().filter_map(parse_rule).collect();

    println!("Star 1: {}", star1(&rules));
    println!("Star 2: {}", star2(&rules));

    Ok(())
}

fn star1(rules: &[Rule<'_>]) -> usize {
    let mut contained_by: HashMap<&str, HashSet<&str>> = HashMap::new();
    for rule in rules {
        for (_,contained_bag) in &rule.contains {
            contained_by
                .entry(contained_bag)
                .or_insert(HashSet::new())
                .insert(rule.bag);
        }
    }

    let mut found_colours = HashSet::new();
    let mut search_list = vec!["shiny gold"];
    while let Some(bag) = search_list.pop() {
        for &item in contained_by.get(bag).unwrap_or(&HashSet::new()).iter() {
            found_colours.insert(item);
            search_list.push(item);
        }
    }

    found_colours.len()
}

fn star2(rules: &[Rule<'_>]) -> usize {
    let contains: HashMap<&str, &[(usize, &str)]> = rules
        .iter()
        .map(|r| (r.bag, &*r.contains))
        .collect();

    let mut bag_count = 0;
    let mut search_list = vec![(1, "shiny gold")];
    while let Some((n,bag)) = search_list.pop() {
        bag_count += n;
        if let Some(&contained) = contains.get(bag) {
            for (contained_n, contained_bag) in contained {
                search_list.push((contained_n * n, contained_bag));
            }
        }
    }

    bag_count - 1
}

#[derive(Debug,Clone)]
struct Rule<'a> {
    bag: &'a str,
    contains: Vec<(usize, &'a str)>
}

fn parse_rule(line: &str) -> Option<Rule<'_>> {
    let (bag, contained_str) = {
        let mut s = line.split(" bags contain ");
        (s.next()?.trim(), s.next()?.trim())
    };
    let contains = regex!(r"([0-9]+) ([a-z]+ [a-z]+) bag(s)?")
        .captures_iter(contained_str)
        .filter_map(|cap| Some((cap.get(1)?.as_str().parse().ok()?, cap.get(2)?.as_str())))
        .collect();
    Some(Rule { bag, contains })
}

#[cfg(test)]
mod test {

    use super::*;

    const EXAMPLE1: &str = r"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
    ";

    const EXAMPLE2: &str = r"
        shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.
    ";

    fn rules(s: &str) -> Vec<Rule<'_>> {
        s.lines().filter_map(parse_rule).collect()
    }

    #[test]
    fn test_example1_parsing() {
        let rules = rules(EXAMPLE1);
        assert_eq!(rules.len(), 9);
        let counts: usize = rules.into_iter().map(|r| r.contains.iter().map(|c| c.0).sum::<usize>()).sum();
        assert_eq!(counts, 1 + 2 + 3 + 4 + 1 + 2 + 9 + 1 + 2 + 3 + 4 + 5 + 6);
    }

    #[test]
    fn test_example1_star1() {
        let rules = rules(EXAMPLE1);
        assert_eq!(star1(&rules), 4);
    }

    #[test]
    fn test_example1_star2() {
        let rules = rules(EXAMPLE1);
        assert_eq!(star2(&rules), 32);
    }

    #[test]
    fn test_example2_star2() {
        let rules = rules(EXAMPLE2);
        assert_eq!(star2(&rules), 126);
    }

}