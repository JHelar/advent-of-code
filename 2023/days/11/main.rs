use std::{
    collections::HashSet,
    fmt::Display,
};

use ati::At;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Map = Vec<Vec<Node>>;
type Position = (i64, i64);

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Galaxy(u16),
    Space,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            State::Galaxy(num) => num.to_string(),
            State::Space => ".".to_string(),
        };

        write!(f, "{c}")
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Node {
    x: i64,
    y: i64,
    state: State,
}

// fn print_map(map: &Map, path: &Vec<Node>) {
//     for (y, row) in map.iter().enumerate() {
//         for (x, node) in row.iter().enumerate() {
//             if path
//                 .iter()
//                 .any(|node| node.x == x as i64 && node.y == y as i64)
//             {
//                 print!("#");
//             } else {
//                 print!("{}", node.state);
//             }
//         }
//         print!("\n");
//     }
// }

fn read_input() -> (Map, Vec<(Node, Node)>, Vec<usize>, Vec<usize>) {
    let mut galaxy_num = 0;

    let lines = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    let mut portal_rows = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        if !line.chars().any(|c| c == '#') {
            portal_rows.push(y);
        }
    }
    let mut portal_columns = Vec::new();
    for x in 0..lines[0].len() {
        if !lines.iter().any(|line| {
            let c = line.clone().chars().nth(x).unwrap();
            c == '#'
        }) {
            portal_columns.push(x);
        }
    }

    let mut galaxies = Vec::new();

    let map: Map = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let node = Node {
                        x: x as i64,
                        y: y as i64,
                        state: {
                            match c {
                                '.' => State::Space,
                                '#' => {
                                    galaxy_num += 1;
                                    State::Galaxy(galaxy_num)
                                }
                                _ => panic!("No node state matched {c}"),
                            }
                        },
                    };
                    if c == '#' {
                        galaxies.push((x, y));
                    }
                    node
                })
                .collect::<Vec<Node>>()
        })
        .collect();

    let mut galaxy_pairs = Vec::new();
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let a = galaxies[i];
            let b = galaxies[j];

            let a_node = map[a.1][a.0];
            let b_node = map[b.1][b.0];

            galaxy_pairs.push((a_node, b_node));
        }
    }

    (map, galaxy_pairs, portal_rows, portal_columns)
}

const DELTAS: [Position; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_neighbours(from: Node, map: &Map) -> Vec<Node> {
    DELTAS
        .iter()
        .filter_map(|(dx, dy)| {
            let x = from.x + dx;
            let y = from.y + dy;

            if y >= 0 && y < map.len() as i64 && x >= 0 && x < map[0].len() as i64 {
                Some(*map.at(y).at(x))
            } else {
                None
            }
        })
        .collect::<Vec<Node>>()
}

fn get_distance(from: Node, to: Node) -> i64 {
    (from.x - to.x).abs() + (from.y - to.y).abs()
}

fn find_shortest_path(from: Node, to: Node, map: &Map) -> Option<Vec<Node>> {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    stack.push((from, vec![], get_distance(from, to)));

    while let Some((current, path, _)) = stack.pop() {
        if current == to {
            let mut new_path = path;
            new_path.push(current);
            return Some(new_path);
        }

        for neighbour in get_neighbours(current, map) {
            if visited.contains(&(neighbour.x, neighbour.y)) {
                continue;
            }

            stack.push((
                neighbour,
                {
                    let mut new_path = path.clone();
                    new_path.push(current);
                    new_path
                },
                get_distance(neighbour, to),
            ));
        }

        stack.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));
        visited.insert((current.x, current.y));
    }

    None
}

fn get_shortest_path_length(
    from: Node,
    to: Node,
    expansion_size: u64,
    map: &Map,
    portal_rows: Vec<usize>,
    portal_columns: Vec<usize>,
) -> u64 {
    let mut path_length = 0;
    let mut px = from.x;
    let mut py = from.y;
    if let Some(path) = find_shortest_path(from, to, &map) {
        for node in path.iter() {
            let dx = px - node.x;
            let dy = py - node.y;
            px = node.x;
            py = node.y;

            if dx != 0 && portal_columns.contains(&(node.x as usize)) {
                path_length += expansion_size
            } else if dy != 0 && portal_rows.contains(&(node.y as usize)) {
                path_length += expansion_size
            } else {
                path_length += 1;
            }
        }
        path_length - 1
    } else {
        panic!("No path found!")
    }
}

fn part1() -> Option<u64> {
    let (map, pairs, portal_rows, portal_columns) = read_input();
    let expansion_size = 2;
    let result = pairs
        .par_iter()
        .map(|(from, to)| {
            get_shortest_path_length(*from, *to, expansion_size, &map, portal_rows.clone(), portal_columns.clone())
        })
        .sum();

    Some(result)
}

fn part2() -> Option<u64> {
    let (map, pairs, portal_rows, portal_columns) = read_input();
    let expansion_size = 1000000;
    let result = pairs
        .par_iter()
        .map(|(from, to)| {
            get_shortest_path_length(*from, *to, expansion_size, &map, portal_rows.clone(), portal_columns.clone())
        })
        .sum();

    Some(result)
}

fn main() {
    println!("--- Day 11: Cosmic Expansion ---");
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
