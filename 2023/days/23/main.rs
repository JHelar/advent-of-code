use hashbrown::{HashMap, HashSet};
use std::cmp::Ordering;

type Map = Vec<Vec<Node>>;
type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum SlopeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Tile {
    Path,
    Forest,
    Slope(SlopeDirection),
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '#' => Tile::Forest,
            '.' => Tile::Path,
            '>' => Tile::Slope(SlopeDirection::Right),
            '<' => Tile::Slope(SlopeDirection::Left),
            '^' => Tile::Slope(SlopeDirection::Up),
            'v' => Tile::Slope(SlopeDirection::Down),
            _ => panic!("Unknown tile type {char}"),
        }
    }
}

#[derive(Debug)]
struct Node {
    tile: Tile,
    position: (usize, usize),
    g: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    position: Position,
    f: u32,
    steps: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(
    position: Position,
    from_direction: SlopeDirection,
    map: &Map,
    follow_slopes: bool,
) -> Vec<(Position, SlopeDirection)> {
    let mut positions = Vec::new();

    // Top
    if position.1 > 0 {
        let new_x = position.0;
        let new_y = position.1 - 1;
        positions.push(((new_x, new_y), SlopeDirection::Up));
    }
    // Bottom
    if position.1 < map.len() - 1 {
        let new_x = position.0;
        let new_y = position.1 + 1;

        positions.push(((new_x, new_y), SlopeDirection::Down));
    }
    // Left
    if position.0 > 0 {
        let new_x = position.0 - 1;
        let new_y = position.1;

        positions.push(((new_x, new_y), SlopeDirection::Left));
    }
    // Right
    if position.0 < map.len() - 1 {
        let new_x = position.0 + 1;
        let new_y = position.1;

        positions.push(((new_x, new_y), SlopeDirection::Right));
    }

    positions
        .into_iter()
        .filter(|(neighbour, to_direction)| {
            *neighbour != position
                && match (from_direction, to_direction) {
                    (SlopeDirection::Down, SlopeDirection::Up) => false,
                    (SlopeDirection::Up, SlopeDirection::Down) => false,
                    (SlopeDirection::Left, SlopeDirection::Right) => false,
                    (SlopeDirection::Right, SlopeDirection::Left) => false,
                    _ => true,
                }
                && match &map[neighbour.1][neighbour.0].tile {
                    Tile::Forest => false,
                    Tile::Path => true,
                    Tile::Slope(direction) => {
                        if follow_slopes {
                            direction == to_direction
                        } else {
                            true
                        }
                    }
                }
        })
        .map(|(neighbour, direction)| (neighbour, direction))
        .collect()
}

fn jump_position(
    position: Position,
    from_direction: SlopeDirection,
    steps: u32,
    map: &Map,
    follow_slopes: bool,
) -> (Position, SlopeDirection, u32) {
    let mut positions = Vec::new();

    // Top
    if position.1 > 0 {
        let new_x = position.0;
        let new_y = position.1 - 1;
        positions.push(((new_x, new_y), SlopeDirection::Up));
    }
    // Bottom
    if position.1 < map.len() - 1 {
        let new_x = position.0;
        let new_y = position.1 + 1;

        positions.push(((new_x, new_y), SlopeDirection::Down));
    }
    // Left
    if position.0 > 0 {
        let new_x = position.0 - 1;
        let new_y = position.1;

        positions.push(((new_x, new_y), SlopeDirection::Left));
    }
    // Right
    if position.0 < map.len() - 1 {
        let new_x = position.0 + 1;
        let new_y = position.1;

        positions.push(((new_x, new_y), SlopeDirection::Right));
    }
    positions = positions
        .into_iter()
        .filter(|(neighbour, to_direction)| {
            *neighbour != position
                && match (from_direction, to_direction) {
                    (SlopeDirection::Down, SlopeDirection::Up) => false,
                    (SlopeDirection::Up, SlopeDirection::Down) => false,
                    (SlopeDirection::Left, SlopeDirection::Right) => false,
                    (SlopeDirection::Right, SlopeDirection::Left) => false,
                    _ => true,
                }
                && match &map[neighbour.1][neighbour.0].tile {
                    Tile::Forest => false,
                    Tile::Path => true,
                    Tile::Slope(direction) => {
                        if follow_slopes {
                            direction == to_direction
                        } else {
                            true
                        }
                    }
                }
        })
        .collect();

    if positions.len() > 1 {
        return (position, from_direction, steps);
    } else if positions.len() == 1 && positions[0].0 == (map.len() - 2, map.len() - 1) {
        return (positions[0].0, positions[0].1, steps);
    } else if positions.len() == 1 {
        return jump_position(
            positions[0].0,
            positions[0].1,
            steps + 1,
            map,
            follow_slopes,
        );
    }
    panic!()
}
fn get_longest_path(
    start_position: Position,
    goal_position: Position,
    map: &Map,
    follow_slopes: bool,
) -> u32 {
    let start_state = jump_position(start_position, SlopeDirection::Down, 1, map, follow_slopes);

    let mut visit: Vec<((usize, usize), SlopeDirection, u32, HashSet<(usize, usize)>)> =
        vec![(start_state.0, start_state.1, start_state.2, HashSet::new())];
    let mut longest_path = 0;
    let mut jumped_map: HashMap<(Position, SlopeDirection), (Position, SlopeDirection, u32)> =
        HashMap::new();

    while let Some((position, direction, steps, mut visited)) = visit.pop() {
        if position == goal_position {
            if longest_path < steps {
                longest_path = steps;
            }
            continue;
        }

        if !visited.insert(position) {
            continue;
        }

        let neighbours = get_neighbours(position, direction, map, follow_slopes);

        for (neighbour, to_direction) in neighbours
            .iter()
            .filter(|(neighbour, _)| !visited.contains(neighbour))
        {
            let jumped_position = jumped_map
                .entry((*neighbour, *to_direction))
                .or_insert_with(|| jump_position(*neighbour, *to_direction, 1, map, follow_slopes));

            visit.push((
                jumped_position.0,
                jumped_position.1,
                steps + jumped_position.2,
                visited.clone(),
            ));
        }
    }

    longest_path
}

fn read_input() -> Map {
    let map: Map = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, char)| Node {
                    position: (x, y),
                    tile: Tile::from_char(char),
                    g: 0,
                })
                .collect()
        })
        .collect();
    map
}

fn print_map(map: &Map, path: &Vec<Position>) {
    for (y, row) in map.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("O");
            } else {
                match &node.tile {
                    Tile::Path => print!("."),
                    Tile::Forest => print!("#"),
                    Tile::Slope(direction) => match direction {
                        SlopeDirection::Up => print!("^"),
                        SlopeDirection::Down => print!("v"),
                        SlopeDirection::Left => print!("<"),
                        SlopeDirection::Right => print!(">"),
                    },
                }
            }
        }
        print!("\n");
    }
}

fn part1() -> Option<u32> {
    let map = read_input();
    let path = get_longest_path((1, 0), (map.len() - 1 - 1, map.len() - 1), &map, true);
    Some(path)
}

fn part2() -> Option<u32> {
    let map = read_input();
    let path = get_longest_path((1, 0), (map.len() - 1 - 1, map.len() - 1), &map, false);
    Some(path)
}

fn main() {
    println!("--- Day 23: A Long Walk ---");
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
