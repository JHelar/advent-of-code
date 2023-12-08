use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn from_str(line: &String) -> Self {
        let (id, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right.split_once(", ").unwrap();

        Self {
            id: id.to_string(),
            left: left[1..].to_string(),
            right: right[0..right.len() - 1].to_string(),
        }
    }
}

fn read_input() -> (Vec<char>, HashMap<String, Node>) {
    let lines = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let directions = lines.first().unwrap().clone().chars().collect();
    let nodes = lines
        .iter()
        .skip(1)
        .map(Node::from_str)
        .map(|node| (node.id.clone(), node))
        .collect();

    (directions, nodes)
}

fn gcd(one: u64, another: u64) -> u64 {
    let mut a = one;
    let mut b = another;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(one: u64, another: u64) -> u64 {
    let gcd = gcd(one, another);
    (one * another) / gcd
}

fn lcm_vec(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .map(|num| *num)
        .reduce(|one, another| lcm(one, another))
        .unwrap()
}

fn walk(
    from: &String,
    direction_index: u64,
    nodes: &HashMap<String, Node>,
    directions: &Vec<char>,
) -> String {
    let node = nodes.get(from).unwrap();
    let next_node = match directions[direction_index as usize % directions.len()] {
        'R' => &node.right,
        'L' => &node.left,
        _ => panic!("Should not be here"),
    };

    next_node.clone()
}

fn part1() -> Option<u64> {
    let (directions, nodes) = read_input();
    let mut from = "AAA".to_string();
    let destination = "ZZZ".to_string();
    let mut result = 0;

    for direction_index in 0.. {
        from = walk(&from, direction_index, &nodes, &directions);
        if from == destination {
            result = direction_index + 1;
            break;
        }
    }

    Some(result)
}

fn part2() -> Option<u64> {
    let (directions, nodes) = read_input();
    let mut ghost_cycles = nodes
        .iter()
        .filter(|(id, _)| id.chars().last().unwrap() == 'A')
        .map(|(id, _)| (id.clone(), 0_u64))
        .collect::<Vec<(String, u64)>>();

    let mut ghosts = ghost_cycles
        .iter()
        .map(|(id, _)| id.clone())
        .collect::<Vec<String>>();

    let mut ghosts_found_exit = 0;
    for direction_index in 0.. {
        for ghost_index in 0..ghosts.len() {
            if ghost_cycles[ghost_index].1 > 0 {
                continue;
            }

            let ghost = ghosts[ghost_index].clone();
            let next_node = walk(&ghost, direction_index, &nodes, &directions);
            if next_node.chars().last().unwrap() == 'Z' {
                ghost_cycles[ghost_index].1 = direction_index + 1;
                ghosts_found_exit += 1;
            }

            ghosts[ghost_index] = next_node;
        }

        if ghosts_found_exit == ghosts.len() {
            break;
        }
    }
    
    let cycles = ghost_cycles
        .iter()
        .map(|(_, cycle)| *cycle)
        .collect::<Vec<u64>>();

    let result = lcm_vec(cycles);
    Some(result)
}

fn main() {
    println!("--- Day 8: Haunted Wasteland ---");
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
