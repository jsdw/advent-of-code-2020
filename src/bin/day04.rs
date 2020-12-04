use structopt::StructOpt;
use shared::FileContentOpts;
use shared::try_bool;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let parts: Vec<_> = opts.file.split("\n\n").filter_map(PassportParts::from_str).collect();

    let num_valid = parts.iter().filter(|p| p.is_valid_part1()).count();
    println!("Star 1: {}", num_valid);

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
        let byr: usize = try_bool!(self.byr.parse());
        // if byr < 1920 || byr > 2002 {  }
        false
    }
}
