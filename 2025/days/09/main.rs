mod vector2;
use std::fmt::Display;

use hashbrown::{HashMap, HashSet};
use vector2::Vector2;

#[derive(Debug, Hash, PartialEq, Eq)]
enum TileType {
    Red,
    Green,
    Black,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Tile {
    position: Vector2,
    tile_type: TileType,
}

impl Tile {
    fn from_str(str: &str) -> Self {
        let (x, y) = str.split_once(",").unwrap();

        Self {
            position: Vector2::new(x.parse().unwrap(), y.parse().unwrap()),
            tile_type: TileType::Red,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tile_type {
            TileType::Red => write!(f, "O"),
            TileType::Green => write!(f, "X"),
            TileType::Black => write!(f, "."),
        }
    }
}

struct Theater {
    tiles: Vec<Tile>,
    edge_tiles: HashSet<Vector2>,
    fill_tiles: HashSet<Vector2>,
    square_cache: HashMap<(usize, usize), bool>,
    height: usize,
    width: usize,
}

impl Theater {
    fn new() -> Self {
        Self {
            tiles: Vec::default(),
            edge_tiles: HashSet::default(),
            fill_tiles: HashSet::default(),
            height: 0,
            width: 0,
            square_cache: HashMap::default(),
        }
    }

    fn add_tile(&mut self, tile: Tile) {
        self.width = self.width.max(tile.position.0 as usize);
        self.height = self.height.max(tile.position.1 as usize);
        self.tiles.push(tile);
    }

    fn connect_edges(&mut self) {
        let tile_length = self.tiles.len();
        for a_index in 0..tile_length {
            let b_index = (a_index + 1) % tile_length;

            let diff = self.tiles[a_index]
                .position
                .get_direction(&self.tiles[b_index].position);
            let direction = Vector2(
                if diff.0 == 0 {
                    0
                } else if diff.0 > 0 {
                    1
                } else {
                    -1
                },
                if diff.1 == 0 {
                    0
                } else if diff.1 > 0 {
                    1
                } else {
                    -1
                },
            );
            let mut new_tile_position = self.tiles[a_index].position.add(&direction);
            while new_tile_position != self.tiles[b_index].position {
                self.edge_tiles.insert(new_tile_position);
                new_tile_position = new_tile_position.add(&direction);
            }
            self.edge_tiles.insert(self.tiles[a_index].position);
            self.edge_tiles.insert(self.tiles[b_index].position);
        }
    }

    fn is_point_inside(&mut self, point: &Vector2) -> bool {
        if self.edge_tiles.contains(point) {
            return true;
        }
        if self.fill_tiles.contains(point) {
            return true;
        }

        let tile_length = self.tiles.len();
        let mut crossings = 0;
        for a_index in 0..tile_length {
            let b_index = (a_index + 1) % tile_length;

            let a_point = self.tiles[a_index].position;
            let b_point = self.tiles[b_index].position;

            if a_point.1.min(b_point.1) < point.1
                && point.1 <= a_point.1.max(b_point.1)
                && point.0 < a_point.0
            {
                crossings += 1;
            }
        }
        if crossings % 2 == 1 {
            self.fill_tiles.insert(*point);
            true
        } else {
            false
        }
    }

    fn get_largest_square(&mut self, inside: bool) -> (usize, usize, isize) {
        let mut best: Option<(usize, usize, isize)> = None;
        for a_index in 0..self.tiles.len() {
            for b_index in 0..self.tiles.len() {
                if a_index == b_index {
                    continue;
                }

                let size_vec = self.tiles[a_index]
                    .position
                    .sub(&self.tiles[b_index].position)
                    .abs()
                    .add(&Vector2(1, 1));
                let size = size_vec.0 * size_vec.1;
                if best.is_none() || size > best.unwrap().2 {
                    if inside {
                        if *self.square_cache.get(&(a_index, b_index)).unwrap_or(&false) {
                            best = Some((a_index, b_index, size))
                        } else {
                            let p1 = self.tiles[a_index].position;
                            let p2 = self.tiles[b_index].position;

                            let mut is_inside = true;
                            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                                let a = Vector2(p1.0, y);
                                let b = Vector2(p2.0, y);
                                if self.is_point_inside(&a) && self.is_point_inside(&b) {
                                    continue;
                                }
                                is_inside = false;
                                break;
                            }
                            if is_inside {
                                for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                                    let a = Vector2(x, p1.1);
                                    let b = Vector2(x, p2.1);
                                    if self.is_point_inside(&a) && self.is_point_inside(&b) {
                                        continue;
                                    }
                                    is_inside = false;
                                    break;
                                }
                            }
                            self.square_cache.insert((a_index, b_index), is_inside);
                            self.square_cache.insert((b_index, a_index), is_inside);
                            if is_inside {
                                best = Some((a_index, b_index, size))
                            }
                        }
                    } else {
                        best = Some((a_index, b_index, size))
                    }
                }
            }
        }

        best.expect("Should have a largest square")
    }
}

impl Display for Theater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width({}) height({})\n", self.width, self.height)?;
        for y in 0..=self.height {
            for x in 0..=self.width {
                if let Some(tile) = self
                    .tiles
                    .iter()
                    .find(|tile| tile.position.0 == x as isize && tile.position.1 == y as isize)
                {
                    write!(f, "{tile}")?;
                } else if let Some(_) = self
                    .edge_tiles
                    .iter()
                    .find(|tile| tile.0 == x as isize && tile.1 == y as isize)
                {
                    write!(f, "X")?;
                } else if let Some(_) = self
                    .fill_tiles
                    .iter()
                    .find(|tile| tile.0 == x as isize && tile.1 == y as isize)
                {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input() -> Theater {
    let mut theater = Theater::new();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| theater.add_tile(Tile::from_str(&line.trim())));

    theater.connect_edges();
    // theater.fill();
    theater
}

fn part1() -> Option<isize> {
    let mut theater = read_input();
    let square = theater.get_largest_square(false);
    Some(square.2)
}

fn part2() -> Option<isize> {
    let mut theater = read_input();
    let square = theater.get_largest_square(true);
    Some(square.2)
}

fn main() {
    println!("--- Day 9: Movie Theater ---");
    if let Some(part) = std::env::args().skip(1).next() {
        if let Some(result) = match part.as_str() {
            "1" => part1(),
            "2" => part2(),
            _ => panic!("💥 Invalid part number: {part}"),
        } {
            println!("🎁 Result part {part}: {result}");
        }
    } else {
        if let Some(result_1) = part1() {
            println!("🎁 Result part 1: {result_1}");
        }
        if let Some(result_2) = part2() {
            println!("🎁 Result part 2: {result_2}");
        }
    }
}
