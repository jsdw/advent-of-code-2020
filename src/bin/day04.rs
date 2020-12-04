use structopt::StructOpt;
use shared::FileContentOpts;
use shared::try_bool;
use shared::regex;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let parts: Vec<_> = opts.file.split("\n\n").filter_map(PassportParts::from_str).collect();

    let num_valid = parts.iter().filter(|p| p.is_valid_part1()).count();
    println!("Star 1: {}", num_valid);

    let num_valid = parts.iter().filter(|p| p.is_valid_part2()).count();
    println!("Star 2: {}", num_valid);

    Ok(())
}

#[derive(Default)]
struct PassportParts<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: &'a str,
}

impl <'a> PassportParts<'a> {
    fn from_str(s: &'a str) -> Option<PassportParts<'a>> {
        let mut p = PassportParts::default();
        for part in s.split_whitespace() {
            let mut bits = part.trim().split(':');
            let field = bits.next()?;
            let val = bits.next()?;
            match field {
                "byr" => { p.byr = val },
                "iyr" => { p.iyr = val },
                "eyr" => { p.eyr = val },
                "hgt" => { p.hgt = val },
                "hcl" => { p.hcl = val },
                "ecl" => { p.ecl = val },
                "pid" => { p.pid = val },
                "cid" => { p.cid = val },
                _     => { /* ignore invalid */ }
            }
        }
        Some(p)
    }
    fn is_valid_part1(&self) -> bool {
        if self.byr.is_empty() { return false }
        if self.iyr.is_empty() { return false }
        if self.eyr.is_empty() { return false }
        if self.hgt.is_empty() { return false }
        if self.hcl.is_empty() { return false }
        if self.ecl.is_empty() { return false }
        if self.pid.is_empty() { return false }
        true
    }
    fn is_valid_part2(&self) -> bool {
        let byr: u16 = try_bool!(self.byr.parse());
        try_bool!(byr >= 1920 && byr <= 2002);
        let iyr: u16 = try_bool!(self.iyr.parse());
        try_bool!(iyr >= 2010 && iyr <= 2020);
        let eyr: u16 = try_bool!(self.eyr.parse());
        try_bool!(eyr >= 2020 && eyr <= 2030);
        try_bool!(height_is_valid(&self.hgt));
        try_bool!(hair_is_valid(&self.hcl));
        try_bool!(eye_is_valid(&self.ecl));
        try_bool!(self.pid.len() == 9);
        try_bool!(self.pid.parse::<u64>());
        true
    }
}

fn height_is_valid(s: &str) -> bool {
    if let Some(n) = s.strip_suffix("cm") {
        let n: u16 = try_bool!(n.parse());
        n >= 150 && n <= 193
    } else if let Some(n) = s.strip_suffix("in") {
        let n: u16 = try_bool!(n.parse());
        n >= 59 && n <= 76
    } else {
        false
    }
}

fn hair_is_valid(s: &str) -> bool {
    regex!(r"^#[0-9a-f]{6}$").is_match(s)
}

fn eye_is_valid(s: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .find(|&&c| c == s)
        .is_some()
}