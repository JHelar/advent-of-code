use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::fs;

type Position = (i32, i32);

enum NodeType {
    Start,
    End,
    Tile(u32)
}

impl NodeType {
    fn from_str(str: &str) -> NodeType {
        if str == "S" {
            NodeType::Start
        } else if str == "E" {
            NodeType::End
        } else {
            let elevation = str.chars().next().unwrap().to_digit(10).unwrap();
            NodeType::Tile(elevation)
        }
    }
}

struct Node {
    node_type: NodeType,
    position: Position,
    f: i32,
    g: i32,
    h: i32,
    parent: Option<Position>
}

impl Node {
    fn is_neighbour(self: &Node, position: Position, map: &HeightMap) -> bool {
        if let Some(neighbour) = map.get(&position) {
            return match neighbour.node_type {
                NodeType::Tile(elevation) => {
                    match self.node_type {
                        NodeType::Tile(node_elevation) => {
                            if node_elevation == elevation - 1 || node_elevation == elevation + 1 || node_elevation == elevation { true } else { false }
                        },
                        _ => true
                    }
                },
                _ => true
            }
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

    content.lines().enumerate().for_each(|(y, row)| row.split("").enumerate().for_each(|(x, elevation)| {
        let position = (x as i32, y as i32);
        map.insert(position, Node { node_type: NodeType::from_str(elevation), position, f: 0, g: 0, h: 0, parent: None });
    } ));

    map
}

fn get_neighbours(node_position: Position, map: &HeightMap) -> Vec<Position>{
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

fn find_path(map: &mut HeightMap) -> Option<Position> {
    let start_position = map.values().find(|node| matches!(node.node_type, NodeType::Start)).unwrap().position;
    let end_position = map.values().find(|node| matches!(node.node_type, NodeType::End)).unwrap().position;

    let mut open: Vec<(Position, i32)> = vec![(start_position, 0)];
    let mut closed: Vec<Position> = Vec::new();
    let mut visited: HashMap<Position, Vec<i32>> = Default::default();

    while open.len() > 0 {
        if let Some((node,_)) = open.pop() {
            let neighbours = get_neighbours(node, map);
            for neighbour in neighbours {
                let neighbour_node = map.get_mut(&neighbour).unwrap();
                neighbour_node.h = (neighbour_node.position.0 - end_position.0).abs() + (neighbour_node.position.1 - end_position.1).abs();
                neighbour_node.f = neighbour_node.h + neighbour_node.g;
                neighbour_node.parent = Some(node);

                if matches!(neighbour_node.node_type, NodeType::End) {
                    return Some(neighbour);
                }

                // Check if we have allready visited node from another position
                if visited.contains_key(&neighbour) {
                    let visited_fs = visited.get_mut(&neighbour).unwrap();
                    if visited_fs.iter().find(|f| f <= &&neighbour_node.f).is_some() {
                        continue;
                    } else {
                        visited_fs.push(neighbour_node.f);
                    }
                } else {
                    visited.insert(neighbour, vec![neighbour_node.f]);
                }
                open.push((neighbour, neighbour_node.f));
            }
            open.sort_by(|(_,a), (_, b)| a.cmp(b));
            closed.push(node);
        }
    }
    None
}

fn part1() {

}

fn part2() {

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
