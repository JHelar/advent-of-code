use hashbrown::{HashMap, HashSet};
use std::fmt::Display;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

type Position = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Plot,
    Rock,
    Visited,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Unknown tile {char}"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Plot => '.',
            Self::Rock => '#',
            Self::Start => 'S',
            Self::Visited => 'O',
        };

        write!(f, "{c}")
    }
}

fn read_input() -> (HashMap<Position, Tile>, i64, i64) {
    let mut max_x = 0;
    let mut max_y = 0;
    let map = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .flat_map(|(y, line)| {
            max_y = (y + 1) as i64;
            line.trim()
                .chars()
                .into_iter()
                .enumerate()
                .map(|(x, tile_char)| {
                    max_x = (x + 1) as i64;
                    ((x as i64, y as i64), Tile::from_char(tile_char))
                })
                .collect::<Vec<(Position, Tile)>>()
        })
        .collect();

    (map, max_x, max_y)
}

const DELTAS: [Position; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn visit(
    position: &Position,
    map: &mut HashMap<Position, Tile>,
    original_map: &HashMap<Position, Tile>,
    max_x: i64,
    max_y: i64,
) -> Vec<Position> {
    let mut visited = Vec::new();

    if let Some(tile) = map.get_mut(position) {
        *tile = Tile::Plot;
    }

    DELTAS.iter().for_each(|(dx, dy)| {
        let neighbour = (position.0 + dx, position.1 + dy);
        if map.get(&neighbour).is_none() {
            let map_delta_x = if neighbour.0 > -1 && neighbour.0 < max_x {
                0
            } else {
                ((neighbour.0.abs() as f64) / (max_x as f64)).ceil() as i64
                    * if neighbour.0 < 0 { -1 } else { 1 }
            };
            let map_delta_y = if neighbour.1 > -1 && neighbour.1 < max_y {
                0
            } else {
                ((neighbour.1.abs() as f64) / (max_y as f64)).ceil() as i64
                    * if neighbour.1 < 0 { -1 } else { 1 }
            };

            let new_min_x = map_delta_x * max_x;
            let new_min_y = map_delta_y * max_y;

            original_map.iter().for_each(|((x, y), tile)| {
                let position = (x + new_min_x, y + new_min_y);
                if matches!(tile, Tile::Start) {
                    map.insert(position, Tile::Plot);
                } else {
                    map.insert(position, tile.clone());
                }
            })
        }
        if let Some(tile) = map.get(&neighbour) {
            match tile {
                Tile::Rock => {}
                _ => {
                    *map.get_mut(&neighbour).unwrap() = Tile::Visited;
                    visited.push(neighbour);
                }
            }
        }
    });

    visited
}

fn print_map(map: &HashMap<Position, Tile>) {
    let min_x = *map.iter().map(|((x, _), _)| x).min().unwrap();
    let min_y = *map.iter().map(|((_, y), _)| y).min().unwrap();

    let max_x = *map.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *map.iter().map(|((_, y), _)| y).max().unwrap();

    print!("\n");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(tile) = map.get(&(x, y)) {
                print!("{tile}");
            } else {
                print!(" ");
            }
        }
        print!("\n")
    }
}

fn step(steps: i64, map: &HashMap<Position, Tile>, max_x: i64, max_y: i64) -> u64 {
    let mut mut_map = map.clone();
    let start_position = mut_map
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Start))
        .unwrap()
        .0
        .clone();
    let mut positions = vec![start_position];
    let original_map = mut_map.clone();
    
    (1..=steps).for_each(|_| {
        let mut new_positions = HashSet::new();
        while let Some(visit_position) = positions.pop() {
            let visited_positions = visit(
                &visit_position,
                &mut mut_map,
                &original_map,
                max_x,
                max_y,
            );
            visited_positions.into_iter().for_each(|position| {
                new_positions.insert(position);
            })
        }
        positions = new_positions.into_iter().collect();
    });
    // print_map(&mut_map);
    positions.len() as u64
}

fn part1() -> Option<u64> {
    let (map, max_x, max_y) = read_input();
    let result = step(64, &map, max_x, max_y);

    Some(result as u64)
}

fn part2() -> Option<u64> {
    let (map, max_x, max_y) = read_input();
    let max_steps = 26501365;
    
    let b = Array::from((0..3)
    .map(|i| step((i * max_x) + max_steps % max_x, &map, max_x, max_y) as f64)
    .collect::<Vec<f64>>());

    let a: Array2<f64> = array![[0., 0., 1.], [1., 1., 1.], [4., 2., 1.]];
    let x = a.solve_into(b).unwrap();
    let p = (max_steps / max_x) as f64;
    let y = array![p * p, p, 1.];
    let result = x.dot(&y);

    Some(result as u64)
}

fn main() {
    println!("--- Day 21: Step Counter ---");
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
