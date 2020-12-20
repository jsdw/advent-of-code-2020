use structopt::StructOpt;
use shared::{ FileContentOpts, regex, unwrap_or };
use std::collections::{ HashMap, HashSet };
use std::iter::successors;

const TILE_SIZE: i32 = 10;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    // Put our puzzle pieces together:
    let mut tiles = parse_tiles(&opts.file);
    let mut tile_map = TileMap::new();
    tile_map.insert((0,0), tiles.pop().unwrap());
    while tiles.len() > 0 {
        // Look at each possible position we can place a tile..
        'inner: for xy in valid_next_positions(&tile_map).collect::<Vec<_>>() {
            // Take each tile..
            for tile_idx in 0..tiles.len() {
                // Look at each possible orientation of the tile..
                for tile in tiles[tile_idx].orientations() {
                    // If this rotation fits, insert it into the map,
                    // remove it from the list of tiles, and start over.
                    if tile_can_go_here(&tile, xy, &tile_map) {
                        tile_map.insert(xy, tile);
                        tiles.swap_remove(tile_idx);
                        break 'inner
                    }
                }
            }
        }
    }

    // Get the product of the map corners to solve part 1:
    let product: i64 = tile_map
        .keys()
        .fold([(0,0),(0,0),(0,0),(0,0)], |[tl,tr,bl,br], &xy| [
            (tl.0.min(xy.0),tl.1.min(xy.1)),
            (tr.0.max(xy.0),tr.1.min(xy.1)),
            (bl.0.min(xy.0),bl.1.max(xy.1)),
            (br.0.max(xy.0),br.1.max(xy.1))
        ])
        .iter()
        .filter_map(|xy| tile_map.get(xy))
        .map(|tile| tile.id as i64)
        .product();
    println!("Star 1: {}", product);

    // Find sea monsters, stopping once we find some.
    for mut map in merge_tile_map(&tile_map).orientations() {
        let monster_tails: Vec<(i32,i32)> = map.pixels
            .iter()
            .copied()
            .filter(|&px| sea_monster_at(px).all(|px| map.pixels.contains(&px)))
            .collect();
        if !monster_tails.is_empty() {
            for px in monster_tails.into_iter().flat_map(|xy| sea_monster_at(xy)) {
                map.remove(&px);
            }
            println!("Star 2: {}", map.len());
            break
        }
    }

    Ok(())
}

// Draw the sea monster starting from its tail end, at the coords provided.
fn sea_monster_at((x,y): (i32,i32)) -> impl Iterator<Item=(i32,i32)> {
    static COORDS: [(i32,i32);15] = [
                                                                                               (18,0),
      (0,1),                (5,1),(6,1),                 (11,1),(12,1),                 (17,1),(18,1),(19,1),
            (1,2),    (4,2),            (7,2),    (10,2),              (13,2),    (16,2)
    ];
    COORDS.iter().map(move |&(x2,y2)| (x+x2,y+y2-1))
}

fn merge_tile_map(tile_map: &TileMap) -> Pixels {
    // Squash the pixels of each tile into a single set, removing borders:
    let set: HashSet<_> = tile_map
        .iter()
        .flat_map(|((x,y),t)| {
            let size = t.pixels.size;
            t.pixels.iter()
                .filter(move |&&(px,py)| px > 0 && px < size-1 && py > 0 && py < size-1)
                .map(move |(px,py)| (px-1,py-1))
                .map(move |(px,py)| (x*(size-2)+px,y*(size-2)+py))
        })
        .collect();

    // find the bounds and thus size of the map:
    let (lowx, highx) = set.iter().fold((0,0), |(low,high),&(x,_)| (x.min(low), x.max(high)));
    let (lowy, highy) = set.iter().fold((0,0), |(low,high),&(_,y)| (y.min(low), y.max(high)));
    let size = (highx - lowx).max(highy - lowy);

    // Make sure the top left is 0,0 so that we can rotate etc sensibly:
    let pixels = set.into_iter().map(|(x,y)| (x-lowx,y-lowy)).collect();

    Pixels { size, pixels }
}

fn tile_can_go_here(tile: &Tile, xy: (i32,i32), tile_map: &TileMap) -> bool {
    surrounding(xy).filter(|xy2| tile_map.contains_key(xy2)).all(|xy2| {
        let tile_map_tile = tile_map.get(&xy2).expect("tile map tile");
        match (xy2.0-xy.0, xy2.1-xy.1) {
            // tile | tile_map_tile
            (1,0)  => tile.pixels.rights().zip(tile_map_tile.pixels.lefts()).all(|(p1,p2)| p1 == p2),
            // tile_map_tile | tile
            (-1,0) => tile.pixels.lefts().zip(tile_map_tile.pixels.rights()).all(|(p1,p2)| p1 == p2),
            // tile v tile_map_tile
            (0,1)  => tile.pixels.bottoms().zip(tile_map_tile.pixels.tops()).all(|(p1,p2)| p1 == p2),
            // tile_map_tile v tile
            (0,-1) => tile.pixels.tops().zip(tile_map_tile.pixels.bottoms()).all(|(p1,p2)| p1 == p2),
            // unreachable..
            _ => { unreachable!("coords should always be touching") }
        }
    })
}

fn valid_next_positions<'a>(tile_map: &'a TileMap) -> impl Iterator<Item = (i32,i32)> + 'a {
    tile_map.keys()
        .flat_map(|&xy| surrounding(xy))
        .filter(move |xy| !tile_map.contains_key(xy))
}

fn surrounding((x,y): (i32,i32)) -> impl Iterator<Item = (i32,i32)> {
    static OFFSETS: [(i32,i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];
    OFFSETS.iter().map(move |&(x1,y1)| (x-x1,y-y1))
}

type TileMap = HashMap<(i32,i32), Tile>;

#[derive(Debug,Clone,PartialEq)]
struct Tile {
    id: i32,
    pixels: Pixels
}

impl Tile {
    fn orientations(&self) -> impl Iterator<Item=Tile> {
        let id = self.id;
        self.pixels.orientations().map(move |pixels| Tile { id, pixels })
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Pixels {
    size: i32,
    pixels: HashSet<(i32,i32)>
}

impl Pixels {
    fn rotate_cw(&self) -> Pixels {
        Pixels { size: self.size, pixels: self.pixels.iter().map(|&(x,y)| (self.size-1-y, x)).collect() }
    }
    fn flip_horizontal(&self) -> Pixels {
        Pixels { size: self.size, pixels: self.pixels.iter().map(|&(x,y)| (self.size-1-x, y)).collect() }
    }
    fn orientations(&self) -> impl Iterator<Item=Pixels> {
        let rots = successors(Some(self.clone()), move |t| Some(t.rotate_cw())).take(4);
        let flip_rots = successors(Some(self.flip_horizontal()), move |t| Some(t.rotate_cw())).take(4);
        rots.chain(flip_rots)
    }
    fn bottoms(&self) -> impl Iterator<Item=bool> + '_ {
        (0..self.size).map(move |x| self.pixels.contains(&(x,self.size-1)))
    }
    fn tops(&self) -> impl Iterator<Item=bool> + '_ {
        (0..self.size).map(move |x| self.pixels.contains(&(x,0)))
    }
    fn lefts(&self) -> impl Iterator<Item=bool> + '_ {
        (0..self.size).map(move |y| self.pixels.contains(&(0,y)))
    }
    fn rights(&self) -> impl Iterator<Item=bool> + '_ {
        (0..self.size).map(move |y| self.pixels.contains(&(self.size-1,y)))
    }
}

impl std::ops::Deref for Pixels {
    type Target = HashSet<(i32,i32)>;
    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}
impl std::ops::DerefMut for Pixels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

fn parse_tiles(s: &str) -> Vec<Tile> {
    let mut tiles = vec![];
    for tile_str in s.split("\n\n") {
        let mut lines = tile_str.trim().lines().map(|l| l.trim());
        let id_line = unwrap_or!(lines.next(), continue);
        let id = unwrap_or!(parse_tile_id(id_line), continue);
        let pixels = parse_tile_pixels(lines);
        tiles.push(Tile { id, pixels: Pixels { size: TILE_SIZE, pixels } });
    }
    tiles
}

fn parse_tile_id(s: &str) -> Option<i32> {
    let s = s.trim();
    let n = regex!("Tile ([0-9]+):").captures(s)?[1].parse().ok()?;
    Some(n)
}

fn parse_tile_pixels<'a>(lines: impl Iterator<Item = &'a str>) -> HashSet<(i32,i32)> {
    let mut tile = HashSet::new();
    for (y,line) in lines.filter(|l| regex!("^[#.]+$").is_match(l)).enumerate() {
        for (x,char) in line.chars().enumerate() {
            if char == '#' {
                tile.insert((x as i32,y as i32));
            }
        }
    }
    tile
}
