use core::num;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

type Map = Vec<Vec<Tile>>;
type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Number(u8),
    Symbol(char),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            _ if c.is_numeric() => Tile::Number(c.to_string().parse().unwrap()),
            _ => Tile::Symbol(c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => ".".to_string(),
            Tile::Number(digit) => digit.to_string(),
            Tile::Symbol(symbol) => symbol.to_string(),
        };
        write!(f, "{c}")
    }
}

const DELTAS: [Position; 8] = [
    (1, 0),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn get_adjacent_tiles((x, y): Position, map: &Map) -> Vec<(Position, Tile)> {
    DELTAS
        .iter()
        .map(|(x_delta, y_delta)| {
            let new_x = x + x_delta;
            let new_y = y + y_delta;

            if new_x < 0 {
                return None;
            }

            if new_y < 0 {
                return None;
            }

            if new_y >= (map.len() as i32) {
                return None;
            }

            if let Some(tile) = map[new_y as usize].get(new_x as usize) {
                return Some(((new_x, new_y), *tile));
            }

            None
        })
        .filter(|tile| tile.is_some())
        .map(|tile| tile.unwrap())
        .collect()
}

fn read_input() -> Map {
    let mut map: Map = Vec::default();
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| {
            line.unwrap()
                .trim()
                .chars()
                .map(Tile::from_char)
                .collect::<Vec<Tile>>()
        })
        .for_each(|row| {
            map.push(row);
        });

    map
}

fn part1() -> Option<u32> {
    let map = read_input();
    let mut part_numbers = Vec::new();
    let mut part_number = String::new();
    let mut is_part = false;

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Number(digit) => {
                    part_number.push_str(digit.to_string().as_str());
                    if !is_part {
                        is_part = get_adjacent_tiles((x as i32, y as i32), &map).iter().any(
                            |(_, tile)| match tile {
                                Tile::Symbol(_) => true,
                                _ => false,
                            },
                        );
                    }
                }
                _ => {
                    if is_part {
                        part_numbers.push(part_number.clone().parse::<u32>().unwrap());
                        is_part = false;
                    }
                    part_number.clear();
                }
            }
        }
        if is_part {
            part_numbers.push(part_number.clone().parse::<u32>().unwrap());
            is_part = false;
        }
    }

    let result = part_numbers.iter().sum();
    println!("{}", part_numbers.len());
    Some(result)
}

fn part2() -> Option<u32> {
    let map = read_input();
    let mut part_numbers: HashMap<Position, u32> = HashMap::default();
    let mut part_number = String::new();
    let mut part_number_pos = Vec::new();
    let mut is_gear_part = false;
    let mut gears: HashMap<Position, HashSet<Position>> = HashMap::default();

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let tile_position = (x as i32, y as i32);
            match tile {
                Tile::Number(digit) => {
                    part_number.push_str(digit.to_string().as_str());
                    part_number_pos.push(tile_position);
                    if !is_gear_part {
                        is_gear_part = get_adjacent_tiles(tile_position, &map).iter().any(
                            |(position, tile)| match tile {
                                Tile::Symbol('*') => {
                                    if let Some(part_numbers) = gears.get_mut(position) {
                                        part_numbers.insert(tile_position);
                                    } else {
                                        gears.insert(*position, HashSet::from([tile_position]));
                                    }
                                    true
                                }
                                _ => false,
                            },
                        );
                    }
                }
                _ => {
                    if is_gear_part {
                        let part_no = part_number.clone().parse::<u32>().unwrap();
                        part_number_pos.iter().for_each(|pos| {
                            part_numbers.insert(*pos, part_no);
                        });
                    }
                    is_gear_part = false;
                    part_number_pos.clear();
                    part_number.clear();
                }
            }
        }
        if is_gear_part {
            let part_no = part_number.clone().parse::<u32>().unwrap();
            part_number_pos.iter().for_each(|pos| {
                part_numbers.insert(*pos, part_no);
            });
        }
        is_gear_part = false;
        part_number_pos.clear();
        part_number.clear();
    }

    let mut sum: u32 = 0;
    for (_, digit_position) in gears.iter() {
        if digit_position.len() > 2 || digit_position.len() < 2 {
            continue;
        }
        
        sum += digit_position
            .iter()
            .map(|digit_position| *part_numbers.get(digit_position).unwrap())
            .fold(1, |acc, num| acc * num);
    }

    Some(sum)
}

fn main() {
    println!("--- Day 3: Gear Ratios ---");
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
