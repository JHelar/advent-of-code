use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use ati::At;

type Map = Vec<Vec<Pipe>>;
type Position = (i32, i32);

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
    Air,
    Animal,
    Undetermined,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Unknown pipe {c}"),
        }
    }
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::NE => '└',
            Pipe::NW => '┘',
            Pipe::SW => '┐',
            Pipe::SE => '┌',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
            Pipe::Air => ' ',
            Pipe::Animal => '@',
            Pipe::Undetermined => '?',
        };
        write!(f, "{c}")
    }
}

fn read_input() -> Map {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.trim()
                .to_string()
                .chars()
                .map(Pipe::from_char)
                .collect()
        })
        .collect()
}

fn print_map(map: &Map) {
    for row in map.iter() {
        for pipe in row.iter() {
            print!("{pipe}");
        }
        print!("\n");
    }
}

fn get_connecting_pipes((x, y): Position, map: &Map) -> Vec<Position> {
    let mut positions = Vec::new();
    let pipe = map.at(y).at(x);

    if y - 1 >= 0
        && (matches!(pipe, Pipe::NE)
            || matches!(pipe, Pipe::NW)
            || matches!(pipe, Pipe::Vertical)
            || matches!(pipe, Pipe::Start))
    {
        match map.at(y - 1).at(x) {
            Pipe::SE => positions.push((x, y - 1)),
            Pipe::SW => positions.push((x, y - 1)),
            Pipe::Vertical => positions.push((x, y - 1)),
            Pipe::Start => positions.push((x, y - 1)),
            _ => {}
        };
    }
    if y + 1 < map.len() as i32
        && (matches!(pipe, Pipe::SE)
            || matches!(pipe, Pipe::SW)
            || matches!(pipe, Pipe::Vertical)
            || matches!(pipe, Pipe::Start))
    {
        match map.at(y + 1).at(x) {
            Pipe::NE => positions.push((x, y + 1)),
            Pipe::NW => positions.push((x, y + 1)),
            Pipe::Vertical => positions.push((x, y + 1)),
            Pipe::Start => positions.push((x, y + 1)),
            _ => {}
        };
    }
    if x - 1 >= 0
        && (matches!(pipe, Pipe::SW)
            || matches!(pipe, Pipe::NW)
            || matches!(pipe, Pipe::Horizontal)
            || matches!(pipe, Pipe::Start))
    {
        match map.at(y).at(x - 1) {
            Pipe::NE => positions.push((x - 1, y)),
            Pipe::SE => positions.push((x - 1, y)),
            Pipe::Horizontal => positions.push((x - 1, y)),
            Pipe::Start => positions.push((x - 1, y)),
            _ => {}
        };
    }
    if x + 1 < map[0].len() as i32
        && (matches!(pipe, Pipe::SE)
            || matches!(pipe, Pipe::NE)
            || matches!(pipe, Pipe::Horizontal)
            || matches!(pipe, Pipe::Start))
    {
        match map.at(y).at(x + 1) {
            Pipe::NW => positions.push((x + 1, y)),
            Pipe::SW => positions.push((x + 1, y)),
            Pipe::Horizontal => positions.push((x + 1, y)),
            Pipe::Start => positions.push((x + 1, y)),
            _ => {}
        };
    }

    positions
}

fn find_loop(
    from: Position,
    destination: Position,
    map: &Map,
    visited: &mut HashSet<(Position, Position)>,
) -> Option<Vec<Position>> {
    let mut stack = VecDeque::new();
    stack.push_front((from, vec![]));

    while let Some((current, path)) = stack.pop_front() {
        if current == destination {
            let mut new_path = path;
            new_path.push(current);
            return Some(new_path);
        }

        let connecting_pipes = get_connecting_pipes(current, map)
            .iter()
            .filter(|pos| {
                !visited.contains(&(**pos, current)) && !visited.contains(&(current, **pos))
            })
            .map(|pos| *pos)
            .collect::<Vec<Position>>();

        for connecting_pipe in connecting_pipes {
            stack.push_front((connecting_pipe, {
                let mut new_path = path.clone();
                new_path.push(current);
                new_path
            }));
            visited.insert((current, connecting_pipe));
        }
    }

    None
}

fn search(map: &Map) -> Vec<Position> {
    let mut start = (0, 0);
    'outer: for (y, row) in map.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if matches!(pipe, Pipe::Start) {
                start = (x as i32, y as i32);
                break 'outer;
            }
        }
    }

    let connecting_pipes = get_connecting_pipes(start, &map);
    let mut visited = HashSet::new();
    for connecting_pipe in connecting_pipes {
        visited.insert((start, connecting_pipe));
        if let Some(path) = find_loop(connecting_pipe, start, map, &mut visited) {
            return path;
        }
    }

    panic!("No loop found!")
}

fn fill_undetermined(map: &mut Map, path: &Vec<Position>) {
    for (y, row) in map.iter_mut().enumerate() {
        for (x, pipe) in row.iter_mut().enumerate() {
            if !path.contains(&(x as i32, y as i32)) {
                *pipe = Pipe::Undetermined
            }
        }
    }
}

fn is_point_in_path(point: Position, path: &Vec<Position>) -> bool {
    let mut is_inside = false;

    let n = path.len();
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = path[i];
        let (xj, yj) = path[j];

        // Check if the ray from the test point intersects with the polygon edge
        if (yi > point.1) != (yj > point.1)
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi)
        {
            is_inside = !is_inside;
        }

        j = i;
    }

    is_inside
}

fn fill_air_or_animal(map: &mut Map, path: &Vec<Position>) {
    for (y, row) in map.iter_mut().enumerate() {
        for (x, pipe) in row.iter_mut().enumerate() {
            if !matches!(pipe, Pipe::Undetermined) {
                continue;
            }
            let is_inside = is_point_in_path((x as i32, y as i32), path);
            if is_inside {
                *pipe = Pipe::Animal;
            } else {
                *pipe = Pipe::Air;
            }
        }
    }
}

fn part1() -> Option<u32> {
    let map = read_input();
    let path = search(&map);
    let result = (path.len() as f32 / 2_f32).ceil() as u32;
    print_map(&map);
    Some(result)
}

fn part2() -> Option<u32> {
    let mut map = read_input();
    let path = search(&map);
    fill_undetermined(&mut map, &path);
    fill_air_or_animal(&mut map, &path);
    print_map(&map);

    let result = map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|pipe| matches!(pipe, Pipe::Animal))
                .count() as u32
        })
        .sum();
    Some(result)
}

fn main() {
    println!("--- Day 10: Pipe Maze ---");
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
