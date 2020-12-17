use structopt::StructOpt;
use shared::{ FileContentOpts };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let mut cube = cube::parse_input(&opts.file);
    for _ in 0..6 {
        cube = cube::step(&cube);
    }
    println!("Star 1: {}", cube.len());

    let mut hypercube = hypercube::parse_input(&opts.file);
    for _ in 0..6 {
        hypercube = hypercube::step(&hypercube);
    }
    println!("Star 2: {}", hypercube.len());

    Ok(())
}

mod cube {
    type Cube = std::collections::HashSet<(i32,i32,i32)>;

    pub fn parse_input(s: &str) -> Cube {
        let mut cube = Cube::new();
        for (y,line) in s.trim().lines().enumerate() {
            for (x,b) in line.trim().bytes().enumerate() {
                if b == b'#' {
                    cube.insert((x as i32, y as i32, 0));
                }
            }
        }
        cube
    }

    pub fn step(cube: &Cube) -> Cube {
        let locs_to_check: Cube = cube
            .iter()
            .flat_map(|&c| std::iter::once(c).chain(surrounding(c)))
            .collect();
        let mut new_cube = Cube::new();
        for loc in locs_to_check {
            let is_active = cube.contains(&loc);
            let num_active_surrounding = surrounding(loc).filter(|loc| cube.contains(loc)).count();
            let is_new_active = if !is_active && num_active_surrounding == 3 {
                true
            } else if is_active && (num_active_surrounding == 2 || num_active_surrounding == 3) {
                true
            } else {
                false
            };
            if is_new_active {
                new_cube.insert(loc);
            }
        }
        new_cube
    }

    fn surrounding((x,y,z): (i32,i32,i32)) -> impl Iterator<Item=(i32,i32,i32)> {
        use itertools::iproduct;
        iproduct!(-1..=1,-1..=1,-1..=1)
            .filter(|&c| c != (0,0,0))
            .map(move |(x1,y1,z1)| (x-x1,y-y1,z-z1))
    }
}

mod hypercube {
    type HyperCube = std::collections::HashSet<(i32,i32,i32,i32)>;

    pub fn parse_input(s: &str) -> HyperCube {
        let mut cube = HyperCube::new();
        for (y,line) in s.trim().lines().enumerate() {
            for (x,b) in line.trim().bytes().enumerate() {
                if b == b'#' {
                    cube.insert((x as i32, y as i32, 0, 0));
                }
            }
        }
        cube
    }

    pub fn step(cube: &HyperCube) -> HyperCube {
        let locs_to_check: HyperCube = cube
            .iter()
            .flat_map(|&c| std::iter::once(c).chain(surrounding(c)))
            .collect();
        let mut new_cube = HyperCube::new();
        for loc in locs_to_check {
            let is_active = cube.contains(&loc);
            let num_active_surrounding = surrounding(loc).filter(|loc| cube.contains(loc)).count();
            let is_new_active = if !is_active && num_active_surrounding == 3 {
                true
            } else if is_active && (num_active_surrounding == 2 || num_active_surrounding == 3) {
                true
            } else {
                false
            };
            if is_new_active {
                new_cube.insert(loc);
            }
        }
        new_cube
    }

    fn surrounding((x,y,z,a): (i32,i32,i32,i32)) -> impl Iterator<Item=(i32,i32,i32,i32)> {
        use itertools::iproduct;
        iproduct!(-1..=1,-1..=1,-1..=1,-1..=1)
            .filter(|&c| c != (0,0,0,0))
            .map(move |(x1,y1,z1,a1)| (x-x1,y-y1,z-z1,a-a1))
    }
}