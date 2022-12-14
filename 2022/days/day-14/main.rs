use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
enum Tile {
    Rock,
    Air,
    Sand,
    Void,
}

type Coord = (i32, i32);
struct CaveMap {
    map: HashMap<Coord, Tile>,
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

impl CaveMap {
    fn new() -> CaveMap {
        CaveMap {
            map: Default::default(),
            x_max: 0,
            y_max: 0,
            x_min: i32::MAX,
            y_min: i32::MAX,
        }
    }

    fn insert_rock_chain(self: &mut CaveMap, chain: Vec<Coord>) {
        let mut chain_iter = chain.iter().peekable();
        let mut x_max = 0;
        let mut y_max = 0;
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;

        while let Some(link) = chain_iter.next() {
            x_max = x_max.max(link.0);
            y_max = y_max.max(link.1);

            x_min = x_min.min(link.0);
            y_min = y_min.min(link.1);

            if let Some(next_link) = chain_iter.peek() {
                let x_start = link.0.min(next_link.0);
                let x_end = link.0.max(next_link.0);

                let y_start = link.1.min(next_link.1);
                let y_end = link.1.max(next_link.1);

                for x in x_start..=x_end {
                    for y in y_start..=y_end {
                        let coord = (x, y);
                        self.map.insert(coord, Tile::Rock);
                    }
                }
            }
        }

        self.x_max = self.x_max.max(x_max);
        self.y_max = self.y_max.max(y_max);

        self.x_min = self.x_min.min(x_min);
        self.y_min = self.y_min.min(y_min);
    }

    fn fill_air(self: &mut CaveMap) {
        for x in self.x_min..=self.x_max {
            for y in 0..=self.y_max {
                let coord = (x, y);
                if self.map.get(&coord).is_none() {
                    self.map.insert(coord, Tile::Air);
                }
            }
        }
    }

    fn set_floor(self: &mut CaveMap) {
        self.y_max = self.y_max + 2;
        self.y_min = 0;
        self.x_min = 0;
        self.x_max = self.x_max * 2;

        for x in self.x_min..=self.x_max {
            let coord = (x, self.y_max);
            self.map.insert(coord, Tile::Rock);
        }
    }

    fn get_tile(self: &CaveMap, coord: Coord) -> Tile {
        if let Some(tile) = self.map.get(&coord) {
            tile.clone()
        } else {
            Tile::Void
        }
    }

    fn drop_sand(self: &mut CaveMap, start_coord: Coord) -> bool {
        let mut tile_coord = start_coord;
        loop {
            let down = (tile_coord.0, tile_coord.1 + 1);
            let tile_down = self.get_tile(down);
            if matches!(tile_down, Tile::Air) {
                tile_coord = down;
                continue;
            }

            let down_left = (tile_coord.0 - 1, tile_coord.1 + 1);
            let tile_down_left = self.get_tile(down_left);
            if matches!(tile_down_left, Tile::Air) {
                tile_coord = down_left;
                continue;
            }

            let down_right = (tile_coord.0 + 1, tile_coord.1 + 1);
            let tile_down_right = self.get_tile(down_right);
            if matches!(tile_down_right, Tile::Air) {
                tile_coord = down_right;
                continue;
            }

            // Fell out
            if matches!(tile_down, Tile::Void)
                || matches!(tile_down_left, Tile::Void)
                || matches!(tile_down_right, Tile::Void)
            {
                return false;
            }
            break;
        }

        if matches!(self.get_tile(tile_coord), Tile::Air) {
            self.map.insert(tile_coord, Tile::Sand);
            return true;
        }
        return false;
    }

    fn print(self: &CaveMap) {
        let mut rows: Vec<String> = vec![];
        for y in 0..=self.y_max {
            let mut row: Vec<&str> = vec![];
            for x in 0..=self.x_max {
                if let Some(tile) = self.map.get(&(x, y)) {
                    match tile {
                        Tile::Void => {
                            row.push("");
                        }
                        Tile::Air => {
                            row.push("🌫");
                        }
                        Tile::Rock => {
                            row.push("🪨");
                        }
                        Tile::Sand => {
                            row.push("🥪");
                        }
                    }
                }
            }
            rows.push(row.join(""));
        }
        let content = rows.join("\n");
        fs::write("cave.txt", content).unwrap();
    }
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_cave_map() -> CaveMap {
    let mut map = CaveMap::new();
    let content = parse_input();

    content.lines().for_each(|line| {
        let rock_chain: Vec<Coord> = line
            .trim()
            .split(" -> ")
            .map(|link_str| {
                let mut link_iter = link_str.split(',');
                let x_coord = link_iter.next().unwrap().parse::<i32>().unwrap();
                let y_coord = link_iter.next().unwrap().parse::<i32>().unwrap();

                (x_coord, y_coord)
            })
            .collect();

        map.insert_rock_chain(rock_chain);
    });

    map
}

fn fill_sand(map: &mut CaveMap, start_sand_coord: Coord) -> i32 {
    let mut settles = 0;
    loop {
        let settled = map.drop_sand(start_sand_coord);
        if settled {
            settles += 1;
        } else {
            break;
        }
    }
    settles
}

fn part1() {
    let map = &mut parse_cave_map();
    map.fill_air();

    let sand_coord: Coord = (500, 0);
    let result = fill_sand(map, sand_coord);

    map.print();
    println!("Result: {}", result);
}

fn part2() {
    let map = &mut parse_cave_map();
    map.set_floor();
    map.fill_air();

    let sand_coord: Coord = (500, 0);
    let result = fill_sand(map, sand_coord);

    map.print();
    println!("Result: {}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
