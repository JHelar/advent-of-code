mod graph;
use std::usize;

use graph::{count_paths_through_a_then_b, Edge, Graph, Node, NodeValue};
use hashbrown::HashMap;

use crate::graph::{bfs, count_paths};

#[derive(Debug)]
struct Device {
    name: String,
}

impl NodeValue for Device {
    fn to_name(&self) -> String {
        self.name.clone()
    }
}

fn read_input() -> Graph<Device> {
    let mut nodes = HashMap::default();
    let mut edges = Vec::default();

    for line in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
    {
        let (node_str, edge_str) = line.split_once(": ").unwrap();

        nodes
            .entry(node_str.to_string())
            .or_insert(Node::new_ref(Device {
                name: node_str.to_string(),
            }));

        for edge_node_str in edge_str.split_whitespace() {
            nodes
                .entry(edge_node_str.to_string())
                .or_insert(Node::new_ref(Device {
                    name: edge_node_str.to_string(),
                }));

            edges.push(Edge::new(
                nodes.get(&node_str.to_string()).unwrap(),
                nodes.get(&edge_node_str.to_string()).unwrap(),
            ));
        }
    }

    Graph::new(nodes, edges)
}

fn part1() -> Option<usize> {
    let graph = read_input();
    let paths = count_paths(
        &graph,
        &Device {
            name: "you".to_string(),
        },
        &Device {
            name: "out".to_string(),
        },
    );
    Some(paths as usize)
}

fn part2() -> Option<usize> {
    let graph = read_input();
    let count = count_paths_through_a_then_b(
        &graph,
        &Device {
            name: "svr".to_string(),
        },
        &Device {
            name: "out".to_string(),
        },
        &Device {
            name: "fft".to_string(),
        },
        &Device {
            name: "dac".to_string(),
        },
    );
    Some(count as usize)
}

fn main() {
    println!("--- Day 11: Reactor ---");
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
