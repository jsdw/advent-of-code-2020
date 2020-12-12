use structopt::StructOpt;
use shared::{ FileContentOpts, regex };

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();
    let cmds = parse_commands(&opts.file);

    println!("Star 1: {}", part1(&cmds));
    println!("Star 2: {}", part2(&cmds));

    Ok(())
}

fn part1(cmds: &[Command]) -> i64 {
    let mut ship = Ship::new();
    for cmd in cmds {
        ship.apply_command(cmd);
    }
    ship.x.abs() + ship.y.abs()
}

fn part2(cmds: &[Command]) -> i64 {
    let mut ship_and_waypoint = ShipAndWaypoint::new();
    for cmd in cmds {
        ship_and_waypoint.apply_command(cmd);
    }
    ship_and_waypoint.ship_x.abs() + ship_and_waypoint.ship_y.abs()
}

fn parse_commands(s: &str) -> Vec<Command> {
    use Command::*;
    s.lines()
     .filter_map(|l| {
        let caps = regex!(r"^([A-Z])([0-9]+)$").captures(l.trim())?;
        let l = caps.get(1)?.as_str();
        let n = caps.get(2)?.as_str().parse().ok()?;
        match l {
            "N" => Some(N(n)),
            "S" => Some(S(n)),
            "E" => Some(E(n)),
            "W" => Some(W(n)),
            "L" => Some(L(n)),
            "R" => Some(R(n)),
            "F" => Some(F(n)),
            _ => None
        }
     })
     .collect()
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum Command {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn left(&mut self) {
        use Direction::*;
        match self {
            North => { *self = West },
            South => { *self = East },
            East => { *self = North },
            West => { *self = South },
        }
    }
    fn right(&mut self) {
        use Direction::*;
        match self {
            North => { *self = East },
            South => { *self = West },
            East => { *self = South },
            West => { *self = North },
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Ship {
    direction: Direction,
    x: i64,
    y: i64
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: Direction::East,
            x: 0,
            y: 0
        }
    }
    fn apply_command(&mut self, c: &Command) {
        use Command::*;
        match c {
            N(n) => { self.y -= n },
            S(n) => { self.y += n },
            E(n) => { self.x += n },
            W(n) => { self.x -= n },
            L(n) => {
                let n_lefts = n / 90;
                for _ in 0..n_lefts { self.direction.left() }
            },
            R(n) => {
                let n_rights = n / 90;
                for _ in 0..n_rights { self.direction.right() }
            },
            F(n) => {
                use Direction::*;
                match self.direction {
                    North => { self.apply_command(&N(*n)) },
                    South => { self.apply_command(&S(*n)) },
                    East  => { self.apply_command(&E(*n)) },
                    West  => { self.apply_command(&W(*n)) },
                }
            },
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct ShipAndWaypoint {
    ship_x: i64,
    ship_y: i64,
    waypoint_x: i64,
    waypoint_y: i64
}

impl ShipAndWaypoint {
    fn new() -> ShipAndWaypoint {
        ShipAndWaypoint {
            ship_x: 0,
            ship_y: 0,
            waypoint_x: 10,
            waypoint_y: -1
        }
    }
    fn apply_command(&mut self, c: &Command) {
        use Command::*;
        match c {
            N(n) => { self.waypoint_y -= n },
            S(n) => { self.waypoint_y += n },
            E(n) => { self.waypoint_x += n },
            W(n) => { self.waypoint_x -= n },
            L(n) => {
                let n_lefts = n / 90;
                for _ in 0..n_lefts {
                    let (x,y) = (self.waypoint_x, self.waypoint_y);
                    self.waypoint_y = x * -1;
                    self.waypoint_x = y;
                }
            },
            R(n) => {
                let n_rights = n / 90;
                for _ in 0..n_rights {
                    let (x,y) = (self.waypoint_x, self.waypoint_y);
                    self.waypoint_y = x;
                    self.waypoint_x = y * -1;
                }
            },
            F(n) => {
                let delta_x = self.waypoint_x * n;
                let delta_y = self.waypoint_y * n;
                self.ship_x += delta_x;
                self.ship_y += delta_y;
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example_part1() {
        let example = "
            F10
            N3
            F7
            R90
            F11
        ";
        let cmds = parse_commands(example);
        assert_eq!(part1(&cmds), 25);
    }

    #[test]
    fn test_example_part2() {
        let example = "
            F10
            N3
            F7
            R90
            F11
        ";
        let cmds = parse_commands(example);
        assert_eq!(part2(&cmds), 286);
    }

}