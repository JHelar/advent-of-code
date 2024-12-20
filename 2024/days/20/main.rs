use std::usize;

use hashbrown::HashMap;

use graph::{dijkstra, Edge, Graph, Node, NodeValue};
use vector2::{Vector2, DIRECTIONS, ZERO};

mod graph;
mod vector2;

#[derive(Debug, Clone)]
enum Tile {
    Wall(Vector2),
    Track(Vector2),
    Start(Vector2),
    Goal(Vector2),
}

impl Tile {
    fn get_position(&self) -> &Vector2 {
        match self {
            Tile::Wall(position) => position,
            Tile::Track(position) => position,
            Tile::Start(position) => position,
            Tile::Goal(position) => position,
        }
    }
}

impl NodeValue for Vector2 {
    fn to_name(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}

impl NodeValue for Tile {
    fn to_name(&self) -> String {
        self.get_position().to_name()
    }
}

struct RaceTrack {
    graph: Graph<Tile>,
    size: Vector2,
    start: Tile,
    goal: Tile,
}

impl RaceTrack {
    fn run(&mut self, cheat_duration: isize, min_time_reduction: isize) -> usize {
        let race_path = dijkstra::get_path(
            &self.start,
            &self.goal,
            |edge| match edge.get_destination().value {
                Tile::Wall(_) => false,
                _ => true,
            },
            &self.graph,
        )
        .expect("Should have a race path");

        let mut result = 0;
        for s_i in 0..race_path.len() {
            let (source_node, source_cost) = race_path[s_i].clone();
            let source = source_node.lock().unwrap().value.get_position().clone();
            for d_i in 0..race_path.len() {
                let (destination_node, destination_cost) = race_path[d_i].clone();
                let destination = destination_node
                    .lock()
                    .unwrap()
                    .value
                    .get_position()
                    .clone();

                let cheat_distance = source.manhattan_distance(&destination);

                // Calculate the cost reduction
                let saved = destination_cost as isize - source_cost as isize - cheat_distance;
                if cheat_distance <= cheat_duration && saved >= min_time_reduction {
                    result += 1;
                }
            }
        }
        result
    }
}

fn read_input() -> RaceTrack {
    let mut start = ZERO;
    let mut goal = ZERO;
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    let mut size = ZERO;

    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        size.0 = row.len() as isize;
        size.1 = row.len() as isize;

        for (x, tile_char) in row.chars().enumerate() {
            let position = Vector2::new(x, y);
            let tile = match tile_char {
                '#' => Tile::Wall(position.clone()),
                '.' => Tile::Track(position.clone()),
                'S' => {
                    start = position.clone();
                    Tile::Start(position.clone())
                }
                'E' => {
                    goal = position.clone();
                    Tile::Goal(position.clone())
                }
                _ => panic!("Unknown tile {tile_char}"),
            };

            let node = Node::new_ref(tile);
            nodes.insert(position.to_name(), node);
        }
    }

    for x in 0..size.0 {
        for y in 0..size.1 {
            let position = Vector2(x, y);
            let node = nodes.get(&position.to_name()).unwrap();
            for direction in DIRECTIONS.iter() {
                let neighbor = position.add(direction);
                if let Some(neighbor_node) = nodes.get(&neighbor.to_name()) {
                    let edge = Edge::new(node, neighbor_node);
                    edges.push(edge);
                }
            }
        }
    }

    RaceTrack {
        graph: Graph::new(nodes, edges),
        size,
        start: Tile::Start(start),
        goal: Tile::Goal(goal),
    }
}

fn part1() -> Option<usize> {
    let mut race = read_input();

    let result = race.run(2, 100);
    Some(result)
}

fn part2() -> Option<usize> {
    let mut race = read_input();

    let result = race.run(20, 100);
    Some(result)
}

fn main() {
    println!("--- Day 20: Race Condition ---");
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
