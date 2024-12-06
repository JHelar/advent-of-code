mod vector2;
use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display};
use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Guard(Vector2),
    Air,
    Obstruction,
    Visited,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Air,
            '#' => Tile::Obstruction,
            '^' => Tile::Guard(UP),
            _ => panic!("Unknown tile '{c}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => write!(f, "."),
            Tile::Obstruction => write!(f, "#"),
            Tile::Guard(direction) => match *direction {
                UP => write!(f, "^"),
                DOWN => write!(f, "v"),
                LEFT => write!(f, "<"),
                RIGHT => write!(f, ">"),
                _ => panic!("Invalid direction"),
            },
            Tile::Visited => write!(f, "X"),
        }
    }
}

fn get_map_tile(point: &Vector2, map: &Map) -> Option<Tile> {
    if let Some(row) = map.get(point.1 as usize) {
        row.get(point.0 as usize).cloned()
    } else {
        None
    }
}

fn get_map_tile_mut<'a>(point: &Vector2, map: &'a mut Map) -> Option<&'a mut Tile> {
    if let Some(row) = map.get_mut(point.1 as usize) {
        row.get_mut(point.0 as usize)
    } else {
        None
    }
}

fn step(guard: &Vector2, map: &mut Map) -> Result<Vector2, ()> {
    let mut guard_position = *guard;
    loop {
        match get_map_tile(&guard_position, map) {
            Some(Tile::Guard(direction)) => {
                let new_position = guard_position.add(&direction);
                match get_map_tile(&new_position, map) {
                    Some(Tile::Air) | Some(Tile::Visited) => {
                        *get_map_tile_mut(&guard_position, map).unwrap() = Tile::Visited;
                        *get_map_tile_mut(&new_position, map).unwrap() = Tile::Guard(direction);

                        guard_position = new_position;
                    }
                    Some(Tile::Obstruction) => {
                        *get_map_tile_mut(&guard_position, map).unwrap() =
                            Tile::Guard(direction.rot_right_90());
                        return Ok(guard_position);
                    }
                    Some(_) => {
                        panic!("Invalid tile")
                    }
                    None => {
                        *get_map_tile_mut(&guard_position, map).unwrap() = Tile::Visited;
                        return Err(());
                    }
                }
            }
            _ => {
                panic!("Invalid guard tile")
            }
        }
    }
}

fn is_looped(guard: &Vector2, map: &mut Map) -> bool {
    let mut corner_mem = HashSet::new();
    let mut guard_position = *guard;

    while let Ok(new_position) = step(&guard_position, map) {
        guard_position = new_position;
        match get_map_tile(&guard_position, map).unwrap() {
            Tile::Guard(dir) => {
                if !corner_mem.insert((guard_position, dir)) {
                    return true;
                }
            }
            _ => panic!("Oh dear"),
        }
    }
    return false;
}

fn read_input() -> (Map, Vector2) {
    let mut map: Map = Vec::new();
    let mut guard = UP;

    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        let mut tile_row = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let tile = Tile::from_char(c);
            if matches!(tile, Tile::Guard(_)) {
                guard = Vector2(x as isize, y as isize)
            }
            tile_row.push(tile);
        }
        map.push(tile_row);
    }

    (map, guard)
}

fn print_map(map: &Map) {
    for row in map.iter() {
        for tile in row.iter() {
            print!("{tile}");
        }
        print!("\n")
    }
    print!("\n")
}

fn part1() -> Option<usize> {
    let (mut map, mut guard) = read_input();
    while let Ok(new_position) = step(&guard, &mut map) {
        guard = new_position;
    }
    let result = map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|tile| match tile {
                    Tile::Visited => true,
                    _ => false,
                })
                .count()
        })
        .sum();

    print_map(&map);

    Some(result)
}

fn part2() -> Option<usize> {
    let (map, guard) = read_input();
    let map_clone = map.clone();

    let candidates = map_clone.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, tile)| match tile {
                Tile::Air => Some(Vector2(x as isize, y as isize)),
                _ => None,
            })
    });

    let result = candidates
        .par_bridge()
        .filter(|position| {
            let mut map_clone = map.clone();
            *get_map_tile_mut(&position, &mut map_clone).unwrap() = Tile::Obstruction;
            is_looped(&guard, &mut map_clone)
        })
        .count();

    Some(result)
}

fn main() {
    println!("--- Day 6: Guard Gallivant ---");
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
