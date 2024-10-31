use std::borrow::Borrow;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

type NumberIterator = dyn Iterator<Item = u32>;

impl Node {
    fn from_iter(numbers: &mut NumberIterator) -> Self {
        let mut head = numbers.take(2);
        let child_count = head.next().unwrap();
        let metadata_count = head.next().unwrap() as usize;

        let children: Vec<Node> = (0..child_count).map(|_| Node::from_iter(numbers)).collect();
        let metadata: Vec<u32> = numbers.take(metadata_count).collect();

        Self { children, metadata }
    }

    fn get_metadata_value(&self) -> u32 {
        let child_count: u32 = self
            .children
            .iter()
            .map(|child| child.get_metadata_value())
            .sum();
        child_count + self.metadata.iter().sum::<u32>()
    }

    fn get_value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum::<u32>()
        } else {
            self.metadata
                .iter()
                .map(|&child_index| {
                    if child_index == 0 {
                        0
                    } else if let Some(child) = self.children.get((child_index - 1) as usize) {
                        child.get_value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

fn read_input() -> Node {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);

    let numbers = line
        .split_ascii_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut numbers_iter = numbers.into_iter();

    Node::from_iter(&mut numbers_iter)
}

fn part1() -> Option<u32> {
    let root_node = read_input();
    let result = root_node.get_metadata_value();
    Some(result)
}

fn part2() -> Option<u32> {
    let root_node = read_input();
    let result = root_node.get_value();
    Some(result)
}

fn main() {
    println!("--- Day 8: Memory Maneuver ---");
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
