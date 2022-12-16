use std::collections::HashMap;
use std::env;
use std::fs;

type Cave = HashMap<String, Node>;

#[derive(Debug, Clone)]
struct Node {
    links: Vec<String>,
    rate: i32,
    name: String
}

impl Node {
    fn from_str(str: &str) -> Node {
        let mut str_split = str.split(';');
        let name_rate_str = str_split.next().unwrap();
        let links_str = str_split.next().unwrap();

        let name = name_rate_str[6..8].to_string();
        let rate = name_rate_str[23..].parse::<i32>().unwrap();

        let mut links_str_split = links_str.split(',').map(|x| x.trim());
        let first_link_string = links_str_split.next().unwrap().chars().rev().collect::<String>();
        let first_link = &first_link_string[0..2];
        let mut links: Vec<String> = vec![first_link.to_string()];
        
        links_str_split.for_each(|link| {
            links.push(link.to_string());
        });

        Node { links, rate, name }
    }
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_cave() -> Cave {
    let content = parse_input();
    let mut nodes: Cave = Default::default();
    content.lines().map(Node::from_str).for_each(|node| {
        nodes.insert(node.name.clone(), node);
    });
    nodes
}

fn find_best_path(from_node: String, prev_visited: Vec<String>, map: &Cave) -> i32 {
    let node = map.get(&from_node).unwrap();
    let mut visited = Vec::from_iter(prev_visited.iter());
    let mut best_rate = node.rate;

    visited.push(&from_node);

    for link in node.links.iter() {
        let link_rate = find_best_path(link.clone(), visited, map) + node.rate;
        best_rate = best_rate.max(link_rate);
    }

    best_rate
}

fn part1() {
    let cave = parse_cave();
    println!("{:?}", cave);
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
