use structopt::StructOpt;
use shared::{ FileContentOpts, regex };
use std::collections::{ HashMap, HashSet };
use std::iter;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let (mut rules, strings) = parse_input(&opts.file).unwrap();

    let num_matches: usize = strings.iter().filter(|s| str_matches_rule(s, 0, &rules).contains("")).count();
    println!("Star 1: {}", num_matches);

    rules.insert(8, Rule::OneOrMore(42));
    rules.insert(11, Rule::SameNumberOf(42, 31));

    let num_matches: usize = strings.iter().filter(|s| str_matches_rule(s, 0, &rules).contains("")).count();
    println!("Star 2: {}", num_matches);

    Ok(())
}

fn str_matches_rule<'a>(s: &'a str, idx: usize, rules: &HashMap<usize,Rule>) -> HashSet<&'a str> {
    match rules.get(&idx).unwrap() {
        Rule::List(idxs) => {
            str_matches_rules(s, &idxs, rules)
        },
        Rule::Or(idxsa, idxsb) => {
            let a = str_matches_rules(s, &idxsa, rules);
            if a.is_empty() {
                str_matches_rules(s, &idxsb, rules)
            } else {
                a
            }
        },
        Rule::Char(c) => {
            if s.as_bytes().get(0) == Some(&(*c as u8)) {
                iter::once(&s[1..]).collect()
            } else {
                HashSet::new()
            }
        },
        // Special handling for part 2: match one or more of a rule
        Rule::OneOrMore(idx) => {
            str_matches_rule_ntimes(s, *idx, rules)
                .into_iter()
                .map(|(_,s)| s)
                .collect()
        },
        // Special handling for part 2: match one or more of a rule,
        // followed by that same number of another rule
        Rule::SameNumberOf(idxa, idxb) => {
            let mut out = HashSet::new();
            for (times, s) in str_matches_rule_ntimes(s, *idxa, rules) {
                let next_rules: Vec<_> = iter::repeat(*idxb).take(times).collect();
                for s in str_matches_rules(s, &next_rules, rules) {
                    out.insert(s);
                }
            }
            out
        }
    }
}

fn str_matches_rules<'a>(s: &'a str, idxs: &[usize], rules: &HashMap<usize,Rule>) -> HashSet<&'a str> {
    let mut curr: HashSet<_> = iter::once(s).collect();
    for &idx in idxs {
        let mut next = HashSet::new();
        for s in curr {
            for m in str_matches_rule(s, idx, rules) {
                next.insert(m);
            }
        }
        curr = next;
    }
    curr
}

fn str_matches_rule_ntimes<'a>(s: &'a str, idx: usize, rules: &HashMap<usize,Rule>) -> HashSet<(usize, &'a str)> {
    let mut output = HashSet::new();
    let mut to_visit: HashSet<_> = iter::once(s).collect();
    let mut times = 0;
    loop {
        times += 1;
        let mut n = 0;
        let mut to_visit_next = HashSet::new();
        for s in to_visit {
            for m in str_matches_rule(s, idx, rules) {
                output.insert((times,m));
                to_visit_next.insert(m);
                n += 1;
            }
        }
        to_visit = to_visit_next;
        if n == 0 { break }
    }
    output
}

#[derive(Debug,Clone,PartialEq,Eq)]
enum Rule {
    List(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
    Char(char),
    // Special rules to cater for part 2 tweaks:
    OneOrMore(usize),
    SameNumberOf(usize, usize)
}

fn parse_input(s: &str) -> Option<(HashMap<usize,Rule>, Vec<&str>)> {
    let mut rules = HashMap::new();
    let mut strings = Vec::new();
    for line in s.lines() {
        if let Some(caps) = regex!(r"(\d+): (.*)").captures(line) {
            let idx = caps[1].parse().ok()?;
            let rule = parse_rule_fragment(&caps[2])?;
            rules.insert(idx, rule);
        } else if regex!(r"[ab]+").is_match(line) {
            strings.push(line.trim());
        }
    }
    Some((rules, strings))
}

fn parse_rule_fragment(s: &str) -> Option<Rule> {
    if let Some((a,b)) = parse_or_fragment(s) {
        Some(Rule::Or(a,b))
    } else if let Some(a) = parse_list_fragment(s) {
        Some(Rule::List(a))
    } else if let Some(a) = parse_char_fragment(s) {
        Some(Rule::Char(a))
    } else {
        None
    }
}

fn parse_or_fragment(s: &str) -> Option<(Vec<usize>,Vec<usize>)> {
    let mut or_parts = s.split('|');
    let (a,b) = (or_parts.next()?, or_parts.next()?);
    Some((parse_list_fragment(a)?, parse_list_fragment(b)?))
}

fn parse_list_fragment(s: &str) -> Option<Vec<usize>> {
    s.trim().split(' ').map(|n| n.parse().ok()).collect()
}

fn parse_char_fragment(s: &str) -> Option<char> {
    Some(regex!(r#""([ab])""#).captures(s)?[1].chars().next()?)
}