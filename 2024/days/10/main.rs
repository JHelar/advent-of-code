use graph::{Edge, Graph, Node, NodeValue};
use hashbrown::HashMap;
use rayon::prelude::*;
use std::{sync::Arc, usize};
use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

mod graph;
mod vector2;

const DIRECTIONS: [Vector2; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Debug)]
struct CaveTile(Vector2, u8);

impl NodeValue for CaveTile {
    fn to_name(&self) -> String {
        format!("({},{})", self.0 .0, self.0 .1)
    }
}

impl NodeValue for Vector2 {
    fn to_name(&self) -> String {
        format!("({},{})", self.0, self.1)
    }
}

fn read_input() -> (Graph<CaveTile>, HashMap<u8, Vec<Arc<CaveTile>>>) {
    let mut nodes = HashMap::new();
    let mut tiles = HashMap::new();
    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        for (x, height) in row.chars().enumerate() {
            if height == '.' {
                continue;
            }

            let tile = Arc::new(CaveTile(
                Vector2(x as isize, y as isize),
                height.to_digit(10).unwrap() as u8,
            ));
            let node = Arc::new(Node::new(Arc::clone(&tile)));
            nodes.insert(tile.to_name(), node);

            let same_height = tiles.entry(tile.1).or_insert(Vec::new());
            same_height.push(tile);
        }
    }

    let mut edges = Vec::new();
    for node in nodes.values() {
        for direction in DIRECTIONS.iter() {
            let position = node.value.0.add(direction);
            match nodes.get(&position.to_name()) {
                Some(destination) if destination.value.1.abs_diff(node.value.1) == 1 => {
                    let edge = Edge::new(Arc::clone(node), Arc::clone(destination));
                    edges.push(edge);
                }
                _ => {}
            }
        }
    }

    (Graph::new(nodes, edges), tiles)
}

fn part1() -> Option<isize> {
    let (graph, tiles) = read_input();
    let mut trails = Vec::new();

    for start_tile in tiles.get(&0).unwrap().iter() {
        for end_tile in tiles.get(&9).unwrap().iter() {
            trails.push((start_tile.clone(), end_tile.clone()));
        }
    }

    let result = trails
        .par_iter()
        .filter(|(start, end)| {
            graph::dijkstra::get_path(
                start,
                end,
                &|edge: &Edge<CaveTile>| edge.origin.value.1 < edge.destination.value.1,
                &graph,
            )
            .is_some()
        })
        .count() as isize;

    Some(result)
}

fn part2() -> Option<isize> {
    let (graph, tiles) = read_input();
    let mut trails = Vec::new();

    for start_tile in tiles.get(&0).unwrap().iter() {
        for end_tile in tiles.get(&9).unwrap().iter() {
            trails.push((start_tile.clone(), end_tile.clone()));
        }
    }

    let result = trails
        .into_par_iter()
        .filter(|(start, end)| {
            graph::dijkstra::get_path(
                start,
                end,
                &|edge: &Edge<CaveTile>| edge.origin.value.1 < edge.destination.value.1,
                &graph,
            )
            .is_some()
        })
        .map(|(start, end)| {
            graph::bfs::get_paths(
                &start,
                &end,
                &|edge: &Edge<CaveTile>| edge.origin.value.1 < edge.destination.value.1,
                &graph,
            )
            .len()
        })
        .sum::<usize>() as isize;

    Some(result)
}

fn main() {
    println!("--- Day 10: Hoof It ---");
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
