use std::{borrow::BorrowMut, fmt::Display, sync::Arc};

use graph::{Edge, Graph, Node, NodeValue};
use hashbrown::{HashMap, HashSet};
use vector2::{Vector2, DIRECTIONS};

mod graph;
mod vector2;

fn read_input() -> impl Iterator<Item = Vector2> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let (x_str, y_str) = line.split_once(",").unwrap();
            Vector2(x_str.parse().unwrap(), y_str.parse().unwrap())
        })
}

impl NodeValue for Vector2 {
    fn to_name(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Byte(Vector2),
    Empty(Vector2),
    Path(Vector2),
}

impl NodeValue for Tile {
    fn to_name(&self) -> String {
        match self {
            Tile::Byte(position) => position.to_name(),
            Tile::Empty(position) => position.to_name(),
            Tile::Path(position) => position.to_name(),
        }
    }
}

struct Memory {
    graph: Graph<Tile>,
    size: Vector2,
}

impl Memory {
    fn new(size: Vector2) -> Self {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        for x in 0..size.0 {
            for y in 0..size.1 {
                let position = Vector2(x, y);
                nodes
                    .entry(position.to_name())
                    .or_insert(Node::new_ref(Tile::Empty(position)));

                for direction in DIRECTIONS.iter() {
                    let neighbor = position.add(direction);
                    if neighbor.0 < 0
                        || neighbor.0 >= size.0
                        || neighbor.1 < 0
                        || neighbor.1 >= size.1
                    {
                        continue;
                    }

                    nodes
                        .entry(neighbor.to_name())
                        .or_insert(Node::new_ref(Tile::Empty(neighbor)));

                    let neighbor_node = nodes.get(&neighbor.to_name()).unwrap();
                    let node = nodes.get(&position.to_name()).unwrap();

                    let edge = Edge::new(node, neighbor_node);
                    edges.push(edge);
                }
            }
        }

        Memory {
            graph: Graph::new(nodes, edges),
            size,
        }
    }

    fn insert_byte(&mut self, position: Vector2) {
        if let Some(node) = self.graph.nodes.get_mut(&position.to_name()) {
            let mut lock_node = node.lock().unwrap();
            lock_node.value = Tile::Byte(position)
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let position = Vector2(x, y);
                match self.graph.nodes.get(&position.to_name()) {
                    Some(node) => match node.lock().unwrap().value {
                        Tile::Byte(_) => write!(f, "#"),
                        Tile::Empty(_) => write!(f, "."),
                        Tile::Path(_) => write!(f, "O"),
                    },
                    _ => Ok(()),
                }?
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn part1() -> Option<String> {
    let mut memory = Memory::new(Vector2(71, 71));
    for byte in read_input().take(1024) {
        memory.insert_byte(byte);
    }

    if let Some(mut path) = graph::dijkstra::get_path(
        &Tile::Empty(Vector2(0, 0)),
        &Tile::Empty(Vector2(70, 70)),
        |edge| match edge.get_destination().value {
            Tile::Empty(_) => true,
            Tile::Byte(_) => false,
            Tile::Path(_) => false,
        },
        &memory.graph,
    ) {
        for node in path.iter_mut() {
            let mut lock_node = node.lock().unwrap();
            lock_node.value = Tile::Path(Vector2(0, 0))
        }
        println!("{memory}");
        Some((path.len() - 1).to_string())
    } else {
        None
    }
}

fn part2() -> Option<String> {
    let mut memory = Memory::new(Vector2(71, 71));
    for byte in read_input() {
        memory.insert_byte(byte);

        if graph::dijkstra::get_path(
            &Tile::Empty(Vector2(0, 0)),
            &Tile::Empty(Vector2(70, 70)),
            |edge| match edge.get_destination().value {
                Tile::Empty(_) => true,
                Tile::Byte(_) => false,
                Tile::Path(_) => false,
            },
            &memory.graph,
        )
        .is_none()
       { 
            return Some(byte.to_name());
        }
    }
    None
}

fn main() {
    println!("--- Day 18: RAM Run ---");
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
