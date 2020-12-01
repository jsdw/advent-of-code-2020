use structopt::StructOpt;
use shared::FileContent;
use std::collections::HashSet;

#[derive(StructOpt)]
struct Opts {
    file: FileContent
}

fn main() -> Result<(),anyhow::Error> {
    let opts = Opts::from_args();

    // Find two numbers that sum to 2020:
    let mut ns = HashSet::new();
    for line in opts.file.lines() {
        let n: i64 = line.parse()?;
        let other = 2020 - n;
        if ns.contains(&other) {
            println!("Star 1: {}", n * other)
        } else {
            ns.insert(n);
        }
    }

    Ok(())
}
