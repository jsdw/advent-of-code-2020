use structopt::StructOpt;
use shared::FileContentOpts;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let mut seat_ids: Vec<usize> = opts.file
        .lines()
        .map(Seat::from_str)
        .map(|s| s.id())
        .collect();

    seat_ids.sort();

    let biggest_id = seat_ids.last().unwrap();
    println!("Star 1: {}", biggest_id);

    let (prev_id, next_id) = seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .filter(|&(&a,&b)| a + 1 != b)
        .next()
        .unwrap();
    println!("Star 2: {} (between {} and {})", prev_id + 1, prev_id, next_id);

    Ok(())
}

#[derive(Clone,Copy)]
struct Seat { row: usize, col: usize }

impl Seat {
    fn from_str(s: &str) -> Seat {
        let mut row = Chop::new(127);
        let mut col = Chop::new(7);
        for b in s.as_bytes().iter() {
            match b {
                b'F' => row.keep_left(),
                b'B' => row.keep_right(),
                b'L' => col.keep_left(),
                b'R' => col.keep_right(),
                _ => {/* ignore */}
            }
        }
        Seat { row: row.first(), col: col.last() }
    }
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

#[derive(Clone,Copy)]
struct Chop(usize, usize);

impl Chop {
    fn new(upper: usize) -> Chop {
        Chop(0, upper)
    }
    fn keep_left(&mut self) {
        self.1 = (self.0 + self.1) / 2
    }
    fn keep_right(&mut self) {
        self.0 = (self.0 + self.1 + 1) / 2
    }
    fn first(&self) -> usize {
        self.0
    }
    fn last(&self) -> usize {
        self.1
    }
}