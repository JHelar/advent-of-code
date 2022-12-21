use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;

type Cave = HashMap<String, Valve>;

#[derive(Debug, Clone)]
struct Valve {
    links: Vec<String>,
    rate: i32,
    name: String,
}

impl Valve {
    fn from_str(str: &str) -> Valve {
        let mut str_split = str.split(';');
        let name_rate_str = str_split.next().unwrap();
        let links_str = str_split.next().unwrap();

        let name = name_rate_str[6..8].to_string();
        let rate = name_rate_str[23..].parse::<i32>().unwrap();

        let mut links_str_split = links_str.split(',').map(|x| x.trim());
        let first_link_string = links_str_split
            .next()
            .unwrap()
            .chars()
            .rev()
            .collect::<String>();
        let first_link = &first_link_string[0..2];
        let mut links: Vec<String> = vec![first_link.to_string()];

        links_str_split.for_each(|link| {
            links.push(link.to_string());
        });

        Valve { links, rate, name }
    }
}

struct Node<'a> {
    valve: &'a Valve,
    f: i32,
    g: i32,
    h: i32,
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_cave() -> Cave {
    let content = parse_input();
    let mut valves: Cave = Default::default();
    content.lines().map(Valve::from_str).for_each(|valve| {
        valves.insert(valve.name.clone(), valve);
    });
    valves
}

fn find_path(start_node: String, end_node: String, nodes: &mut HashMap<String, Node>) -> Option<Vec<String>> {
    let mut came_from: HashMap<String, String> = Default::default();
    let mut open: Vec<String> = vec![start_node.clone()];

    nodes.values_mut().for_each(|node| {
        node.f = 0;
        node.g = if node.valve.name == start_node {
            0
        } else {
            i32::MAX
        };
    });

    while let Some(current_node_name) = open.pop() {
        // Check end state
        if current_node_name == end_node {
            let mut path = vec![current_node_name.clone()];
            let mut current_key = current_node_name.clone();

            loop {
                if let Some(next_key) = came_from.get(&current_key) {
                    path.push(next_key.clone());
                    if *next_key == start_node {
                        break;
                    }
                    current_key = next_key.clone();
                } else {
                    break;
                }
            }
            return Some(path);
        }

        let neighbours = nodes.get(&current_node_name).unwrap().valve.links.iter();
        for neighbour in neighbours {
            let g_score = nodes.get(&current_node_name).unwrap().g + 1;
            if g_score < nodes.get(neighbour).unwrap().g {

                let neighbour_node = nodes.get_mut(neighbour).unwrap();
                neighbour_node.g = g_score;
                neighbour_node.f = g_score + neighbour_node.h;
                came_from.insert(neighbour.clone(), current_node_name.clone());

                if !open.contains(&neighbour) {
                    open.push(neighbour.clone());
                }
            }
        }
        open.sort_by(|a, b| nodes.get(b).unwrap().f.cmp(&nodes.get(a).unwrap().f));
    }
    None
}

fn find_best_path(from_node: String, map: &Cave) {
    let mut nodes: HashMap<String, Node> = HashMap::default();
    let mut goal_valves: Vec<String> = Vec::new();

    map.values().for_each(|valve| {
        nodes.insert(
            valve.name.clone(),
            Node {
                f: 0,
                g: if valve.name == from_node { 0 } else { i32::MAX },
                valve,
                h: valve.rate,
            },
        );

        if valve.rate > 0 {
            goal_valves.push(valve.name.clone());
        }
    });

    let goal_valve_combinations = goal_valves.iter().permutations(goal_valves.len()).unique();
    let mut start_valve = from_node.clone();

    for goal_valve_combination in goal_valve_combinations {
        let mut total_path: Vec<String> = vec![];
        for goal_valve in goal_valve_combination {
            if let Some(path) = find_path(start_valve.clone(), goal_valve.clone(), &mut nodes) {
                path.iter().for_each(|valve| total_path.push(valve.clone()));
                start_valve = goal_valve.clone();
            } else {
                println!("No path found between: {} => {}", start_valve, goal_valve);
                break;
            }
        }
        println!("Path: {:?}", total_path);
    }
}

fn part1() {
    let cave = parse_cave();
    let start_node = "AA".to_string();
    find_best_path(start_node, &cave)
    // println!("{:?}", result);
}

fn part2() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
