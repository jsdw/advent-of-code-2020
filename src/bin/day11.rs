use structopt::StructOpt;
use shared::{ FileContentOpts, Grid };
use std::convert::TryFrom;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let grid = parse_seats(&opts.file);

    // Star 1: how many occupied seats when stabilises
    let stable_seats = stabilised_seats(4, grid.clone(), num_occupied_part1);
    println!("Star 1: {}", stable_seats.iter().filter(|(_,s)| s.is_occupied()).count());

    // Star 1: how many occupied seats when stabilises (line of sight)
    let stable_seats = stabilised_seats(5, grid, num_occupied_part2);
    println!("Star 2: {}", stable_seats.iter().filter(|(_,s)| s.is_occupied()).count());

    Ok(())
}

fn parse_seats(s: &str) -> Grid<Space> {
    let width = s.trim().lines().next().unwrap().trim().len();
    let seats = s.chars().filter_map(|c| {
        match c {
            '#' => Some(Space::Seat { occupied: true }),
            'L' => Some(Space::Seat { occupied: false }),
            '.' => Some(Space::Empty),
            _ => None
        }
    });
    Grid::from_iter(width, seats)
}

fn stabilised_seats(tolerance: usize, grid: Grid<Space>, get_occupied: fn(usize,usize,&Grid<Space>) -> usize) -> Grid<Space> {
    let mut last_grid = grid;
    loop {
        let next_grid = step_grid(tolerance, &last_grid, get_occupied);
        if next_grid == last_grid { break }
        last_grid = next_grid;
    }
    last_grid
}

fn step_grid(tolerance: usize, grid: &Grid<Space>, get_occupied: fn(usize,usize,&Grid<Space>) -> usize) -> Grid<Space> {
    let it = grid.iter()
        .map(|((x,y), it)| {
            if it.is_empty_space() {
                return *it
            }
            let num_occupied = get_occupied(x,y,grid);
            let was_occupied = it.is_occupied();
            let is_occupied = if !was_occupied && num_occupied == 0 {
                true
            } else if was_occupied && num_occupied >= tolerance {
                false
            } else {
                was_occupied
            };
            Space::Seat {
                occupied: is_occupied
            }
        });
    Grid::from_iter(grid.width(), it)
}

fn num_occupied_part1(x: usize, y: usize, grid: &Grid<Space>) -> usize {
    num_occupied(x, y, 1, grid)
}

fn num_occupied_part2(x: usize, y: usize, grid: &Grid<Space>) -> usize {
    num_occupied(x, y, usize::MAX, grid)
}

fn num_occupied(x: usize, y: usize, n: usize, grid: &Grid<Space>) -> usize {
    [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]
        .iter()
        .map(|&diff| is_los_occupied(x,y,n, grid,diff))
        .filter(|b| *b)
        .count()
}

fn is_los_occupied(x: usize, y: usize, n: usize, grid: &Grid<Space>, (dx,dy): (isize,isize)) -> bool {
    let succ = |&(x,y): &(usize,usize)| Some((
        usize::try_from(x as isize - dx).ok()?,
        usize::try_from(y as isize - dy).ok()?
    ));
    std::iter::successors(Some((x,y)), succ)
        .skip(1)
        .take(n)
        .map(|(x,y)| grid.get(x,y))
        .take_while(|s| s.is_some())
        .flatten()
        .find(|s| s.is_seat())
        .unwrap_or(&Space::Empty)
        .is_occupied()
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum Space {
    Seat { occupied: bool },
    Empty
}

impl Space {
    fn is_occupied(&self) -> bool {
        match self {
            Space::Seat { occupied } => *occupied,
            _ => false
        }
    }
    fn is_seat(&self) -> bool {
        match self {
            Space::Seat{..} => true,
            _ => false
        }
    }
    fn is_empty_space(&self) -> bool {
        match self {
            Space::Empty => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const EXAMPLE_STR: &str = r"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    ";

    #[test]
    fn test_example_part1() {
        let grid = parse_seats(EXAMPLE_STR);
        let stable_seats = stabilised_seats(4, grid, num_occupied_part1);
        assert_eq!(stable_seats.iter().filter(|(_,s)| s.is_occupied()).count(), 37);
    }

    #[test]
    fn test_example_part2() {
        let grid = parse_seats(EXAMPLE_STR);
        let stable_seats = stabilised_seats(5, grid, num_occupied_part2);
        assert_eq!(stable_seats.iter().filter(|(_,s)| s.is_occupied()).count(), 26);
    }

    #[test]
    fn test_occupied_part2() {
        let grid = parse_seats(r"
            L#L
            #.L
            #L#
        ");
        let s = vec![
            ((0,0), 3),
            ((1,0), 1),
            ((2,0), 2),
            ((0,1), 2),
            ((1,1), 4),
            ((2,1), 3),
            ((0,2), 1),
            ((1,2), 4),
            ((2,2), 0),
        ];
        for ((x,y),n) in s {
            assert_eq!(num_occupied_part2(x, y, &grid), n, "left actual, right expected for num occupied at {:?}", (x,y));
        }
    }

    #[test]
    fn test_occupied_example1() {
        let grid = parse_seats(r"
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        ");
        assert_eq!(num_occupied_part2(3, 4, &grid), 8);
    }

}