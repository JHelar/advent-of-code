use std::collections::HashMap;
use std::env;
use std::fs;

type Position = (i32, i32);

#[derive(Debug)]
enum NodeType {
    Start,
    End,
    Tile(u32),
}

impl NodeType {
    fn from_str(str: &str) -> NodeType {
        if str == "S" {
            NodeType::Start
        } else if str == "E" {
            NodeType::End
        } else {
            let elevation = str.chars().next().unwrap() as u32;
            NodeType::Tile(elevation)
        }
    }
}

#[derive(Debug)]
struct Node {
    elevation: u32,
    node_type: NodeType,
    position: Position,
    visited: bool,
    distance: i64,
    parent: Option<Position>,
}

impl Node {
    fn is_neighbour(self: &Node, position: Position, map: &HeightMap) -> bool {
        if let Some(neighbour) = map.get(&position) {
            return neighbour.elevation <= self.elevation + 1;
        }
        false
    }
}

type HeightMap = HashMap<Position, Node>;

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_map() -> HeightMap {
    let content = parse_input();
    let mut map: HeightMap = HashMap::new();

    content
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .for_each(|(y, row)| {
            row.trim()
                .split("")
                .filter(|x| !x.is_empty())
                .enumerate()
                .for_each(|(x, elevation)| {
                    let position = (x as i32, y as i32);
                    let node_type = NodeType::from_str(elevation);
                    let distance = if matches!(node_type, NodeType::Start) {
                        0
                    } else {
                        i32::MAX as i64
                    };
                    map.insert(
                        position,
                        Node {
                            elevation: if elevation == "S" {
                                'a' as u32
                            } else if elevation == "E" {
                                'z' as u32
                            } else {
                                elevation.chars().next().unwrap() as u32
                            },
                            node_type,
                            position,
                            distance,
                            visited: false,
                            parent: None,
                        },
                    );
                })
        });

    map
}

fn get_neighbours(node_position: Position, map: &HeightMap) -> Vec<Position> {
    let mut positions = Vec::new();
    let node = map.get(&node_position).unwrap();

    let up: Position = (node.position.0, node.position.1 - 1);
    if node.is_neighbour(up, map) {
        positions.push(up);
    }

    let down: Position = (node.position.0, node.position.1 + 1);
    if node.is_neighbour(down, map) {
        positions.push(down);
    }

    let left: Position = (node.position.0 - 1, node.position.1);
    if node.is_neighbour(left, map) {
        positions.push(left);
    }

    let right: Position = (node.position.0 + 1, node.position.1);
    if node.is_neighbour(right, map) {
        positions.push(right);
    }

    positions
}

fn find_path(map: &mut HeightMap, start_node_position: Position) -> Option<Position> {
    map.values_mut().for_each(|node| {
        node.distance = if node.position == start_node_position { 0 } else { i32::MAX as i64 };
        node.visited = false;
        node.parent = None
    });
    
    let mut unvisited: Vec<Position> = vec![start_node_position];

    while let Some(current_position) = unvisited.pop() {
        let current_node = map.get(&current_position).unwrap();
        if current_node.visited {
            continue;
        }
        let current_distance = current_node.distance;
        let neighbours = get_neighbours(current_position, map);

        for neighbour_position in neighbours {
            let neighbour_node = map.get_mut(&neighbour_position).unwrap();
            let neighbour_distance = current_distance + 1;

            if neighbour_node.distance >= neighbour_distance {
                neighbour_node.distance = neighbour_distance;
                neighbour_node.parent = Some(current_position);
            }

            if matches!(neighbour_node.node_type, NodeType::End) {
                return  Some(neighbour_position);
            }

            if neighbour_node.visited == false {
                unvisited.push(neighbour_position);
            }
        }
        map.get_mut(&current_position).unwrap().visited = true;
        unvisited.sort_by(|a, b| {
            let a_node = map.get(a).unwrap();
            let b_node = map.get(b).unwrap();

            return b_node.distance.cmp(&a_node.distance);
        })
    }
    None
}

fn part1() {
    let map = &mut parse_map();
    let start_node_position = map
        .values()
        .find(|node| matches!(node.node_type, NodeType::Start))
        .unwrap()
        .position;

    let end_pos = find_path(map, start_node_position).unwrap();
    let steps = map.get(&end_pos).unwrap().distance;

    println!("Steps: {}", steps);
}

fn part2() {
    let map = &mut parse_map();
    let elevation = 'a' as u32;

    let positions: Vec<Position> = map
        .values()
        .filter(|node| node.elevation == elevation)
        .map(|node| node.position).collect();

    let mut best = i32::MAX as i64;
    for start_node_position in positions {
        if let Some(end_pos) = find_path(map, start_node_position) {
            let steps = map.get(&end_pos).unwrap().distance;
            if steps < best {
                best = steps;
            }
        }
    }

    println!("Steps: {}", best);
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
