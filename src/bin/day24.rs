use structopt::StructOpt;
use shared::{ FileContentOpts };
use std::collections::HashSet;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let all_directions = parse_input(&opts.file);

    // Part 1: Set up the initial black tiles:
    let mut black_tiles = HashSet::new();
    for ds in all_directions {
        let mut x = 0;
        let mut y = 0;
        for d in ds {
            // We assume hexagons are a staggered
            // 2d grid like this, so that we can give
            // standard x and y coords to them:
            //
            //    /\/\/\
            //    | | | |
            //   /\/\/\/
            //   | | | |
            //  /\/\/\/
            //  | | | |
            //  \/\/\/
            //
            match d {
                Direction::E => { x += 1 },
                Direction::SE => { y += 1; x += 1 },
                Direction::SW => { y += 1 },
                Direction::W => { x -= 1 },
                Direction::NW => { y -= 1; x -= 1 },
                Direction::NE => { y -= 1 },
            }
        }
        if !black_tiles.insert((x,y)) {
            black_tiles.remove(&(x,y));
        }
    }
    println!("Star 1: {}", black_tiles.len());

    // Part 2: flip them according to rules:
    for _ in 0..100 {
        black_tiles = step(&black_tiles);
    }
    println!("Star 2: {}", black_tiles.len());

    Ok(())
}

fn surrounding(x: i32, y: i32) -> impl Iterator<Item=(i32,i32)> {
    static OFFSETS: [(i32,i32);6] = [
           (-1,-1),(0,-1),
        (-1, 0),     ( 1, 0),
           ( 0, 1),( 1, 1)
    ];
    OFFSETS.iter().map(move |&(x1,y1)| (x+x1,y+y1))
}

fn step(black_tiles: &HashSet<(i32,i32)>) -> HashSet<(i32,i32)> {
    let mut new_black_tiles = HashSet::new();
    let check_these: HashSet<(i32,i32)> = black_tiles
        .iter()
        .copied()
        .flat_map(|(x,y)| surrounding(x,y))
        .chain(black_tiles.iter().copied())
        .collect();
    for (x,y) in check_these {
        let is_black = black_tiles.contains(&(x,y));
        let surrounding_blacks = surrounding(x,y)
            .filter(|xy| black_tiles.contains(xy))
            .count();
        let new_is_black
            = (is_black && (surrounding_blacks == 1 || surrounding_blacks == 2))
           || (!is_black && surrounding_blacks == 2);
        if new_is_black {
            new_black_tiles.insert((x,y));
        }
    }
    new_black_tiles
}

#[derive(Debug,Clone,Copy)]
enum Direction {
    E, SE, SW, W, NW, NE
}

fn parse_input(s: &str) -> Vec<Vec<Direction>> {
    let mut all_ds = vec![];
    for line in s.trim().lines() {
        let mut ds = vec![];
        let mut rest = line.trim();
        loop {
            if rest.starts_with("ne") {
                rest = &rest[2..];
                ds.push(Direction::NE);
            } else if rest.starts_with("se") {
                rest = &rest[2..];
                ds.push(Direction::SE);
            } else if rest.starts_with("nw") {
                rest = &rest[2..];
                ds.push(Direction::NW);
            } else if rest.starts_with("sw") {
                rest = &rest[2..];
                ds.push(Direction::SW);
            } else if rest.starts_with("e") {
                rest = &rest[1..];
                ds.push(Direction::E);
            } else if rest.starts_with("w") {
                rest = &rest[1..];
                ds.push(Direction::W);
            } else {
                break
            }
        }
        all_ds.push(ds);
    }
    all_ds
}
