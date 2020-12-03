use structopt::StructOpt;
use shared::FileContentOpts;
use std::collections::HashSet;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let map = Map::from_str(&opts.file);

    let trees_seen = count_trees(&map, 3, 1);
    println!("Star 1: {}", trees_seen);

    let trees_seen
        = count_trees(&map, 1, 1)
        * count_trees(&map, 3, 1)
        * count_trees(&map, 5, 1)
        * count_trees(&map, 7, 1)
        * count_trees(&map, 1, 2);
    println!("Star 2: {}", trees_seen);

    Ok(())
}

fn count_trees(map: &Map, right: usize, down: usize) -> usize {
    slope(right, down)
        .take(map.height())
        .filter(|&(x,y)| map.is_tree(x,y))
        .count()
}

fn slope(right: usize, down: usize) -> impl Iterator<Item=(usize,usize)> {
    let mut x = 0;
    let mut y = 0;
    std::iter::from_fn(move || {
        x += right;
        y += down;
        Some((x,y))
    })
}

struct Map {
    inner: HashSet<(usize,usize)>,
    width: usize,
    height: usize
}

impl Map {
    fn from_str(s: &str) -> Map {
        let mut set = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    set.insert((x,y));
                }
                width = usize::max(width, x+1);
                height = usize::max(height, y+1);
            }
        }
        Map {
            inner: set,
            width,
            height
        }
    }
    fn is_tree(&self, x: usize, y: usize) -> bool {
        let x = x % self.width;
        self.inner.contains(&(x,y))
    }
    fn height(&self) -> usize {
        self.height
    }
}