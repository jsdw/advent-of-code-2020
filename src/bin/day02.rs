use structopt::StructOpt;
use shared::FileContentOpts;
use shared::regex;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let lines: Vec<InputLine> = opts.file.lines().filter_map(InputLine::from_str).collect();

    let mut valid = 0;
    for line in &lines {
        let n = line.pass.chars().fold(0, |acc, c| if c == line.letter { acc + 1 } else { acc });
        if n >= line.low && n <= line.high { valid += 1 }
    }
    println!("Star 1: {}", valid);

    let mut valid = 0;
    for line in &lines {
        let mut n = 0;
        if line.pass.chars().nth(line.low - 1).unwrap() == line.letter { n += 1 };
        if line.pass.chars().nth(line.high - 1).unwrap() == line.letter { n += 1 };
        if n == 1 { valid += 1 }
    }
    println!("Star 2: {}", valid);

    Ok(())
}

#[derive(Debug)]
pub struct InputLine {
    pub low: usize,
    pub high: usize,
    pub letter: char,
    pub pass: String
}

impl InputLine {
    fn from_str(s: &str) -> Option<InputLine> {
        let caps = regex!(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").captures(s)?;
        Some(InputLine {
            low: caps.get(1)?.as_str().parse().ok()?,
            high: caps.get(2)?.as_str().parse().ok()?,
            letter: caps.get(3)?.as_str().parse().ok()?,
            pass: caps.get(4)?.as_str().to_owned()
        })
    }
}